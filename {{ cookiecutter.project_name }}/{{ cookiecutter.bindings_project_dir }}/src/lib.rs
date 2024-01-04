use numpy::{IntoPyArray, PyArray1, PyArray2, PyArray3, PyReadonlyArray2};
use pyo3::{pymodule, types::PyModule, PyResult, Python};
use {{ cookiecutter.rust_project_slug }}::maximize;

#[pymodule]
fn {{ cookiecutter.python_project_slug }}(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "maximize")]
    fn maximize_py<'py>(
        py: Python<'py>,
        data: PyReadonlyArray2<f64>,
        responsibilities: PyReadonlyArray2<f64>,
    ) -> (&'py PyArray2<f64>, &'py PyArray3<f64>, &'py PyArray1<f64>) {
        {% if cookiecutter.include_sample_code %} 

        let data = data.as_array();
        let responsibilities = responsibilities.as_array();
        let (means, covs, weights) = maximize(data, responsibilities);
        (means.into_pyarray(py), covs.into_pyarray(py), weights.into_pyarray(py))
        {% else-%}
        todo!()
        {% endif %}
    }
    Ok(())
}

