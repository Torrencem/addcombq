use addcomb_comp::comb::*;

use addcomb_comp::exactset::GElem;
use addcomb_comp::fastset::FastSet;

use paste;

use std::any::Any;

use std::rc::Rc;

use std::u8;

use cpython::exc;
use cpython::{
    NoArgs, ObjectProtocol, PyDict, PyErr, PyIterator, PyObject, PyResult, PyTuple, Python,
    PythonObject, ToPyObject, PyInt, FromPyObject
};

use cachetools as cache;

pub fn wrap_binding(py: Python, ob: PyObject, numargs: u32, fnid: u8, s: &str) -> PyResult<PyObject> {
    let type_fn = py.eval("type", None, None)?;
    let obj_t = py.eval("(object,)", None, None)?;

    let name = "InterruptableBinding".into_py_object(py);
    let fnid = fnid.into_py_object(py);

    let d = PyDict::new(py);
    d.set_item(py, "_wrapped", ob)?;
    d.set_item(py, "__call__", py_fn!(py, __call__(slf: PyObject, arga: Option<PyObject> = None, argb: Option<PyObject> = None,
                        argc: Option<PyObject> = None, argd: Option<PyObject> = None,
                        arge: Option<PyObject> = None, argf: Option<PyObject> = None, verbose: bool = false) -> PyResult<PyObject> {
        let numargs = slf.getattr(py, "_numargs")?;
        let numargs = usize::extract(py, &numargs).unwrap();
        let mut all_args: Vec<PyObject> = vec![];
        for maybe_arg in vec![arga, argb, argc, argd, arge, argf].into_iter() {
            if let Some(arg) = maybe_arg {
                all_args.push(arg);
            }
        }
        if all_args.len() != numargs {
            return Err(PyErr::new::<exc::TypeError, _>(py, format!("Incorrect number of args given to {:?}: expected {}, got {}", &slf, all_args.len(), numargs)));
        }
        let fnid = slf.getattr(py, "_fnid")?;
        let fnid = u8::extract(py, &fnid).unwrap();
        // Parse the argument types to get a cache entry
        let group: Vec<u32>;
        let mut other_args: Vec<u32> = vec![];
        let n = &all_args[0];
        let numb = into_pyint(py, n);
        if let Ok(n) = numb {
            group = vec![u32::extract(py, &n.as_object()).unwrap()];
        } else {
            let list = into_pyiter(py, n)?; // Will return here if something awful is given
            let mut tmp = vec![];
            for pyob in list {
                let numb = into_pyint(py, &pyob?)?;
                let val = u32::extract(py, &numb.as_object()).unwrap();
                tmp.push(val);
            }
            group = tmp;
        }
        for indx in 1..all_args.len() {
            match format_arg(py, &all_args[indx])? {
                ArgEither::Val(x) => {
                    other_args.push(x);
                },
                ArgEither::Tpl(x, y) => {
                    other_args.push(x);
                    other_args.push(y);
                },
            }
        }
        // Check if all the entries fit into a u8
        let valid_cache = group.iter()
            .all(|&x| {
                x < u32::from(u8::MAX)
            }) && other_args.iter()
            .all(|&x| {
                x < u32::from(u8::MAX)
            });

        let cache_entry = if !valid_cache {
            None
        } else {
            let ce = cache::CacheEntry {
                fid: fnid,
                group: group.into_iter()
                    .map(|x| x as u8)
                    .collect(),
                other_args: other_args.into_iter()
                    .map(|x| x as u8)
                    .collect(),
            };
            if !verbose {
                if let Some(val) = cache::cache_get(&ce) {
                    return Ok(val.to_py_object(py).into_object());
                }
            }
            Some(ce)
        };
        
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
        
        if let Some(cache_entry) = cache_entry {
            cache::cache_set(cache_entry, u8::extract(py, &res.as_object()).unwrap());
        }

        Ok(res)
    }))?;
    d.set_item(py, "__doc__", s.into_py_object(py))?;

    let class = type_fn.call(py, (name, obj_t, d), None)?;

    let inst = class.call(py, NoArgs, None)?;

    // Bind __call__ to the instance correctly
    let types = py.import("types")?;
    let mt = types.get(py, "MethodType")?;
    let cll = inst.getattr(py, "__call__")?;
    
    // https://stackoverflow.com/questions/48857612/types-methodtype-third-argument-in-python2
    #[cfg(feature = "python2")]
    let bound_func = mt.call(py, (cll, &inst, &class), None)?;
    #[cfg(feature = "python3")]
    let bound_func = mt.call(py, (cll, &inst), None)?;

    // Assign the bound func to the module (important!) and class
    class.setattr(py, "__call__", &bound_func)?;
    inst.setattr(py, "__call__", bound_func)?;

    inst.setattr(py, "_numargs", numargs)?;
    inst.setattr(py, "_fnid", fnid)?;

    Ok(inst)
}

