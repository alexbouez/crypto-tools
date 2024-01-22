#![warn(missing_docs)]
#![allow(non_snake_case)]
#![feature(rustdoc_missing_doc_code_examples)]

//! Crypto Tools - Main function
//!
//! The sandbox area of the main function allows for direct use of the cryptographic tools. 

use std::io::Error;
use std::time::Instant;
use CryptoTools::utilities::ustates::Ux4;

/// Main function of the crate. 
/// Contains sandbox area, allows access to all tools.
fn main() -> Result<(), Error>{
    println!("\n################\n# Crypto Tools #\n################\n");
    let execution_start = Instant::now();

    // Sandox area
    // println!("Nothing to be done.");
    
    let a = Ux4::<u8>::new([255,0,0,0].into());
    let b = Ux4::<u8>::new([1,0,0,0].into());
    let c = Ux4::<u8>::new([0,1,0,0].into());
    
    let d = a.clone()+b;
    println!("a+b: {} {}", d.get()[0], d.get()[1]);
    println!("c: {} {}", c.get()[0], c.get()[1]);

    let e = c-a;
    println!("c-a: {} {}", e.get()[0], e.get()[1]);


    println!("\n-> Total execution time: {:.2?}", execution_start.elapsed());
    Ok(())
}
