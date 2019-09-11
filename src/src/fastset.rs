use std::fmt;
use std::u64::MAX;

#[inline]
fn bit_scan_low(val: u64) -> u32 {
    return val.trailing_zeros() as u32;
}

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

#[derive(Copy, Clone)]
pub struct FastSet {
    pub contents: u64,
}

pub fn singleton(i: u32) -> FastSet {
    return FastSet {
        contents: (1u64 << i),
    };
}

pub fn empty_set() -> FastSet {
    return FastSet { contents: 0u64 };
}

impl FastSet {
    #[inline]
    pub fn access(&self, i: u32) -> bool {
        // assert!(i < 64);
        return self.contents & (1u64 << i) > 0;
    }

    #[inline]
    pub fn add(&mut self, i: u32) {
        self.contents |= 1u64 << i;
    }

    #[inline]
    pub fn isfull(&self, n: u32) -> bool {
        // Tests if the set is full up to (and including) n
        (!(self.contents & ((1u64 << (n + 1)) - 1)) << (64 - n)) == 0
    }

    #[inline]
    pub fn isempty(&self) -> bool {
        self.contents == 0u64
    }

    #[inline]
    pub fn size(&self) -> u32 {
        return self.contents.count_ones() as u32;
    }

    // #[inline]
    // pub fn union(&mut self, other: &FastSet) {
    //     self.contents |= other.contents;
    // }

    #[inline]
    pub fn intersect(&mut self, other: &FastSet) {
        self.contents &= other.contents;
    }

    // #[inline]
    // pub fn cycle(&mut self, i: u32, m: u32) {
    //     // Add i (mod n) to every element of the set
    //     assert!(i < m);
    //     self.contents = cycle(self.contents, i, m);
    // }

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

// pub struct EachSet {
//     state: u64,
//     cap: u64,
// }

// impl Iterator for EachSet {
//     type Item = FastSet;

//     fn next(&mut self) -> Option<FastSet> {
//         let curr = self.state;
//         if curr >= self.cap {
//             None
//         } else {
//             self.state += 1;
//             Some(FastSet {contents: self.state - 1})
//         }
//     }
// }

// Note to initialize this struct correctly
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

// pub fn each_set(max_size: u32) -> EachSet {
//     return EachSet { state: 0, cap: (1u64 << max_size) }
// }

pub fn each_set_exact(max_size: u32, set_size: u32) -> EachSetExact {
    assert!(max_size >= set_size);
    let naivestate = (1u64 << (set_size)) - 1;
    let setmask = !((1u64 << (max_size)) - 1);
    return EachSetExact {
        state: naivestate,
        setmask: setmask,
        doneflag: false,
    };
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
        return Some(ret);
    }
}

pub fn each_set_exact_zero(max_size: u32, set_size: u32) -> EachSetExactZero {
    return EachSetExactZero {
        esetiter: each_set_exact(max_size - 1, set_size - 1),
    };
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
    return EachSetExactNoZero {
        esetiter: each_set_exact(max_size - 1, set_size),
    };
}

impl fmt::Debug for FastSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FastSet {:?}",
            (0..64).filter(|n| self.access(*n)).collect::<Vec<u32>>()
        )
    }
}

impl fmt::Display for FastSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self.as_vec());
        write!(f, "{}", s.replace("[", "{").replace("]", "}"))
    }
}

impl<'a> From<&'a [u32]> for FastSet {
    fn from(vals: &[u32]) -> Self {
        let mut me = empty_set();
        for val in vals {
            me.add(*val);
        }
        me
    }
}
