#![feature(specialization)]
#![feature(proc_macro_hygiene)]
// Necessary for py_module_initializer! to disable unsafe warnings
#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate cpython;

extern crate paste;

extern crate addcomb_comp;

extern crate cachetools;

use cpython::PyObject;

mod public;

// A macro that evaluates to the number of arguments passed to it
macro_rules! num_args {
    () => { 0 };
    ($_e: expr $(, $rest: expr)*) => { 1 + num_args!($($rest),*) }
}

macro_rules! add_bfunc_bindings_to_mod {
    ($py:ident, $pymod:ident, $fn_name:ident, $fn_id:ident, $fn_var_name:ident, $($ex_args:ident | $ex_arg_type:ident),+) => {
        let docstring = include_str!(concat!("../doc/compiled/", stringify!($fn_name), ".md"));
        let $fn_var_name = py_fn!($py, $fn_name(n: PyObject, $($ex_args : $ex_arg_type),+ , verbose: bool = false));
        $pymod.add($py, concat!("_", stringify!($fn_name)), &$fn_var_name)?;
        let numargs = num_args!($($ex_args),+) + 1; // Plus one for group
        let $fn_var_name = wrap_binding($py, $fn_var_name, numargs, $fn_id(), docstring)?;
        $pymod.add($py, stringify!($fn_name), &$fn_var_name)?;
    };
}

macro_rules! add_variations_to_mod {
    ($py:ident, $pymod:ident, $name:tt, $fn_id:ident, $($ex_args:ident | $ex_arg_type:ident),+) => {
        paste::item! {
            add_bfunc_bindings_to_mod!($py, $pymod, $name, $fn_id, tmp_, $($ex_args | $ex_arg_type),+);
            add_bfunc_bindings_to_mod!($py, $pymod, [<$name _signed>], $fn_id, tmp_, $($ex_args | $ex_arg_type),+);
            add_bfunc_bindings_to_mod!($py, $pymod, [<$name _restricted>], $fn_id, tmp_, $($ex_args | $ex_arg_type),+);
            add_bfunc_bindings_to_mod!($py, $pymod, [<$name _signed_restricted>], $fn_id, tmp_, $($ex_args | $ex_arg_type),+);
        }
    }
}

py_module_initializer!(addcomb, initaddcomb, PyInit_addcomb, |py, m| {
    m.add(py, "__name__", "addcomb")?;
    m.add(py, "__package__", "addcomb")?;
    m.add(py, "__doc__", include_str!("../doc/blurb.txt"))?;

    use public::*;
    
    let mut _fn_id = 0u8;
    let mut fn_id = || {_fn_id += 1; _fn_id - 1 };

    add_variations_to_mod!(py, m, nu, fn_id, m | u32, h | PyObject);

    add_variations_to_mod!(py, m, phi, fn_id, h | PyObject);

    add_variations_to_mod!(py, m, sigma, fn_id, h | PyObject);

    add_variations_to_mod!(py, m, rho, fn_id, m | u32, h | PyObject);

    add_variations_to_mod!(py, m, chi, fn_id, h | PyObject);

    add_variations_to_mod!(py, m, tau, fn_id, h | PyObject);

    add_variations_to_mod!(py, m, mu, fn_id, k | u32, l | u32);

    // Add extra combinatorics functions
    
    m.add(py, "a", py_fn!(py, comb_a(h: u32, m: u32)))?;
    m.add(py, "c", py_fn!(py, comb_c(h: u32, m: u32)))?;
    m.add(py, "choose", py_fn!(py, comb_choose(n: u32, k: u32)))?;
    m.add(py, "v", py_fn!(py, comb_v(g: u32, n: u32, h: u32)))?;
    m.add(py, "v_signed", py_fn!(py, comb_v_signed(n: u32, h: u32)))?;

    Ok(())
});
