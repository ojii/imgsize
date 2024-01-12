mod bmp;
mod gif;
mod jpg;
mod png;
mod utils;

use pyo3::prelude::*;

#[cfg(test)]
use serde::Deserialize;

#[pyclass(get_all)]
#[derive(Debug, Eq, PartialEq)]
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
}

const FORMATS: &'static [fn(&[u8]) -> Option<Size>] =
    &[png::get_size, jpg::get_size, gif::get_size, bmp::get_size];
pub fn get_size(data: &[u8]) -> Option<Size> {
    for format in FORMATS {
        if let Some(size) = format(data) {
            return Some(size);
        }
    }
    None
}

#[pyfunction]
#[pyo3(name = "get_size")]
fn py_get_size(data: &[u8]) -> PyResult<Option<Size>> {
    Ok(get_size(data))
}

#[pymodule]
fn imgsize(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_get_size, m)?)?;
    m.add_class::<Size>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(data: &[u8], output: &[u8]) {
        let output: Size = serde_json::from_slice(output).unwrap();
        let size = get_size(data);
        assert_eq!(size, Some(output));
    }

    #[test]
    fn test_bmp() {
        check(
            include_bytes!("test-data/example.bmp.input"),
            include_bytes!("test-data/example.bmp.output"),
        )
    }

    #[test]
    fn test_animated_gif() {
        check(
            include_bytes!("test-data/example.gif.input"),
            include_bytes!("test-data/example.gif.output"),
        )
    }

    #[test]
    fn test_gif() {
        check(
            include_bytes!("test-data/example2.gif.input"),
            include_bytes!("test-data/example2.gif.output"),
        )
    }

    #[test]
    fn test_jpg() {
        check(
            include_bytes!("test-data/example.jpg.input"),
            include_bytes!("test-data/example.jpg.output"),
        )
    }

    #[test]
    fn test_jpg2() {
        check(
            include_bytes!("test-data/hackerman.jpeg.input"),
            include_bytes!("test-data/hackerman.jpeg.output"),
        )
    }

    #[test]
    fn test_png() {
        check(
            include_bytes!("test-data/example.png.input"),
            include_bytes!("test-data/example.png.output"),
        );
    }

    #[test]
    fn test_apng() {
        check(
            include_bytes!("test-data/example.apng.input"),
            include_bytes!("test-data/example.apng.output"),
        );
    }
}
