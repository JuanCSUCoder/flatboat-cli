use pyo3::{types::{PyAnyMethods, PyDict, PyString}, Bound, IntoPyObject, PyAny, PyErr, Python};
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
            let key = PyString::new(py, key);
            let value = match value {
                serde_json::Value::String(s) => PyString::new(py, s),
                _ => return Err(
                  PyErr::new::<PyAny, _>("Invalid configuration file.")
                ),
            };
            dict.set_item(key, value)?;
        }
        Ok(dict.into())
    }
}

impl ValidMap {
    pub fn from(map: Map<String, serde_json::Value>) -> Self {
      Self(map)
    }
}

mod tests {
    use super::*;

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
