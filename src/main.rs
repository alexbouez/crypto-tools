#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Main function of the Crypto Tools crate.
//!
//! The sandbox area of the main function allows for direct use of the cryptographic tools.

use std::io::Error;
use std::time::Instant;
// use CryptoTools::utilities::ustates::Ux4;

/// Main function - Sandbox area.
///
/// This is the main entry point of the Crypto Tools crate.
/// It currently contains a sandbox area from where you can access all the tools provided by the crate.
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();

    // Sandbox area
    println!("Nothing to be done.");

    println!("\n-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
