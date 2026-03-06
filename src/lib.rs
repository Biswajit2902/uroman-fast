use pyo3::prelude::*;
use uroman::{rom_format, Uroman};

/// Romanize text from any Unicode script to Latin characters.
///
/// Args:
///     text: Input text in any script (e.g., Japanese, Arabic, Devanagari)
///     lang: Optional ISO 639-3 language code for better romanization (e.g., "jpn", "ara")
///
/// Returns:
///     Romanized text in Latin script
#[pyfunction]
#[pyo3(signature = (text, lang=None))]
fn romanize(text: &str, lang: Option<&str>) -> PyResult<String> {
    let uroman = Uroman::new();
    let lang_code = lang.map(|s| s.to_string());
    let result = uroman
        .romanize_string::<rom_format::Str>(text, lang_code.as_deref())
        .to_string();
    Ok(result)
}

/// uroman-fast: Fast Python wrapper for uroman-rs universal romanizer
#[pymodule]
fn uroman_fast(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(romanize, m)?)?;
    Ok(())
}
