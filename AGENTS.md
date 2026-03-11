# Agent Guidelines: mmex_lib

This repository contains a Rust library (`mmex_lib`) designed to interact with Money Manager EX (MMEX) data and logic. It features a clean architecture with domain, infrastructure, service, and API layers, and supports multi-language bindings via UniFFI and a C-ABI.

## 🛠 Build and Test Commands

### General
- **Build**: `cargo build`
- **Check (Fast)**: `cargo check`
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Format**: `cargo fmt`

### Testing
- **Run all tests**: `cargo test`
- **Run a single test file**: `cargo test --test <filename>` (e.g., `cargo test --test transactions_test`)
- **Run a specific test**: `cargo test -- <test_name>` (e.g., `cargo test -- test_get_all_accounts`)
- **Show stdout during tests**: Add `-- --nocapture` (e.g., `cargo test -- --nocapture`)

### UniFFI Binding Generation
Generate bindings after building the shared library:
- **Build library**: `cargo build`
- **Generate bindings**:
  ```bash
  cargo run --bin uniffi-bindgen generate --library target/debug/libmmex_lib.so --language <LANGUAGE> --out-dir <DIR>
  ```
  Supported languages: `python`, `kotlin`, `swift`, `ruby`.

---

## 🏗 Project Architecture

Adhere to the following layered structure:

1.  **`src/domain`**: Data models (`structs`), error types (`enums`), and repository traits.
    - Uses `serde` for serialization/deserialization.
    - Uses `thiserror` for error definitions.
2.  **`src/infrastructure`**: Implementations of domain traits (e.g., SQL repositories).
    - Uses `rusqlite` for direct SQLite access.
    - Uses `sea-query` for dynamic SQL building.
    - Contains `mapper.rs` for converting DB rows to domain models.
3.  **`src/services`**: High-level business logic that coordinates repositories.
    - Coordinate between domain services and repositories.
    - Responsible for high-level operations (e.g., performing a transfer between accounts).
    - Usually takes ownership or references of repositories.

---

## 📦 Key Dependencies and Usage

### `rusqlite`
- Primary database driver for SQLite.
- Use `bundled-sqlcipher` feature for encrypted database support.
- Map errors to `MmexError::Database(e.to_string())`.

### `sea-query`
- Use for building SQL queries programmatically to avoid string concatenation and SQL injection.
- Prefer `sea-query-rusqlite` for direct integration.

### `serde`
- Used across all domain types for JSON serialization (critical for JSON-based FFI).
- Ensure `derive(Serialize, Deserialize)` is added to all domain structs.

### `rust_decimal`
- All monetary values must use `Decimal` to ensure precision.
- Aliased as `Money` in `src/domain/types.rs`.

---

## 🎨 Code Style Guidelines

### 1. Naming Conventions
- **Functions & Variables**: `snake_case` (e.g., `get_db_version`).
- **Types & Traits**: `PascalCase` (e.g., `MmexError`, `AccountRepository`).
- **File Names**: Match the module name (e.g., `account_service.rs` for `AccountService`).
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `DB_VERSION`).


### 2. Error Handling
- **Never panic** in library code. Use `Result<T, MmexError>`.
- Define errors in `src/domain/error.rs` or local domain modules using `thiserror`.
- Convert third-party errors (like `rusqlite::Error`) to `MmexError` using `From` implementations or `.map_err()`.
- Example:
  ```rust
  pub enum TransactionError {
      #[error("Not found: {0}")]
      NotFound(TransactionId),
      #[error(transparent)]
      Common(#[from] MmexError),
  }
  ```

### 3. Imports and Formatting
- Group imports logically: standard library first, third-party crates next, then local modules.
- Run `cargo fmt` before committing.
- Prefer explicit imports over `*` imports, except for `sea_query` preludes or local `mod` re-exports.

### 4. Types and Safety
- Use domain-specific IDs (e.g., `AccountId(i64)`) instead of raw `i64` where possible to prevent mixing up IDs.
- Use `Arc<Mutex<T>>` for shared, mutable state in the `MmexEngine` to ensure thread safety across FFI boundaries.
- For money-related values, always use `rust_decimal::Decimal` (aliased as `Money` in domain types).

### 5. Documentation
- Use `///` for doc comments on public items (traits, structs, public functions).
- Keep comments focused on the "why" and complex logic, avoiding trivial "what" descriptions.

---

## 🔌 FFI and Interop Guidelines

### UniFFI
- Use `#[uniffi::export]` for functions and methods intended for multi-language use.
- Use `uniffi::setup_scaffolding!()` in `src/lib.rs`.
- Ensure all types passed through UniFFI boundaries are compatible (Primitive types, `String`, or `uniffi::Object` / `uniffi::Record`).

### Raw C-ABI
- Public C functions must be `#[no_mangle] pub extern "C"`.
- Use `*mut T` for opaque pointers to Rust objects (e.g., `*mut MmexEngine`).
- Provide "free" functions for any memory allocated in Rust that is passed to C (e.g., `mmex_engine_free`, `mmex_free_string`).
- Use `std::ffi::CString` for returning strings and `into_raw()` to transfer ownership to the caller.

---

## 🧪 Testing Guidelines
- **Integration Tests**: Place in the `tests/` directory. They should use the public API of the library.
- **Unit Tests**: Place in the same file as the code being tested in a `mod tests` block, guarded by `#[cfg(test)]`.
- **Database Tests**: Use in-memory or temporary SQLite databases for consistency.
- **Verification**: Always run `cargo check` and `cargo test` after modifying domain logic or repository implementations.

### Writing a new Integration Test
When adding a new feature, create a corresponding test file in `tests/`.
Use the `MmexContext` or `MmexEngine` to interact with the library.
Example structure:
```rust
#[test]
fn test_new_feature() {
    let ctx = MmexContext::open_in_memory().unwrap();
    // 1. Setup data
    // 2. Perform operation
    // 3. Assert results
}
```
If the test requires a real database file, use a temporary file and ensure it is cleaned up after the test.
Prefer `MmexContext::open_in_memory()` for speed and isolation.
