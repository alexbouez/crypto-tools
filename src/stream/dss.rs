#![allow(non_snake_case)]

//! Module implementing the DSS construction.
//!
//! Based on [TODO].

use std::io::Error;
use getset::Getters;
use std::{ops::{BitXor, BitAnd, BitOr, Not, Add, Sub, Shl, Shr}, cmp::PartialEq, convert::From};
use std::fmt::{LowerHex, Debug};

use crate::utilities::{ToLeBytes, bitops::bits_to_bytes};

// DSS structure.

#[allow(dead_code)]
#[derive(Getters, Clone, Debug)]
/// Structure implementing [TODO].
///
/// Note that the state is reversed for easier use of the outputs.
/// The outer part is stored in the lower bits.
pub struct DSS<U>
where
    U: Clone,
{
    /// Size of the state in bits.
    #[getset(get = "pub")]
    b: usize,

    /// Size of the outer part (b = c + r).
    #[getset(get = "pub")]
    r: usize,

    /// Size of the inner part (b = c + r).
    #[getset(get = "pub")]
    c: usize,

    /// Size of the key (k <= b).
    #[getset(get = "pub")]
    k: usize,

    /// Key mask.
    kmask: U,

    /// Outer part (rate) mask.
    rmask:  U,

    /// Domain mask.
    dmask:  U,

    /// Permutation function.
    perm:  fn(U) -> U,

    /// Secret key.
    key: Option<U>,

    /// Inner state, top branch.
    state_up: Option<U>,

    /// Inner state, bottom branch.
    state_down: Option<U>,
}

impl<U> DSS<U>
where
    U: Copy + From<u8> + Not<Output = U> + Shl<usize, Output = U> + Shr<usize, Output = U> + Add<Output = U>
    + Sub<Output = U> + BitAnd<Output = U> + BitOr<Output = U> + BitXor<Output = U> + Shr<usize, Output = U>
    + LowerHex + Debug + ToLeBytes + PartialEq,
{
  /// Creates a new DSS instance.
  pub fn new(params: Vec<usize>, perm: fn(U) -> U) -> Result<Self, Error> {
    let (b, r, k) = (params[0], params[1], params[2]);
    assert!(b > k, "Invalid parameters: State size b must be greater than key size k");
    assert!(b > r, "Invalid parameters: State size b must be greater than rate r");
    let c = b - r; // Calculate the capacity size

    // Create the key mask (lower k bits)
    let mut kmask: U = 1_u8.into();
    kmask = (kmask << k) - 1_u8.into();

    // Create the outer part mask (lower r bits)
    let mut rmask: U = 1_u8.into();
    rmask = (rmask << r) - 1_u8.into();

    //Create the domain mask (highest bit)
    let mut dmask: U = 1_u8.into();
    dmask = dmask << (b - 1);

    Ok(Self {
        b,
        r,
        c,
        k,
        kmask,
        rmask,
        dmask,
        perm,
        key: None,
        state_up: None,
        state_down: None,
    })
  }

  /// Rekeys the DSS instance.
  ///
  /// This function reinitializes the state of the DSS instance using a new key.
  pub fn rekey(&mut self, key: U) -> Result<(), Error> {
    assert!(key != U::from(0_u8), "Key must be non-zero");

    self.key = Some(key);
    self.state_up = None;
    self.state_down = None;
    Ok(())
  }

  /// Initializes the DSS state.
  ///
  /// This function initializes the state of the DSS instance using a new nonce.
  pub fn init(&mut self, nonce: U) -> Result<(), Error> {
    assert!(nonce != U::from(0_u8), "Nonce must be non-zero");
    if self.key.is_none() {
      return Err(Error::new(std::io::ErrorKind::Other, "DSS key is not set"));
    }

    // Initialize the state with the key
    let mut state = (self.key.unwrap() & self.kmask) << (self.b - self.k);
    state = (self.perm)(state);

    // Absorb the nonce bit by bit
    for i in 0..self.k {
      let bit = (nonce >> i) & 1_u8.into();
      state = state ^ bit;
      state = (self.perm)(state);
    }

    // Set the k first bits to the nonce
    state = (nonce & self.kmask) | (state & !self.rmask);
    self.state_up = Some(state & !self.dmask); // domain bit is set to 0
    self.state_down = Some(state | self.dmask); // domain bit is set to 1

    Ok(())
  }

  /// Squeeze the DSS state.
  ///
  /// This function squeezes the DSS instance to produce a stream block of size `r`.
  pub fn next(&mut self) -> Result<Vec<u8>, Error> {
    if self.state_up.is_none() {
      return Err(Error::new(std::io::ErrorKind::Other, "DSS state is not initialized"));
    }

    // Permute the states
    let state_up = (self.perm)(self.state_up.unwrap());
    let state_down = (self.perm)(self.state_down.unwrap());

    // Extract the outer part (rate) from state_up (k_i)
    let sub_key = state_up & self.rmask;

    // Extract the outer part (rate) from state_down (S_i)
    let r_bits = (self.r + 7) / 8;
    let output = state_down & self.rmask;

    // Construct the new states
    let mixed_c = (state_down & !self.rmask) ^ (state_up & !self.rmask);

    let mixed_state = sub_key | mixed_c;
    self.state_up = Some(mixed_state & !self.dmask); // domain bit is set to 0
    self.state_down = Some(mixed_state | self.dmask); // domain bit is set to 1

    // Output the first r bits of state_down
    Ok(output.to_le_bytes()[..r_bits].into())
  }

  /// Collects `p` bytes of keystream bits.
  ///
  /// This function collects `p` bytes of keystream bits from the DSS instance,
  /// assuming the state is initialized. The output is a vector of bytes.
  /// The superfluous bits are discarded.
  fn next_p_bytes(&mut self, p: usize) -> Result<Vec<u8>, Error> {
    if self.state_up.is_none() {
      return Err(Error::new(std::io::ErrorKind::Other, "DSS state is not initialized"));
    }

    let mut keystream_bits: Vec<u8> = Vec::new();
    let p_bits = p*8;

    // Collect r bits of successive next() outputs until we have p bits
    let mut collected_bits = 0;
    while keystream_bits.len() < p_bits {
      let mut output = self.next()?;
      let mut output_index = 0;

      for byte in output.drain(..) {
        for i in 0..8 {
          if output_index < self.r && collected_bits < p_bits { // avoid padding from r & superfluous bits\
            // Collect the i-th bit of the byte
            // Note: The bits are collected in LSB-first order
            keystream_bits.push((byte >> i) & 1_u8);
            collected_bits += 1;
            output_index += 1;
          } else {
            break;
          }
        }
      }
    }

    Ok(bits_to_bytes(&keystream_bits))
  }

  /// Encrypt a plaintext using the DSS instance.
  ///
  /// This function encrypts the given plaintext using the DSS instance.
  /// It produces a ciphertext of the same length as the plaintext.
  pub fn encrypt(&mut self, key: U, nonce: U, input: impl AsRef<[u8]>) -> Result<Vec<u8>, Error>
  {
    // Rekey and initialize the DSS instance
    self.rekey(key)?;
    self.init(nonce)?;

    // Compute size of the plaintext in bits
    let plaintext = input.as_ref();
    let p = plaintext.len();

    // Collect p bytes of keystream
    let keystream_bytes = self.next_p_bytes(p)?;

    // XOR
    let ciphertext: Vec<u8> = plaintext
      .iter()
      .zip(keystream_bytes.iter())
      .map(|(p, k)| p ^ k)
      .collect();

    Ok(ciphertext)
  }
}