use std::fmt::Debug;

use crate::exactset;
use crate::fastset;
use crate::fastset::{FastSet, BitSetContents};

use crate::exactset::GElem;

use std::rc::Rc;

use array_tool::vec::Intersect;

/// A trait for representations of abelian groups (i.e. Z_5 as "5")
pub trait Group: Clone {
    /// The type of elements of the group (i.e. u32)
    type Element;

    /// Given a representation of a group, returns the identity element
    ///
    /// # Example
    ///
    /// ```
    /// use addcomb_comp::setlike::Group;
    /// let g = 15u32;       // Z_15
    /// let zero = g.zero(); // 0u32
    ///
    /// use std::rc::Rc;
    /// let g = Rc::new(vec![50, 30, 20]);  // Z_50 x Z_30 x Z_20
    /// let zero = g.zero();  // GElem(vec![0u32, 0u32, 0u32])
    /// ```
    fn zero(&self) -> Self::Element;

    /// Returns the size of an abelian group
    ///
    /// # Example
    ///
    /// ```
    /// use addcomb_comp::setlike::Group;
    /// let g = 15u32;     // Z_15
    /// let s = g.gsize(); // 15u32
    ///
    /// use std::rc::Rc;
    /// let g = Rc::new(vec![50, 30, 20]);  // Z_50 x Z_30 x Z_20
    /// let s = g.gsize();  // 50 * 30 * 20
    /// ```
    fn gsize(&self) -> u32;
}

/// A trait for things you can take hfold sumsets of (e.g. set-like things)
pub trait HFolds {
    /// The elements of our set; the things we're adding together
    type Element;

    /// The underlying group representation for our set. Useful
    /// for constraining our set type to a particular kind of
    /// group (i.e. small cyclic groups, for FastSets)
    type Group: Group<Element = Self::Element>;

    /// Calculate the h-fold sumset of a set
    ///
    /// # Arguments
    ///
    /// * `h` - The number of times we add elements of our set with each other
    ///
    /// * `n` - The group to use
    ///
    fn hfold_sumset(&self, h: u32, n: Self::Group) -> Self;

    /// Calculate the h-fold sumset of a set over an interval
    ///
    /// # Arguments
    ///
    /// * `hs` - A tuple `a, b` representing the closed interval [a, b]
    ///
    /// * `n` - The group to use
    ///
    fn hfold_interval_sumset(&self, hs: (u32, u32), n: Self::Group) -> Self;

    /// Calculate the h-fold restricted sumset (terms are only allowed to appear once in a sum)
    ///
    /// # Arguments
    ///
    /// * `h` - The number of times we add elements of our set with each other
    ///
    /// * `n` - The group to use
    ///
    fn hfold_restricted_sumset(&self, h: u32, n: Self::Group) -> Self;

    /// Calculate the h-fold restricted sumset over an interval. The same as the
    /// union of the h-fold restricted sumset over each number in the interval
    ///
    /// # Arguments
    ///
    /// * `hs` - A tuple `a, b` representing the closed interval [a, b]
    ///
    /// * `n` - The group to use
    ///
    fn hfold_interval_restricted_sumset(&self, hs: (u32, u32), n: Self::Group) -> Self;

    /// Calculate the h-fold signed sumset (terms are allowed to be subtracted)
    ///
    /// # Arguments
    ///
    /// * `h` - The number of times we add/subtract elements of our set with each other
    ///
    /// * `n` - The group to use
    ///
    fn hfold_signed_sumset(&self, h: u32, n: Self::Group) -> Self;

    /// Calculate the h-fold signed sumset over an interval. The same as the
    /// union of the h-fold signed sumset over each number in the interval
    ///
    /// # Arguments
    ///
    /// * `hs` - A tuple `a, b` representing the closed interval [a, b]
    ///
    /// * `n` - The group to use
    ///
    fn hfold_interval_signed_sumset(&self, hs: (u32, u32), n: Self::Group) -> Self;

    /// Calculate the h-fold restricted signed sumset (terms cannot repeat, but can
    /// be either added or subtracted)
    ///
    /// # Arguments
    ///
    /// * `h` - The number of times we add/subtract elements of our set with each other
    ///
    /// * `n` - The group to use
    ///
    fn hfold_restricted_signed_sumset(&self, h: u32, n: Self::Group) -> Self;

    /// Calculate the h-fold restricted signed sumset over an interval. The same as
    /// the union of the h-fold restricted sumset over each number in the interval
    ///
    /// # Arguments
    ///
    /// * `hs` - A tuple `a, b` representing the closed interval [a, b]
    ///
    /// * `n` - The group to use
    ///
    fn hfold_interval_restricted_signed_sumset(&self, hs: (u32, u32), n: Self::Group) -> Self;
}

/// A trait for sets which can be used internally for b-functions
pub trait SetLike: Debug + Clone + HFolds {
    /// An iterator type which gives each `Self` in a given group
    type EachSetExact: Iterator<Item = Self>;

    /// An iterator type which gives each `Self` in a given group, where each
    /// element is required to contain the zero element
    type EachSetExactZero: Iterator<Item = Self>;

