use anyhow::Context;
use anyhow::Result;

use std::convert::TryInto;
use std::num::Wrapping;

use num_bigint::BigUint;

// This is a port of the go pyrand code:
//
// https://github.com/drbrain/gopyrand
//
// which was ported from the C code in:
//
// What follows is the preamble from that file; note that not the full content of the original C
// file was ported here:
//
// http://www.math.keio.ac.jp/~matumoto/MT2002/emt19937ar.html
//
// The code in this module was based on a download from:
//
//    http://www.math.keio.ac.jp/~matumoto/MT2002/emt19937ar.html
//
// It was modified in 2002 by Raymond Hettinger as follows:
//
//  * the principal computational lines untouched.
//
//  * renamed genrand_res53() to random_random() and wrapped in python calling/return code.
//
//  * genrand_int32() and the helper functions, init_genrand() and init_by_array(), were declared
//    static, wrapped in Python calling/return code.  also, their global data references were
//    replaced with structure references.
//
//  * unused functions from the original were deleted.  New, original C python code was added to
//    implement the Random() interface.
//
// The following are the verbatim comments from the original code:
//
// A C-program for MT19937, with initialization improved 2002/1/26.  Coded by Takuji Nishimura and
// Makoto Matsumoto.
//
// Before using, initialize the state by using init_genrand(seed) or init_by_array(init_key,
// key_length).
//
// Copyright (C) 1997 - 2002, Makoto Matsumoto and Takuji Nishimura, All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification, are permitted
// provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of
//    conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of
//    conditions and the following disclaimer in the documentation and/or other materials provided
//    with the distribution.
//
// 3. The names of its contributors may not be used to endorse or promote products derived from
//    this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//
// Any feedback is very welcome.
// http://www.math.keio.ac.jp/matumoto/emt.html
// email: matumoto@math.keio.ac.jp

const N: usize = 624;
const M: usize = 397;

const MATRIX_A: u32 = 0x9908_b0df;
const UPPER_MASK: u32 = 0x8000_0000;
const LOWER_MASK: u32 = 0x7fff_ffff;

const MAG01: [u32; 2] = [0, MATRIX_A];

const MAXWIDTH: u64 = 1 << 53;

pub struct Random {
    state: [u32; N],
    index: usize,
}

impl Random {
    pub fn new() -> Self {
        let state: [u32; N] = [0; N];
        let index = 0;

        Random { state, index }
    }

    fn init(&mut self, seed: u32) {
        let mt = &mut self.state;
        mt[0] = seed;

        for mti in 1..N {
            // See Knuth TAOCP Vol2. 3rd Ed. P.106 for multiplier.
            //
            // In the previous versions, MSBs of the seed affect only MSBs of the array mt[].
            //
            // 2002/01/09 modified by Makoto Matsumoto
            mt[mti] = (Wrapping(1812433253u32)
                * Wrapping(mt[mti - 1] ^ (mt[(mti - 1) as usize] >> 30)))
            .0 + mti as u32;
        }

        self.index = N;
    }

    /// seed_big_u seeds with an unsigned big integer causing the same sequence of numbers to be
    /// generated as when seeding Python's PRNG with the corresponding number.
    pub fn seed_big_u(&mut self, seed: BigUint) -> Result<()> {
        self.seed_bytes(seed.to_bytes_be())
    }

    /// seed_bytes seeds with a byte slice.
    pub fn seed_bytes(&mut self, seed: Vec<u8>) -> Result<()> {
        if seed.len() == 0 {
            return Ok(self.seed_u32(0));
        }

        // pad to 4 byte multiple to create a Vec<u32>
        let fill = 4 - seed.len() % 4;

        let seed = if fill != 4 {
            let mut seed = seed.clone();
            for _ in 0..fill {
                seed.insert(0, 0);
            }
            seed
        } else {
            seed
        };

        let seed = seed
            .chunks(4)
            .rev()
            .map(|c: &[u8]| chunk_to_u32(c))
            .collect();

        self.seed_vec(seed)
    }

    /// seed_hex_string takes a string of hex digits and seeds the PRNG with the corresponding
    /// number.
    ///
    /// Seeding Pythons PRNG with 0x12345678901337cafe is equivalent to calling
    ///
    ///     use unicornify::pyrand::Random;
    ///
    ///     let seed = String::from("12345678901337cafe");
    ///     Random::new().seed_hex_string(seed);
    pub fn seed_hex_string(&mut self, seed: String) -> Result<()> {
        let bytes = seed.as_bytes();

        let big = BigUint::parse_bytes(bytes, 16)
            .with_context(|| format!("Invalid hex string {}", seed))?;

        self.seed_big_u(big)
    }

    /// Create a new Random from a u32 seed
    pub fn seed_u32(&mut self, seed: u32) {
        self.seed_vec(vec![seed]).unwrap(); // 1 shouldn't be too long
    }

    /// Create a new Random from a u64 seed
    pub fn seed_u64(&mut self, seed: u64) {
        if seed <= 0xffff_ffff {
            self.seed_u32(seed as u32);
        } else {
            let seed_vec = vec![seed as u32, (seed >> 32) as u32];

            self.seed_vec(seed_vec).unwrap(); // 2 shouldn't be too long
        }
    }

