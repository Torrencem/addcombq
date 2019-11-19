use crate::setlike::{Group, SetLike};

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn rho<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_sumset(h, n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfold_sumset(h, n)
    );
    curr_smallest
}

pub fn rho_interval<S: SetLike>(n: S::Group, m: u32, (ia, ib): (u32, u32), verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_interval_sumset((ia, ib), n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfold_interval_sumset((ia, ib), n)
    );
    curr_smallest
}

pub fn rho_signed<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_signed_sumset(h, n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfold_signed_sumset(h, n)
    );
    curr_smallest
}

pub fn rho_signed_interval<S: SetLike>(
    n: S::Group,
    m: u32,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_interval_signed_sumset((ia, ib), n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfold_interval_signed_sumset((ia, ib), n)
    );
    curr_smallest
}

pub fn rho_restricted<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_restricted_sumset(h, n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfold_restricted_sumset(h, n)
    );
    curr_smallest
}

pub fn rho_restricted_interval<S: SetLike>(
    n: S::Group,
    m: u32,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a
            .hfold_interval_restricted_sumset((ia, ib), n.clone())
            .size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfold_interval_restricted_sumset((ia, ib), n)
    );
    curr_smallest
}

pub fn rho_signed_restricted<S: SetLike>(n: S::Group, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a.hfold_restricted_signed_sumset(h, n.clone()).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfold_restricted_signed_sumset(h, n)
    );
    curr_smallest
}

pub fn rho_signed_restricted_interval<S: SetLike>(
    n: S::Group,
    m: u32,
    (ia, ib): (u32, u32),
    verbose: bool,
) -> u32 {
    let mut smallest_set = S::empty();
    let mut curr_smallest = n.gsize();
    for a in S::each_set_exact(n.clone(), m) {
        let size = a
            .hfold_interval_restricted_signed_sumset((ia, ib), n.clone())
            .size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(
        verbose,
        "(sumsets to:) {:?}",
        smallest_set.hfold_interval_restricted_signed_sumset((ia, ib), n)
    );
    curr_smallest
}