    /// An iterator type which gives each `Self` in a given group, where each
    /// element does not contain the zero element
    type EachSetExactNoZero: Iterator<Item = Self>;

    /// Returns the empty set
    fn empty() -> Self;

    /// Returns a set with a single element
    fn singleton(i: Self::Element) -> Self;

    /// Iterates through each set of some size, with elements in a given group
    ///
    /// # Arguments
    ///
    /// * `max_size` - The group of elements in the sets
    ///
    /// * `set_size` - The exact size of each set iterated through
    ///
    fn each_set_exact(max_size: Self::Group, set_size: u32) -> Self::EachSetExact;

    /// Iterates through each set containing 0 of some size, with elements in
    /// a given group
    ///
    /// # Arguments
    ///
    /// * `max_size` - The group of elements in the sets
    ///
    /// * `set_size` - The exact size of each set iterated through
    ///
    fn each_set_exact_zero(max_size: Self::Group, set_size: u32) -> Self::EachSetExactZero;

    /// Iterates through each set that doesn't contain 0 of some size, with
    /// elements in a given group
    ///
    /// # Arguments
    ///
    /// * `max_size` - The group of elements in the sets
    ///
    /// * `set_size` - The exact size of each set iterated through
    ///
    fn each_set_exact_no_zero(max_size: Self::Group, set_size: u32) -> Self::EachSetExactNoZero;

    /// Returns whether or not this set is empty
    fn is_empty(&self) -> bool;

    /// Returns whether or not this set contains every element in
    /// a given group
    fn is_full(&self, n: Self::Group) -> bool;

    /// Returns the size of this set
    fn size(&self) -> u32;

    /// Adds an element to this set
    fn add(&mut self, i: Self::Element);

    /// Returns whether or not this set contains a given element
    fn has(&self, i: &Self::Element) -> bool;

    /// Compute the intersection of this set with another
    fn intersect(&mut self, other: Self);

    /// Compute whether or not this set contains 0 of a given group
    fn zero_free(&self, n: Self::Group) -> bool {
        !self.has(&n.zero())
    }
}

// Implementations of Group for the representations used
// for FastSet and exactset respectively

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
        GElem(vec![0u32; (**self).len()])
    }

    fn gsize(&self) -> u32 {
        let mut res = 1;
        for num in (**self).iter() {
            res *= num;
        }
        res
    }
}

impl HFolds for Vec<GElem> {
    type Group = Rc<Vec<u32>>;
    type Element = GElem;

    fn hfold_sumset(&self, h: u32, n: Self::Group) -> Self {
        exactset::hfold_sumset(&self, h, n).into_iter().collect()
    }
    fn hfold_interval_sumset(&self, hs: (u32, u32), n: Self::Group) -> Self {
        exactset::hfold_interval_sumset(&self, hs, n)
            .into_iter()
            .collect()
    }
    fn hfold_restricted_sumset(&self, h: u32, n: Self::Group) -> Self {
        exactset::hfold_restricted_sumset(&self, h, n)
            .into_iter()
            .collect()
    }
    fn hfold_interval_restricted_sumset(&self, hs: (u32, u32), n: Self::Group) -> Self {
        exactset::hfold_interval_restricted_sumset(&self, hs, n)
            .into_iter()
            .collect()
    }
    fn hfold_signed_sumset(&self, h: u32, n: Self::Group) -> Self {
        exactset::hfold_signed_sumset(&self, h, n)
            .into_iter()
            .collect()
    }
    fn hfold_interval_signed_sumset(&self, hs: (u32, u32), n: Self::Group) -> Self {
        exactset::hfold_interval_signed_sumset(&self, hs, n)
            .into_iter()
            .collect()
    }
    fn hfold_restricted_signed_sumset(&self, h: u32, n: Self::Group) -> Self {
        exactset::hfold_restricted_signed_sumset(&self, h, n)
            .into_iter()
            .collect()
    }
    fn hfold_interval_restricted_signed_sumset(&self, hs: (u32, u32), n: Self::Group) -> Self {
        exactset::hfold_interval_restricted_signed_sumset(&self, hs, n)
            .into_iter()
            .collect()
    }
}

impl<B: BitSetContents> SetLike for fastset::FastSet<B> {
    type EachSetExact = fastset::EachSetExact<B>;
    type EachSetExactZero = fastset::EachSetExactZero<B>;
    type EachSetExactNoZero = fastset::EachSetExactNoZero<B>;
    fn empty() -> Self {
        fastset::empty_set()
    }
    fn singleton(i: u32) -> Self {
        fastset::singleton(i)
    }
    fn each_set_exact(max_size: u32, set_size: u32) -> fastset::EachSetExact<B> {
        fastset::each_set_exact(max_size, set_size)
    }
    fn each_set_exact_zero(max_size: u32, set_size: u32) -> fastset::EachSetExactZero<B> {
        fastset::each_set_exact_zero(max_size, set_size)
    }
    fn each_set_exact_no_zero(max_size: u32, set_size: u32) -> fastset::EachSetExactNoZero<B> {
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

    fn intersect(&mut self, other: Self) {
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
