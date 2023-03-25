use pyo3::prelude::*;
use uwuifier::uwuify_str_sse;

/// uwuify a given string
/// 
/// this func takes a string as input and returns a uwufied input
/// it uses "uwuify" lib (https://github.com/Daniel-Liu-c0deb0t/uwu).
#[pyfunction]
fn uwuify_str(text: &str) -> PyResult<String> {
    Ok(uwuify_str_sse(text))
}

/// a IPython module to provide binding to the lib
/// 
/// this mod uwuify_str func to Python, allowing users to
/// uwuify from their code.
#[pymodule]
fn uwupy(_py: Python, m: &PyModule) -> PyResult<()> {
    // add uwuify_str func to our module
    m.add_function(wrap_pyfunction!(uwuify_str, m)?)?;

    // return success of uwuify
    Ok(())
}

fn main() {
    // this isn't used, tho we leave as this due to "unused" warning
}
