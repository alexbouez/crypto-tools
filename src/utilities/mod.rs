#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Module implementing various utilities.

/// Trait for converting types to little-endian byte representation.
///
/// This trait provides a method `to_le_bytes` that converts the implementing type
/// into a vector of bytes in little-endian order.
/// This approach generalizes the to_le_bytes method already present for u16, u32, u64, u128, and usize types.
pub trait ToLeBytes {
    /// Converts the implementing type to a vector of bytes in little-endian order.
    fn to_le_bytes(&self) -> Vec<u8>;
}

pub mod bitops;
pub mod ustates;
