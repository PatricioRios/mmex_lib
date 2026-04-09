# MMEX Lib

Rust library for interacting with Money Manager EX (MMEX) data and logic, with UniFFI support for multi-language bindings.

> ⚠️ **PROJECT STATUS: BETA**
> This library is in beta phase. API changes may occur. Use with caution.

---

### 🌐 Documentación en Español (Spanish)
Toda la documentación detallada y guías en español están disponibles en:
👉 **[docs-es/README.md](docs-es/README.md)**

---

## 🌍 Language Support

| Language | Package Manager | Status |
| :--- | :--- | :--- |
| **Python** (3.8+) | PyPI | ✅ Available (`pip install mmex-lib`) |
| **Rust** | crates.io | ✅ Available (`cargo add mmex-lib`) |
| **Kotlin/JVM** | Maven Central | ⏳ Coming soon... |

---

## 🚀 Quick Start (Python)

The easiest way to use `mmex_lib` is through Python.

### Installation

Install the library directly from PyPI:

```bash
pip install mmex-lib
```

### Usage

```python
import mmex_lib

# Initialize the engine (path to .mmb file, password if encrypted)
engine = mmex_lib.MmexEngine("my_finance.mmb", None)

# Access account manager and get all accounts
accounts = engine.accounts().get_all_accounts()

print(f"Found {len(accounts)} accounts:")
for account in accounts:
    print(f"- {account.name} (Initial Balance: {account.initial_balance})")
```

For more details, see the [Python Getting Started Guide](docs/guides/getting_started_python.md).

---

## 📊 Feature Matrix (Beta)

| Module | Status | Description |
| :--- | :--- | :--- |
| **Accounts** | ⚠️ Beta | Account CRUD and balance calculations. |
| **Transactions** | ⚠️ Beta | Income, expenses, and transfers management. |
| **Categories** | ⚠️ Beta | Hierarchical category management. |
| **Currencies** | ⚠️ Beta | Currency and exchange rate management. |
| **Assets** | 🧪 Alpha | Fixed assets tracking. |
| **Stocks** | 🧪 Alpha | Stock portfolio management. |

---

## 🏗 Development & Contributing

If you want to build the library from source or contribute to its development, please refer to the documentation:

- [Project Architecture (ES)](docs-es/architecture/overview.md)
- [Contributing Guide (ES)](docs-es/CONTRIBUTING.md)

### Building from source (Python)

```bash
make setup      # Complete setup (venv + deps + lib compilation)
make develop    # Recompile after Rust changes
make test       # Run all tests (Rust + Python)
```

## 📄 License
Check the LICENSE file for details.
