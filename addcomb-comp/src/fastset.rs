use std::fmt;
use std::u64;
use std::u128;

use std::ops::*;

pub trait BitSetContents : 
    Shl<u32, Output=Self> + ShlAssign<u32> + Shr<u32, Output=Self> + ShrAssign<u32>
  + Shl<usize, Output=Self> + ShlAssign<usize> + Shr<usize, Output=Self> + ShrAssign<usize>
  + BitAnd<Self, Output=Self> + BitOr<Self, Output=Self> + BitAndAssign + BitOrAssign + Not<Output=Self> + PartialOrd
  + Add<Self, Output=Self> + Sub<Self, Output=Self>
  + Copy + Clone + fmt::Debug + fmt::Display + PartialEq + Eq
{
    fn one() -> Self;
    fn zero() -> Self;
    fn max() -> Self;
    fn bit_size() -> u32;
    fn num_ones(&self) -> u32;
    fn bit_scan_low(&self) -> u32;
    fn bit_scan_high(&self) -> u32;
}

impl BitSetContents for u64 {
    #[inline(always)]
    fn one() -> u64 {
        1
    }
    #[inline(always)]
    fn zero() -> u64 {
        0
    }
    #[inline(always)]
    fn max() -> u64 {
        u64::MAX
    }
    #[inline(always)]
    fn bit_size() -> u32 {
        64u32
    }
    #[inline(always)]
    fn num_ones(&self) -> u32 {
        self.count_ones() as u32
    }
    #[inline(always)]
    fn bit_scan_low(&self) -> u32 {
        self.trailing_zeros() as u32
    }
    #[inline(always)]
    fn bit_scan_high(&self) -> u32 {
        self.leading_zeros() as u32
    }
}

impl BitSetContents for u128 {
    #[inline(always)]
    fn one() -> u128 {
        1
    }
    #[inline(always)]
    fn zero() -> u128 {
        0
    }
    #[inline(always)]
    fn max() -> u128 {
        u128::MAX
    }
    #[inline(always)]
    fn bit_size() -> u32 {
        128u32
    }
    #[inline(always)]
    fn num_ones(&self) -> u32 {
        self.count_ones() as u32
    }
    #[inline(always)]
    fn bit_scan_low(&self) -> u32 {
        self.trailing_zeros() as u32
    }
    #[inline(always)]
    fn bit_scan_high(&self) -> u32 {
        self.leading_zeros() as u32
    }
}

/// Cycle the first m bits of an integer by i
#[inline(always)]
pub fn cycle<B: BitSetContents>(scontents: B, i: u32, m: u32) -> B {
    let mut ret = scontents;
    let mut wrapped: B = B::max() << (m - i); // Mask the elements which will get wrapped around
    wrapped &= ret;
    wrapped >>= m - i;
    ret <<= i;
    ret |= wrapped;
    ret &= !(B::max() << m);
    ret
}

#[inline(always)]
pub fn cycle_rev<B: BitSetContents>(scontents: B, i: u32, m: u32) -> B {
    cycle(scontents, m - i, m)
}

/// FastSet definition. A FastSet is a set of integers all between 0 and 63,
/// which can be represented as the 1-bits of a 64-bit integer. Specifically,
/// the n-th bit of contents is 1 if 1 is in the FastSet
#[derive(Copy, Clone)]
pub struct FastSet<B: BitSetContents = u64> {
    pub contents: B,
}

pub fn singleton<B: BitSetContents>(i: u32) -> FastSet<B> {
    FastSet {
        contents: (B::one() << i),
    }
}

pub fn empty_set<B: BitSetContents>() -> FastSet<B> {
    FastSet { contents: B::zero() }
}

impl<B: BitSetContents> FastSet<B> {
    /// Check whether a given element is in this FastSet or not.
    #[inline(always)]
    pub fn access(&self, i: u32) -> bool {
        debug_assert!(i < B::bit_size());
        self.contents & (B::one() << i) > B::zero()
    }

    /// Adds a given element to this FastSet.
    #[inline(always)]
    pub fn add(&mut self, i: u32) {
        self.contents |= B::one() << i;
    }

    /// Check if this FastSet is full up to and including n.
    #[inline(always)]
    pub fn isfull(&self, n: u32) -> bool {
        // Tests if the set is full up to (and including) n
        (!(self.contents & ((B::one() << (n + 1)) - B::one())) << (B::bit_size() - n)) == B::zero()
    }

    /// Check if this FastSet is empty
    #[inline]
    pub fn isempty(&self) -> bool {
        self.contents == B::zero()
    }

