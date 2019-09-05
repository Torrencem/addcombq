#![feature(concat_idents)]

#[macro_use]
extern crate cpython;

use cpython::{PyObject};

mod fastset;
mod exactset;
mod comb;

mod public;

macro_rules! add_bindings_to_mod {
    ($py:ident, $pymod:ident, $fn_name:ident, $fn_var_name:ident, $($ex_args:ident),+) => {
        let docstring = include_str!(concat!("../doc/compiled/", stringify!($fn_name), ".md"));
        let $fn_var_name = py_fn!($py, $fn_name(n: PyObject, $($ex_args : u32),+ , verbose: bool = false));
        $pymod.add($py, concat!("_", stringify!($fn_name)), &$fn_var_name)?;
        let $fn_var_name = wrap_binding($py, $fn_var_name, docstring)?;
        $pymod.add($py, stringify!($fn_name), &$fn_var_name)?;
    };
}

py_module_initializer!(addcomb, initaddcomb, PyInit_addcomb, |py, m| {
    use public::*;
    
    add_bindings_to_mod!(py, m, nu, a_, m, h);
    add_bindings_to_mod!(py, m, nu_signed, a_, m, h);
    add_bindings_to_mod!(py, m, nu_restricted, a_, m, h);
    add_bindings_to_mod!(py, m, nu_signed_restricted, a_, m, h);
    add_bindings_to_mod!(py, m, nu_interval, a_, m, ia, ib);
    add_bindings_to_mod!(py, m, nu_signed_interval, a_, m, ia, ib);
    add_bindings_to_mod!(py, m, nu_restricted_interval, a_, m, ia, ib);
    add_bindings_to_mod!(py, m, nu_signed_restricted_interval, a_, m, ia, ib);

    add_bindings_to_mod!(py, m, phi, a_, h);
    add_bindings_to_mod!(py, m, phi_signed, a_, h);
    add_bindings_to_mod!(py, m, phi_restricted, a_, h);
    add_bindings_to_mod!(py, m, phi_signed_restricted, a_, h);
    add_bindings_to_mod!(py, m, phi_interval, a_, ia, ib);
    add_bindings_to_mod!(py, m, phi_signed_interval, a_, ia, ib);
    add_bindings_to_mod!(py, m, phi_restricted_interval, a_, ia, ib);
    add_bindings_to_mod!(py, m, phi_signed_restricted_interval, a_, ia, ib);

    add_bindings_to_mod!(py, m, sigma, a_, h);
    add_bindings_to_mod!(py, m, sigma_signed, a_, h);
    add_bindings_to_mod!(py, m, sigma_restricted, a_, h);
    add_bindings_to_mod!(py, m, sigma_signed_restricted, a_, h);
    add_bindings_to_mod!(py, m, sigma_interval, a_, s);
    add_bindings_to_mod!(py, m, sigma_signed_interval, a_, s);
    add_bindings_to_mod!(py, m, sigma_restricted_interval, a_, s);
    add_bindings_to_mod!(py, m, sigma_signed_restricted_interval, a_, s);

    add_bindings_to_mod!(py, m, rho, a_, m, h);
    add_bindings_to_mod!(py, m, rho_signed, a_, m, h);
    add_bindings_to_mod!(py, m, rho_restricted, a_, m, h);
    add_bindings_to_mod!(py, m, rho_signed_restricted, a_, m, h);
    add_bindings_to_mod!(py, m, rho_interval, a_, m, ia, ib);
    add_bindings_to_mod!(py, m, rho_signed_interval, a_, m, ia, ib);
    add_bindings_to_mod!(py, m, rho_restricted_interval, a_, m, ia, ib);
    add_bindings_to_mod!(py, m, rho_signed_restricted_interval, a_, m, ia, ib);

    add_bindings_to_mod!(py, m, chi, a_, h);
    add_bindings_to_mod!(py, m, chi_signed, a_, h);
    add_bindings_to_mod!(py, m, chi_restricted, a_, h);
    add_bindings_to_mod!(py, m, chi_signed_restricted, a_, h);
    add_bindings_to_mod!(py, m, chi_interval, a_, ia, ib);
    add_bindings_to_mod!(py, m, chi_signed_interval, a_, ia, ib);
    add_bindings_to_mod!(py, m, chi_restricted_interval, a_, ia, ib);
    add_bindings_to_mod!(py, m, chi_signed_restricted_interval, a_, ia, ib);

    add_bindings_to_mod!(py, m, tau, a_, h);
    add_bindings_to_mod!(py, m, tau_signed, a_, h);
    add_bindings_to_mod!(py, m, tau_restricted, a_, h);
    add_bindings_to_mod!(py, m, tau_signed_restricted, a_, h);
    add_bindings_to_mod!(py, m, tau_interval, a_, ia, ib);
    add_bindings_to_mod!(py, m, tau_signed_interval, a_, ia, ib);
    add_bindings_to_mod!(py, m, tau_restricted_interval, a_, ia, ib);
    add_bindings_to_mod!(py, m, tau_signed_restricted_interval, a_, ia, ib);

    add_bindings_to_mod!(py, m, mu, a_, k, l);
    add_bindings_to_mod!(py, m, mu_signed, a_, k, l);
    add_bindings_to_mod!(py, m, mu_restricted, a_, k, l);
    add_bindings_to_mod!(py, m, mu_signed_restricted, a_, k, l);

    Ok(())
});