    /// Create a new Random from a Vec<u32> seed
    pub fn seed_vec(&mut self, seed: Vec<u32>) -> Result<()> {
        let seed_len = seed
            .len()
            .try_into()
            .with_context(|| format!("Seed Vec too big ({})", seed.len()))?;

        self.init(19650218);

        let k = if N > seed_len { N } else { seed_len };

        let mt = &mut self.state;

        let mut i: usize = 1;
        let mut j: usize = 0;

        for _ in (1..=k).rev() {
            mt[i] =
                (Wrapping(mt[i] ^ (Wrapping(mt[i - 1] ^ (mt[i - 1] >> 30)) * Wrapping(1664525)).0)
                    + Wrapping(seed[j]))
                .0 + j as u32; // non linear

            i += 1;
            j += 1;

            if i >= N {
                mt[0] = mt[N - 1];
                i = 1;
            }

            if j >= seed_len {
                j = 0;
            }
        }

        for _ in (1..N).rev() {
            mt[i] = (mt[i] ^ (Wrapping(mt[i - 1] ^ (mt[i - 1] >> 30)) * Wrapping(1566083941)).0)
                - i as u32; // non linear

            i += 1;
            if i >= N {
                mt[0] = mt[N - 1];
                i = 1;
            }
        }

        mt[0] = 0x8000_0000; // MSB is 1; assuring non-zero initial array

        Ok(())
    }

    /// Choice is essentially RandRange with a first argument of 0.  It's provided here as the
    /// equivalent to Python's random.choice(), where:
    ///
    ///     use unicornify::pyrand::Random;
    ///
    ///     let mut rand = Random::new();
    ///     let l: [u8;3] = [42, 66, 13];
    ///     let choice = rand.choice(l.len() as i32) as usize;
    ///     let c = l[choice];
    ///
    /// is equivalent to Python's
    ///
    /// '''python
    /// l := [42, 666, 13]
    /// c := r.choice(l)
    /// ```
    pub fn choice(&mut self, length: i32) -> i32 {
        (self.rand() * length as f64) as i32
    }

    /// genrandRes53 generates a random number on [0,1) with 53-bit resolution; note that
    /// 9007199254740992 == 2**53; I assume they're spelling "/2**53" as multiply-by-reciprocal in
    /// the (likely vain) hope that the compiler will optimize the division away at compile-time.
    /// 67108864 is 2**26.  In effect, a contains 27 random bits shifted left 26, and b fills in
    /// the lower 26 bits of the 53-bit numerator.
    ///
    /// The original code credited Isaku Wada for this algorithm, 2002/01/09.
    fn gen_res53(&mut self) -> f64 {
        let a = (self.gen_u32() >> 5) as f64;
        let b = (self.gen_u32() >> 6) as f64;

        (a * 67108864.0f64 + b) * (1.0 / 9007199254740992.0f64)
    }

    fn gen_u32(&mut self) -> u32 {
        let mut y: u32;
        let mt = &mut self.state;

        if self.index >= N {
            // generates N words at one time
            for kk in 0..N - M {
                y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
                mt[kk] = mt[kk + M] ^ (y >> 1) ^ MAG01[(y & 1) as usize];
            }
            for kk in N - M..N - 1 {
                y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
                mt[kk] = mt[kk + M - N] ^ (y >> 1) ^ MAG01[(y & 1) as usize];
            }
            y = (mt[N - 1] & UPPER_MASK) | (mt[0] & LOWER_MASK);
            mt[N - 1] = mt[M - 1] ^ (y >> 1) ^ MAG01[(y & 1) as usize];

            self.index = 0;
        }

        y = mt[self.index];
        self.index += 1;

        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c_5680;
        y ^= (y << 15) & 0xefc6_0000;
        y ^= y >> 18;

        y
    }

    /// Returns the next random floating point number in the range [0.0, 1.0).
    pub fn rand(&mut self) -> f64 {
        self.gen_res53()
    }

    /// Returns a u64 strictly smaller than the given upper boundary n.
    pub fn rand_below(&mut self, n: u64) -> u64 {
        // This is what Python 2 does. Python 3 counts the actual number of bits of n and thus has
        // different behavior.
        let bits: u32 = (1.00001 + (n as f64 - 1.0).log2()) as u32;

        let two = bits > 32;

        // we will never need more than 64 bits, but the bits formular above may cause more to be
        // requested
        let three = bits > 64;

        let mut v = n;

        while v >= n {
            let s = self.rand_bits(bits);

            // since we're constrained to uint64, and s[2] > 0 means there's a 1 beyond the 64th
            // bit of the number represented by s, this is the v >= n case, and thus we retry
            if three && s[2] > 0 {
                v = n;
                continue;
            }

            v = s[0] as u64;
            if two {
                v |= (s[1] as u64) << 32;
            }
        }

        v
    }

    /// Returns a slice of uint32 that is filled with random bits. If k is not divisible by 32,
    /// then the 32-k most significant bits of the last slice element will always be zero.
    pub fn rand_bits(&mut self, mut k: u32) -> Vec<u32> {
        if k == 0 {
            panic!("number of bits must be > 0")
        }

        let quads: usize = ((k - 1) / 32 + 1) as usize;
        let mut result: Vec<u32> = Vec::new();

        for _ in 0..quads {
            let mut v = self.gen_u32();

            if k < 32 {
                v >>= 32 - k;
            }
            result.push(v);

            k = (Wrapping(k) - Wrapping(32)).0;
        }

        result
    }

    /// Returns a random integer in range [a, b], including both end points.
    pub fn rand_i32(&mut self, a: i32, b: i32) -> i32 {
        self.rand_range(a, b + 1)
    }

    /// Returns a random integer in range [start, stop), including the low value but excluding the
    /// high value.
    fn rand_range(&mut self, start: i32, stop: i32) -> i32 {
        if start >= stop {
            panic!("empty range for randrange")
        }
        let width = (stop - start) as u64;

        if width >= MAXWIDTH {
            return start + self.rand_below(width) as i32;
        }

        start + (self.rand() * width as f64) as i32
    }
}

fn chunk_to_u32(chunk: &[u8]) -> u32 {
    let chunk = chunk
        .try_into()
        .expect("BUG: byte slice must have length 4");

    u32::from_be_bytes(chunk)
}
