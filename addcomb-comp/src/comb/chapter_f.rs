use crate::setlike::{Group, SetLike};

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn tau<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact_no_zero(n.clone(), m) {
            if a.hfold_sumset(h, n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", a.hfold_sumset(h, n.clone()));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets which give zero-free sumsets");
    return 0;
}

pub fn tau_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact_no_zero(n.clone(), m) {
            if a.hfold_interval_sumset((ia, ib), n.clone())
                .zero_free(n.clone())
            {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_interval_sumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets which give zero-free sumsets");
    return 0;
}

pub fn tau_restricted<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    // Theorem F.88
    let val = &n;
    if let Some(&n) = constrain!(ref val as u32) {
        if n >= 12 && n % 2 == 0 && (3 <= h) && (h <= n - 1) && (h % 2 == 1) {
            if h == 1 {
                return n - 1;
            }
            if (3 <= h) && (h <= n / 2 - 2) {
                return n / 2;
            }
            if h == n / 2 - 1 {
                return n / 2 + 1;
            }
            if (n / 2 <= h) && (h <= n - 2) {
                return h + 1;
            }
            // h = n - 1 (guaranteed)
            return n - 1;
        }
    }
    if n.gsize() == 1 {
        return 1;
    }
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_restricted_sumset(h, n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_restricted_sumset(h, n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets which give zero-free sumsets");
    return 0;
}

pub fn tau_restricted_interval<S: SetLike>(
    n: S::Group,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_interval_restricted_sumset((ia, ib), n.clone())
                .zero_free(n.clone())
            {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_interval_restricted_sumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets which give zero-free sumsets");
    return 0;
}

pub fn tau_signed<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact_no_zero(n.clone(), m) {
            if a.hfold_signed_sumset(h, n.clone()).zero_free(n.clone()) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_signed_sumset(h, n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets which give zero-free sumsets");
    return 0;
}

pub fn tau_signed_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in (1..n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact_no_zero(n.clone(), m) {
            if a.hfold_interval_signed_sumset((ia, ib), n.clone())
                .zero_free(n.clone())
            {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_interval_signed_sumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets which give zero-free sumsets");
    return 0;
}

pub fn tau_signed_restricted<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_restricted_signed_sumset(h, n.clone())
                .zero_free(n.clone())
            {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_restricted_signed_sumset(h, n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets which give zero-free sumsets");
    return 0;
}

pub fn tau_signed_restricted_interval<S: SetLike>(
    n: S::Group,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    for m in (1..=n.gsize()).rev() {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if a.hfold_interval_restricted_signed_sumset((ia, ib), n.clone())
                .zero_free(n.clone())
            {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_interval_restricted_signed_sumset((ia, ib), n.clone())
                );
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    info!(verbose, "Found no sets which give zero-free sumsets");
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fastset::FastSet;

    // Page 297
    #[test]
    fn test_tau_restricted() {
        let correct_table: Vec<u32> = vec![1, 2, 2, 3, 4, 4, 4, 5, 6, 6, 6, 6, 6, 7, 8, 8, 8, 9];
        let actual_table: Vec<u32> = (1..=18)
            .map(|n| tau_restricted::<FastSet>(n, 3, false))
            .collect();
        assert_eq!(correct_table, actual_table);
    }
}
