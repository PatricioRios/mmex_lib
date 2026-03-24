# MMEX Lib

Rust library for interacting with Money Manager EX data and logic, with UniFFI support for multi-language bindings.

> ⚠️ **PROJECT STATUS: BETA**
> This library is in beta phase. API changes may occur. Use with caution.

---

### 🌐 Documentación en Español (Spanish)
Toda la documentación detallada y guías en español están disponibles en:
👉 **[docs-es/README.md](docs-es/README.md)**

---

## 🛠 Prerequisites

- **Rust**: `cargo` installed.
- **Python 3.8+**: `python3` installed.
- **Kotlin/Java**: `kotlinc` and `java` (JRE) installed.

## 🚀 Quick Start (Python)

### Option A: Automatic Setup (Recommended)
```bash
make setup
```
This will create a virtual environment, install dependencies, and build the library.

### Option B: Manual Setup

1. **Create a virtual environment**:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   ```

2. **Install dependencies**:
   ```bash
   pip install maturin rich
   ```

3. **Install the library**:
   ```bash
   maturin develop
   ```

4. **Usage**:
   ```python
   import mmex_lib
   engine = mmex_lib.MmexEngine("my_finance.mmb", None)
   ```

## 🏗 Development

We use a `Makefile` to simplify common development tasks.

### Installation & Build
```bash
make setup      # Complete setup (venv + deps + lib)
make develop    # Recompile after Rust changes
make build      # Build release wheel
```

### Running Tests
```bash
make test-rust    # Rust unit and integration tests
make test-python  # Python examples as integration tests
make test         # Run all tests
```

### Manual Binding Generation
To generate bindings for other languages (Kotlin, Swift, etc.):
```bash
cargo run --bin uniffi-bindgen generate --library target/debug/libmmex_lib.so --language <LANGUAGE> --out-dir <OUT_DIR>
```

## 📁 Project Structure

- `src/`: Core logic in Rust (Domain, Infrastructure, Services).
- `python/mmex_lib/`: Python package wrapper.
- `examples/`: Usage examples in different languages.
- `docs/`: Technical documentation and plans.

## ⚠️ Troubleshooting

### "externally-managed-environment" error
This happens when trying to install packages globally on modern Linux distributions. **Always use a virtual environment**:
```bash
python3 -m venv .venv
source .venv/bin/activate
```

### patchelf warning
Maturin may show a warning about `patchelf`. This doesn't affect local development. For wheel distribution:
```bash
sudo apt install patchelf
```

## 📄 License
Check the LICENSE file for details.