fn into_pyint(py: Python, x: &PyObject) -> PyResult<PyInt> {
    let int_converter = py.eval("int", None, None)?;

    let as_int = int_converter.call(py, (x,), None)?;

    Ok(as_int.cast_into(py)?)
}

fn into_pyiter<'p>(py: Python<'p>, x: &PyObject) -> PyResult<PyIterator<'p>> {
    let iter_converter = py.eval("iter", None, None)?;

    let as_iter = iter_converter.call(py, (x,), None)?;

    Ok(PyIterator::from_object(py, as_iter)?)
}

// Run the code from capture_c_out.py to setup
// the capturing functionality
fn setup_capt_c_out(py: Python) -> PyResult<PyObject> {
    #[cfg(feature = "python2")]
    py.run(
        r#"
def handle_stream(stream):
    if stream:
        print stream,
        "#,
        None,
        None,
    )?;
    #[cfg(feature = "python3")]
    py.run(
        r#"
def handle_stream(stream):
    if stream:
        print(stream, end="")
        "#,
        None,
        None,
    )?;

    
    #[cfg(feature = "python2")]
    py.run(include_str!("capture_c_out.py"), None, None)?;
    
    #[cfg(feature = "python3")]
    py.run(include_str!("capture_c_out_3.py"), None, None)?;

    let capt_cout = py.eval("capture_c_stdout(handle_stream)", None, None)?;

    #[cfg(feature = "python2")]
    { Ok(capt_cout) }
    #[cfg(feature = "python3")]
    {
        capt_cout.getattr(py, "gen")
    }
}

