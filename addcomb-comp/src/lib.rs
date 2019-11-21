#![feature(specialization)]

#[macro_use]
extern crate specialize;

pub mod comb;
pub mod exactset;
pub mod fastset;
pub mod setlike;

#[cfg(test)]
mod tests {
    use crate::comb::chapter_a::*;
    use crate::comb::chapter_b::*;
    use crate::comb::chapter_c::*;
    use crate::comb::chapter_d::*;
    use crate::comb::chapter_e::*;
    use crate::comb::chapter_f::*;
    use crate::comb::chapter_g::*;
    use crate::exactset::GElem;
    use crate::fastset::FastSet;

    use crate::setlike::SetLike;

    extern crate rand;

    use rand::{thread_rng, Rng};

    use std::rc::Rc;

    // Setup some simple harnesses for tests
    macro_rules! assert_consistency {
        ($f1:expr, $f2:expr, $a1:expr, $($args:expr),+) => {
            println!("Running test on: {} {:?}", stringify!($f1), ($a1, $($args),+));
            let a = $f1($a1, $($args),+);
            let b = $f2(Rc::new(vec![$a1]), $($args),+);
            assert_eq!(a, b,
                       concat!("Consistency error: ",
                               stringify!($f1),
                               " and ",
                               stringify!($f2),
                               " give different values with arguments: {:?}"
                               ), ($a1, $($args),+)
                       );
        }
    }

    // Make sure a function gives the same values for
    // it's exactset version and fastset version
    macro_rules! comp_fs {
        ($f:tt, $($args:expr),+) => {
            assert_consistency!($f::<FastSet>, $f::<Vec<GElem>>, $($args),+, false);
        }
    }

    macro_rules! comp_all_3 {
        ($iters:expr, $vgen:expr, $($f:ident),+) => {
            $(
            for _ in 0..$iters {
                let (x, y, z) = $vgen();
                comp_fs!($f, x, y, z);
            }
            )+
        }
    }

    // The same as above but for 2 argument functions
    macro_rules! comp_all_2 {
        ($iters:expr, $vgen:expr, $($f:ident),+) => {
            $(
            for _ in 0..$iters {
                let (x, y) = $vgen();
                comp_fs!($f, x, y);
            }
            )+
        }
    }

    #[test]
    fn test_fuzz_consistency() {
        let mut rng = thread_rng();
        let mut vars3 = || {
            (
                rng.gen_range(1, 13),
                rng.gen_range(1, 4),
                rng.gen_range(1, 6),
            )
        };
        let mut rng2 = thread_rng();
        let mut vars2 = || (rng2.gen_range(1, 13), rng2.gen_range(1, 4));

        comp_all_3!(
            30,
            vars3,
            nu,
            nu_signed,
            nu_restricted,
            nu_signed_restricted
        );

        comp_all_2!(
            30,
            vars2,
            phi,
            phi_signed,
            phi_restricted,
            phi_signed_restricted
        );

        comp_all_2!(
            7,
            vars2,
            sigma,
            sigma_signed,
            sigma_restricted,
            sigma_signed_restricted
        );

        comp_all_3!(
            20,
            vars3,
            rho,
            rho_signed,
            rho_restricted,
            rho_signed_restricted
        );

        comp_all_2!(
            30,
            vars2,
            chi,
            chi_signed,
            chi_restricted,
            chi_signed_restricted
        );

        comp_all_2!(
            20,
            vars2,
            tau,
            tau_signed,
            tau_restricted,
            tau_signed_restricted
        );

        comp_all_3!(
            15,
            vars3,
            mu,
            mu_signed,
            mu_restricted,
            mu_signed_restricted
        );
    }

    #[test]
    fn test_tricky_consistency() {
        // Test old consistency errors from previous failures
        // just to make sure there were no regressions
        // If new errors are found, add them here
        comp_fs!(sigma_signed, 10, 3);
        comp_fs!(sigma_signed, 11, 2);

        let s = vec![GElem(vec![1]), GElem(vec![0])];
        assert!(!s.zero_free(Rc::new(vec![7])));
        comp_fs!(tau, 7, 3);
    }
}
