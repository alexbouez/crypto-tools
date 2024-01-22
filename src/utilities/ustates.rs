#![allow(non_snake_case)]

//! Crypto Tools - Utilities - U-States
//!
//! Module implementing multi-register states.

use std::convert::From;
use std::num::Wrapping;
use std::cmp::{PartialOrd, PartialEq};
use std::ops::{BitXor, BitAnd, Not, Shl, Sub, Add};
use rand::{Rng, thread_rng, distributions::Standard, prelude::Distribution};

#[derive(Clone, Debug)]
/// Structure for four-register states.
pub struct Ux4<U>([U; 4]);

impl<U> Ux4<U> 
    where U: From<u8> + Copy
{
    /// Return a new Ux4 with values `state`.
    pub fn new(state: [U; 4]) -> Self {
        Ux4(state)
    }

    /// Return a new zero-formatted Ux4.
    pub fn zero() -> Self {
        Ux4([0_u8.into(); 4])
    }
}

impl<U> Ux4<U> 
    where Standard: Distribution<U>
{
    /// Draw a random Ux4. 
    pub fn rand() -> Self {
        let mut rng = thread_rng();
        Ux4([rng.gen::<U>(), rng.gen::<U>(), rng.gen::<U>(), rng.gen::<U>()])
    }
}

impl<U> Ux4<U>
    where U: Copy
{
    /// Getter for the state values.
    pub fn get(&self) -> [U; 4] {
        [self.0[0], self.0[1], self.0[2], self.0[3]]
    }

    /// Setter for the state values.
    pub fn set(&mut self, i: [U;4]) {
        *self = Ux4(i);
    }
}

impl<U> From<u8> for Ux4<U> 
    where U: From<u8> + Copy
{
    fn from(item: u8) -> Self {
        let mut state = Ux4::<U>::zero();
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

impl<U> Shl<usize> for Ux4<U> 
    where U: From<u8> + Copy + Shl<usize, Output = U>
{
    // TODO
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        let shifted = self.0.map(|x| x << rhs);
        Ux4(shifted)
    }
}

impl<U> Add for Ux4<U> 
    where U: From<u8> + Copy + Add<Output = U> + PartialOrd, 
        Wrapping<U>: Add<Output = Wrapping<U>>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result: [U; 4] = [0_u8.into(); 4];
        let mut carry: U = 0_u8.into();

        for i in 0..4 {
            let sum = Wrapping(self.0[i]) + Wrapping(rhs.0[i]) + Wrapping(carry);
            result[i] = sum.0;
            carry = if sum < Wrapping(self.0[i]) || sum < Wrapping(rhs.0[i]) {
                1_u8.into()
            } else {
                0_u8.into()
            };
        }

        Ux4(result)
    }
}

impl<U> Sub for Ux4<U> 
    where U: From<u8> + Copy + Add<Output = U> + Not<Output = U> + PartialOrd, 
        Wrapping<U>: Add<Output = Wrapping<U>>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let two_complement_rhs = (!rhs) + 
            Ux4([1u8.into(),0u8.into(),0u8.into(),0u8.into()]);
        self + two_complement_rhs
    }
}

impl<U> PartialEq for Ux4<U> 
    where U: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn addition() {
        let a = Ux4::<u64>([1,0,0,0].into());
        let b = Ux4::<u64>([1,0,0,0].into());
        let c = Ux4::<u64>([2,0,0,0].into());
        assert!(a + b == c);
    }

    #[test]
    fn addition_carry() {
        let a = Ux4::<u8>([255,0,0,0].into());
        let b = Ux4::<u8>([1,0,0,0].into());
        let c = Ux4::<u8>([0,1,0,0].into());
        assert!(a + b == c);
    }

    // #[test]
    // #[should_panic]
    // fn invalid_bound_sigma_0() {
    //     let _cauchy = Cauchy::new(0.0, 2.0, 3).unwrap();
    // }
}