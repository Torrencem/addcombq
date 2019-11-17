use crate::fastset::*;

use crate::setlike::{SetLike, Group};

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn chi<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a.hfoldsumset(h, n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {:?}", a.hfoldsumset(h, n.clone()));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a.hfoldintervalsumset((ia, ib), n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {:?}", a.hfoldintervalsumset((ia, ib), n.clone()));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_signed<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a.hfoldsignedsumset(h, n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {:?}", a.hfoldsignedsumset(h, n.clone()));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_signed_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a.hfoldintervalsignedsumset((ia, ib), n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfoldintervalsignedsumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_restricted<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a.hfoldrestrictedsumset(h, n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {:?}", a.hfoldrestrictedsumset(h, n.clone()));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_restricted_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a.hfoldintervalrestrictedsumset((ia, ib), n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfoldintervalrestrictedsumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_signed_restricted<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a.hfoldrestrictedsignedsumset(h, n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {:?}", a.hfoldrestrictedsignedsumset(h, n.clone()));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_signed_restricted_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a.hfoldintervalrestrictedsignedsumset((ia, ib), n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfoldintervalrestrictedsignedsumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}
