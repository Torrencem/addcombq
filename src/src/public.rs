
use std::convert::TryInto;

use crate::comb::chapter_a;
use crate::comb::chapter_b;
use crate::comb::chapter_c;
use crate::comb::chapter_d;
use crate::comb::chapter_e;
use crate::comb::chapter_f;
use crate::comb::chapter_g;

use crate::comb::exacts;

use cpython::{Python, PyResult, PyObject, PyInt, PyDict, PyTuple, PyIterator, ObjectProtocol, NoArgs};

py_class!(pub class InterruptableBinding |py| {
    data wrapped: PyObject;
    def __new__(_cls, func: PyObject) -> PyResult<InterruptableBinding> {
        InterruptableBinding::create_instance(py, func)
    }
    // I'd really like to have *args,
    // but it looks like this isn't supported in
    // macros yet? Or buggy
    def __call__(&self, arga: Option<PyObject> = None, argb: Option<PyObject> = None,
                        argc: Option<PyObject> = None, argd: Option<PyObject> = None,
                        arge: Option<PyObject> = None, argf: Option<PyObject> = None, verbose: bool = false) -> PyResult<PyObject> {
        let mut all_args: Vec<PyObject> = vec![];
        for maybe_arg in vec![arga, argb, argc, argd, arge, argf].into_iter() {
            if let Some(arg) = maybe_arg {
                all_args.push(arg);
            }
        }
        let args = PyTuple::new(py, &all_args.as_slice());
        // Based from https://stackoverflow.com/questions/21550418/how-to-interrupt-native-extension-code-without-killing-the-interpreter/33525470#33525470
        let mpr = py.import("multiprocessing")?;

        let event = mpr.call(py, "Event", NoArgs, None)?;
        let q = mpr.call(py, "Queue", NoArgs, None)?;

        let locals = PyDict::new(py);
        locals.set_item(py, "event", &event)?;
        locals.set_item(py, "q", &q)?;
        locals.set_item(py, "f", self.wrapped(py))?;
        locals.set_item(py, "args", args)?;
        locals.set_item(py, "verbose", verbose)?;

        // Create our signalling python function
        // in __globals__
        py.run(r#"
def signalling_f():
    q.put(f(*args, verbose=verbose))
    event.set()
        "#, Some(&locals), None)?;
        // Retrieve the function we created to pass into mpr
        let signalling_f = py.eval("signalling_f", Some(&locals), None)?;

        let p_args = PyDict::new(py);
        p_args.set_item(py, "target", signalling_f)?;
        let f_process = mpr.call(py, "Process", NoArgs, Some(&p_args))?;
        f_process.call_method(py, "start", NoArgs, None)?;

        let wait = event.call_method(py, "wait", NoArgs, None);

        if let Err(x) = wait {
            f_process.call_method(py, "terminate", NoArgs, None)?;
            f_process.call_method(py, "join", NoArgs, None)?;
            return Err(x);
        }

        let res = q.call_method(py, "get", NoArgs, None)?;
        Ok(res)
    }
});

pub fn wrap_binding(py: Python, ob: PyObject) -> PyResult<InterruptableBinding> {
    InterruptableBinding::create_instance(py, ob)
}

fn into_pyint(py: Python, x: &PyObject) -> PyResult<PyInt> {
    let int_converter = py.eval("int", None, None)?;

    let as_int = int_converter.call(py, (x,), None)?;

    Ok(as_int.cast_into(py)?)
}

fn into_pyiter<'p>(py: &'p Python, x: &PyObject) -> PyResult<PyIterator<'p>> {
    let iter_converter = py.eval("iter", None, None)?;

    let as_iter = iter_converter.call(*py, (x,), None)?;

    Ok(PyIterator::from_object(*py, as_iter)?)
}

