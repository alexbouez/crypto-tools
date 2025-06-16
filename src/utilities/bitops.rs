#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Module implementing additional bitwise operations.

use std::mem::size_of;
use std::ops::{BitOr, Shl, Shr, Sub};

/// Unsigned integer rotation function
pub fn urot<U>(value: U, shift: usize) -> U
    where U: Copy + BitOr<Output = U> + Shl<usize, Output = U> +
        Shr<usize, Output = U> + Sub<Output = U> + Sized
{
    let size: usize = size_of::<U>()*8;
    (value << shift) | (value >> (size - shift))
}
