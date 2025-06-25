#![warn(missing_docs)]
#![allow(non_snake_case)]

//! SPRNG demo with 256-bit state.
//!
//! Demonstration for using the sponge-based PRNG of [GT2016](https://doi.org/10.1007/978-3-662-49890-3_4),
//! with an inner state of 64x4 bits.

use std::io::Error;
use std::time::Instant;

use CryptoTools::prng::{PRNG, sprng::SPRNG};
use CryptoTools::{utilities::ustates::Ux4, hash::siphash::SipHash_perm};

/// SPRNG demonstration with 256-bit state.
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();
    println!("# SPRNG Demonstration, 256-bit state\n");

    // Define permutation
    fn perm(state: Ux4::<u64>) -> Ux4::<u64> {SipHash_perm(&state)} // Example using the SipHash permutation [AB2012]

    // Define parameters
    let (n, r, t, s) = (256, 32, 1, 3);     // input size, rate, number of truncations, size of seed
    let nb_inputs: usize = 8;               // number of inputs for refresh
    let nb_next: usize = 128;               // number of calls to next per refresh

    // Setup
    let mut sprng = SPRNG::new(vec!(n, r, t, s), perm)?;

    for i in 0..8 {
        // Generate refresh inputs
        let mask = *sprng.mask();
        let mut inputs: Vec<Ux4::<u64>> = Vec::with_capacity(nb_inputs);
        for _ in 0..nb_inputs {
            inputs.push(Ux4::<u64>::rand() & mask);
        }

        // Refresh
        sprng.refresh(inputs)?;

        // Next & Print
        let mut R: Ux4::<u64>;
        print!("Output {}: ",i);
        for _ in 0..nb_next {
            R = sprng.next()?;
            print!("{:X}", R.0[0]);
        }
        println!("\n");
    }

    println!("-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
