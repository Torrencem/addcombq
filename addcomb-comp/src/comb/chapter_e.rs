use crate::setlike::SetLike;

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
            if !a.hfold_sumset(h, n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(verbose, "(gives:) {:?}", a.hfold_sumset(h, n.clone()));
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
            if !a
                .hfold_interval_sumset((ia, ib), n.clone())
                .is_full(n.clone())
            {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_interval_sumset((ia, ib), n.clone())
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

pub fn chi_signed<S: SetLike>(n: S::Group, h: u32, verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a.hfold_signed_sumset(h, n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_signed_sumset(h, n.clone())
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

pub fn chi_signed_interval<S: SetLike>(n: S::Group, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a
                .hfold_interval_signed_sumset((ia, ib), n.clone())
                .is_full(n.clone())
            {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_interval_signed_sumset((ia, ib), n.clone())
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
            if !a.hfold_restricted_sumset(h, n.clone()).is_full(n.clone()) {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_restricted_sumset(h, n.clone())
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

pub fn chi_restricted_interval<S: SetLike>(
    n: S::Group,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a
                .hfold_interval_restricted_sumset((ia, ib), n.clone())
                .is_full(n.clone())
            {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_interval_restricted_sumset((ia, ib), n.clone())
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
            if !a
                .hfold_restricted_signed_sumset(h, n.clone())
                .is_full(n.clone())
            {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_restricted_signed_sumset(h, n.clone())
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

pub fn chi_signed_restricted_interval<S: SetLike>(
    n: S::Group,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in S::each_set_exact(n.clone(), m) {
            if !a
                .hfold_interval_restricted_signed_sumset((ia, ib), n.clone())
                .is_full(n.clone())
            {
                info!(
                    verbose,
                    "For m={:?}, found {:?}, which doesn't give a full sumset", m, a
                );
                info!(
                    verbose,
                    "(gives:) {:?}",
                    a.hfold_interval_restricted_signed_sumset((ia, ib), n.clone())
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
