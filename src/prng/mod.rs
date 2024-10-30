#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Crypto Tools - PRNG
//!
//! This module groups all Pseudo Random Number Generators.
//! These are accessible through the PRNG trait,
//! which implements the `new', `refresh' and `next' functions.

use std::io::Error;

/// Module implementing the sponge-based PRNG of Gazi and Tessaro 2016.
pub mod gt2016;

pub trait PRNG
{
    //! Allows general access to PRNG via functions `setup', `refresh', and `next'.

    /// Type of the state of the PRNG.
    type State;
    /// Type of the input to the PRNG.
    type Input;
    /// Type of the output of the PRNG.
    type Output;

    /// General `setup' function.
    /// Initializes the PRNG with the given parameters and permutation function.
    fn setup(params: Vec<usize>, func: fn(Self::State)->Self::State) -> Result<Self, Error> where Self: Sized;

    /// General `refresh' function.
    /// Reinitialize or refresh the state of the PRNG using the given inputs.
    fn refresh(&mut self, inputs: Vec<Self::Input>) -> Result<(), Error>;

    /// General `next' function.
    /// Generate the next output of the PRNG.
    fn next(&mut self) -> Result<Self::Output, Error>;
}
