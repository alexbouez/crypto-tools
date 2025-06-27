#![warn(missing_docs)]
#![allow(non_snake_case)]
#![feature(rustdoc_missing_doc_code_examples)]

//! DSS demo with 256-bit state.
//!
//! Demonstration for using the DSS construction of [TODO],
//! with an inner state of 64x4 bits.

use std::io::Error;
use std::time::Instant;
use rand::{Rng, thread_rng};
use CryptoTools::stream::dss::DSS;
use CryptoTools::{utilities::ustates::Ux4, hash::siphash::SipHash_perm};

/// DSS demonstration with 256-bit state.
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();
    println!("# DSS Demonstration, 256-bit state\n");

    let mut rng = thread_rng();

    // Define permutation
    fn perm(mut state: Ux4<u64>) -> Ux4<u64> {
        const N_ROUNDS: usize = 4;
        for _ in 0..N_ROUNDS {state = SipHash_perm(&state);}
        state
    } // Example using the SipHash permutation

    // Setup
    let (b, r, k): (usize, usize, usize) = (256, 32, 32);
    let mut double_sponge = DSS::<Ux4::<u64>>::new(vec![b, r, k], perm)?;
    println!("DSS parameters: b = {}, r = {}, k = {}",
        double_sponge.b(), double_sponge.r(), double_sponge.k());

    // Generate key
    let key = rng.gen::<Ux4::<u64>>();
    println!("Key: {:X?}", key);
    double_sponge.rekey(key)?;

    // Initialize
    let nonce = rng.gen::<Ux4::<u64>>();
    println!("Nonce: {:X?}\n", nonce);
    double_sponge.init(nonce)?;

    // Produce stream
    let nb_rounds: usize = 4;
    for i in 0..nb_rounds {
        let output = double_sponge.next()?;
        println!("Round {}, Output: {:X?}", i, output);
    }

    // Encryption
    let plaintext = b"Hello, world!";
    let ciphertext = double_sponge.encrypt(key, nonce, plaintext)?;
    println!("\nPlaintext: {:?}", String::from_utf8(plaintext.to_vec()).unwrap());
    println!("Ciphertext: {:X?}", ciphertext);

    // Decryption
    let decrypted_text = double_sponge.encrypt(key, nonce, &ciphertext)?;
    println!("Decrypted: {:?}", String::from_utf8(decrypted_text).unwrap());

    println!("\n-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
