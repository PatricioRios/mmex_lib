# MMEX Lib

Rust library for interacting with Money Manager EX data and logic, with UniFFI support for multi-language bindings.

## Prerequisites

- **Rust**: `cargo` installed.
- **Python**: `python3` installed.
- **Kotlin/Java**: `kotlinc` (Kotlin compiler) and `java` (JRE) installed.

## Build the Library

First, compile the Rust project to generate the shared library:

```bash
cargo build
```

## Running Examples

### Python

1. **Setup**: The example requires the shared library to be linked or present in the folder.
2. **Run** (run before "cargo run --bin uniffi-bindgen generate --library target/debug/libmmex_lib.so --language python --out-dir examples/python":
   ```bash
   PYTHONPATH=examples/python/ python3 examples/python/main.py
   ```

#### Example
   ```bash
   cargo run --bin uniffi-bindgen generate --library target/debug/libmmex_lib.so --language python --out-dir examples/python

   PYTHONPATH=examples/python/ python3 examples/python/main.py

   ```


### Kotlin

1. **Dependencies**: The example requires `jna.jar` (located in `examples/kotlin/`).
2. **Compile**:
   ```bash
   # Use your kotlinc executable
   kotlinc -cp "examples/kotlin/jna.jar" \
     examples/kotlin/Main.kt examples/kotlin/uniffi/mmex_lib/mmex_lib.kt \
     -include-runtime -d examples/kotlin/main.jar
   ```
3. **Run**:
   ```bash
   java -Djna.library.path=target/debug -cp "examples/kotlin/jna.jar:examples/kotlin/main.jar" MainKt
   ```
#### Example
```bash
   cargo run --bin uniffi-bindgen generate --library target/debug/libmmex_lib.so --language kotlin --out-dir examples/kotlin

   kotlinc -cp "examples/kotlin/jna.jar" \
     examples/kotlin/Main.kt examples/kotlin/uniffi/mmex_lib/mmex_lib.kt \
     -include-runtime -d examples/kotlin/main.jar

   java -Djna.library.path=target/debug -cp "examples/kotlin/jna.jar:examples/kotlin/main.jar" MainKt
   
   ```

## Development

To generate bindings for other languages, use the included `uniffi-bindgen` binary:

```bash
cargo run --bin uniffi-bindgen generate --library target/debug/libmmex_lib.so --language <LANGUAGE> --out-dir <OUT_DIR>
```
