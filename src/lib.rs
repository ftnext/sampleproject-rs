use pyo3::prelude::*;

#[pyfunction]
fn add_one(number: i32) -> i32 {
    number + 1
}

#[pymodule]
mod sample {
    #[pymodule_export]
    use super::add_one;
}
