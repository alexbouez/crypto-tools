#![allow(non_snake_case)]

//! Crypto Tools - PRNG - GT2016
//!
//! Module implementing the sponge-based PRNG of Gazi and Tessaro 2016.

use std::io::Error;
use rand::{Rng, thread_rng, distributions::Standard, prelude::Distribution};
use std::{ops::{BitXor, BitAnd, Not, Sub, Shl}, convert::From};
use crate::prng::PRNG;

#[derive(Clone, Debug)]
/// Structure implementing GT2016.
pub struct SPRG<U>
{
    n: usize,         // size of the state
    r: usize,         // size of the outer part (n=c+r)
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
    /// Getter for the parameters (n,r,t,s).
    pub fn get_params(&self) -> Vec<usize> {
        vec![self.n, self.r, self.t, self.s]
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
        Shl<Output = U> + Sub<Output = U> + Copy, Standard: Distribution<U>
{
    /// General setup function.
    fn setup(params: Vec<usize>, func: fn(U) -> U) -> Result<Self, Error> {
        let (n, r, t, s) = (params[0], params[1], params[2], params[3]);
        assert!(r <= n);    // 0 < r <= n   
        
        // Generate the seed using rand
        let mask = ((1 << r) - 1).into();
        let mut rng = thread_rng();
        let mut seed: Vec<U> = Vec::with_capacity(s);
        for _ in 0..s {
            seed.push(rng.gen::<U>() & mask);
        }

        Ok(SPRG{
            n: n,
            r: r,
            t: t,
            s: s,
            j: 1_usize,
            mask: mask,
            perm: func,
            seed: seed,
            state: 0_u8.into()
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
        self.j = 1;
        R
    }
}
