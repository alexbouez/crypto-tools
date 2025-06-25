#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Module implementing additional bitwise operations.

use std::mem::size_of;
use std::ops::{BitOr, Shl, Shr, Sub};

use crate::utilities::ToLeBytes;

/// Unsigned integer rotation function
pub fn urot<U>(value: U, shift: usize) -> U
    where U: Copy + BitOr<Output = U> + Shl<usize, Output = U> +
        Shr<usize, Output = U> + Sub<Output = U> + Sized
{
    let size: usize = size_of::<U>()*8;
    (value << shift) | (value >> (size - shift))
}

/// Converts a slice of bits (0s and 1s) into a vector of bytes.
pub fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8).map(|chunk| {
        chunk.iter().enumerate().fold(0u8, |acc, (i, &bit)| {
            acc | (bit << i)  // LSB-first
        })
    }).collect()
}

// ToLeBytes trait for converting to little-endian byte representation.

impl ToLeBytes for u8 {
    fn to_le_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl ToLeBytes for u16 {
    fn to_le_bytes(&self) -> Vec<u8> {
        u16::to_le_bytes(*self).to_vec()
    }
}

impl ToLeBytes for u32 {
    fn to_le_bytes(&self) -> Vec<u8> {
        u32::to_le_bytes(*self).to_vec()
    }
}

impl ToLeBytes for u64 {
    fn to_le_bytes(&self) -> Vec<u8> {
        u64::to_le_bytes(*self).to_vec()
    }
}

impl ToLeBytes for u128 {
    fn to_le_bytes(&self) -> Vec<u8> {
        u128::to_le_bytes(*self).to_vec()
    }
}

// /// UNFINISHED.
// impl ToLeBytes for u256 {
//     fn to_le_bytes(&self) -> Vec<u8> {
//         let mut bytes = Vec::with_capacity(32);
//         for i in 0..32 {
//             bytes.push((self.0 >> (i * 8)) as u8);
//         }
//         bytes
//     }
// }

impl ToLeBytes for usize {
    fn to_le_bytes(&self) -> Vec<u8> {
        usize::to_le_bytes(*self).to_vec()
    }
}
