macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

use crate::setlike::{SetLike, Group};

pub fn nu<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldsumset(h, n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(verbose, "(sumsets to:) {}", greatest_set.hfoldsumset(h, n));
    curr_greatest
}

pub fn nu_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldintervalsumset((ia, ib), n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {}",
        greatest_set.hfoldintervalsumset((ia, ib), n)
    );
    curr_greatest
}

pub fn nu_signed<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldsignedsumset(h, n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {}",
        greatest_set.hfoldsignedsumset(h, n)
    );
    curr_greatest
}

pub fn nu_signed_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldintervalsignedsumset((ia, ib), n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {}",
        greatest_set.hfoldintervalsignedsumset((ia, ib), n)
    );
    curr_greatest
}

pub fn nu_restricted<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldrestrictedsumset(h, n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {}",
        greatest_set.hfoldrestrictedsumset(h, n)
    );
    curr_greatest
}

pub fn nu_restricted_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldintervalrestrictedsumset((ia, ib), n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {}",
        greatest_set.hfoldintervalrestrictedsumset((ia, ib), n)
    );
    curr_greatest
}

pub fn nu_signed_restricted<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldrestrictedsignedsumset(h, n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {}",
        greatest_set.hfoldrestrictedsignedsumset(h, n)
    );
    curr_greatest
}

pub fn nu_signed_restricted_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfoldintervalrestrictedsignedsumset((ia, ib), n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {}",
        greatest_set.hfoldintervalrestrictedsignedsumset((ia, ib), n)
    );
    curr_greatest
}
