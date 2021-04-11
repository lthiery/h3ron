use std::collections::HashMap;

use numpy::{IntoPyArray, PyReadonlyArray1};
use pyo3::{prelude::*, wrap_pyfunction, IntoPy, PyObject, Python};

use h3ron::grid::bloom::{Bloom, BloomAggregationMethod, BloomGradientType};

#[pyfunction]
fn bloom(py: Python, np_array: PyReadonlyArray1<u64>, radius: u32) -> PyObject {
    let out = np_array.as_array().iter().bloom(
        radius,
        BloomGradientType::Linear,
        BloomAggregationMethod::Max,
    );

    let mut hm = HashMap::new();
    hm.insert(
        "h3index".to_string(),
        out.h3indexes.into_pyarray(py).to_owned().into_py(py),
    );
    hm.insert(
        "value".to_string(),
        out.values.into_pyarray(py).to_owned().into_py(py),
    );
    hm.into_py(py)
}

pub fn init_grid_submodule(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bloom, m)?)?;
    Ok(())
}
