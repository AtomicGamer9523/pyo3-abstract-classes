//! This file is imported in the Binary crate
//! and is used to implement the Python interfaces

use pyo3::prelude::*;
use abstracto3::*;

abstracto3!{
    abstract struct Entity {
        @AbstractProperty
        abstract fn uuid(&self) -> String;
    }
}

#[pyclass]
struct GreeterEntity;

abstracto3!{
    impl Entity for GreeterEntity {
        fn uuid(&self) -> String {
            "123".to_string()
        }
    }
}

#[pymethods]
impl GreeterEntity {
    #[new]
    fn new() -> Self { Self }
    fn greet(&self, name: &str) -> String {
        format!("Hello, {}!", name)
    }
    #[classmethod]
    fn class_greet(_cls: &pyo3::types::PyType, name: &str) -> String {
        format!("Hello, {}!", name)
    }
}


#[pymodule]
#[doc(hidden)]
#[pyo3(name = "abstracto3")]
pub fn abstracto3_module(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Entity::add_to_module(m)?;

    Ok(())
}