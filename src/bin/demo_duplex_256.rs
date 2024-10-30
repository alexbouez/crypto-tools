#![warn(missing_docs)]
#![allow(non_snake_case)]
#![feature(rustdoc_missing_doc_code_examples)]

//! Crypto Tools - Demo Duplex 256-bit
//!
//! Demonstration for using the Duplex construction of Dobraunig and Mennink (2019),
//! with inner state of 4x64 bits.

use std::io::Error;
use std::time::Instant;
use rand::{Rng, thread_rng};
use CryptoTools::construction::duplex::Duplex;
use CryptoTools::{utilities::ustates::Ux4, hash::siphash::SipHash_perm};

/// Main function.
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();
    println!("Duplex Demonstration\n");

    // Define permutation
    fn perm(state: Ux4::<u64>) -> Ux4::<u64> {
        let mut ret = state.clone();
        SipHash_perm(&mut ret);         // Example using SipHash as permutation
        ret
    }

    // Define parameters
    let (b, r, k, u, alpha) = (256, 32, 32, 3, 17);
    let nb_rounds: usize = 3;
    let nb_calls: usize = 128;
    let flag = true;

    // Setup
    let mut duplex = Duplex::setup(vec!(b, r, k, u, alpha), perm)?;

    let mut delta = 0;
    for i in 0..nb_rounds {
        // Reset
        duplex.reset(delta);

        // Next
        let mut input: Ux4::<u64>;
        let mut output: Ux4::<u64>;
        let mut rng = thread_rng();
        print!("Round {}: ", i);
        for _ in 0..nb_calls {
            input = rng.gen::<Ux4::<u64>>();
            output = duplex.duplex(flag, input);
            print!("{:X}", output.0[0]);
        }
        println!("\n");

        delta += 1;
    }

    println!("-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