macro_rules! py_binding {
    ($bound_name:ident, $fs_version:expr, $ex_version:expr, $($ex_args:ident),+) => {
        pub fn $bound_name(py: Python, n: PyObject, $($ex_args : u32),+ , verbose: bool) -> PyResult<u32> {
            let numb = into_pyint(py, &n);
            if let Ok(n) = numb {
                let n: u32 = n.value(py).try_into().unwrap(); // Will panic here if negative
                if n <= 63 {
                    Ok($fs_version(n, $($ex_args),+, verbose))
                } else {
                    Ok($ex_version(&[n], $($ex_args),+, verbose))
                }
            } else {
                let list = into_pyiter(&py, &n)?; // Will return here if something awful is given
                let mut tmp = vec![];
                for pyob in list {
                    let numb = into_pyint(py, &pyob?)?;
                    let val = numb.value(py).try_into().unwrap();
                    tmp.push(val);
                }
                Ok($ex_version(tmp.as_slice(), $($ex_args),+, verbose))
            }
        }
    };
}
py_binding!(nu,                             chapter_a::nu,                            exacts::nu_exact,                            m, h);
py_binding!(nu_signed,                      chapter_a::nu_signed,                     exacts::nu_signed_exact,                     m, h);
py_binding!(nu_restricted,                  chapter_a::nu_restricted,                 exacts::nu_restricted_exact,                 m, h);
py_binding!(nu_signed_restricted,           chapter_a::nu_signed_restricted,          exacts::nu_signed_restricted_exact,          m, h);
py_binding!(nu_interval,                    chapter_a::nu_interval,                   exacts::nu_interval_exact,                   m, ia, ib);
py_binding!(nu_signed_interval,             chapter_a::nu_signed_interval,            exacts::nu_signed_interval_exact,            m, ia, ib);
py_binding!(nu_restricted_interval,         chapter_a::nu_restricted_interval,        exacts::nu_restricted_interval_exact,        m, ia, ib);
py_binding!(nu_signed_restricted_interval,  chapter_a::nu_signed_restricted_interval, exacts::nu_signed_restricted_interval_exact, m, ia, ib);

py_binding!(phi,                             chapter_b::phi,                            exacts::phi_exact,                            h);
py_binding!(phi_signed,                      chapter_b::phi_signed,                     exacts::phi_signed_exact,                     h);
py_binding!(phi_restricted,                  chapter_b::phi_restricted,                 exacts::phi_restricted_exact,                 h);
py_binding!(phi_signed_restricted,           chapter_b::phi_signed_restricted,          exacts::phi_signed_restricted_exact,          h);
py_binding!(phi_interval,                    chapter_b::phi_interval,                   exacts::phi_interval_exact,                   ia, ib);
py_binding!(phi_signed_interval,             chapter_b::phi_signed_interval,            exacts::phi_signed_interval_exact,            ia, ib);
py_binding!(phi_restricted_interval,         chapter_b::phi_restricted_interval,        exacts::phi_restricted_interval_exact,        ia, ib);
py_binding!(phi_signed_restricted_interval,  chapter_b::phi_signed_restricted_interval, exacts::phi_signed_restricted_interval_exact, ia, ib);

py_binding!(sigma,                             chapter_c::sigma,                            exacts::sigma_exact,                            h);
py_binding!(sigma_signed,                      chapter_c::sigma_signed,                     exacts::sigma_signed_exact,                     h);
py_binding!(sigma_restricted,                  chapter_c::sigma_restricted,                 exacts::sigma_restricted_exact,                 h);
py_binding!(sigma_signed_restricted,           chapter_c::sigma_signed_restricted,          exacts::sigma_signed_restricted_exact,          h);
py_binding!(sigma_interval,                    chapter_c::sigma_interval,                   exacts::sigma_interval_exact,                   s);
py_binding!(sigma_signed_interval,             chapter_c::sigma_signed_interval,            exacts::sigma_signed_interval_exact,            s);
py_binding!(sigma_restricted_interval,         chapter_c::sigma_restricted_interval,        exacts::sigma_restricted_interval_exact,        s);
py_binding!(sigma_signed_restricted_interval,  chapter_c::sigma_signed_restricted_interval, exacts::sigma_signed_restricted_interval_exact, s);

