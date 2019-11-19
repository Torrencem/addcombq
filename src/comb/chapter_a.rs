macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

use crate::setlike::{Group, SetLike};

pub fn nu<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_sumset(h, n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {:?}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        greatest_set.hfold_sumset(h, n)
    );
    curr_greatest
}

pub fn nu_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_interval_sumset((ia, ib), n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {:?}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        greatest_set.hfold_interval_sumset((ia, ib), n)
    );
    curr_greatest
}

pub fn nu_signed<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_signed_sumset(h, n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {:?}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        greatest_set.hfold_signed_sumset(h, n)
    );
    curr_greatest
}

pub fn nu_signed_interval<S: SetLike>(
    n: S::Group,
    m: u32,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_interval_signed_sumset((ia, ib), n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {:?}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        greatest_set.hfold_interval_signed_sumset((ia, ib), n)
    );
    curr_greatest
}

pub fn nu_restricted<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_restricted_sumset(h, n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {:?}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        greatest_set.hfold_restricted_sumset(h, n)
    );
    curr_greatest
}

pub fn nu_restricted_interval<S: SetLike>(
    n: S::Group,
    m: u32,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a
            .hfold_interval_restricted_sumset((ia, ib), n.clone())
            .size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {:?}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        greatest_set.hfold_interval_restricted_sumset((ia, ib), n)
    );
    curr_greatest
}

pub fn nu_signed_restricted<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_restricted_signed_sumset(h, n.clone()).size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {:?}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        greatest_set.hfold_restricted_signed_sumset(h, n)
    );
    curr_greatest
}

pub fn nu_signed_restricted_interval<S: SetLike>(
    n: S::Group,
    m: u32,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    let mut greatest_set = S::empty();
    let mut curr_greatest = 0;
    for a in S::each_set_exact(n.clone(), m) {
        let size = a
            .hfold_interval_restricted_signed_sumset((ia, ib), n.clone())
            .size();
        if size > curr_greatest {
            if size == n.gsize() {
                info!(verbose, "Found spanning set: {:?}", a);
                return n.gsize();
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        greatest_set.hfold_interval_restricted_signed_sumset((ia, ib), n)
    );
    curr_greatest
}