fn is_int<P: ToPyObject>(py: Python, x: &P) -> bool {
    into_pyint(py, x.to_py_object(py).as_object()).is_ok()
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

fn format_arg<T: Any>(py: Python, arg: &T) -> PyResult<ArgEither> {
    let value_arg = arg as &dyn Any;
    match value_arg.downcast_ref::<PyObject>() {
        Some(pobj) => {
            let asint = into_pyint(py, pobj);
            if let Ok(x) = asint {
                Ok(ArgEither::Val(u32::extract(py, &x.as_object()).unwrap() as u32))
            } else {
                let err_message = "expected h argument to be either integer h value or iterable interval [i.e. (0, 3)]";
                let type_err = || Err(PyErr::new::<exc::TypeError, _>(py, err_message));
                let piter = into_pyiter(py, pobj);
                if let Ok(mut piter) = piter {
                    let a: PyInt = into_pyint(py, &piter.next().unwrap_or_else(type_err)?)?;
                    let b: PyInt = into_pyint(py, &piter.next().unwrap_or_else(type_err)?)?;
                    Ok(ArgEither::Tpl(u32::extract(py, &a.as_object()).unwrap() as u32, u32::extract(py, &b.as_object()).unwrap() as u32))
                } else {
                    Err(PyErr::new::<exc::TypeError, _>(py, err_message))
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
    ($bound_name:ident, $fs_version:expr, $fs2_version:expr, $ex_version:expr, $fs_int_version:expr, $fs2_int_version:expr, $ex_int_version:expr, $($ex_args:ident | $ex_arg_type:ident),+) => {
        pub fn $bound_name(py: Python, n: PyObject, $($ex_args : $ex_arg_type),+ , verbose: bool) -> PyResult<u32> {
            // Setup c_out capturing
            let capt_c_out = &(
                if verbose {
                    Some(setup_capt_c_out(py)?)
                } else { None });
            
            #[cfg(feature = "python2")]
            let gen_next = "next";
            #[cfg(feature = "python3")]
            let gen_next = "__next__";
            
            if verbose {
                capt_c_out.as_ref().unwrap().call_method(py, gen_next, NoArgs, None)?;
            }
            let numb = into_pyint(py, &n);
            if let Ok(n) = numb {
                let n: u32 = u32::extract(py, &n.as_object()).unwrap(); // Will panic here if negative
                if n <= 127 {
                    let icall: bool = interval_call!(py, $($ex_args | $ex_arg_type),+);
                    let val = if !icall {
                        $fs2_version(n, $(format_arg(py, &$ex_args)?.into()),+, verbose)
                    } else {
                        $fs2_int_version(n, $(format_arg(py, &$ex_args)?.into()),+, verbose)
                    };
                    if verbose {
                        capt_c_out.as_ref().unwrap().call_method(py, gen_next, NoArgs, None).expect_err("fatal capture error");
                    }
                    Ok(val)
                } else if n <= 63 {
                    let icall: bool = interval_call!(py, $($ex_args | $ex_arg_type),+);
                    let val = if !icall {
                        $fs_version(n, $(format_arg(py, &$ex_args)?.into()),+, verbose)
                    } else {
                        $fs_int_version(n, $(format_arg(py, &$ex_args)?.into()),+, verbose)
                    };
                    if verbose {
                        // Stop c_out capturing
                        capt_c_out.as_ref().unwrap().call_method(py, gen_next, NoArgs, None).expect_err("fatal capture error");
                    }
                    Ok(val)
                } else {
                    let icall: bool = interval_call!(py, $($ex_args | $ex_arg_type),+);
                    let val = if !icall {
                        $ex_version(Rc::new(vec![n]), $(format_arg(py, &$ex_args)?.into()),+, verbose)
                    } else {
                        $ex_int_version(Rc::new(vec![n]), $(format_arg(py, &$ex_args)?.into()),+, verbose)
                    };
                    if verbose {
                        capt_c_out.as_ref().unwrap().call_method(py, gen_next, NoArgs, None).expect_err("fatal capture error");
                    }
                    Ok(val)
                }
            } else {
                let list = into_pyiter(py, &n)?; // Will return here if something awful is given
                let mut tmp = vec![];
                for pyob in list {
                    let numb = into_pyint(py, &pyob?)?;
                    let val = u32::extract(py, &numb.as_object()).unwrap();
                    tmp.push(val);
                }
                let icall: bool = interval_call!(py, $($ex_args | $ex_arg_type),+);
                let val = if !icall {
                    $ex_version(Rc::new(tmp), $(format_arg(py, &$ex_args)?.into()),+, verbose)
                } else {
                    $ex_int_version(Rc::new(tmp), $(format_arg(py, &$ex_args)?.into()),+, verbose)
                };
                if verbose {
                    capt_c_out.as_ref().unwrap().call_method(py, gen_next, NoArgs, None).expect_err("fatal capture error");
                }
                Ok(val)
            }
        }
    };
}

// Ignore interval stuff
// only for mu
macro_rules! py_binding_mu {
    ($bound_name:ident, $fs_version:expr, $fs2_version:expr, $ex_version:expr, $($ex_args:ident),+) => {
        pub fn $bound_name(py: Python, n: PyObject, $($ex_args : u32),+ , verbose: bool) -> PyResult<u32> {
            // Setup c_out capturing
            let capt_c_out = &(if verbose {
                Some(setup_capt_c_out(py)?)
            } else {
                None
            });

            if verbose {
                capt_c_out.as_ref().unwrap().call_method(py, "next", NoArgs, None)?;
            }
            let numb = into_pyint(py, &n);
            if let Ok(n) = numb {
                let n: u32 = u32::extract(py, &n.as_object()).unwrap(); // Will panic here if negative
                if n <= 127 {
                    let val = $fs2_version(n, $($ex_args),+, verbose);
                    if verbose {
                        // Stop c_out capturing
                        capt_c_out.as_ref().unwrap().call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
                    }
                    Ok(val)
                } else if n <= 63 {
                    let val = $fs_version(n, $($ex_args),+, verbose);
                    if verbose {
                        capt_c_out.as_ref().unwrap().call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
                    }
                    Ok(val)
                } else {
                    let val = $ex_version(Rc::new(vec![n]), $($ex_args),+, verbose);
                    if verbose {
                        capt_c_out.as_ref().unwrap().call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
                    }
                    Ok(val)
                }
            } else {
                let list = into_pyiter(py, &n)?; // Will return here if something awful is given
                let mut tmp = vec![];
                for pyob in list {
                    let numb = into_pyint(py, &pyob?)?;
                    let val = u32::extract(py, &numb.as_object()).unwrap();
                    tmp.push(val);
                }
                let val = $ex_version(Rc::new(tmp), $($ex_args),+, verbose);
                if verbose {
                    capt_c_out.as_ref().unwrap().call_method(py, "next", NoArgs, None).expect_err("fatal capture error");
                }
                Ok(val)
            }
        }
    };
}

macro_rules! bind_all {
    ($to:tt, $md:tt, $($ex_args:ident | $ex_arg_type:ident),+) => {
        paste::item! {
            py_binding! (
                $to,
                $md::$to::<FastSet<u64>>,
                $md::$to::<FastSet<u128>>,
                $md::$to::<Vec<GElem>>,
                $md::[<$to _interval>]::<FastSet<u64>>,
                $md::[<$to _interval>]::<FastSet<u128>>,
                $md::[<$to _interval>]::<Vec<GElem>>,
                $($ex_args | $ex_arg_type),+
            );
        }
    }
}

macro_rules! bind_variants {
    ($to:tt, $md:tt, $($ex_args:ident | $ex_arg_type:ident),+) => {
        paste::item! {
            bind_all!($to, $md, $($ex_args | $ex_arg_type),+);
            bind_all!([<$to _signed>], $md, $($ex_args | $ex_arg_type),+);
            bind_all!([<$to _restricted>], $md, $($ex_args | $ex_arg_type),+);
            bind_all!([<$to _signed_restricted>], $md, $($ex_args | $ex_arg_type),+);
        }
    }
}

bind_variants!(nu, chapter_a, m | u32, h | PyObject);

bind_variants!(phi, chapter_b, h | PyObject);

bind_variants!(sigma, chapter_c, h | PyObject);

bind_variants!(rho, chapter_d, m | u32, h | PyObject);

bind_variants!(chi, chapter_e, h | PyObject);

bind_variants!(tau, chapter_f, h | PyObject);

// Mu functions don't fit pattern

py_binding_mu!(
    mu,
    chapter_g::mu::<FastSet<u64>>,
    chapter_g::mu::<FastSet<u128>>,
    chapter_g::mu::<Vec<GElem>>,
    k,
    l
);
py_binding_mu!(
    mu_signed,
    chapter_g::mu_signed::<FastSet<u64>>,
    chapter_g::mu_signed::<FastSet<u128>>,
    chapter_g::mu_signed::<Vec<GElem>>,
    k,
    l
);
py_binding_mu!(
    mu_restricted,
    chapter_g::mu_restricted::<FastSet<u64>>,
    chapter_g::mu_restricted::<FastSet<u128>>,
    chapter_g::mu_restricted::<Vec<GElem>>,
    k,
    l
);
py_binding_mu!(
    mu_signed_restricted,
    chapter_g::mu_signed_restricted::<FastSet<u64>>,
    chapter_g::mu_signed_restricted::<FastSet<u128>>,
    chapter_g::mu_signed_restricted::<Vec<GElem>>,
    k,
    l
);

pub fn comb_a(_py: Python, h: u32, m: u32) -> PyResult<u32> {
    Ok(a(h, m))
}

pub fn comb_c(_py: Python, h: u32, m: u32) -> PyResult<u32> {
    Ok(c(h, m))
}

pub fn comb_choose(_py: Python, n: u32, k: u32) -> PyResult<u32> {
    Ok(choose(n, k))
}

pub fn comb_v(_py: Python, g: u32, n: u32, h: u32) -> PyResult<u32> {
    Ok(v(g, n, h))
}

pub fn comb_v_signed(_py: Python, n: u32, h: u32) -> PyResult<u32> {
    Ok(v_signed(n, h))
}
