#![warn(missing_docs)]
#![allow(non_snake_case)]
#![feature(rustdoc_missing_doc_code_examples)]

//! Crypto Tools - Demo SPRNG 256-bit
//!
//! Demonstration for using the sponge-based PRNG of Gazi and Tessaro (2016),
//! with inner state of 4x64 bits.

use std::io::Error;
use std::time::Instant;

// use CryptoTools::prng::{PRNG, gt2016::SPRG};
use CryptoTools::{utilities::ustates::Ux4, hash::siphash::SipHash_perm};

/// Main function. 
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();
    println!("SPRNG Demonstration\n");

    // Define permutation
    fn perm(state: Ux4::<u64>) -> Ux4::<u64> {
        let mut ret = state.clone();
        SipHash_perm(&mut ret);
        ret
    }

    // Setup
    // let mut sprg = SPRG::setup(vec!(256, 16, 2, 32), perm)?;

    // for i in 0..8 {
    //     // Refresh
    //     let mask = sprg.get_mask();
    //     let mut rng = thread_rng();
    //     let mut inputs: Vec<u64> = Vec::with_capacity(6_usize);
    //     for _ in 0..6 {
    //         inputs.push(rng.gen::<u64>() & mask);
    //     }
    //     sprg.refresh(inputs);

    //     // Next
    //     let mut R: u64;
    //     print!("Output {}:\t0x",i);
    //     for _ in 0..8 {
    //         R = sprg.next();
    //         print!("{:X}", R);
    //     }
    //     print!("\n");
    // }

    println!("\n-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
