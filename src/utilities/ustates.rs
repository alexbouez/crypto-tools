#![allow(non_snake_case)]

//! Crypto Tools - Utilities - U-States
//!
//! Module implementing multi-register states.

use std::{ops::{BitXor, BitAnd, Not}, convert::From};

#[derive(Clone, Debug)]
/// Structure for four-register states.
pub struct Ux4<U>([U; 4]);

impl<U> Ux4<U> 
    where U: From<u8> + Copy
{
    fn new() -> Self {
        Ux4([0_u8.into(); 4])
    }

    // Getter for the state values.
    pub fn get(&self) -> (U, U, U, U) {
        (self.0[0], self.0[1], self.0[2], self.0[3])
    }

    // Setter for the state values.
    pub fn set(&mut self, i0: U, i1: U, i2: U, i3: U) {
        *self = Ux4([i0, i1, i2, i3]);
    }
}

impl<U> From<u8> for Ux4<U> 
    where U: From<u8> + Copy
{
    fn from(item: u8) -> Self {
        let mut state = Ux4::<U>::new();
        state.0[0] = item.into();
        state
    }
}

impl<U> Not for Ux4<U> 
    where U: Not<Output = U> + Copy
{
    type Output = Self;
    fn not(self) -> Self::Output {
        Ux4([!self.0[0], !self.0[1], !self.0[2], !self.0[3]])
    }
}

impl<U> BitAnd for Ux4<U> 
    where U: BitAnd<Output = U> + Copy
{
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Ux4([self.0[0] & rhs.0[0], self.0[1] & rhs.0[1], 
            self.0[2] & rhs.0[2], self.0[3] & rhs.0[3]])
    }
}

impl<U> BitXor for Ux4<U>
    where U: BitXor<Output = U> + Copy
{
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Ux4([self.0[0] ^ rhs.0[0], self.0[1] ^ rhs.0[1], 
            self.0[2] ^ rhs.0[2], self.0[3] ^ rhs.0[3]])
    }
}
