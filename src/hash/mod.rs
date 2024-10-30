#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Crypto Tools - Hash
//!
//! This module groups all Hash functions.

/// Module implementing the ARX hash functions SipHash and Half-SipHash.
pub mod siphash;

// pub trait Hash<U, I, R>
// {
//     //! Allows general access to PRNG via functions `setup', `refresh', and `next'.

//     /// General `setup' function.
//     fn setup(params: Vec<usize>, func: fn(U)->U) -> Result<Self, Error> where Self: Sized;

//     /// General `hash' function.
//     fn next(&mut self) -> R;
// }
