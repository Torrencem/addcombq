#![feature(concat_idents)]
#![feature(specialization)]
#![feature(proc_macro_hygiene)]
#![allow(unused_imports)] // TODO
#![allow(dead_code)] // TODO

#[macro_use]
extern crate cpython;

#[macro_use]
extern crate specialize;

#[macro_use]
extern crate paste;

use cpython::PyObject;

pub mod comb;
pub mod exactset;
pub mod fastset;
pub mod setlike;

mod public;

macro_rules! add_bindings_to_mod {
    ($py:ident, $pymod:ident, $fn_name:ident, $fn_var_name:ident, $($ex_args:ident | $ex_arg_type:ident),+) => {
        let docstring = include_str!(concat!("../doc/compiled/", stringify!($fn_name), ".md"));
        let $fn_var_name = py_fn!($py, $fn_name(n: PyObject, $($ex_args : $ex_arg_type),+ , verbose: bool = false));
        $pymod.add($py, concat!("_", stringify!($fn_name)), &$fn_var_name)?;
        let $fn_var_name = wrap_binding($py, $fn_var_name, docstring)?;
        $pymod.add($py, stringify!($fn_name), &$fn_var_name)?;
    };
}

macro_rules! add_variations_to_mod {
    ($py:ident, $pymod:ident, $name:tt, $($ex_args:ident | $ex_arg_type:ident),+) => {
        paste::item! {
            add_bindings_to_mod!($py, $pymod, $name, a_, $($ex_args | $ex_arg_type),+);
            add_bindings_to_mod!($py, $pymod, [<$name _signed>], a_, $($ex_args | $ex_arg_type),+);
            add_bindings_to_mod!($py, $pymod, [<$name _restricted>], a_, $($ex_args | $ex_arg_type),+);
            add_bindings_to_mod!($py, $pymod, [<$name _signed_restricted>], a_, $($ex_args | $ex_arg_type),+);
        }
    }
}

py_module_initializer!(addcomb, initaddcomb, PyInit_addcomb, |py, m| {
    m.add(py, "__name__", "addcomb")?;
    m.add(py, "__doc__", include_str!("../doc/blurb.txt"))?;

    use public::*;
    
    add_variations_to_mod!(py, m, nu, m | u32, h | PyObject);
    
    add_variations_to_mod!(py, m, phi, h | PyObject);
    
    add_variations_to_mod!(py, m, sigma, h | PyObject);
    
    add_variations_to_mod!(py, m, rho, m | u32, h | PyObject);
    
    add_variations_to_mod!(py, m, chi, h | PyObject);
    
    add_variations_to_mod!(py, m, tau, h | PyObject);
    
    add_variations_to_mod!(py, m, mu, k | u32, l | u32);

    Ok(())
});

#[cfg(test)]
mod tests {
    use crate::fastset::FastSet;
    use crate::exactset::GElem;
    use crate::comb::chapter_a::*;
    use crate::comb::chapter_b::*;
    use crate::comb::chapter_c::*;

    extern crate rand;

    use rand::{Rng, thread_rng};

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
        let mut vars3 = || { (rng.gen_range(1, 13), rng.gen_range(1, 4), rng.gen_range(1, 6)) };
        let mut rng2 = thread_rng();
        let mut vars2 = || { (rng2.gen_range(1, 13), rng2.gen_range(1, 4)) };
        
        comp_all_3!(30, vars3,
                    nu,
                    nu_signed,
                    nu_restricted,
                    nu_signed_restricted);
        
        comp_all_2!(30, vars2,
                    phi,
                    phi_signed,
                    phi_restricted,
                    phi_signed_restricted,
                    sigma,
                    sigma_signed,
                    sigma_restricted,
                    sigma_signed_restricted);
    }

    #[test]
    fn test_tricky_consistency() {
        // Test old consistency errors from previous failures
        // just to make sure there were no regressions
        // If new errors are found, add them here
        comp_fs!(sigma_signed, 10, 3);
        comp_fs!(sigma_signed, 11, 2);
    }
}
