#[path = "abstracto3/__init__.rs"]
mod __init__;

use __init__::*;
use pyo3::*;

fn main() -> PyResult<()> {
    append_to_inittab!(abstracto3_module);

    prepare_freethreaded_python();

    Python::with_gil(|py| {
        // add local files to sys.path
        py.import("sys")?
            .getattr("path")?
            .downcast::<pyo3::types::PyList>()?
            .insert(0, ".")?;

        // import main module
        py.import("main")?;

        Ok(())
    })
}