py_binding!(rho,                             chapter_d::rho,                            exacts::rho_exact,                            m, h);
py_binding!(rho_signed,                      chapter_d::rho_signed,                     exacts::rho_signed_exact,                     m, h);
py_binding!(rho_restricted,                  chapter_d::rho_restricted,                 exacts::rho_restricted_exact,                 m, h);
py_binding!(rho_signed_restricted,           chapter_d::rho_signed_restricted,          exacts::rho_signed_restricted_exact,          m, h);
py_binding!(rho_interval,                    chapter_d::rho_interval,                   exacts::rho_interval_exact,                   m, ia, ib);
py_binding!(rho_signed_interval,             chapter_d::rho_signed_interval,            exacts::rho_signed_interval_exact,            m, ia, ib);
py_binding!(rho_restricted_interval,         chapter_d::rho_restricted_interval,        exacts::rho_restricted_interval_exact,        m, ia, ib);
py_binding!(rho_signed_restricted_interval,  chapter_d::rho_signed_restricted_interval, exacts::rho_signed_restricted_interval_exact, m, ia, ib);

py_binding!(chi,                             chapter_e::chi,                            exacts::chi_exact,                            h);
py_binding!(chi_signed,                      chapter_e::chi_signed,                     exacts::chi_signed_exact,                     h);
py_binding!(chi_restricted,                  chapter_e::chi_restricted,                 exacts::chi_restricted_exact,                 h);
py_binding!(chi_signed_restricted,           chapter_e::chi_signed_restricted,          exacts::chi_signed_restricted_exact,          h);
py_binding!(chi_interval,                    chapter_e::chi_interval,                   exacts::chi_interval_exact,                   ia, ib);
py_binding!(chi_signed_interval,             chapter_e::chi_signed_interval,            exacts::chi_signed_interval_exact,            ia, ib);
py_binding!(chi_restricted_interval,         chapter_e::chi_restricted_interval,        exacts::chi_restricted_interval_exact,        ia, ib);
py_binding!(chi_signed_restricted_interval,  chapter_e::chi_signed_restricted_interval, exacts::chi_signed_restricted_interval_exact, ia, ib);

py_binding!(tau,                             chapter_f::tau,                            exacts::tau_exact,                            h);
py_binding!(tau_signed,                      chapter_f::tau_signed,                     exacts::tau_signed_exact,                     h);
py_binding!(tau_restricted,                  chapter_f::tau_restricted,                 exacts::tau_restricted_exact,                 h);
py_binding!(tau_signed_restricted,           chapter_f::tau_signed_restricted,          exacts::tau_signed_restricted_exact,          h);
py_binding!(tau_interval,                    chapter_f::tau_interval,                   exacts::tau_interval_exact,                   ia, ib);
py_binding!(tau_signed_interval,             chapter_f::tau_signed_interval,            exacts::tau_signed_interval_exact,            ia, ib);
py_binding!(tau_restricted_interval,         chapter_f::tau_restricted_interval,        exacts::tau_restricted_interval_exact,        ia, ib);
py_binding!(tau_signed_restricted_interval,  chapter_f::tau_signed_restricted_interval, exacts::tau_signed_restricted_interval_exact, ia, ib);

py_binding!(mu,                             chapter_g::mu,                            exacts::mu_exact,                            k, l);
py_binding!(mu_signed,                      chapter_g::mu_signed,                     exacts::mu_signed_exact,                     k, l);
py_binding!(mu_restricted,                  chapter_g::mu_restricted,                 exacts::mu_restricted_exact,                 k, l);
py_binding!(mu_signed_restricted,           chapter_g::mu_signed_restricted,          exacts::mu_signed_restricted_exact,          k, l);