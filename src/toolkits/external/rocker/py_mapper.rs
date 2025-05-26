use pyo3::{exceptions::PyValueError, types::PyAnyMethods, Bound, PyAny, PyErr};

pub fn map2rs(result_tuple: Bound<'_, PyAny>) -> Result<(String, String), PyErr> {
    let mut iter = result_tuple.try_iter()?;
    let first = iter.next().ok_or(PyErr::new::<PyValueError, _>("Expected at least one element"))??;
    let second = iter.next().ok_or(PyErr::new::<PyValueError, _>("Expected at least two elements"))??;
    
    // Convert PyAny to String
    if first.is_none() || second.is_none() {
        return Err(PyErr::new::<PyValueError, _>("Expected two elements in the tuple"));
    }
    let first_str: String = first.extract()?;
    let second_str: String = second.extract()?;
    
    Ok((first_str, second_str))
}