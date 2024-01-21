#![allow(non_snake_case)]

//! Crypto Tools - Hash - SipHash
//!
//! Module implementing the ARX hash functions SipHash and Half-SipHash.

// use std::io::Error;
use std::{ops::{BitXor, BitAnd, Not}, convert::From};
use crate::utilities::ustates::Ux4;

/// SipHash permutation for 4x64 states.
pub fn SipHash_general_permutation<U>(state: Ux4<U>, input1: U, input2: U, params: [U;5]) -> Ux4<U> 
    where U: Copy + From<u8>
{
    let (P0,P1,P2,P3) = state.get();
    let [a, b, c, d, e] = params;
    state
}