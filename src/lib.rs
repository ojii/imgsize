pub mod avif;
pub mod bmp;
pub mod gif;
pub mod jpg;
pub mod png;
mod utils;

use pyo3::prelude::*;
use pyo3::types::PyDict;
#[cfg(test)]
use serde::Deserialize;
use std::array::IntoIter;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[pyclass(get_all)]
#[derive(Debug, Eq, PartialEq, Hash)]
#[cfg_attr(test, derive(Deserialize))]
pub struct Size {
    pub width: u64,
    pub height: u64,
    pub mime_type: String,
    pub is_animated: bool,
}

#[pymethods]
impl Size {
    #[new]
    fn new(width: u64, height: u64, mime_type: String, is_animated: bool) -> Self {
        Self {
            width,
            height,
            mime_type,
            is_animated,
        }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<SizeIter>> {
        let itr = SizeIter {
            inner: [slf.width, slf.height].into_iter(),
        };
        Py::new(slf.py(), itr)
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn as_dict(&self) -> PyResult<Py<PyDict>> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("width", self.width)?;
            dict.set_item("height", self.height)?;
            dict.set_item("mime_type", self.mime_type.clone())?;
            dict.set_item("is_animated", self.is_animated)?;
            Ok(dict.unbind())
        })
    }
}

#[pyclass]
struct SizeIter {
    inner: IntoIter<u64, 2>,
}

#[pymethods]
impl SizeIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<u64> {
        slf.inner.next()
    }
}

pub fn get_size(data: &[u8]) -> Option<Size> {
    match data.get(0..8)? {
        [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a] => png::get_size(data),
        [0xff, 0xd8, 0xff, _, _, _, _, _] => jpg::get_size(data),
        [b'G', b'I', b'F', b'8', b'7', b'a', _, _] | [b'G', b'I', b'F', b'8', b'9', b'a', _, _] => {
            gif::get_size(data)
        }
        [_, _, _, _, b'f', b't', b'y', b'p'] => avif::get_size(data),
        [b'B', b'M', _, _, _, _, _, _] => bmp::get_size(data),
        _ => None,
    }
}

/// Given the data in the bytes provided, attempts to determine the image format, size and whether it
/// is an animated image or not, otherwise returns None.
/// The data provided does not need to be the entire image data, the first kilobyte or so should
/// suffice.
#[pyfunction]
#[pyo3(name = "get_size")]
fn py_get_size(data: &[u8]) -> PyResult<Option<Size>> {
    Ok(get_size(data))
}

#[pymodule]
fn imgsize(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_get_size, m)?)?;
    m.add_class::<Size>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    macro_rules! define_test {
        ($name:ident) => {
            paste! {
                #[test]
                fn [<test_ $name>]() {
                    let output: Size = serde_json::from_slice(include_bytes!(concat!(
                        "test-data/",
                        stringify!($name),
                        ".output"
                    )))
                    .unwrap();
                    let size = get_size(include_bytes!(concat!(
                        "test-data/",
                        stringify!($name),
                        ".input"
                    )));
                    assert_eq!(size, Some(output));
                }
            }
        };
    }

    define_test!(bmp);
    define_test!(gifanim);
    define_test!(gif);
    define_test!(jpg);
    define_test!(jpeg);
    define_test!(png);
    define_test!(apng);
    define_test!(avif);
    define_test!(avis);
}
