use std::convert::TryInto;

use crate::comb::chapter_a;
// use crate::comb::chapter_b;
// use crate::comb::chapter_c;
use crate::comb::chapter_d;
// use crate::comb::chapter_e;
// use crate::comb::chapter_f;
// use crate::comb::chapter_g;

use crate::fastset::FastSet;
use crate::exactset::GElem;

use std::any::Any;

use std::rc::Rc;

use cpython::exc;
use cpython::{
    NoArgs, ObjectProtocol, PyDict, PyErr, PyInt, PyIterator, PyObject, PyResult, PyTuple, Python,
    PythonObject, ToPyObject,
};

pub fn wrap_binding(py: Python, ob: PyObject, s: &str) -> PyResult<PyObject> {
    let type_fn = py.eval("type", None, None)?;
    let obj_t = py.eval("(object,)", None, None)?;

    let name = "InterruptableBinding".into_py_object(py);

    let d = PyDict::new(py);
    d.set_item(py, "_wrapped", ob)?;
    d.set_item(py, "__call__", py_fn!(py, __call__(slf: PyObject, arga: Option<PyObject> = None, argb: Option<PyObject> = None,
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

        let wrapped = slf.getattr(py, "_wrapped")?;

        let locals = PyDict::new(py);
        locals.set_item(py, "event", &event)?;
        locals.set_item(py, "q", &q)?;
        locals.set_item(py, "f", wrapped)?;
        locals.set_item(py, "args", args)?;
        locals.set_item(py, "verbose", verbose)?;

        // Fix stdout in jupyter notebooks (https://stackoverflow.com/questions/45200375/stdout-redirect-from-jupyter-notebook-is-landing-in-the-terminal)
        let sys = py.import("sys")?;
        let old_stdout = sys.get(py, "stdout")?;
        locals.set_item(py, "old_stdout", old_stdout)?;
        locals.set_item(py, "__import__", py.eval("__import__", None, None)?)?;

        // Create our signalling python function
        // in __globals__
        py.run(r#"
def signalling_f():
    try:
        q.put(f(*args, verbose=verbose))
    finally:
        event.set()
        q.put(None)
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
    }))?;
    d.set_item(py, "__doc__", s.into_py_object(py))?;

    let class = type_fn.call(py, (name, obj_t, d), None)?;

    let inst = class.call(py, NoArgs, None)?;

    // Bind __call__ to the instance correctly
    let types = py.import("types")?;
    let mt = types.get(py, "MethodType")?;
    let cll = inst.getattr(py, "__call__")?;

    let bound_func = mt.call(py, (cll, &inst, &class), None)?;

    // Assign the bound func to the module (important!) and class
    class.setattr(py, "__call__", &bound_func)?;
    inst.setattr(py, "__call__", bound_func)?;

    Ok(inst)
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

// Run the code from capture_c_out.py to setup
// the capturing functionality
fn setup_capt_c_out(py: Python) -> PyResult<PyObject> {
    py.run(
        r#"
def handle_stream(stream):
    if stream:
        print stream,
        "#,
        None,
        None,
    )?;

    py.run(include_str!("capture_c_out.py"), None, None)?;

    py.eval("capture_c_stdout(handle_stream)", None, None)
}

fn is_int<P: ToPyObject>(py: Python, x: &P) -> bool {
    !into_pyint(py, x.to_py_object(py).as_object()).is_err()
}

enum ArgEither {
    Val(u32),
    Tpl(u32, u32),
}

impl Into<u32> for ArgEither {
    fn into(self) -> u32 {
        if let ArgEither::Val(x) = self {
            x
        } else {
            panic!()
        }
    }
}

impl Into<(u32, u32)> for ArgEither {
    fn into(self) -> (u32, u32) {
        if let ArgEither::Tpl(a, b) = self {
            (a, b)
        } else {
            panic!()
        }
    }
}

fn format_arg<T: Any>(py: &Python, arg: &T) -> PyResult<ArgEither> {
    let value_arg = arg as &dyn Any;
    match value_arg.downcast_ref::<PyObject>() {
        Some(pobj) => {
            let asint = into_pyint(*py, pobj);
            if let Ok(x) = asint {
                Ok(ArgEither::Val(x.value(*py) as u32))
            } else {
                let err_message = "expected h argument to be either integer h value or iterable interval [i.e. (0, 3)]";
                let type_err = || Err(PyErr::new::<exc::TypeError, _>(*py, err_message));
                let piter = into_pyiter(py, pobj);
                if let Ok(mut piter) = piter {
                    let a: PyInt = into_pyint(*py, &piter.next().unwrap_or_else(type_err)?)?;
                    let b: PyInt = into_pyint(*py, &piter.next().unwrap_or_else(type_err)?)?;
                    Ok(ArgEither::Tpl(a.value(*py) as u32, b.value(*py) as u32))
                } else {
                    Err(PyErr::new::<exc::TypeError, _>(*py, err_message).into())
                }
            }
        }
        None => Ok(ArgEither::Val(*value_arg.downcast_ref::<u32>().unwrap())),
    }
}

macro_rules! interval_call {
    ($py:ident, $($ex_arg:ident | $ex_arg_type:ident),+) => {
        (|| {
            $(
                if stringify!($ex_arg_type) != "u32" {
                    if !is_int($py, &$ex_arg) {
                        return true;
                    }
                }
            )*
            return false;
        })()
    };
}

macro_rules! py_binding {
    ($bound_name:ident, $fs_version:expr, $ex_version:expr, $fs_int_version:expr, $ex_int_version:expr, $($ex_args:ident | $ex_arg_type:ident),+) => {
        pub fn $bound_name(py: Python, n: PyObject, $($ex_args : $ex_arg_type),+ , verbose: bool) -> PyResult<u32> {
            // Setup c_out capturing
            let capt_c_out = setup_capt_c_out(py)?;
            capt_c_out.call_method(py, "next", NoArgs, None)?;
            let numb = into_pyint(py, &n);
            if let Ok(n) = numb {
                let n: u32 = n.value(py).try_into().unwrap(); // Will panic here if negative
                if n <= 63 {
                    let icall: bool = interval_call!(py, $($ex_args | $ex_arg_type),+);
                    let val: u32;
                    if !icall {
                        val = $fs_version(n, $(format_arg(&py, &$ex_args)?.into()),+, verbose);
                    } else {
                        val = $fs_int_version(n, $(format_arg(&py, &$ex_args)?.into()),+, verbose);
                    }
                    // Stop c_out capturing
                    capt_c_out.call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
                    Ok(val)
                } else {
                    let icall: bool = interval_call!(py, $($ex_args | $ex_arg_type),+);
                    let val: u32;
                    if !icall {
                        val = $ex_version(Rc::new(vec![n]), $(format_arg(&py, &$ex_args)?.into()),+, verbose);
                    } else {
                        val = $ex_int_version(Rc::new(vec![n]), $(format_arg(&py, &$ex_args)?.into()),+, verbose);
                    }
                    capt_c_out.call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
                    Ok(val)
                }
            } else {
                let list = into_pyiter(&py, &n)?; // Will return here if something awful is given
                let mut tmp = vec![];
                for pyob in list {
                    let numb = into_pyint(py, &pyob?)?;
                    let val = numb.value(py).try_into().unwrap();
                    tmp.push(val);
                }
                let icall: bool = interval_call!(py, $($ex_args | $ex_arg_type),+);
                let val: u32;
                if !icall {
                    val = $ex_version(Rc::new(tmp), $(format_arg(&py, &$ex_args)?.into()),+, verbose);
                } else {
                    val = $ex_int_version(Rc::new(tmp), $(format_arg(&py, &$ex_args)?.into()),+, verbose);
                }
                capt_c_out.call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
                Ok(val)
            }
        }
    };
}

// Ignore interval stuff
// only for mu
// macro_rules! py_binding_mu {
//     ($bound_name:ident, $fs_version:expr, $ex_version:expr, $($ex_args:ident),+) => {
//         pub fn $bound_name(py: Python, n: PyObject, $($ex_args : u32),+ , verbose: bool) -> PyResult<u32> {
//             // Setup c_out capturing
//             let capt_c_out = setup_capt_c_out(py)?;
//             capt_c_out.call_method(py, "next", NoArgs, None)?;
//             let numb = into_pyint(py, &n);
//             if let Ok(n) = numb {
//                 let n: u32 = n.value(py).try_into().unwrap(); // Will panic here if negative
//                 if n <= 63 {
//                     let val = $fs_version(n, $($ex_args),+, verbose);
//                     // Stop c_out capturing
//                     capt_c_out.call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
//                     Ok(val)
//                 } else {
//                     let val = $ex_version(&[n], $($ex_args),+, verbose);
//                     capt_c_out.call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
//                     Ok(val)
//                 }
//             } else {
//                 let list = into_pyiter(&py, &n)?; // Will return here if something awful is given
//                 let mut tmp = vec![];
//                 for pyob in list {
//                     let numb = into_pyint(py, &pyob?)?;
//                     let val = numb.value(py).try_into().unwrap();
//                     tmp.push(val);
//                 }
//                 let val = $ex_version(tmp.as_slice(), $($ex_args),+, verbose);
//                 capt_c_out.call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
//                 Ok(val)
//             }
//         }
//     };
// }

py_binding!(
    nu,
    chapter_a::nu::<FastSet>,
    chapter_a::nu::<Vec<GElem>>,
    chapter_a::nu_interval::<FastSet>,
    chapter_a::nu_interval::<Vec<GElem>>,
    m | u32,
    h | PyObject
);
py_binding!(
    nu_signed,
    chapter_a::nu_signed::<FastSet>,
    chapter_a::nu_signed::<Vec<GElem>>,
    chapter_a::nu_signed_interval::<FastSet>,
    chapter_a::nu_signed_interval::<Vec<GElem>>,
    m | u32,
    h | PyObject
);
py_binding!(
    nu_restricted,
    chapter_a::nu_restricted::<FastSet>,
    chapter_a::nu_restricted::<Vec<GElem>>,
    chapter_a::nu_restricted_interval::<FastSet>,
    chapter_a::nu_restricted_interval::<Vec<GElem>>,
    m | u32,
    h | PyObject
);
py_binding!(
    nu_signed_restricted,
    chapter_a::nu_signed_restricted::<FastSet>,
    chapter_a::nu_signed_restricted::<Vec<GElem>>,
    chapter_a::nu_signed_restricted_interval::<FastSet>,
    chapter_a::nu_signed_restricted_interval::<Vec<GElem>>,
    m | u32,
    h | PyObject
);

// py_binding!(
//     phi,
//     chapter_b::phi,
//     exacts::phi_exact,
//     chapter_b::phi_interval,
//     exacts::phi_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     phi_signed,
//     chapter_b::phi_signed,
//     exacts::phi_signed_exact,
//     chapter_b::phi_signed_interval,
//     exacts::phi_signed_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     phi_restricted,
//     chapter_b::phi_restricted,
//     exacts::phi_restricted_exact,
//     chapter_b::phi_restricted_interval,
//     exacts::phi_restricted_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     phi_signed_restricted,
//     chapter_b::phi_signed_restricted,
//     exacts::phi_signed_restricted_exact,
//     chapter_b::phi_signed_restricted_interval,
//     exacts::phi_signed_restricted_interval_exact,
//     h | PyObject
// );
//
// py_binding!(
//     sigma,
//     chapter_c::sigma,
//     exacts::sigma_exact,
//     chapter_c::sigma_interval,
//     exacts::sigma_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     sigma_signed,
//     chapter_c::sigma_signed,
//     exacts::sigma_signed_exact,
//     chapter_c::sigma_signed_interval,
//     exacts::sigma_signed_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     sigma_restricted,
//     chapter_c::sigma_restricted,
//     exacts::sigma_restricted_exact,
//     chapter_c::sigma_restricted_interval,
//     exacts::sigma_restricted_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     sigma_signed_restricted,
//     chapter_c::sigma_signed_restricted,
//     exacts::sigma_signed_restricted_exact,
//     chapter_c::sigma_signed_restricted_interval,
//     exacts::sigma_signed_restricted_interval_exact,
//     h | PyObject
// );

py_binding!(
    rho,
    chapter_d::rho::<FastSet>,
    chapter_d::rho::<Vec<GElem>>,
    chapter_d::rho_interval::<FastSet>,
    chapter_d::rho_interval::<Vec<GElem>>,
    m | u32,
    h | PyObject
);
py_binding!(
    rho_signed,
    chapter_d::rho_signed::<FastSet>,
    chapter_d::rho_signed::<Vec<GElem>>,
    chapter_d::rho_signed_interval::<FastSet>,
    chapter_d::rho_signed_interval::<Vec<GElem>>,
    m | u32,
    h | PyObject
);
py_binding!(
    rho_restricted,
    chapter_d::rho_restricted::<FastSet>,
    chapter_d::rho_restricted::<Vec<GElem>>,
    chapter_d::rho_restricted_interval::<FastSet>,
    chapter_d::rho_restricted_interval::<Vec<GElem>>,
    m | u32,
    h | PyObject
);
py_binding!(
    rho_signed_restricted,
    chapter_d::rho_signed_restricted::<FastSet>,
    chapter_d::rho_signed_restricted::<Vec<GElem>>,
    chapter_d::rho_signed_restricted_interval::<FastSet>,
    chapter_d::rho_signed_restricted_interval::<Vec<GElem>>,
    m | u32,
    h | PyObject
);
//
// py_binding!(
//     chi,
//     chapter_e::chi,
//     exacts::chi_exact,
//     chapter_e::chi_interval,
//     exacts::chi_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     chi_signed,
//     chapter_e::chi_signed,
//     exacts::chi_signed_exact,
//     chapter_e::chi_signed_interval,
//     exacts::chi_signed_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     chi_restricted,
//     chapter_e::chi_restricted,
//     exacts::chi_restricted_exact,
//     chapter_e::chi_restricted_interval,
//     exacts::chi_restricted_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     chi_signed_restricted,
//     chapter_e::chi_signed_restricted,
//     exacts::chi_signed_restricted_exact,
//     chapter_e::chi_signed_restricted_interval,
//     exacts::chi_signed_restricted_interval_exact,
//     h | PyObject
// );
//
// py_binding!(
//     tau,
//     chapter_f::tau,
//     exacts::tau_exact,
//     chapter_f::tau_interval,
//     exacts::tau_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     tau_signed,
//     chapter_f::tau_signed,
//     exacts::tau_signed_exact,
//     chapter_f::tau_signed_interval,
//     exacts::tau_signed_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     tau_restricted,
//     chapter_f::tau_restricted,
//     exacts::tau_restricted_exact,
//     chapter_f::tau_restricted_interval,
//     exacts::tau_restricted_interval_exact,
//     h | PyObject
// );
// py_binding!(
//     tau_signed_restricted,
//     chapter_f::tau_signed_restricted,
//     exacts::tau_signed_restricted_exact,
//     chapter_f::tau_signed_restricted_interval,
//     exacts::tau_signed_restricted_interval_exact,
//     h | PyObject
// );
//
// py_binding_mu!(mu, chapter_g::mu, exacts::mu_exact, k, l);
// py_binding_mu!(
//     mu_signed,
//     chapter_g::mu_signed,
//     exacts::mu_signed_exact,
//     k,
//     l
// );
// py_binding_mu!(
//     mu_restricted,
//     chapter_g::mu_restricted,
//     exacts::mu_restricted_exact,
//     k,
//     l
// );
// py_binding_mu!(
//     mu_signed_restricted,
//     chapter_g::mu_signed_restricted,
//     exacts::mu_signed_restricted_exact,
//     k,
//     l
// );
