use crate::fastset::*;

use crate::setlike::HFolds;

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn chi(n: u32, h: u32, verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldsumset(h, n).isfull(n) {
                info!(
                    verbose,
                    "For m={}, found {}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {}", a.hfoldsumset(h, n));
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

pub fn chi_interval(n: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsumset((ia, ib), n).isfull(n) {
                info!(
                    verbose,
                    "For m={}, found {}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {}", a.hfoldintervalsumset((ia, ib), n));
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

pub fn chi_signed(n: u32, h: u32, verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldsignedsumset(h, n).isfull(n) {
                info!(
                    verbose,
                    "For m={}, found {}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {}", a.hfoldsignedsumset(h, n));
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

pub fn chi_signed_interval(n: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsignedsumset((ia, ib), n).isfull(n) {
                info!(
                    verbose,
                    "For m={}, found {}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {}",
                    a.hfoldintervalsignedsumset((ia, ib), n)
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

pub fn chi_restricted(n: u32, h: u32, verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsumset(h, n).isfull(n) {
                info!(
                    verbose,
                    "For m={}, found {}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {}", a.hfoldrestrictedsumset(h, n));
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

pub fn chi_restricted_interval(n: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsumset((ia, ib), n).isfull(n) {
                info!(
                    verbose,
                    "For m={}, found {}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {}",
                    a.hfoldintervalrestrictedsumset((ia, ib), n)
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

pub fn chi_signed_restricted(n: u32, h: u32, verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                info!(
                    verbose,
                    "For m={}, found {}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {}", a.hfoldrestrictedsignedsumset(h, n));
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

pub fn chi_signed_restricted_interval(n: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsignedsumset((ia, ib), n).isfull(n) {
                info!(
                    verbose,
                    "For m={}, found {}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {}",
                    a.hfoldintervalrestrictedsignedsumset((ia, ib), n)
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
