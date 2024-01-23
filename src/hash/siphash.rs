#![allow(non_snake_case)]

//! Crypto Tools - Hash - SipHash
//!
//! Module implementing the ARX hash functions SipHash and Half-SipHash.

// use std::io::Error;
use std::ops::{BitOr, BitXor, Add, Sub, Shl, Shr};
use crate::utilities::{ustates::Ux4, bitops::urot};

/// General SipHash permutation for 4xU states.
pub fn SipHash_general_permutation<U>(state: &mut Ux4<U>, params: [usize;5])
    where U: Copy + BitOr<Output = U> + BitXor<Output = U> + Sub<Output = U> + 
        Add<Output = U> + Shl<usize, Output = U> + Shr<usize, Output = U> 
{
    let [mut p0, mut p1, mut p2, mut p3] = state.get();
    let [a, b, c, d, e] = params;

    p0 = p0 + p1;
    p1 = urot::<U>(p1, a) ^ p0;
    p0 = urot::<U>(p0, e);
    p2 = p2 + p3;
    p3 = urot::<U>(p3, b) ^ p2;
    p0 = p0 + p3;
    p3 = urot::<U>(p3, d) ^ p0;
    p2 = p2 + p1;
    p1 = urot::<U>(p1, c) ^ p2;
    p2 = urot::<U>(p2, e);

    state.set([p0, p1, p2, p3]);
}

/// SipHash permutation for 4x64 states.
pub fn SipHash_perm(state: &mut Ux4::<u64>) {
    let params: [usize; 5] = [13, 16, 17, 21, 32];
    SipHash_general_permutation(state, params)
}

/// SipHash permutation for 4x32 states.
pub fn Half_SipHash_perm(state: &mut Ux4::<u32>) {
    let params: [usize; 5] = [5, 8, 7, 13, 16];
    SipHash_general_permutation(state, params)
}
