#![allow(non_snake_case)]

//! Crypto Tools - PRNG - GT2016
//!
//! Module implementing the sponge-based PRNG of Gazi and Tessaro 2016 (https://eprint.iacr.org/2016/169.pdf).

use std::io::Error;
use rand::{Rng, thread_rng, distributions::Standard, prelude::Distribution};
use std::{ops::{BitXor, BitAnd, BitOr, Not, Sub, Shl}, convert::From};
use crate::prng::PRNG;

#[derive(Clone, Debug)]
/// Structure implementing GT2016.
/// Note that the state of the sponge is reversed for easier use of the outputs. 
/// The outer part is stored in the lower bits.
pub struct SPRG<U>
{
    t: usize,         // number of permutation rounds in `next' 
    s: usize,         // length of the seed vector'
    j: usize,         // seed iterator
    mask: U,
    perm: fn(U) -> U,
    seed: Vec<U>,
    state: U
}

impl<U> SPRG<U> 
    where Vec<U>: Clone, U: Clone
{
    /// Getter for the parameters (t,s).
    pub fn get_params(&self) -> Vec<usize> {
        vec![self.t, self.s]
    }

    /// Getter for the seed.
    pub fn get_seed(&self) -> Vec<U> {
        self.seed.clone()
    }

    /// Getter for the mask.
    pub fn get_mask(&self) -> U {
        self.mask.clone()
    }
}

impl<U> PRNG<U, Vec<U>, U> for SPRG<U>
    where U: From<u8> + Not<Output = U> + BitAnd<Output = U> + BitXor<Output = U> + 
        BitOr<Output = U> + Shl<usize, Output = U> + Sub<Output = U> + Copy + std::fmt::UpperHex, 
        Standard: Distribution<U> 
{
    /// General setup function.
    fn setup(params: Vec<usize>, func: fn(U) -> U) -> Result<Self, Error> {
        let (n, r, t, s) = (params[0], params[1], params[2], params[3]);
        assert!((0_usize < r) && (r <= n));    // 0 < r <= n   
        
        // Generate the mask
        let mut mask: U = 1_u8.into();
        mask = (mask << r) - 1_u8.into();

        // Generate the seed using rand
        let mut rng = thread_rng();
        let mut seed: Vec<U> = Vec::with_capacity(s);
        for _ in 0..s {
            seed.push(rng.gen::<U>() & mask);
        }

        // Initial state is r '0' bits and c random bits (n=c+r)
        let mut state: U = 0_u8.into();
        state = state | (rng.gen::<U>() & !mask);

        Ok(SPRG{
            t: t,
            s: s,
            j: 1_usize,
            mask: mask,
            perm: func,
            seed: seed,
            state: state
        })  
    }

    /// General refresh function.
    fn refresh(&mut self, inputs: Vec<U>) {
        let l = inputs.len();
        for i in 1..l {
            self.state = (self.perm)(self.state ^ 
                ((inputs[i-1] ^ self.seed[self.j]) & self.mask)
            );
            self.j = (self.j + 1) % self.s;
        }
    }
    
    /// General next function.
    fn next(&mut self) -> U {
        self.state = (self.perm)(self.state);
        let R = self.state & self.mask;
        for _ in 1..self.t {
            self.state = (self.perm)(self.state);
            self.state = self.state & !(self.mask);
        }
        self.j = 1_u8.into();
        R
    }
}
