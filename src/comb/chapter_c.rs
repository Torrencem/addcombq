use crate::comb::*;

use crate::setlike::{Group, SetLike};

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn sigma<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let expected = choose(m + h - 1, h);
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_sumset(h, n.clone()).size() == expected {
                info!(verbose, "for m={:?}, found a={:?}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets of the required size");
    return 0;
}

pub fn sigma_interval<S: SetLike>(n: S::Group, s: u32, verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let expected = choose(m + s, s);
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_interval_sumset((0, s), n.clone()).size() == expected {
                info!(verbose, "for m={:?}, found a={:?}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets of the required size");
    return 0;
}

pub fn sigma_signed<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let expected = c(h, m);
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_signed_sumset(h, n.clone()).size() == expected {
                info!(verbose, "for m={:?}, found a={:?}", m, a);
                dbg!(expected);
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets of the required size");
    return 0;
}

pub fn sigma_signed_interval<S: SetLike>(n: S::Group, s: u32, verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let expected = a(m, s);
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_interval_signed_sumset((0, s), n.clone()).size() == expected {
                info!(verbose, "for m={:?}, found a={:?}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets of the required size");
    return 0;
}

pub fn sigma_restricted<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let expected = choose(m, h);
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_restricted_sumset(h, n.clone()).size() == expected {
                info!(verbose, "for m={:?}, found a={:?}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets of the required size");
    return 0;
}

pub fn sigma_restricted_interval<S: SetLike>(n: S::Group, s: u32, verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let expected: u32 = (0..=cmp::min(s, m)).map(|h| choose(m, h)).sum();
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_interval_restricted_sumset((0, s), n.clone()).size() == expected {
                info!(verbose, "for m={:?}, found a={:?}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets of the required size");
    return 0;
}

pub fn sigma_signed_restricted<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let expected = choose(m, h) * (2u32).pow(h);
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_restricted_signed_sumset(h, n.clone()).size() == expected {
                info!(verbose, "for m={:?}, found a={:?}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets of the required size");
    return 0;
}

pub fn sigma_signed_restricted_interval<S: SetLike>(n: S::Group, s: u32, verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let expected: u32 = (0..=cmp::min(s, m))
            .map(|h| choose(m, h) * (2u32).pow(h))
            .sum();
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_interval_restricted_sumset((0, s), n.clone()).size() == expected {
                info!(verbose, "for m={:?}, found a={:?}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets of the required size");
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fastset::FastSet;

    // Verify examples according to table on page 153 (details page 154)
    #[test]
    pub fn test_sigma() {
        for n in 5..10 {
            let expected = (((4.0 * (n as f64) - 3.0).sqrt() + 1.0) / 2.0).floor() as u32;

            assert!(sigma::<FastSet>(n, 2, false) == expected);
        }
    }
}
