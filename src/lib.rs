extern crate pyo3;

use numpy::{PyArray4, PyReadonlyArray4, IntoPyArray};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::Python;
use ndarray::{ArrayView4, Array4};

mod screen_module;

fn screen_stack_wrap(stack1: ArrayView4<'_, u8>, stack2: ArrayView4<'_, u8>) -> Array4<u8>{
    screen_module::screen_stack(stack1, stack2)
}

#[pyfunction]
pub fn screen_stack_py<'py>(_py:Python<'py>, stack1: PyReadonlyArray4<'_, u8>,
 stack2: PyReadonlyArray4<'_, u8>) -> &'py PyArray4<u8>{
    let stack_1 = stack1.as_array();
    let stack_2 = stack2.as_array();
    let array: Array4<u8> = screen_stack_wrap(stack_1, stack_2);
    array.into_pyarray(_py)
}



#[pymodule]
fn img_rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(screen_stack_py, m)?)?;
    Ok(())
}