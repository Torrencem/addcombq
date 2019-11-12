use crate::fastset::*;

use crate::setlike::{SetLike, Group};

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn rho<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldsumset(h, n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(verbose, "(sumsets to:) {:?}", smallest_set.hfoldsumset(h, n));
    curr_smallest
}

pub fn rho_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldintervalsumset((ia, ib), n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfoldintervalsumset((ia, ib), n)
    );
    curr_smallest
}

pub fn rho_signed<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldsignedsumset(h, n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfoldsignedsumset(h, n)
    );
    curr_smallest
}

pub fn rho_signed_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldintervalsignedsumset((ia, ib), n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfoldintervalsignedsumset((ia, ib), n)
    );
    curr_smallest
}

pub fn rho_restricted<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldrestrictedsumset(h, n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfoldrestrictedsumset(h, n)
    );
    curr_smallest
}

pub fn rho_restricted_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldintervalrestrictedsumset((ia, ib), n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfoldintervalrestrictedsumset((ia, ib), n)
    );
    curr_smallest
}

pub fn rho_signed_restricted<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldrestrictedsignedsumset(h, n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfoldrestrictedsignedsumset(h, n)
    );
    curr_smallest
}

pub fn rho_signed_restricted_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldintervalrestrictedsignedsumset((ia, ib), n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfoldintervalrestrictedsignedsumset((ia, ib), n)
    );
    curr_smallest
}
