use std::fmt::Debug;

use crate::fastset;
use crate::fastset::FastSet;
use crate::exactset;

use crate::exactset::GElem;

use std::rc::Rc;

use array_tool::vec::Intersect;

pub trait Group: Clone {
    type Element;
    fn zero(&self) -> Self::Element;
    fn gsize(&self) -> u32;
}

impl Group for u32 {
    type Element = u32;
    #[inline(always)]
    fn zero(&self) -> u32 {
        0u32
    }

    #[inline(always)]
    fn gsize(&self) -> u32 {
        *self
    }
}

impl Group for Rc<Vec<u32>> {
    type Element = GElem;
    fn zero(&self) -> GElem {
        GElem(vec![0u32, (**self).len() as u32])
    }

    fn gsize(&self) -> u32 {
        let mut res = 1;
        for num in (**self).iter() {
            res *= num;
        }
        res
    }
}

pub trait HFolds {
    type Element;
    type Group: Group<Element = Self::Element>;
    
    fn hfoldsumset(&self, h: u32, n: Self::Group) -> Self;
    fn hfoldintervalsumset(&self, hs: (u32, u32), n: Self::Group) -> Self;
    fn hfoldrestrictedsumset(&self, h: u32, n: Self::Group) -> Self;
    fn hfoldintervalrestrictedsumset(&self, hs: (u32, u32), n: Self::Group) -> Self;
    fn hfoldsignedsumset(&self, h: u32, n: Self::Group) -> Self;
    fn hfoldintervalsignedsumset(&self, hs: (u32, u32), n: Self::Group) -> Self;
    fn hfoldrestrictedsignedsumset(&self, h: u32, n: Self::Group) -> Self;
    fn hfoldintervalrestrictedsignedsumset(&self, hs: (u32, u32), n: Self::Group) -> Self; 
}

pub trait SetLike: Debug + Clone + HFolds {
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
    fn has(&self, i: &Self::Element) -> bool;
    fn intersect(&mut self, other: Self);

    fn zero_free(&self, n: Self::Group) -> bool {
        self.has(&n.zero())
    }
}


impl HFolds for Vec<GElem> {
    type Group = Rc<Vec<u32>>;
    type Element = GElem;
    
    fn hfoldsumset(&self, h: u32, n: Self::Group) -> Self {
        exactset::hfoldsumset(&self, h, n).into_iter().collect()
    }
    fn hfoldintervalsumset(&self, hs: (u32, u32), n: Self::Group) -> Self {
        exactset::hfoldintervalsumset(&self, hs, n).into_iter().collect()
    }
    fn hfoldrestrictedsumset(&self, h: u32, n: Self::Group) -> Self {
        exactset::hfoldrestrictedsumset(&self, h, n).into_iter().collect()
    }
    fn hfoldintervalrestrictedsumset(&self, hs: (u32, u32), n: Self::Group) -> Self {
        exactset::hfoldintervalrestrictedsumset(&self, hs, n).into_iter().collect()
    }
    fn hfoldsignedsumset(&self, h: u32, n: Self::Group) -> Self {
        exactset::hfoldsignedsumset(&self, h, n).into_iter().collect()
    }
    fn hfoldintervalsignedsumset(&self, hs: (u32, u32), n: Self::Group) -> Self {
        exactset::hfoldintervalsignedsumset(&self, hs, n).into_iter().collect()
    }
    fn hfoldrestrictedsignedsumset(&self, h: u32, n: Self::Group) -> Self {
        exactset::hfoldrestrictedsignedsumset(&self, h, n).into_iter().collect()
    }
    fn hfoldintervalrestrictedsignedsumset(&self, hs: (u32, u32), n: Self::Group) -> Self {
        exactset::hfoldintervalrestrictedsignedsumset(&self, hs, n).into_iter().collect()
    }
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

    fn has(&self, i: &u32) -> bool {
        self.access(*i)
    }

    fn intersect(&mut self, other: FastSet) {
        FastSet::intersect(self, &other)
    }
}

impl SetLike for Vec<GElem> {
    type EachSetExact = exactset::EachSetExact;
    type EachSetExactZero = exactset::EachSetExact;
    type EachSetExactNoZero = exactset::EachSetExact;
    fn empty() -> Self {
        exactset::empty_set()
    }
    fn singleton(i: GElem) -> Self {
        vec![i]
    }
    fn each_set_exact(g: Self::Group, set_size: u32) -> Self::EachSetExact {
        exactset::each_set_exact(set_size, g)
    }
    fn each_set_exact_zero(g: Self::Group, set_size: u32) -> Self::EachSetExactZero {
        // TODO: Fix and optimize this
        exactset::each_set_exact(set_size, g)
    }
    fn each_set_exact_no_zero(g: Self::Group, set_size: u32) -> Self::EachSetExactNoZero {
        exactset::each_set_exact_no_zero(set_size, g)
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn is_full(&self, n: Self::Group) -> bool {
        self.len() as u32 == n.gsize()
    }

    fn size(&self) -> u32 {
        self.len() as u32
    }

    fn add(&mut self, i: GElem) {
        self.push(i)
    }

    fn has(&self, i: &GElem) -> bool {
        self.contains(i)
    }

    fn intersect(&mut self, other: Vec<GElem>) {
        let tmp = Intersect::intersect(self, other);
        *self = (*tmp).to_vec();
    }
}
