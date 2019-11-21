#![feature(specialization)]
#![feature(proc_macro_hygiene)]
// Necessary for py_module_initializer! to disable unsafe warnings
#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate cpython;

extern crate paste;

extern crate addcomb_comp;

use cpython::PyObject;

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
