use crate::fastset::*;

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn rho(n: u32, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {}", smallest_set);
    info!(verbose, "(sumsets to:) {}", smallest_set.hfoldsumset(h, n));
    curr_smallest
}

pub fn rho_interval(n: u32, m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsumset((ia, ib), n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {}", smallest_set);
    info!(verbose, "(sumsets to:) {}", smallest_set.hfoldintervalsumset((ia, ib), n));
    curr_smallest
}

pub fn rho_signed(n: u32, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsignedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {}", smallest_set);
    info!(verbose, "(sumsets to:) {}", smallest_set.hfoldsignedsumset(h, n));
    curr_smallest
}


pub fn rho_signed_interval(n: u32, m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsignedsumset((ia, ib), n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {}", smallest_set);
    info!(verbose, "(sumsets to:) {}", smallest_set.hfoldintervalsignedsumset((ia, ib), n));
    curr_smallest
}

pub fn rho_restricted(n: u32, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {}", smallest_set);
    info!(verbose, "(sumsets to:) {}", smallest_set.hfoldrestrictedsumset(h, n));
    curr_smallest
}

pub fn rho_restricted_interval(n: u32, m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsumset((ia, ib), n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {}", smallest_set);
    info!(verbose, "(sumsets to:) {}", smallest_set.hfoldintervalrestrictedsumset((ia, ib), n));
    curr_smallest
}

pub fn rho_signed_restricted(n: u32, m: u32, h: u32, verbose: bool) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsignedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {}", smallest_set);
    info!(verbose, "(sumsets to:) {}", smallest_set.hfoldrestrictedsignedsumset(h, n));
    curr_smallest
}

pub fn rho_signed_restricted_interval(n: u32, m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsignedsumset((ia, ib), n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {}", smallest_set);
    info!(verbose, "(sumsets to:) {}", smallest_set.hfoldintervalrestrictedsignedsumset((ia, ib), n));
    curr_smallest
}