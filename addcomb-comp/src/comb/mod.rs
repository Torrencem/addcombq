use itertools::max;
use std::cmp;

pub mod chapter_a;
pub mod chapter_b;
pub mod chapter_c;
pub mod chapter_d;
pub mod chapter_e;
pub mod chapter_f;
pub mod chapter_g;

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

pub fn factorial(x: u32) -> u32 {
    let mut prod = 1;
    for val in 1..=x {
        prod *= val;
    }
    prod
}

pub fn gcd(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

pub fn v(g: u32, n: u32, h: u32) -> u32 {
    max((1..=n/2)
        .chain([n].iter().map(|&x| x))
        .filter(|x| n % x == 0)
        .map(|d| {
            if d == 1 || gcd(d, g) > (d - 1) {
                0
            } else {
                (((d - 1 - gcd(d, g)) / h) + 1) * (n / d)
            }
        })
    )
    .unwrap()
}

pub fn v_signed(n: u32, h: u32) -> u32 {
    max((1..=n/2)
        .chain([n].iter().map(|&x| x))
        .filter(|x| n % x == 0)
        .map(|d| {
            if d == 1 {
                0
            } else {
                (2 * ((d - 2) / (2 * h)) + 1) * (n / d)
            }
        })
    ).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Compare with the 4.3 table
    #[test]
    pub fn test_v() {
        let correct_table = vec![
            1,  1,  1,  0,  0,  1,  1,  1,
            1,  0,  1,  1,  1,  1,  0,  1,
            2,  2,  2,  1,  0,  2,  2,  2,
            2,  2,  1,  1,  1,  1,  1,  0,
            3,  3,  3,  2,  2,  3,  3,  3,
            2,  2,  2,  2,  2,  2,  2,  2,
            4,  4,  4,  2,  1,  4,  4,  4,
            3,  2,  3,  3,  3,  3,  2,  3,
            5,  5,  5,  2,  2,  5,  5,  5,
            4,  4,  3,  3,  3,  2,  2,  2,
            6,  6,  6,  4,  4,  6,  6,  6,
            4,  4,  3,  3,  3,  3,  3,  3,
            7,  7,  7,  4,  4,  7,  7,  7,
            6,  6,  5,  5,  5,  5,  3,  5,
            8,  8,  8,  4,  3,  8,  8,  8,
            6,  6,  4,  4,  4,  4,  4,  4,
            9,  9,  9,  6,  6,  9,  9,  9,
            6,  6,  5,  5,  5,  4,  4,  4,
            10, 10, 10, 5,  4,  10, 10, 10,
            7,  6,  7,  7,  7,  7,  6,  7,
            11, 11, 11, 6,  6,  11, 11, 11,
            8,  8,  6,  6,  6,  5,  5,  5,
            12, 12, 12, 8,  8,  12, 12, 12,
            10, 10, 6,  6,  6,  5,  5,  4,
            13, 13, 13, 6,  6,  13, 13, 13,
            9,  8,  9,  9,  9,  9,  6,  9,
            14, 14, 14, 8,  8,  14, 14, 14,
            10, 10, 7,  7,  7,  6,  6,  6,
            15, 15, 15, 10, 10, 15, 15, 15,
            10, 10, 8,  8,  8,  6,  6,  6,
            16, 16, 16, 8,  7,  16, 16, 16,
            12, 12, 11, 11, 11, 11, 6,  11,
            17, 17, 17, 8,  8,  17, 17, 17,
            14, 14, 10, 10, 10, 10, 10, 10,
            18, 18, 18, 12, 12, 18, 18, 18,
            12, 12, 9,  9,  9,  8,  8,  8,
            19, 19, 19, 10, 10, 19, 19, 19,
            13, 12, 13, 13, 13, 13, 9,  13,
            20, 20, 20, 10, 9,  20, 20, 20,
        ];
        
        let mut indx = 0;
        for n in 2..=40 {
            for (g, h) in &[(1, 3), (3, 3),
                            (1, 4), (2, 4), (4, 4),
                            (1, 5), (3, 5), (5, 5)] {
                assert_eq!(v(*g, n, *h), correct_table[indx], "v{:?}", (*g, n, *h));
                indx += 1;
            }
        }
        
        // Right before problem 4.6
        assert_eq!(v(5, 437, 5), 95);
    }

    #[test]
    pub fn test_v_signed() {
        // Proposition 4.9
        assert_eq!(v_signed(2, 1), 1);
        assert_eq!(v_signed(3, 1), 1);
        assert_eq!(v_signed(10, 1), 9);
        assert_eq!(v_signed(15, 1), 13);

        assert_eq!(v_signed(10, 2), 5);
        assert_eq!(v_signed(8, 2), 4);
        assert_eq!(v_signed(9, 2), 3);
        assert_eq!(v_signed(11, 2), 5);

        assert_eq!(v_signed(10, 3), 5);
        assert_eq!(v_signed(8, 3), 4);
        assert_eq!(v_signed(15, 3), 5);
        assert_eq!(v_signed(35, 3), 11);
        assert_eq!(v_signed(37, 3), 11);
    }

    // Compare with the 2.4 tables
    #[test]
    pub fn test_c_a() {
        let correct_table = vec![
            1,  1,  1,  1,   1,    1,    1, 
            1,  3,  5,  7,   9,    11,   13, 
            1,  5,  13, 25,  41,   61,   85, 
            1,  7,  25, 63,  129,  231,  377, 
            1,  9,  41, 129, 321,  681,  1289, 
            1,  11, 61, 231, 681,  1683, 3653, 
            1,  13, 85, 377, 1289, 3653, 8989,
        ];
        let mut i = 0;
        for k in 0..=6 {
            for j in 0..=6 {
                assert_eq!(a(j, k), correct_table[i]);
                i += 1;
            }
        }

        let correct_table = vec![
            1, 1,  1,  1,   1,   1,    1, 
            0, 2,  2,  2,   2,   2,    2, 
            0, 4,  8,  12,  16,  20,   24, 
            0, 6,  18, 38,  66,  102,  146, 
            0, 8,  32, 88,  192, 360,  608, 
            0, 10, 50, 170, 450, 1002, 1970, 
            0, 12, 72, 292, 912, 2364, 5336,
        ];
        i = 0;
        for k in 0..=6 {
            for j in 0..=6 {
                assert_eq!(c(j, k), correct_table[i]);
                i += 1;
            }
        }
    }
}
