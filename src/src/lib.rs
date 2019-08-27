#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

use std::mem::swap;

fn fib(_py: Python, n: usize) -> PyResult<usize> {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        b += a;
        swap(&mut a, &mut b);
    }

    Ok(a)
}

py_module_initializer!(addcomb, initaddcomb, PyInit_addcomb, |py, m| {
    m.add(py, "fib", py_fn!(py, fib(n: usize)))?;
    Ok(())
});
