
macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

use crate::fastset::*;

pub fn nu(n: u32, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsumset(h, n).size();
        if size > curr_greatest {
            if size == n {
                info!(verbose, "Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(verbose, "(sumsets to:) {}", greatest_set.hfoldsumset(h, n));
    curr_greatest
}

pub fn nu_interval(n: u32, m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsumset((ia, ib), n).size();
        if size > curr_greatest {
            if size == n {
                info!(verbose, "Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(verbose, "(sumsets to:) {}", greatest_set.hfoldintervalsumset((ia, ib), n));
    curr_greatest
}

pub fn nu_signed(n: u32, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsignedsumset(h, n).size();
        if size > curr_greatest {
            if size == n {
                info!(verbose, "Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(verbose, "(sumsets to:) {}", greatest_set.hfoldsignedsumset(h, n));
    curr_greatest
}

pub fn nu_signed_interval(n: u32, m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsignedsumset((ia, ib), n).size();
        if size > curr_greatest {
            if size == n {
                info!(verbose, "Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(verbose, "(sumsets to:) {}", greatest_set.hfoldintervalsignedsumset((ia, ib), n));
    curr_greatest
}

pub fn nu_restricted(n: u32, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsumset(h, n).size();
        if size > curr_greatest {
            if size == n {
                info!(verbose, "Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(verbose, "(sumsets to:) {}", greatest_set.hfoldrestrictedsumset(h, n));
    curr_greatest
}

pub fn nu_restricted_interval(n: u32, m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsumset((ia, ib), n).size();
        if size > curr_greatest {
            if size == n {
                info!(verbose, "Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(verbose, "(sumsets to:) {}", greatest_set.hfoldintervalrestrictedsumset((ia, ib), n));
    curr_greatest
}

pub fn nu_signed_restricted(n: u32, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsignedsumset(h, n).size();
        if size > curr_greatest {
            if size == n {
                info!(verbose, "Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(verbose, "(sumsets to:) {}", greatest_set.hfoldrestrictedsignedsumset(h, n));
    curr_greatest
}

pub fn nu_signed_restricted_interval(n: u32, m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsignedsumset((ia, ib), n).size();
        if size > curr_greatest {
            if size == n {
                info!(verbose, "Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {}", greatest_set);
    info!(verbose, "(sumsets to:) {}", greatest_set.hfoldintervalrestrictedsignedsumset((ia, ib), n));
    curr_greatest
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 111
    #[test]
    pub fn test_nu_interval() {
        for n in [6, 10, 17].iter() {
            for m in 3..5 {
                for s in 1..3 {
                    assert!(nu_interval(*n, m, 0, s, false) == nu(*n, m + 1, s, false));
                }
            }
        }
    }
}