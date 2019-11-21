use crate::setlike::{Group, SetLike};
use std::cmp;

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn mu<S: SetLike>(n: S::Group, k: u32, l: u32, verbose: bool) -> u32 {
    if k == l {
        return 0;
    }
    for m in 1..n.gsize() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            let mut k_a = a.hfold_sumset(k, n.clone());
            let l_a = a.hfold_sumset(l, n.clone());
            k_a.intersect(l_a.clone());
            if k_a.is_empty() {
                info!(verbose, "For m={:?}, found {:?}, which is sum-free", m, a);
                info!(
                    verbose,
                    "(kA = {:?}, lA = {:?})",
                    a.hfold_sumset(k, n.clone()),
                    l_a
                );
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    return n.gsize() - 1;
}

pub fn mu_signed<S: SetLike>(n: S::Group, k: u32, l: u32, verbose: bool) -> u32 {
    if k == l {
        return 0;
    }
    for m in 1..n.gsize() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            let mut k_a = a.hfold_signed_sumset(k, n.clone());
            let l_a = a.hfold_signed_sumset(l, n.clone());
            k_a.intersect(l_a.clone());
            if k_a.is_empty() {
                info!(verbose, "For m={:?}, found {:?}, which is sum-free", m, a);
                info!(
                    verbose,
                    "(kA = {:?}, lA = {:?})",
                    a.hfold_signed_sumset(k, n.clone()),
                    l_a
                );
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    return n.gsize() - 1;
}

pub fn mu_restricted<S: SetLike>(n: S::Group, k: u32, l: u32, verbose: bool) -> u32 {
    if k == l {
        return 0;
    }
    if k > n.gsize() || l > n.gsize() {
        return n.gsize();
    }
    let mut lower_bound = 1;
    let val = &n;
    if let Some(&n) = constrain!(ref val as u32) {
        if l == 1 && (n == k * (k * k - 1)) {
            lower_bound = cmp::max(n / (k + 1) + k - 1, k * k);
            info!(verbose, "Using lower bound: {:?}", lower_bound);
        }
    }
    for m in lower_bound..n.gsize() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            let mut k_a = a.hfold_restricted_sumset(k, n.clone());
            let l_a = a.hfold_restricted_sumset(l, n.clone());
            k_a.intersect(l_a.clone());
            if k_a.is_empty() {
                info!(verbose, "For m={:?}, found {:?}, which is sum-free", m, a);
                info!(
                    verbose,
                    "(kA = {:?}, lA = {:?})",
                    a.hfold_restricted_sumset(k, n.clone()),
                    l_a
                );
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    return n.gsize() - 1;
}

pub fn mu_signed_restricted<S: SetLike>(n: S::Group, k: u32, l: u32, verbose: bool) -> u32 {
    if k == l {
        return 0;
    }
    if k > n.gsize() || l > n.gsize() {
        return n.gsize();
    }
    for m in 1..n.gsize() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            let mut k_a = a.hfold_restricted_signed_sumset(k, n.clone());
            let l_a = a.hfold_restricted_signed_sumset(l, n.clone());
            k_a.intersect(l_a.clone());
            if k_a.is_empty() {
                info!(verbose, "For m={:?}, found {:?}, which is sum-free", m, a);
                info!(
                    verbose,
                    "(kA = {:?}, lA = {:?})",
                    a.hfold_restricted_signed_sumset(k, n.clone()),
                    l_a
                );
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    return n.gsize() - 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fastset::FastSet;

    // Based on page 358, 359
    #[test]
    fn test_mu_res() {
        assert_eq!(mu_restricted::<FastSet>(9, 3, 1, false), 4);
        assert_eq!(mu_restricted::<FastSet>(14, 3, 1, false), 5);
        assert_eq!(mu_restricted::<FastSet>(19, 3, 1, false), 6);
        assert_eq!(mu_restricted::<FastSet>(15, 4, 2, false), 5);
        assert_eq!(mu_restricted::<FastSet>(12, 4, 3, false), 6);

        assert_eq!(mu_restricted::<FastSet>(6, 8, 1, false), 6);
        assert_eq!(mu_restricted::<FastSet>(11, 6, 1, false), 6);
    }
}
