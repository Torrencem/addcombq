use std::fmt::Display;

use crate::fastset;

pub trait Group: Copy {
    fn zero(&self) -> Self;
    fn gsize(&self) -> u32;
}

impl Group for u32 {
    #[inline(always)]
    fn zero(&self) -> u32 {
        0u32
    }

    #[inline(always)]
    fn gsize(&self) -> u32 {
        *self
    }
}

pub trait HFolds {
    type Element;
    type Group: Group;
    
    fn hfoldsumset(&self, h: u32, n: Self::Group) -> Self;
    fn hfoldintervalsumset(&self, hs: (u32, u32), n: Self::Group) -> Self;
    fn hfoldrestrictedsumset(&self, h: u32, n: Self::Group) -> Self;
    fn hfoldintervalrestrictedsumset(&self, hs: (u32, u32), n: Self::Group) -> Self;
    fn hfoldsignedsumset(&self, h: u32, n: Self::Group) -> Self;
    fn hfoldintervalsignedsumset(&self, hs: (u32, u32), n: Self::Group) -> Self;
    fn hfoldrestrictedsignedsumset(&self, h: u32, n: Self::Group) -> Self;
    fn hfoldintervalrestrictedsignedsumset(&self, hs: (u32, u32), n: Self::Group) -> Self; 
}

pub trait SetLike: Display + Clone + HFolds {
    type EachSetExact: Iterator<Item=Self>;
    type EachSetExactZero: Iterator<Item=Self>;
    type EachSetExactNoZero: Iterator<Item=Self>;
    fn empty() -> Self;
    fn singleton(i: Self::Element) -> Self;
    fn each_set_exact(max_size: Self::Group, set_size: u32) -> Self::EachSetExact;
    fn each_set_exact_zero(max_size: Self::Group, set_size: u32) -> Self::EachSetExactZero;
    fn each_set_exact_no_zero(max_size: Self::Group, set_size: u32) -> Self::EachSetExactNoZero;

    fn is_empty(&self) -> bool;
    fn is_full(&self, n: Self::Group) -> bool;
    fn size(&self) -> u32;
    fn add(&mut self, i: Self::Element);
    fn contains(&self, i: Self::Element) -> bool;
}

impl SetLike for fastset::FastSet {
    type EachSetExact = fastset::EachSetExact;
    type EachSetExactZero = fastset::EachSetExactZero;
    type EachSetExactNoZero = fastset::EachSetExactNoZero;
    fn empty() -> Self {
        fastset::empty_set()
    }
    fn singleton(i: u32) -> Self {
        fastset::singleton(i)
    }
    fn each_set_exact(max_size: u32, set_size: u32) -> fastset::EachSetExact {
        fastset::each_set_exact(max_size, set_size)
    }
    fn each_set_exact_zero(max_size: u32, set_size: u32) -> fastset::EachSetExactZero {
        fastset::each_set_exact_zero(max_size, set_size)
    }
    fn each_set_exact_no_zero(max_size: u32, set_size: u32) -> fastset::EachSetExactNoZero {
        fastset::each_set_exact_no_zero(max_size, set_size)
    }

    fn is_empty(&self) -> bool {
        self.isempty()
    }

    fn is_full(&self, n: u32) -> bool {
        self.isfull(n)
    }

    fn size(&self) -> u32 {
        self.size()
    }

    fn add(&mut self, i: u32) {
        self.add(i)
    }

    fn contains(&self, i: u32) -> bool {
        self.access(i)
    }
}

