use crate::fastset::*;
use crate::setlike::HFolds;
use std::cmp;

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn mu(n: u32, k: u32, l: u32, verbose: bool) -> u32 {
    if k == l {
        return 0;
    }
    for m in 1..n {
        let mut found = false;
        for a in each_set_exact(n, m) {
            let mut k_a = a.hfoldsumset(k, n);
            let l_a = a.hfoldsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!(verbose, "For m={}, found {}, which is sum-free", m, a);
                info!(verbose, "(kA = {}, lA = {})", a.hfoldsumset(k, n), l_a);
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    return n - 1;
}

pub fn mu_signed(n: u32, k: u32, l: u32, verbose: bool) -> u32 {
    if k == l {
        return 0;
    }
    for m in 1..n {
        let mut found = false;
        for a in each_set_exact(n, m) {
            let mut k_a = a.hfoldsignedsumset(k, n);
            let l_a = a.hfoldsignedsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!(verbose, "For m={}, found {}, which is sum-free", m, a);
                info!(
                    verbose,
                    "(kA = {}, lA = {})",
                    a.hfoldsignedsumset(k, n),
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
    return n - 1;
}

pub fn mu_restricted(n: u32, k: u32, l: u32, verbose: bool) -> u32 {
    if k == l {
        return 0;
    }
    if k > n || l > n {
        return n;
    }
    let mut lower_bound = 1;
    if l == 1 && (n == k * (k * k - 1)) {
        lower_bound = cmp::max(n / (k + 1) + k - 1, k * k);
        info!(verbose, "Using lower bound: {}", lower_bound);
    }
    for m in lower_bound..n {
        let mut found = false;
        for a in each_set_exact(n, m) {
            let mut k_a = a.hfoldrestrictedsumset(k, n);
            let l_a = a.hfoldrestrictedsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!(verbose, "For m={}, found {}, which is sum-free", m, a);
                info!(
                    verbose,
                    "(kA = {}, lA = {})",
                    a.hfoldrestrictedsumset(k, n),
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
    return n - 1;
}

pub fn mu_signed_restricted(n: u32, k: u32, l: u32, verbose: bool) -> u32 {
    if k == l {
        return 0;
    }
    if k > n || l > n {
        return n;
    }
    for m in 1..n {
        let mut found = false;
        for a in each_set_exact(n, m) {
            let mut k_a = a.hfoldrestrictedsignedsumset(k, n);
            let l_a = a.hfoldrestrictedsignedsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!(verbose, "For m={}, found {}, which is sum-free", m, a);
                info!(
                    verbose,
                    "(kA = {}, lA = {})",
                    a.hfoldrestrictedsignedsumset(k, n),
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
    return n - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Based on page 358, 359
    #[test]
    fn test_mu_res() {
        assert_eq!(mu_restricted(9, 3, 1, false), 4);
        assert_eq!(mu_restricted(14, 3, 1, false), 5);
        assert_eq!(mu_restricted(19, 3, 1, false), 6);
        assert_eq!(mu_restricted(15, 4, 2, false), 5);
        assert_eq!(mu_restricted(12, 4, 3, false), 6);

        assert_eq!(mu_restricted(6, 8, 1, false), 6);
        assert_eq!(mu_restricted(11, 6, 1, false), 6);
    }
}
