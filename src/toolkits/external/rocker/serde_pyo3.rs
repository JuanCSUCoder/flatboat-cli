use pyo3::{exceptions::PyTypeError, types::{PyAnyMethods, PyBool, PyDict, PyString}, Bound, IntoPyObject, PyAny, PyErr, Python};
use serde_json::Map;

pub struct ValidMap (Map<String, serde_json::Value>);

impl<'py> IntoPyObject<'py> for ValidMap {
    type Target = PyDict;

    type Output = Bound<'py, PyDict>;

    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let map = self.0;
        let dict = PyDict::new(py);

        for (key, value) in map.iter() {
            let key: Bound<'py, PyString> = PyString::new(py, key);
            let value: Bound<'py, PyAny> = match value {
                serde_json::Value::String(s) => PyString::new(py, s).into_any(),
                serde_json::Value::Bool(b) => PyBool::new(py, b.clone()).to_owned().into_any(),
                _ => return Err(
                  PyErr::new::<PyTypeError, _>("Invalid configuration file.")
                ),
            };
            dict.set_item(key, value)?;
        }
        Ok(dict.into())
    }
}

mod tests {
    use super::{ValidMap};
    use serde_json::Map;
    use pyo3::{types::PyAnyMethods, IntoPyObject, PyErr, Python};

    #[test]
    fn test_valid_map() -> Result<(), PyErr> {
        let mut map = Map::new();
        map.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
        map.insert("key2".to_string(), serde_json::Value::String("value2".to_string()));

        let valid_map = ValidMap::from(map);

        return Python::with_gil(|py| {
            let py_dict = valid_map.into_pyobject(py)?;

            assert_eq!(py_dict.len()?, 2);
            assert_eq!(py_dict.get_item("key1")?.extract::<String>()?, "value1");
            assert_eq!(py_dict.get_item("key2")?.extract::<String>()?, "value2");

            return Ok(());
        });
    }
}
