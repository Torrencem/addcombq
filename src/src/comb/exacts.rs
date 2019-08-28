use crate::exactset::*;
use std::cmp;
use crate::comb::*;

macro_rules! info {
    ($verb_cond:ident, $( $arg:tt )+) => {
        if $verb_cond {
            println!($($arg)+);
        }
    };
}

pub fn nu_exact(gname: &[u32], m: u32, h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldsumset(&a,  h, &mod_v).len() as u32;
        if size > curr_greatest {
            if size == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return gsize(&mod_v);
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldsumset(&greatest_set, h, &mod_v));
    curr_greatest
}

pub fn nu_interval_exact(gname: &[u32], m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldintervalsumset(&a,  (ia, ib), &mod_v).len() as u32;
        if size > curr_greatest {
            if size == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return gsize(&mod_v);
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldintervalsumset(&greatest_set, (ia, ib), &mod_v));
    curr_greatest
}

pub fn nu_signed_exact(gname: &[u32], m: u32, h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldsignedsumset(&a,  h, &mod_v).len() as u32;
        if size > curr_greatest {
            if size == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return gsize(&mod_v);
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldsignedsumset(&greatest_set, h, &mod_v));
    curr_greatest
}

pub fn nu_signed_interval_exact(gname: &[u32], m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldintervalsignedsumset(&a,  (ia, ib), &mod_v).len() as u32;
        if size > curr_greatest {
            if size == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return gsize(&mod_v);
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldintervalsignedsumset(&greatest_set, (ia, ib), &mod_v));
    curr_greatest
}

pub fn nu_restricted_exact(gname: &[u32], m: u32, h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldrestrictedsumset(&a,  h, &mod_v).len() as u32;
        if size > curr_greatest {
            if size == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return gsize(&mod_v);
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldrestrictedsumset(&greatest_set, h, &mod_v));
    curr_greatest
}

pub fn nu_restricted_interval_exact(gname: &[u32], m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldintervalrestrictedsumset(&a,  (ia, ib), &mod_v).len() as u32;
        if size > curr_greatest {
            if size == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return gsize(&mod_v);
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldintervalrestrictedsumset(&greatest_set, (ia, ib), &mod_v));
    curr_greatest
}

pub fn nu_signed_restricted_exact(gname: &[u32], m: u32, h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldrestrictedsignedsumset(&a,  h, &mod_v).len() as u32;
        if size > curr_greatest {
            if size == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return gsize(&mod_v);
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldrestrictedsignedsumset(&greatest_set, h, &mod_v));
    curr_greatest
}

pub fn nu_signed_restricted_interval_exact(gname: &[u32], m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldintervalrestrictedsignedsumset(&a,  (ia, ib), &mod_v).len() as u32;
        if size > curr_greatest {
            if size == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return gsize(&mod_v);
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!(verbose, "Set with greatest sumset: {:?}", greatest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldintervalrestrictedsignedsumset(&greatest_set, (ia, ib), &mod_v));
    curr_greatest
}


pub fn phi_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    if gsize(&mod_v) == 1 {
        return 1;
    }
    if h == 1 {
        return gsize(&mod_v);
    }
    let res = _phi_interval(gname, 0, h, verbose);
    res + 1
}

pub fn phi_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    _phi_interval(gname, ia, ib, verbose)
}

fn _phi_interval(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let lower_bound = 1;
    // Proposition B.10
    // if (ia, ib).0 == 0 {
    //     let s = (ia, ib).1;
    //     lower_bound = cmp::max(1, (((factorial(s) * n) as f32).powf(1f32/(s as f32)).ceil() as i32) - (s as i32)) as u32;
    //     info!(verbose, "(Proposition B.10) Using lower bound: {:?}", lower_bound);
    // }

    for m in lower_bound.. {
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalsumset(&a,  (ia, ib), &mod_v).len() as u32 == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return m;
            }
        }
    }
    unreachable!();
}

