#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Crypto Tools - Demo SPRNG 64-bit
//!
//! Demonstration for using the sponge-based PRNG of Gazi and Tessaro [GT2016],
//! with inner state of 64 bits.

use std::io::Error;
use std::time::Instant;
use rand::{Rng, thread_rng};

use CryptoTools::prng::{PRNG, sprng::SPRNG};
use CryptoTools::utilities::bitops::urot;

/// Main function.
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();
    println!("SPRNG Demonstration\n");

    // Define permutation
    let rot_17: fn(u64) -> u64 = move |value| urot::<u64>(value, 17);   // Example using a rotation as permutation

    // Define parameters
    let (n, r, t, s) = (64, 4, 1, 3);   // input size, rate, number of truncations, size of seed
    let nb_inputs: usize = 8;           // number of inputs for refresh
    let nb_next: usize = 24;            // number of calls to next per refresh

    // Setup
    let mut sprng = SPRNG::new(vec!(n, r, t, s), rot_17)?;

    for i in 0..8 {
        // Generate refresh inputs
        let mask = sprng.get_mask();
        let mut rng = thread_rng();
        let mut inputs: Vec<u64> = Vec::with_capacity(nb_inputs);
        for _ in 0..nb_inputs {
            inputs.push(rng.gen::<u64>() & mask);
        }

        // Refresh
        sprng.refresh(inputs)?;

        // Next & Print
        let mut R: u64;
        print!("Output {}:\t0x",i);
        for _ in 0..nb_next {
            R = sprng.next()?;
            print!("{:X}", R);
        }
        println!("\n");
    }

    println!("-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
