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

pub trait PRNG<U, I, R>
{
    //! Allows general access to PRNG via functions `setup', `refresh', and `next'.

    /// General `setup' function.
    fn setup(params: Vec<usize>, func: fn(U)->U) -> Result<Self, Error> where Self: Sized;

    /// General `refresh' function.
    fn refresh(&mut self, inputs: I);

    /// General `next' function.
    fn next(&mut self) -> R;
}
