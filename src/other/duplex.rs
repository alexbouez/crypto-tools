#![allow(non_snake_case)]

//! Crypto Tools - Other - Duplex
//!
//! Module implementing the Duplex construction of Dobraunig and Mennink [DM2019].

use std::io::Error;
use rand::{Rng, thread_rng, distributions::Standard, prelude::Distribution};
use std::{ops::{BitXor, BitAnd, BitOr, Not, Add, Sub, Shl, Shr}, convert::From};
use crate::utilities::bitops::urot;

#[derive(Clone, Debug)]
/// Structure implementing [DM2019].
/// Note that the state is reversed for easier use of the outputs.
/// The outer part is stored in the lower bits.
pub struct Duplex<U>
{
    b: usize,         // size of the state
    r: usize,         // size of the outer part (b=c+r)
    k: usize,         // size of the key (k<=b)
    u: usize,         // number of keys (u>=1)
    alpha: usize,     // rotation value
    perm:  fn(U) -> U,
    keys:  Vec<U>,
    kmask: U,
    mask:  U,
    state: U
}

impl<U> Duplex<U>
    where Vec<U>: Clone, U: Clone
{
    /// Getter for the parameters (b,r,k,u,a).
    pub fn get_params(&self) -> Vec<usize> {
        vec![self.b, self.r, self.k, self.u]
    }

    /// Getter for the mask.
    pub fn get_mask(&self) -> U {
        self.mask.clone()
    }
}

impl<U> Duplex<U>
    where U: Copy + From<u8> + Not<Output = U> + Shl<usize, Output = U>
        + Shr<usize, Output = U> + Add<Output = U> + Sub<Output = U>
        + BitAnd<Output = U> + BitOr<Output = U> + BitXor<Output = U>,
        Standard: Distribution<U>
{
    /// Setup function, part of the init function.
    pub fn new(params: Vec<usize>, func: fn(U) -> U) -> Result<Self, Error> {
        let (b, r, k) = (params[0], params[1], params[2]); // block size, rate, key size
        assert!((0_usize < r) && (r <= b), "The rate r must be smaller than the block size b.");
        assert!((0_usize < k) && (k <= b), "The key size k must be smaller than the block size b.");
        let u = params[3];
        assert!(0_usize < u);

        // Generate the state mask
        let mut mask: U = 1_u8.into();
        mask = (mask << r) - 1_u8.into();

        // Generate the key mask
        let mut kmask: U = 1_u8.into();
        kmask = (kmask << k) - 1_u8.into();

        // Generate the keys using rand
        let mut rng = thread_rng();
        let mut keys: Vec<U> = Vec::with_capacity(u);
        for _ in 0..u {
            keys.push(rng.gen::<U>() & kmask);
        }

        // Initial state is r '0'
        let state: U = 0_u8.into();

        Ok(Duplex{
            b: b,
            r: r,
            k: k,
            u: u,
            alpha: params[4],
            perm:  func,
            keys:  keys,
            kmask: kmask,
            mask:  mask,
            state: state
        })
    }

    /// Reset function, part of the init function.
    /// Reset allows to reuse the same keys and parameters.
    pub fn reset(&mut self, delta: usize) {
        // Generate IV
        let mut rng = thread_rng();
        let initialization_vector = rng.gen::<U>() & !self.kmask;

        self.state = urot::<U>(
            self.keys[delta % self.u] | initialization_vector, self.alpha
        );
        self.state = (self.perm)(self.state);
    }

    /// General duplex function.
    pub fn duplex(&mut self, flag: bool, input: U) -> U {
        let output: U = self.state & self.mask;

        if flag {
            self.state = self.state & !self.mask;
        }

        self.state = (self.perm)(self.state ^ input);
        output
    }
}