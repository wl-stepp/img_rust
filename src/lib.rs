use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use numpy::{PyArray4, PyReadonlyArray4, IntoPyArray};
use ndarray::{ArrayView4, Array4};
mod screen_module;


#[pymodule]
fn img_rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {

    fn screen_stack_wrap(stack1: ArrayView4<'_, u8>, stack2: ArrayView4<'_, u8>) -> Array4<u8>{
        screen_module::screen_stack(stack1, stack2)
    }

    #[pyfn(m, "screen_stack_wrap")]
    pub fn screen_stack_py<'py>(py: Python<'py>, stack1: PyReadonlyArray4<'_, u8>, stack2: PyReadonlyArray4<'_, u8>) -> &'py PyArray4<u8>{
        let stack_1 = stack1.as_array();
        let stack_2 = stack2.as_array();
        screen_stack_wrap(stack_1, stack_2).into_pyarray(py)
    }

    Ok(())
}