extern crate ndarray;
extern crate ndarray_rand;
extern crate num_traits;
extern crate numpy;
extern crate pyo3;
extern crate rand;

use ndarray::prelude::*;
use num_traits::Float;
use numpy::{IntoPyArray, PyArray1};
use pyo3::{prelude::*, types::PyDict, PyResult};
use rand::distributions::Normal;
use ndarray_rand::{RandomExt, F32};
use std::fmt::Debug;

fn assert_close<F>(a: &[F], b: &[F], delta: F)
where
    F: Float + Debug,
{
    assert_eq!(
        a.len(),
        b.len(),
        "assert_close() failed: Array lengths are not the same"
    );
    for (&x, &y) in a.iter().zip(b) {
        if x.is_finite() && y.is_finite() {
            assert!(
                (x - y).abs() <= delta,
                "assert_close() failed: {:?} !~ {:?}",
                x,
                y
            );
        } else {
            assert!(x == y, "assert_close() failed: {:?} !~ {:?}", x, y);
        }
    }
}

fn _test_pyo3(py: Python, x: Array1<f32>) -> PyResult<Array1<f32>> {
    let globals = PyDict::new(py);
    globals.set_item("np", py.import("numpy")?)?;

    let locals = PyDict::new(py);
    locals.set_item("x", x.clone().into_pyarray(py))?;

    let py_x: &PyArray1<f32> = py
        .eval("np.exp(x)", Some(&globals), Some(&locals))?
        .extract()?;

    Ok(py_x.to_owned_array())
}

fn _test(size: usize) {
    let gil = Python::acquire_gil();

    let mut x = Array1::random(size, F32(Normal::new(-1., 1.)));

    let py_x = _test_pyo3(gil.python(), x.clone())
    .map_err(|e| {
        eprintln!("Error calling _test_pyo3(): {:?}", e);
        e.print_and_set_sys_last_vars(gil.python());
    })
    .unwrap();

    x.mapv_inplace(f32::exp);
    assert_close(&x.as_slice().unwrap(), &py_x.as_slice().unwrap(), 1e-6.into());
}

#[test]
fn test_1() {
    _test(1)
}
#[test]
fn test_2() {
    _test(1)
}
#[test]
fn test_3() {
    _test(1)
}
#[test]
fn test_4() {
    _test(1)
}
#[test]
fn test_5() {
    _test(1)
}
#[test]
fn test_6() {
    _test(1)
}
#[test]
fn test_7() {
    _test(1)
}
#[test]
fn test_8() {
    _test(1)
}
#[test]
fn test_9() {
    _test(1)
}
#[test]
fn test_10() {
    _test(1)
}
