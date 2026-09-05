use pyo3::prelude::*;

#[pyfunction]
fn main() {
    println!("Call your main application code here");
}

#[pyfunction]
fn add_one(number: i32) -> i32 {
    sample_core::add_one(number)
}

#[pymodule]
mod sample {
    use super::*;

    #[pymodule_export]
    use super::main;

    #[pymodule]
    mod simple {
        #[pymodule_export]
        use super::add_one;
    }
}
