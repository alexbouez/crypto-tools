#![warn(missing_docs)]
#![allow(non_snake_case)]
#![feature(rustdoc_missing_doc_code_examples)]

//! Duplex Asakey with 256-bit state.
//!
//! Demonstration for using the Asakey construction of [DMP2022](https://doi.org/10.1145/3548606.3560635),
//! with an inner state of 64x4 bits.

use std::io::Error;
use std::time::Instant;
use rand::{Rng, thread_rng};
use CryptoTools::stream::asakey::Asakey;
use CryptoTools::{utilities::ustates::Ux4, hash::siphash::SipHash_perm};

/// Duplex demonstration with 256-bit state.
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();
    println!("# Asakey Demonstration, 256-bit state\n");

    let mut rng = thread_rng();

    // Define permutation
    fn perm(state: Ux4::<u64>) -> Ux4::<u64> {SipHash_perm(&state)} // Example using the SipHash permutation [AB2012]

    // Setup
    let (b, r, k): (usize, usize, usize) = (256, 30, 32);
    let mut asakey = Asakey::<Ux4::<u64>>::new(vec![b, r, k], perm)?;
    println!("Asakey parameters: b = {}, r = {}, k = {}", asakey.b(), asakey.r(), asakey.k());

    // Generate key
    let key = rng.gen::<Ux4::<u64>>();
    println!("Key: {:X?}", key);
    asakey.rekey(key)?;

    // Initialize
    let nonce = rng.gen::<Ux4::<u64>>();
    println!("Nonce: {:X?}\n", nonce);
    asakey.init(nonce)?;

    // Produce stream
    let nb_rounds: usize = 4;
    for i in 0..nb_rounds {
        let output = asakey.next()?;
        println!("Round {}, Output: {:X?}", i, output);
    }

    // Encryption
    let plaintext = b"Hello, world!";
    let ciphertext = asakey.encrypt(key, nonce, plaintext)?;
    println!("\nCiphertext: {:X?}", ciphertext);

    // Decryption
    let decrypted_text = asakey.encrypt(key, nonce, &ciphertext)?;
    println!("Decrypted text: {:?}", String::from_utf8(decrypted_text).unwrap());

    println!("\n-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
