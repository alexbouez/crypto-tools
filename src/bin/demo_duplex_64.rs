#![warn(missing_docs)]
#![allow(non_snake_case)]
#![feature(rustdoc_missing_doc_code_examples)]

//! Crypto Tools - Demo Duplex 64-bit
//!
//! Demonstration for using the Duplex construction of Dobraunig and Mennink (2019),
//! with inner state of 64 bits.

use std::io::Error;
use std::time::Instant;
use rand::{Rng, thread_rng};
use CryptoTools::construction::duplex::Duplex;
use CryptoTools::utilities::bitops::urot;

/// Main function.
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();
    println!("Duplex Demonstration\n");

    // Define permutation
    let perm: fn(u64) -> u64 = move |value| urot::<u64>(value, 8);

    // Define parameters
    let (b, r, k, u, alpha) = (64, 4, 4, 3, 17);
    let nb_rounds: usize = 3;
    let nb_calls: usize = 1024;
    let flag = true;

    // Setup
    let mut duplex = Duplex::setup(vec!(b, r, k, u, alpha), perm)?;

    let mut delta = 0;
    for i in 0..nb_rounds {
        // Reset
        duplex.reset(delta);

        // Next
        let mut input: u64;
        let mut output: u64;
        let mut rng = thread_rng();
        print!("Round {}: ", i);
        for _ in 0..nb_calls {
            input = rng.gen::<u64>();
            output = duplex.duplex(flag, input);
            print!("{:X}", output);
        }
        println!("\n");

        delta += 1;
    }

    println!("-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
