#![allow(unused)]

use numpy::{ndarray::Array1, IntoPyArray};
use pyo3::{types::PyDict, Python};

#[test]
pub fn into_pyarray() {
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let array = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let array = array.into_pyarray(py);

        let locals = PyDict::new(py);

        locals.set_item("array", array).unwrap();

        py.run(
            r#"
import numpy as np
array = np.array([k * 10 for k in array])
        "#,
            None,
            Some(locals),
        )
        .unwrap();

        println!("{:?}", locals.get_item("array").unwrap());
    });
}
