#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Crypto Tools - Hash
//!
//! This module groups all Hash functions.
//! These are accessible through the Hash trait, which implements the `hash' function.

use std::io::Error;

/// Trait for Hash functions,
/// with public general-purpose functions `update' and `finalize'.
pub trait Digest
{
    /// Type of the output of the Hash function.
    type Output;

    /// Reset the Hash function to its initial state.
    fn reset(&mut self);

    /// Collect inputs and update the Hash function.
    fn update(&mut self, data: impl AsRef<[u8]>);

    /// Finalize the Hash function and return the output.
    fn finalize(&mut self) -> Result<Self::Output, Error>;
}

/// Module implementing SipHash [AB2012].
/// Public access to the SipHash permutation functions: SipHash_perm and Half_SipHash_perm.
pub mod siphash;