// TODO: Maybe impliment f and g functions on page 132
//(need an upper bound on n though, maybe read paper?)

pub fn phi_signed_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    if gsize(&mod_v) == 1 {
        return 1;
    }
    for m in 2u32.. {
        for a in each_set_exact(m, &mod_v) {
            if hfoldsignedsumset(&a,  h, &mod_v).len() as u32 == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return m;
            }
        }
    }
    unreachable!();
}

pub fn phi_signed_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 1u32.. {
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalsignedsumset(&a,  (ia, ib), &mod_v).len() as u32 == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return m;
            }
        }
    }
    unreachable!();
}

// Not a very researched function... (page 145)
pub fn phi_restricted_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    if gsize(&mod_v) == 1 {
        return 1;
    }
    if h == 1 {
        return gsize(&mod_v);
    }
    for m in 2u32.. {
        for a in each_set_exact(m, &mod_v) {
            if hfoldrestrictedsumset(&a,  h, &mod_v).len() as u32 == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return m;
            }
        }
    }
    unreachable!();
}

pub fn phi_restricted_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let lower_bound = 1u32;
    // Proposition B.73
    // if (ia, ib) == (0, 2) {
    //     lower_bound = ((((8*n - 7) as f32).sqrt() - 1.0)/2.0).ceil() as u32;
    //     info!(verbose, "(Proposition B.73) Using lower bound: {:?}", lower_bound);
    // }
    for m in lower_bound.. {
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalrestrictedsumset(&a,  (ia, ib), &mod_v).len() as u32 == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return m;
            }
        }
    }
    unreachable!();
}

pub fn phi_signed_restricted_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 2u32.. {
        for a in each_set_exact(m, &mod_v) {
            if hfoldrestrictedsignedsumset(&a,  h, &mod_v).len() as u32 == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return m;
            }
        }
    }
    unreachable!();
}

pub fn phi_signed_restricted_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 1u32.. {
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalrestrictedsignedsumset(&a,  (ia, ib), &mod_v).len() as u32 == gsize(&mod_v) {
                info!(verbose, "Found spanning set: {:?}", a);
                return m;
            }
        }
    }
    unreachable!();
}

