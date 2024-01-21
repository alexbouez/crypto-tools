#![allow(non_snake_case)]

//! Crypto Tools - Hash - BitOps
//!
//! Module implementing bitwise operations.

// use std::io::Error;
use std::{ops::{BitOr, Shl, Shr, Sub}, convert::From};


// Unsigned integer rotation function
// Example of use: 
// ```
//      let rot_17: fn(u64) -> u64 = move |value| rotate_64(value, 17, 64);
// ```
pub fn rotate_U<U>(value: U, shift: U, size: U) -> U 
    where U: Copy + BitOr<Output = U> + Shl<Output = U> +
        Shr<Output = U> + Sub<Output = U>
{
    (value << shift) | (value >> (size - shift))
}
