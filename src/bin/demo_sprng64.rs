#![warn(missing_docs)]
#![allow(non_snake_case)]
#![feature(rustdoc_missing_doc_code_examples)]

//! Crypto Tools - Demo SPRNG 64-bit
//!
//! Demonstration for using the sponge-based PRNG of Gazi and Tessaro (2016),
//! with inner state of 64 bits.

use std::io::Error;
use std::time::Instant;
use rand::{Rng, thread_rng};

use CryptoTools::prng::{PRNG, gt2016::SPRG};
use CryptoTools::utilities::bitops::urot;

/// Main function. 
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();
    println!("SPRNG Demonstration\n");

    // Define permutation
    let rot_17: fn(u64) -> u64 = move |value| urot::<u64>(value, 17);

    // Define parameters 
    let (n, r, t, s) = (64, 4, 2, 16);

    // Setup
    let mut sprg = SPRG::setup(vec!(n, r, t, s), rot_17)?;

    // print!("Seed: \t\t0x");
    // for i in 1..s {print!("{:X}", seed[i-1]);}
    // print!("\n");

    for i in 0..8 {
        // Generate refresh inputs
        let mask = sprg.get_mask();
        let mut rng = thread_rng();
        let mut inputs: Vec<u64> = Vec::with_capacity(6_usize);
        for _ in 0..6 {
            inputs.push(rng.gen::<u64>() & mask);
        }

        // Refresh
        sprg.refresh(inputs);

        // Next
        let mut R: u64;
        print!("Output {}:\t0x",i);
        for _ in 0..8 {
            R = sprg.next();
            print!("{:X}", R);
        }
        print!("\n");
    }

    println!("\n-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
