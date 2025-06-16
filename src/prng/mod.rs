#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Module implementing Pseudo Random Number Generators.
//!
//! This module groups all Pseudo Random Number Generators (PRNGs) implementations.
//! These are accessible through the PRNG trait, which implements the `refresh` and `next` functions.

use std::io::Error;

/// Trait for Pseudo Random Number Generators,
/// with public, general-purpose functions `refresh` and `next`.
pub trait PRNG
{
    /// PRNG input type.
    type Input;
    /// PRNG output type.
    type Output;

    /// General `refresh` function.
    /// Reseed the state of the PRNG using the given inputs.
    fn refresh(&mut self, inputs: Vec<Self::Input>) -> Result<(), Error>;

    /// General `next` function.
    /// Compute the next output of the PRNG.
    fn next(&mut self) -> Result<Self::Output, Error>;
}

pub mod sprng;