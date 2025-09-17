use pyo3::prelude::*;

#[pyfunction]
fn main() {
    println!("Call your main application code here");
}

#[pyfunction]
fn add_one(number: i32) -> i32 {
    number + 1
}

#[pymodule]
mod sample {
    #[pymodule_export]
    use super::main;

    #[pymodule_export]
    use super::add_one;
}
