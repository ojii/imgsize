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
use std::fmt::{Display, Formatter};

#[pyclass(eq, eq_int)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Animation {
    Yes,
    No,
    Unknown,
}

impl From<bool> for Animation {
    fn from(value: bool) -> Self {
        if value {
            Self::Yes
        } else {
            Self::No
        }
    }
}

impl Display for Animation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Animation::Yes => "yes",
            Animation::No => "no",
            Animation::Unknown => "unknown",
        })
    }
}

#[pyclass(get_all, eq, hash, frozen)]
#[derive(Debug, Eq, PartialEq, Hash)]
#[cfg_attr(test, derive(Deserialize))]
pub struct Size {
    pub width: u64,
    pub height: u64,
    pub mime_type: String,
    pub is_animated: Animation,
}

#[pymethods]
impl Size {
    #[new]
    fn new(width: u64, height: u64, mime_type: String, is_animated: Animation) -> Self {
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

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<SizeIter>> {
        let itr = SizeIter {
            inner: [slf.width, slf.height].into_iter(),
        };
        Py::new(slf.py(), itr)
    }

    fn as_dict(&self) -> PyResult<Py<PyDict>> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("width", self.width)?;
            dict.set_item("height", self.height)?;
            dict.set_item("mime_type", self.mime_type.clone())?;
            dict.set_item("is_animated", self.is_animated.clone().into_py(py))?;
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
    m.add_class::<Animation>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use serde::{de, Deserialize, Deserializer};
    use std::collections::HashSet;
    use std::fmt::Formatter;
    use std::path::Path;

    struct AnimationVisitor;

    impl<'de> de::Visitor<'de> for AnimationVisitor {
        type Value = Animation;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("'yes', 'no' or 'unknown' or a boolean")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Animation::from(v))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v {
                "yes" => Ok(Animation::Yes),
                "no" => Ok(Animation::No),
                "unknown" => Ok(Animation::Unknown),
                _ => Err(E::custom(format!("Unexpected value: {}", v))),
            }
        }
    }

    impl<'de> Deserialize<'de> for Animation {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(AnimationVisitor)
        }
    }

    macro_rules! define_tests {
        (impl $name:ident) => {
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
        (impl $name:ident, $($rest:ident),+) => {
            define_tests!(impl $name);
            define_tests!(impl $($rest),+);
        };
        ($($names:ident),+) => {
            define_tests!(impl $($names),+);

            #[test]
            fn test_no_missing_tests() {
                let expected: HashSet<String> = Path::new(file!())
                    .parent()
                    .expect("bad path").join("test-data")
                    .read_dir()
                    .expect("failed to read test data dir")
                    .map(|entry| entry.expect("bad dir entry").path())
                    .filter(|p| p.extension().map(|ext| ext == "input").unwrap_or_default())
                    .map(|p| p.file_stem().expect("no file stem").to_str().expect("failed to convert to str").to_string())
                    .collect();
                let mut tested = HashSet::new();
                $(
                    tested.insert(stringify!($names).to_string());
                )+
                assert_eq!(tested, expected);
            }
        };
    }

    define_tests!(bmp, gif, gif2, gifanim, gifanim2, jpg, jpeg, png, apng, avif, avis);
}
