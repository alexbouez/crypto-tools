#![allow(non_snake_case)]

//! Crypto Tools - Hash - BitOps
//!
//! Module implementing bitwise operations.

// use std::io::Error;
use std::{ops::{BitOr, Shl, Shr}, convert::From};


// Unsigned integer rotation function
fn rotate_U<U>(value: U, shift: usize) -> U 
    where U: From<usize> + Copy + BitOr<Output = U> + Shl<Output = U> + Shr<Output = U>
{
    (value << shift.into()) | (value >> (64 - shift).into())
}

// let rot_17: fn(u64) -> u64 = move |value| rotate_64(value, 17);
