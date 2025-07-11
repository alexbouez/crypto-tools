#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Module implementing SipHash and Half-SipHash.
//!
//! Implements the ARX hash function SipHash from [AB2012](https://doi.org/10.1007/978-3-642-34931-7_28),
//! the SipHash permutation for 64-bit states, and the Half-SipHash permutation variant for 32-bit states
//! as found in [LINUXv6.11](https://elixir.bootlin.com/linux/v6.11.5/source/include/linux/siphash.h#L157).
//! Both permutation implementations are public.

use std::io::Error;
use std::num::Wrapping;
use getset::Getters;
use std::ops::{BitOr, BitXor, Add, Sub, Shl, Shr};

use crate::utilities::{ustates::Ux4, bitops::urot};
use crate::hash::Digest;

// SipHash structure.

#[allow(dead_code)]
#[derive(Getters, Clone, Debug)]
/// Structure implementing [SipHash](https://doi.org/10.1007/978-3-642-34931-7_28)
pub struct SipHash<U>
where
    U: Clone
{
    /// Size of the state in bits.
    #[getset(get = "pub")]
    n: usize,

    /// Number of compression rounds.
    #[getset(get = "pub")]
    c: usize,

    /// Number of finalization rounds.
    #[getset(get = "pub")]
    d: usize,

    /// Inner state.
    state: Ux4<U>,

    /// Permutation function.
    perm: fn(Ux4<U>) -> Ux4<U>,

    /// Input buffer.
    inputs: Vec<u8>
}

impl<U> Default for SipHash<U>
where
    U: Clone + Default,
{
    /// Default constructor for SipHash.
    fn default() -> Self {
        pub fn identity_fn<U>(state: Ux4<U>) -> Ux4<U> {state}
        SipHash {
            n: 256_usize,
            c: 2,
            d: 4,
            state: Ux4::default(),
            perm: identity_fn::<U>,
            inputs: Vec::new(),
        }
    }
}

// SipHash functions.

/// General SipHash permutation function for Ux4 states.
fn SipRound<U>(state: &Ux4<U>, params: [usize;5]) -> Ux4<U>
where 
    U: Copy + From<u8> + BitOr<Output = U> + BitXor<Output = U>
    + Sub<Output = U> + Add<Output = U> + Shl<usize, Output = U>
    + Shr<usize, Output = U> + std::fmt::UpperHex,
    Wrapping<U>: Add<Output = Wrapping<U>>
{
    let [mut p0, mut p1, mut p2, mut p3] = state.get();
    let [a, b, c, d, e] = params;

    // Wrapping is used to avoid overflow checks
    p0 = (Wrapping(p0) + Wrapping(p1)).0;
    p1 = urot::<U>(p1, a) ^ p0;
    p0 = urot::<U>(p0, e);
    p2 = (Wrapping(p2) + Wrapping(p3)).0;
    p3 = urot::<U>(p3, b) ^ p2;
    p0 = (Wrapping(p0) + Wrapping(p3)).0;
    p3 = urot::<U>(p3, d) ^ p0;
    p2 = (Wrapping(p2) + Wrapping(p1)).0;
    p1 = urot::<U>(p1, c) ^ p2;
    p2 = urot::<U>(p2, e);

    Ux4::<U>::new([p0, p1, p2, p3])
}

/// SipHash permutation function for u32x4 states.
pub fn Half_SipHash_perm(state: &Ux4::<u32>) -> Ux4::<u32> {
    SipRound(state, [5, 8, 7, 13, 16])
}

/// SipHash permutation function for u64x4 states.
pub fn SipHash_perm(state: &Ux4::<u64>) -> Ux4::<u64> {
    SipRound(state, [13, 16, 17, 21, 32])
}

// SipHash for u32x4 states.

impl SipHash<u32>
{
    /// [DEPRECATED: Half-SipHash is not a fully instantiated hash function.]
    /// Setup function for Half-SipHash with u32x4 state.
    /// The initial state is set using the constants from the Linux kernel [LINUXv6.11].
    pub fn new(params: Vec<usize>) -> Result<Self, Error> {
        assert!(params.len() == 2, "SipHash Setup: wrong number of parameters. Expected 2, got {}.", params.len());
        let (c, d) = (params[0], params[1]);
        assert!(c > 0, "SipHash Setup: number of compression rounds c must be greater than 0.");
        assert!(d > 0, "SipHash Setup: number of finalization rounds d must be greater than 0.");

        fn perm (state: Ux4::<u32>) -> Ux4::<u32> {Half_SipHash_perm(&state)}
        let state = Ux4::<u32>::new([0x0, 0x0, 0x6c796765, 0x74656462]);

        Ok(Self{
            n: 32_usize,
            c: c,
            d: d,
            state: state,
            perm: perm,
            inputs: Vec::new()
        })
    }
}

// SipHash for u64x4 states.

impl SipHash<u64>
{
    /// Setup function for SipHash with u64x4 state.
    /// The initial state is set using the constants from [AB2012](https://doi.org/10.1007/978-3-642-34931-7_28).
    pub fn new(params: Vec<usize>) -> Result<Self, Error> {
        assert!(params.len() == 2, "SipHash Setup: wrong number of parameters. Expected 2, got {}.", params.len());
        let (c, d) = (params[0], params[1]);
        assert!(c > 0, "SipHash Setup: number of compression rounds c must be greater than 0.");
        assert!(d > 0, "SipHash Setup: number of finalization rounds d must be greater than 0.");

        fn perm (state: Ux4::<u64>) -> Ux4::<u64> {SipHash_perm(&state)}
        let state = Ux4::<u64>::new(
            [0x736f6d6570736575, 0x646f72616e646f6d,
            0x6c7967656e657261, 0x7465646279746573]
        );

        Ok(Self{
            n: 64_usize,
            c: c,
            d: d,
            state: state,
            perm: perm,
            inputs: Vec::new()
        })
    }
}
impl Digest for SipHash<u64>
{
    type Output = u64;

    fn reset(&mut self) {
        self.state.set(
            [0x736f6d6570736575, 0x646f72616e646f6d,
            0x6c7967656e657261, 0x7465646279746573]
        );
        self.inputs.clear();
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        self.inputs.extend(data.as_ref());
    }

    fn finalize(&mut self) -> Result<Self::Output, Error> {
        todo!();
    }
}