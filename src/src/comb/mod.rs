use itertools::max;
use std::cmp;

pub mod chapter_a;
// pub mod chapter_b;
pub mod chapter_c;
pub mod chapter_d;
pub mod chapter_e;
pub mod chapter_f;
// pub mod chapter_g;

pub mod hfolds;

pub fn choose(n: u32, k: u32) -> u32 {
    if k == 0 || n == 0 {
        1
    } else {
        (n * choose(n - 1, k - 1)) / k
    }
}

pub fn c(h: u32, m: u32) -> u32 {
    if m == 0 {
        return 1;
    }
    if h == 0 {
        return 0;
    }
    (1..=cmp::min(m, h - 1) + 1)
        .map(|i| choose(m, i) * choose(h - 1, i - 1) * 2u32.pow(i))
        .sum()
}

pub fn a(h: u32, m: u32) -> u32 {
    if h == 0 || m == 0 {
        return 1;
    }
    (0..=cmp::min(m, h) + 1)
        .map(|i| choose(m, i) * choose(h, i) * 2u32.pow(i))
        .sum()
}

#[inline]
pub fn factorial(x: u32) -> u32 {
    let mut prod = 1;
    for val in 1..=x {
        prod *= val;
    }
    prod
}

// fn slow_prime(n: usize) -> bool {
//     for x in 2..=(n as f32).sqrt().ceil() as usize {
//         if x % n == 0 {
//             return false;
//         }
//     }
//     return true;
// }

// lazy_static! {
//     static ref PRIMES: [bool; 500] = {
//         let mut p = [false; 500];
//         for i in 0..499 {
//             p[i] = slow_prime(i);
//         }
//         p
//     };
// }

// #[inline]
// pub fn prime(n: u32) -> bool {
//     PRIMES[n as usize]
// }

// Somewhat slow, as it includes n/2 .. n when
// it doesn't have to, but concise
#[inline]
pub fn divisors(n: u32) -> Vec<u32> {
    (1..=n).filter(|x| n % x == 0).collect()
}

pub fn gcd(a: u32, b: u32) -> u32 {
    if a == b {
        a
    } else if a % b == 0 {
        b
    } else if b % a == 0 {
        a
    } else if a == 1 {
        1
    } else if b == 1 {
        1
    } else {
        if a > b {
            gcd(a % b, b)
        } else {
            gcd(b % a, b)
        }
    }
}

pub fn v(g: u32, n: u32, h: u32) -> u32 {
    max(divisors(n).iter().map(|d| {
        if *d == 1 || gcd(*d, g) > (d - 1) {
            0
        } else {
            (((d - 1 - gcd(*d, g)) / h) + 1) * (n / d)
        }
    }))
    .unwrap()
}

pub fn is_invariant(v: &Vec<u32>) -> bool {
    for i in 1..v.len() {
        if v[i - 1] % v[i] != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // Compare with the 2.4 tables
    // (page 27) TODO
    #[test]
    pub fn test_c_a() {
        println!("a(j, k): \n");
        for k in 0..=6 {
            for j in 0..=6 {
                print!("{} ", a(j, k));
            }
            println!("");
        }

        println!("\nc(j, k): \n");
        for k in 0..=6 {
            for j in 0..=6 {
                print!("{} ", c(j, k));
            }
            println!("");
        }
    }
}
