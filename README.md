# uroman-fast

Fast Python wrapper for [uroman-rs](https://github.com/stellanomia/uroman-rs), a high-performance universal romanizer that converts text from any Unicode script to Latin characters.

## Features

- **High performance** — Rust-powered implementation via PyO3
- **Universal support** — Handles Japanese, Arabic, Devanagari, Chinese, Korean, and 100+ other scripts
- **Optional language hints** — ISO 639-3 codes improve romanization quality
- **Zero dependencies** — No Python runtime dependencies beyond the extension

## Installation

### From PyPI

```bash
pip install uroman-fast
```

### From source

```bash
git clone https://github.com/YOUR_USERNAME/uroman-fast.git
cd uroman-fast
pip install -e .
```

### Requirements

- **Python** ≥ 3.8
- **Rust** ≥ 1.85 (for building from source; [install via rustup](https://rustup.rs/))

## Usage

```python
from uroman_fast import romanize

# Basic usage
print(romanize("こんにちは、世界！"))
# konnichiha, shijie!

# With optional language code for better romanization
print(romanize("ユーロマン", lang="jpn"))
# yuuroman

# Arabic
print(romanize("مرحبا", lang="ara"))
# marhaba

# Devanagari (Hindi)
print(romanize("नमस्ते", lang="hin"))
# namaste
```

### API

```python
romanize(text: str, lang: str | None = None) -> str
```

- **text** — Input text in any Unicode script
- **lang** — Optional ISO 639-3 language code (e.g., `"jpn"`, `"ara"`, `"hin"`, `"cmn"`)
- **Returns** — Romanized text in Latin script

## Building from source

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/uroman-fast.git
cd uroman-fast

# Ensure Rust 1.85+ is installed
rustup update

# Install in editable mode (development)
pip install -e .

# Or build a wheel
pip install maturin
maturin build --release
```

## License

MIT — see [LICENSE](LICENSE) for details.

## Acknowledgments

- [uroman-rs](https://github.com/stellanomia/uroman-rs) by Stellanomia — the underlying romanization engine
- [uroman](https://github.com/isi-nlp/uroman) — the original Perl implementation