    /// Get the number of elements in this FastSet. Will always be
    /// less than or equal to 64.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        self.contents.num_ones() as u32
    }

    /// Calculate the intersection of this FastSet with another
    #[inline(always)]
    pub fn intersect(&mut self, other: &FastSet<B>) {
        self.contents &= other.contents;
    }

    /// Create a Vec representation of this FastSet. Should only
    /// really be used for printing
    #[inline(always)]
    pub fn as_vec(&self) -> Vec<u32> {
        let mut ret: Vec<u32> = Vec::with_capacity(self.size() as usize);
        let mut c1 = self.contents;
        while c1 != B::zero() {
            let n = c1.bit_scan_low();
            ret.push(n);
            c1 &= c1 - B::one();
        }
        ret
    }
}

pub struct EachSetExact<B: BitSetContents> {
    state: B,
    setmask: B,
    doneflag: bool,
}

impl<B: BitSetContents> Iterator for EachSetExact<B> {
    type Item = FastSet<B>;

    // Based on ideas from https://stackoverflow.com/a/29914908/6504760
    fn next(&mut self) -> Option<FastSet<B>> {
        if self.doneflag {
            return None;
        }
        // Find the greatest number which can be moved to the left
        let can_be_moved_left = self.state & !(self.state >> 1u32) & !(self.setmask >> 1u32);
        // NOTE NOTE NOTE maybe should be bit_scan_high?
        let first_moveable = B::bit_size() - can_be_moved_left.bit_scan_high();
        if first_moveable == 0 {
            self.doneflag = true;
            return Some(FastSet {
                contents: self.state,
            });
        }
        let update_region = !((B::one() << (first_moveable - 1)) - B::one()) & !self.setmask;
        let to_fill_left = (self.state & update_region).num_ones() - 1;

        let old = self.state;
        // Clear the updated region
        self.state &= !update_region;
        let newregion = ((B::one() << (to_fill_left + 1)) - B::one()) << first_moveable;
        self.state |= newregion;

        Some(FastSet { contents: old })
    }
}

pub fn each_set_exact<B: BitSetContents>(max_size: u32, set_size: u32) -> EachSetExact<B> {
    if max_size < set_size {
        return EachSetExact {
            state: B::zero(),
            setmask: B::zero(),
            doneflag: true,
        };
    }
    let naivestate = (B::one() << (set_size)) - B::one();
    let setmask = !((B::one() << (max_size)) - B::one());
    EachSetExact {
        state: naivestate,
        setmask: setmask,
        doneflag: false,
    }
}

pub struct EachSetExactZero<B: BitSetContents> {
    esetiter: EachSetExact<B>,
}

impl<B: BitSetContents> Iterator for EachSetExactZero<B> {
    type Item = FastSet<B>;

    fn next(&mut self) -> Option<FastSet<B>> {
        let mut ret = self.esetiter.next()?;
        ret.contents <<= 1u32;
        ret.contents |= B::one();
        Some(ret)
    }
}

pub fn each_set_exact_zero<B: BitSetContents>(max_size: u32, set_size: u32) -> EachSetExactZero<B> {
    EachSetExactZero {
        esetiter: each_set_exact::<B>(max_size - 1, set_size - 1),
    }
}

pub struct EachSetExactNoZero<B: BitSetContents> {
    esetiter: EachSetExact<B>,
}

impl<B: BitSetContents> Iterator for EachSetExactNoZero<B>{
    type Item = FastSet<B>;

    fn next(&mut self) -> Option<FastSet<B>> {
        let mut ret = self.esetiter.next()?;
        ret.contents <<= 1u32;
        return Some(ret);
    }
}

pub fn each_set_exact_no_zero<B: BitSetContents>(max_size: u32, set_size: u32) -> EachSetExactNoZero<B> {
    EachSetExactNoZero {
        esetiter: each_set_exact::<B>(max_size - 1, set_size),
    }
}

impl<B: BitSetContents> fmt::Debug for FastSet<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self.as_vec());
        write!(f, "{}", s.replace("[", "{").replace("]", "}"))
    }
}

impl<B: BitSetContents> fmt::Display for FastSet<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self.as_vec());
        write!(f, "{}", s.replace("[", "{").replace("]", "}"))
    }
}

impl<'a, T: AsRef<[u32]>, B: BitSetContents> From<T> for FastSet<B> {
    fn from(vals: T) -> Self {
        let mut me = empty_set();
        for val in vals.as_ref() {
            me.add(*val);
        }
        me
    }
}