pub fn sigma_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    if gname.len() != 1 {
        println!("error: non-cyclic group in sigma variant function");
        return 0;
    }
    let n = gname[0];
    let mod_v = gname.to_vec();
    for m in (1..n).rev() {
        let expected = choose(m + h - 1, h);
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldsumset(&a, h, &mod_v).len() == expected as usize {
                info!(verbose, "for m={}, found a={:?}", m, a);
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

pub fn sigma_interval_exact(gname: &[u32], s: u32, verbose: bool) -> u32 {
    if gname.len() != 1 {
        println!("error: non-cyclic group in sigma variant function");
        return 0;
    }
    let n = gname[0];
    let mod_v = gname.to_vec();
    for m in (1..n).rev() {
        let expected = choose(m + s, s);
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalsumset(&a, (0, s), &mod_v).len() == expected as usize {
                info!(verbose, "for m={}, found a={:?}", m, a);
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

pub fn sigma_signed_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    if gname.len() != 1 {
        println!("error: non-cyclic group in sigma variant function");
        return 0;
    }
    let n = gname[0];
    let mod_v = gname.to_vec();
    for m in (1..n).rev() {
        let expected = c(h, m);
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldsignedsumset(&a, h, &mod_v).len() == expected as usize {
                info!(verbose, "for m={}, found a={:?}", m, a);
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

pub fn sigma_signed_interval_exact(gname: &[u32], s: u32, verbose: bool) -> u32 {
    if gname.len() != 1 {
        println!("error: non-cyclic group in sigma variant function");
        return 0;
    }
    let n = gname[0];
    let mod_v = gname.to_vec();
    for m in (1..n).rev() {
        let expected = a(m, s);
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalsignedsumset(&a, (0, s), &mod_v).len() == expected as usize {
                info!(verbose, "for m={}, found a={:?}", m, a);
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

pub fn sigma_restricted_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    if gname.len() != 1 {
        println!("error: non-cyclic group in sigma variant function");
        return 0;
    }
    let n = gname[0];
    let mod_v = gname.to_vec();
    for m in (1..n).rev() {
        let expected = choose(m, h);
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldrestrictedsumset(&a, h, &mod_v).len() == expected as usize {
                info!(verbose, "for m={}, found a={:?}", m, a);
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

pub fn sigma_restricted_interval_exact(gname: &[u32], s: u32, verbose: bool) -> u32 {
    if gname.len() != 1 {
        println!("error: non-cyclic group in sigma variant function");
        return 0;
    }
    let n = gname[0];
    let mod_v = gname.to_vec();
    for m in (1..n).rev() {
        let expected: u32 = (0..=cmp::min(s, m)).map(|h| choose(m, h)).sum();
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalrestrictedsumset(&a, (0, s), &mod_v).len() == expected as usize{
                info!(verbose, "for m={}, found a={:?}", m, a);
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

pub fn sigma_signed_restricted_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    if gname.len() != 1 {
        println!("error: non-cyclic group in sigma variant function");
        return 0;
    }
    let n = gname[0];
    let mod_v = gname.to_vec();
    for m in (1..n).rev() {
        let expected = choose(m, h)*(2u32).pow(h);
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldrestrictedsignedsumset(&a, h, &mod_v).len() == expected as usize {
                info!(verbose, "for m={}, found a={:?}", m, a);
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

pub fn sigma_signed_restricted_interval_exact(gname: &[u32], s: u32, verbose: bool) -> u32 {
    if gname.len() != 1 {
        println!("error: non-cyclic group in sigma variant function");
        return 0;
    }
    let n = gname[0];
    let mod_v = gname.to_vec();
    for m in (1..n).rev() {
        let expected: u32 = (0..=cmp::min(s, m)).map(|h| choose(m, h)*(2u32).pow(h)).sum();
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalrestrictedsumset(&a, (0, s), &mod_v).len() == expected as usize {
                info!(verbose, "for m={}, found a={:?}", m, a);
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

pub fn rho_exact(gname: &[u32], m: u32, h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut smallest_set = empty_set();
    let mut curr_smallest = gsize(&mod_v);
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldsumset(&a,  h, &mod_v).len() as u32;
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldsumset(&smallest_set, h, &mod_v));
    curr_smallest
}

pub fn rho_interval_exact(gname: &[u32], m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut smallest_set = empty_set();
    let mut curr_smallest = gsize(&mod_v);
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldintervalsumset(&a,  (ia, ib), &mod_v).len() as u32;
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldintervalsumset(&smallest_set, (ia, ib), &mod_v));
    curr_smallest
}

pub fn rho_signed_exact(gname: &[u32], m: u32, h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut smallest_set = empty_set();
    let mut curr_smallest = gsize(&mod_v);
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldsignedsumset(&a,  h, &mod_v).len() as u32;
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldsignedsumset(&smallest_set, h, &mod_v));
    curr_smallest
}


pub fn rho_signed_interval_exact(gname: &[u32], m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut smallest_set = empty_set();
    let mut curr_smallest = gsize(&mod_v);
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldintervalsignedsumset(&a,  (ia, ib), &mod_v).len() as u32;
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldintervalsignedsumset(&smallest_set, (ia, ib), &mod_v));
    curr_smallest
}

pub fn rho_restricted_exact(gname: &[u32], m: u32, h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut smallest_set = empty_set();
    let mut curr_smallest = gsize(&mod_v);
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldrestrictedsumset(&a,  h, &mod_v).len() as u32;
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldrestrictedsumset(&smallest_set, h, &mod_v));
    curr_smallest
}

pub fn rho_restricted_interval_exact(gname: &[u32], m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut smallest_set = empty_set();
    let mut curr_smallest = gsize(&mod_v);
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldintervalrestrictedsumset(&a,  (ia, ib), &mod_v).len() as u32;
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldintervalrestrictedsumset(&smallest_set, (ia, ib), &mod_v));
    curr_smallest
}

pub fn rho_signed_restricted_exact(gname: &[u32], m: u32, h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut smallest_set = empty_set();
    let mut curr_smallest = gsize(&mod_v);
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldrestrictedsignedsumset(&a,  h, &mod_v).len() as u32;
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldrestrictedsignedsumset(&smallest_set, h, &mod_v));
    curr_smallest
}

pub fn rho_signed_restricted_interval_exact(gname: &[u32], m: u32, ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut smallest_set = empty_set();
    let mut curr_smallest = gsize(&mod_v);
    for a in each_set_exact(m, &mod_v) {
        let size = hfoldintervalrestrictedsignedsumset(&a,  (ia, ib), &mod_v).len() as u32;
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!(verbose, "Set with smallest sumset: {:?}", smallest_set);
    info!(verbose, "(sumsets to:) {:?}", hfoldintervalrestrictedsignedsumset(&smallest_set, (ia, ib), &mod_v));
    curr_smallest
}

pub fn chi_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 1.. {
            let mut found = false;
            for a in each_set_exact(m, &mod_v) {
                if hfoldsumset(&a,  h, &mod_v).len() as u32 != gsize(&mod_v) {
                    info!(verbose, "For m={:?}, found {:?}, which doesn't give a full sumset", m, a);
                    info!(verbose, "(gives:) {:?}", hfoldsumset(&a,  h, &mod_v));
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

pub fn chi_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalsumset(&a,  (ia, ib), &mod_v).len() as u32 != gsize(&mod_v) {
                info!(verbose, "For m={:?}, found {:?}, which doesn't give a full sumset", m, a);
                info!(verbose, "(gives:) {:?}", hfoldintervalsumset(&a,  (ia, ib), &mod_v));
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

pub fn chi_signed_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
   for m in 1.. {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldsignedsumset(&a,  h, &mod_v).len() as u32 != gsize(&mod_v) {
                info!(verbose, "For m={:?}, found {:?}, which doesn't give a full sumset", m, a);
                info!(verbose, "(gives:) {:?}", hfoldsignedsumset(&a,  h, &mod_v));
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

pub fn chi_signed_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalsignedsumset(&a,  (ia, ib), &mod_v).len() as u32 != gsize(&mod_v) {
                info!(verbose, "For m={:?}, found {:?}, which doesn't give a full sumset", m, a);
                info!(verbose, "(gives:) {:?}", hfoldintervalsignedsumset(&a,  (ia, ib), &mod_v));
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

pub fn chi_restricted_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldrestrictedsumset(&a,  h, &mod_v).len() as u32 != gsize(&mod_v) {
                info!(verbose, "For m={:?}, found {:?}, which doesn't give a full sumset", m, a);
                info!(verbose, "(gives:) {:?}", hfoldrestrictedsumset(&a,  h, &mod_v));
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

pub fn chi_restricted_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalrestrictedsumset(&a,  (ia, ib), &mod_v).len() as u32 != gsize(&mod_v) {
                info!(verbose, "For m={:?}, found {:?}, which doesn't give a full sumset", m, a);
                info!(verbose, "(gives:) {:?}", hfoldintervalrestrictedsumset(&a,  (ia, ib), &mod_v));
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

pub fn chi_signed_restricted_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldrestrictedsignedsumset(&a,  h, &mod_v).len() as u32 != gsize(&mod_v) {
                info!(verbose, "For m={:?}, found {:?}, which doesn't give a full sumset", m, a);
                info!(verbose, "(gives:) {:?}", hfoldrestrictedsignedsumset(&a,  h, &mod_v));
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

pub fn chi_signed_restricted_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if hfoldintervalrestrictedsignedsumset(&a,  (ia, ib), &mod_v).len() as u32 != gsize(&mod_v) {
                info!(verbose, "For m={:?}, found {:?}, which doesn't give a full sumset", m, a);
                info!(verbose, "(gives:) {:?}", hfoldintervalrestrictedsignedsumset(&a,  (ia, ib), &mod_v));
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

pub fn tau_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in (1..=gsize(&mod_v)).rev() {
        let mut found = false;
        for a in each_set_exact_no_zero(m, &mod_v) {
            if zero_free(hfoldsumset(&a,  h, &mod_v), &mod_v) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", hfoldsumset(&a,  h, &mod_v));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in (1..=gsize(&mod_v)).rev() {
        let mut found = false;
        for a in each_set_exact_no_zero(m, &mod_v) {
            if zero_free(hfoldintervalsumset(&a,  (ia, ib), &mod_v), &mod_v) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", hfoldintervalsumset(&a,  (ia, ib), &mod_v));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_restricted_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    // Theorem F.88
    // if n >= 12 && n % 2 == 0 && (3 <= h) && (h <= n - 1) && (h % 2 == 1) {
    //     if h == 1 {
    //         return n - 1;
    //     }
    //     if (3 <= h) && (h <= n/2 - 2) {
    //         return n / 2;
    //     }
    //     if h == gsize(&mod_v)/2 - 1 {
    //         return n / 2 + 1;
    //     }
    //     if (n/2 <= h) && (h <= n - 2) {
    //         return h + 1;
    //     }
    //     // h = gsize(&mod_v) - 1 (guaranteed)
    //     return n - 1;
    // }
    if gsize(&mod_v) == 1 {
        return 1;
    }
    for m in (1..=gsize(&mod_v)).rev() {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if zero_free(hfoldrestrictedsumset(&a,  h, &mod_v), &mod_v) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", hfoldrestrictedsumset(&a,  h, &mod_v));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_restricted_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in (1..=gsize(&mod_v)).rev() {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if zero_free(hfoldintervalrestrictedsumset(&a,  (ia, ib), &mod_v), &mod_v) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", hfoldintervalrestrictedsumset(&a,  (ia, ib), &mod_v));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_signed_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in (1..=gsize(&mod_v)).rev() {
        let mut found = false;
        for a in each_set_exact_no_zero(m, &mod_v) {
            if zero_free(hfoldsignedsumset(&a,  h, &mod_v), &mod_v) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", hfoldsignedsumset(&a,  h, &mod_v));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_signed_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in (1..gsize(&mod_v)).rev() {
        let mut found = false;
        for a in each_set_exact_no_zero(m, &mod_v) {
            if zero_free(hfoldintervalsignedsumset(&a,  (ia, ib), &mod_v), &mod_v) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", hfoldintervalsignedsumset(&a,  (ia, ib), &mod_v));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_signed_restricted_exact(gname: &[u32], h: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in (1..=gsize(&mod_v)).rev() {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if zero_free(hfoldrestrictedsignedsumset(&a,  h, &mod_v), &mod_v) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", hfoldrestrictedsignedsumset(&a,  h, &mod_v));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn tau_signed_restricted_interval_exact(gname: &[u32], ia: u32, ib: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    for m in (1..=gsize(&mod_v)).rev() {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            if zero_free(hfoldintervalrestrictedsignedsumset(&a,  (ia, ib), &mod_v), &mod_v) {
                info!(verbose, "Found {:?}, which gives a zero-free sumset", a);
                info!(verbose, "(gives:) {:?}", hfoldintervalrestrictedsignedsumset(&a,  (ia, ib), &mod_v));
                found = true;
                break;
            }
        }
        if found {
            return m;
        }
    }
    unreachable!();
}

pub fn mu_exact(gname: &[u32], k: u32, l: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    if k == l {
        return 0;
    }
    for m in 1..gsize(&mod_v) {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            let k_a = hfoldsumset(&a,  k, &mod_v);
            let l_a = hfoldsumset(&a,  l, &mod_v);
            if k_a.intersection(&l_a).collect::<Vec<_>>().len() == 0 {
                info!(verbose, "For m={:?}, found {:?}, which is sum-free", m, a);
                info!(verbose, "(kA = {:?}, lA = {:?})", hfoldsumset(&a,  k, &mod_v), l_a);
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    return gsize(&mod_v) - 1;
}

pub fn mu_signed_exact(gname: &[u32], k: u32, l: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    if k == l {
        return 0;
    }
    for m in 1..gsize(&mod_v) {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            let k_a = hfoldsignedsumset(&a,  k, &mod_v);
            let l_a = hfoldsignedsumset(&a,  l, &mod_v);
            if k_a.intersection(&l_a).collect::<Vec<_>>().len() == 0 {
                info!(verbose, "For m={:?}, found {:?}, which is sum-free", m, a);
                info!(verbose, "(kA = {:?}, lA = {:?})", hfoldsignedsumset(&a,  k, &mod_v), l_a);
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    return gsize(&mod_v) - 1;
}

pub fn mu_restricted_exact(gname: &[u32], k: u32, l: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    let mut lower_bound = 1;
    // Prop G.63
    if is_invariant(&mod_v) {
        let kappa = mod_v[mod_v.len() - 1];
        let n = gsize(&mod_v);
        lower_bound = v(k - l, kappa, k + 1) * n/kappa;
        info!(verbose, "Group was entered in invariant form: n = {}, kappa = {}", n, kappa);
        info!(verbose, "Using lower bound (Proposition G.63): {}", lower_bound);
    } else {
        info!(verbose, "Group was not given in invariant form, so not using G.63 lower bound");
        info!(verbose, "(enter again in invariant form to use this bound");
    }
    if k == l {
        return 0;
    }
    if k > gsize(&mod_v) || l > gsize(&mod_v) {
        return gsize(&mod_v);
    }
    // if l == 1 && (n == k*(k*k - 1)) {
    //     lower_bound = cmp::max(n/(k + 1) + k - 1, k*k);
    //     info!(verbose, "Using lower bound: {:?}", lower_bound);
    // }
    for m in lower_bound..gsize(&mod_v) {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            let k_a = hfoldrestrictedsumset(&a,  k, &mod_v);
            let l_a = hfoldrestrictedsumset(&a,  l, &mod_v);
            if k_a.intersection(&l_a).collect::<Vec<_>>().len() == 0 {
                // verbose.flush_lines();
                info!(verbose, "For m={:?}, found {:?}, which is sum-free", m, a);
                info!(verbose, "(kA = {:?}, lA = {:?})", hfoldrestrictedsumset(&a,  k, &mod_v), l_a);
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    return gsize(&mod_v) - 1;
}

pub fn mu_signed_restricted_exact(gname: &[u32], k: u32, l: u32, verbose: bool) -> u32 {
    let mod_v = gname.to_vec();
    if k == l {
        return 0;
    }
    if k > gsize(&mod_v) || l > gsize(&mod_v) {
        return gsize(&mod_v);
    }
    for m in 1..gsize(&mod_v) {
        let mut found = false;
        for a in each_set_exact(m, &mod_v) {
            let k_a = hfoldrestrictedsignedsumset(&a,  k, &mod_v);
            let l_a = hfoldrestrictedsignedsumset(&a,  l, &mod_v);
            if k_a.intersection(&l_a).collect::<Vec<_>>().len() == 0 {
                info!(verbose, "For m={:?}, found {:?}, which is sum-free", m, a);
                info!(verbose, "(kA = {:?}, lA = {:?})", hfoldrestrictedsignedsumset(&a,  k, &mod_v), l_a);
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    return gsize(&mod_v) - 1;
}