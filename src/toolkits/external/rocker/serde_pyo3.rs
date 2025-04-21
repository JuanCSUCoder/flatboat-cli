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
