#![feature(concat_idents)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate cpython;

use cpython::PyObject;

mod comb;
mod exactset;
mod fastset;
mod setlike;

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

py_module_initializer!(addcomb, initaddcomb, PyInit_addcomb, |py, m| {
    m.add(py, "__name__", "addcomb")?;
    m.add(py, "__doc__", include_str!("../doc/blurb.txt"))?;

    use public::*;

    add_bindings_to_mod!(py, m, nu, a_, m | u32, h | PyObject);
    add_bindings_to_mod!(py, m, nu_signed, a_, m | u32, h | PyObject);
    add_bindings_to_mod!(py, m, nu_restricted, a_, m | u32, h | PyObject);
    add_bindings_to_mod!(py, m, nu_signed_restricted, a_, m | u32, h | PyObject);

    // add_bindings_to_mod!(py, m, phi, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, phi_signed, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, phi_restricted, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, phi_signed_restricted, a_, h | PyObject);
    //
    // add_bindings_to_mod!(py, m, sigma, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, sigma_signed, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, sigma_restricted, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, sigma_signed_restricted, a_, h | PyObject);

    add_bindings_to_mod!(py, m, rho, a_, m | u32, h | PyObject);
    add_bindings_to_mod!(py, m, rho_signed, a_, m | u32, h | PyObject);
    add_bindings_to_mod!(py, m, rho_restricted, a_, m | u32, h | PyObject);
    add_bindings_to_mod!(py, m, rho_signed_restricted, a_, m | u32, h | PyObject);

    // add_bindings_to_mod!(py, m, chi, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, chi_signed, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, chi_restricted, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, chi_signed_restricted, a_, h | PyObject);
    //
    // add_bindings_to_mod!(py, m, tau, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, tau_signed, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, tau_restricted, a_, h | PyObject);
    // add_bindings_to_mod!(py, m, tau_signed_restricted, a_, h | PyObject);
    //
    // add_bindings_to_mod!(py, m, mu, a_, k | u32, l | u32);
    // add_bindings_to_mod!(py, m, mu_signed, a_, k | u32, l | u32);
    // add_bindings_to_mod!(py, m, mu_restricted, a_, k | u32, l | u32);
    // add_bindings_to_mod!(py, m, mu_signed_restricted, a_, k | u32, l | u32);

    Ok(())
});
