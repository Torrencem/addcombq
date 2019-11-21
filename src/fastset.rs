use std::fmt;
use std::u64::MAX;

#[inline]
fn bit_scan_low(val: u64) -> u32 {
    return val.trailing_zeros() as u32;
}

/// Cycle the first m bits of a 64 bit integer by i
#[inline]
pub fn cycle(scontents: u64, i: u32, m: u32) -> u64 {
    let mut ret = scontents;
    let mut wrapped: u64 = MAX << (m - i); // Mask the elements which will get wrapped around
    wrapped &= ret;
    wrapped >>= m - i;
    ret <<= i;
    ret |= wrapped;
    ret &= !(MAX << m);
    ret
}

#[inline]
pub fn cycle_rev(scontents: u64, i: u32, m: u32) -> u64 {
    cycle(scontents, m - i, m)
}

/// FastSet definition. A FastSet is a set of integers all between 0 and 63,
/// which can be represented as the 1-bits of a 64-bit integer. Specifically,
/// the n-th bit of contents is 1 if 1 is in the FastSet
#[derive(Copy, Clone)]
pub struct FastSet {
    pub contents: u64,
}

pub fn singleton(i: u32) -> FastSet {
    FastSet {
        contents: (1u64 << i),
    }
}

pub fn empty_set() -> FastSet {
    FastSet { contents: 0u64 }
}

impl FastSet {
    /// Check whether a given element is in this FastSet or not.
    #[inline]
    pub fn access(&self, i: u32) -> bool {
        debug_assert!(i < 64);
        self.contents & (1u64 << i) > 0
    }

    /// Adds a given element to this FastSet.
    #[inline]
    pub fn add(&mut self, i: u32) {
        self.contents |= 1u64 << i;
    }

    /// Check if this FastSet is full up to and including n.
    #[inline]
    pub fn isfull(&self, n: u32) -> bool {
        // Tests if the set is full up to (and including) n
        (!(self.contents & ((1u64 << (n + 1)) - 1)) << (64 - n)) == 0
    }

    /// Check if this FastSet is empty
    #[inline]
    pub fn isempty(&self) -> bool {
        self.contents == 0u64
    }

    /// Get the number of elements in this FastSet. Will always be
    /// less than or equal to 64.
    #[inline]
    pub fn size(&self) -> u32 {
        return self.contents.count_ones() as u32;
    }

    /// Calculate the intersection of this FastSet with another
    #[inline]
    pub fn intersect(&mut self, other: &FastSet) {
        self.contents &= other.contents;
    }

    /// Create a Vec representation of this FastSet. Should only
    /// really be used for printing
    #[inline]
    pub fn as_vec(&self) -> Vec<u32> {
        let mut ret: Vec<u32> = Vec::with_capacity(self.size() as usize);
        let mut c1 = self.contents;
        while c1 != 0 {
            let n = bit_scan_low(c1);
            ret.push(n);
            c1 &= c1 - 1;
        }
        ret
    }
}

pub struct EachSetExact {
    state: u64,
    setmask: u64,
    doneflag: bool,
}

impl Iterator for EachSetExact {
    type Item = FastSet;

    // Based on ideas from https://stackoverflow.com/a/29914908/6504760
    fn next(&mut self) -> Option<FastSet> {
        if self.doneflag {
            return None;
        }
        // Find the greatest number which can be moved to the left
        let can_be_moved_left = self.state & !(self.state >> 1) & !(self.setmask >> 1);
        let first_moveable = 64 - can_be_moved_left.leading_zeros();
        if first_moveable == 0 {
            self.doneflag = true;
            return Some(FastSet {
                contents: self.state,
            });
        }
        let update_region = !((1 << (first_moveable - 1)) - 1) & !self.setmask;
        let to_fill_left = (self.state & update_region).count_ones() - 1;

        let old = self.state;
        // Clear the updated region
        self.state &= !update_region;
        let newregion = ((1 << (to_fill_left + 1)) - 1) << first_moveable;
        self.state |= newregion;

        Some(FastSet { contents: old })
    }
}

pub fn each_set_exact(max_size: u32, set_size: u32) -> EachSetExact {
    if max_size < set_size {
        return EachSetExact {
            state: 0,
            setmask: 0,
            doneflag: true,
        };
    }
    let naivestate = (1u64 << (set_size)) - 1;
    let setmask = !((1u64 << (max_size)) - 1);
    EachSetExact {
        state: naivestate,
        setmask: setmask,
        doneflag: false,
    }
}

pub struct EachSetExactZero {
    esetiter: EachSetExact,
}

impl Iterator for EachSetExactZero {
    type Item = FastSet;

    fn next(&mut self) -> Option<FastSet> {
        let mut ret = self.esetiter.next()?;
        ret.contents <<= 1;
        ret.contents |= 1;
        Some(ret)
    }
}

pub fn each_set_exact_zero(max_size: u32, set_size: u32) -> EachSetExactZero {
    EachSetExactZero {
        esetiter: each_set_exact(max_size - 1, set_size - 1),
    }
}

pub struct EachSetExactNoZero {
    esetiter: EachSetExact,
}

impl Iterator for EachSetExactNoZero {
    type Item = FastSet;

    fn next(&mut self) -> Option<FastSet> {
        let mut ret = self.esetiter.next()?;
        ret.contents <<= 1;
        return Some(ret);
    }
}

pub fn each_set_exact_no_zero(max_size: u32, set_size: u32) -> EachSetExactNoZero {
    EachSetExactNoZero {
        esetiter: each_set_exact(max_size - 1, set_size),
    }
}

impl fmt::Debug for FastSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self.as_vec());
        write!(f, "{}", s.replace("[", "{").replace("]", "}"))
    }
}

impl fmt::Display for FastSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self.as_vec());
        write!(f, "{}", s.replace("[", "{").replace("]", "}"))
    }
}

impl<'a, T: AsRef<[u32]>> From<T> for FastSet {
    fn from(vals: T) -> Self {
        let mut me = empty_set();
        for val in vals.as_ref() {
            me.add(*val);
        }
        me
    }
}
