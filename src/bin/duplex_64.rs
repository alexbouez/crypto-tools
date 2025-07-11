#![warn(missing_docs)]
#![allow(non_snake_case)]
#![feature(rustdoc_missing_doc_code_examples)]

//! Duplex demo with 64-bit state.
//!
//! Demonstration for using the Duplex construction of [DM2019](https://doi.org/10.1007/978-3-030-34618-8_8),
//! with an inner state of 64 bits.

use std::io::Error;
use std::time::Instant;
use rand::{Rng, thread_rng};
use CryptoTools::other::duplex::Duplex;
use CryptoTools::utilities::bitops::urot;

/// Duplex demonstration with 64-bit state.
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();
    println!("# Duplex Demonstration, 64-bit state\n");

    // Define permutation
    let perm: fn(u64) -> u64 = move |value| urot::<u64>(value, 8);  // Example using a rotation as permutation

    // Define parameters
    let (b, r, k, u, alpha) = (64, 4, 4, 3, 17);
    let nb_rounds: usize = 3;
    let nb_calls: usize = 1024;
    let flag = true;

    // Setup
    let mut duplex = Duplex::new(vec!(b, r, k, u, alpha), perm)?;

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
