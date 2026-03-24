# Manejo de Errores en mmex_lib 🛡️

Este documento describe la arquitectura y las mejores prácticas para el manejo de errores en la biblioteca `mmex_lib`, detallando cómo los fallos de Rust se proyectan a otros lenguajes.

## 1. Jerarquía de Errores

La biblioteca utiliza una estructura de errores en capas para separar los fallos técnicos de infraestructura de los fallos de lógica de negocio.

### A. `MmexError` (Infraestructura y Errores Globales)
Definido en `src/domain/error.rs`, captura fallos transversales:
- **Errores de Base de Datos**: Mapeo semántico de `rusqlite::ErrorCode` (Disco lleno, Base de datos bloqueada, Corrupción).
- **Restricciones de Integridad**: Errores comunes de SQLite como `UNIQUE` o `FOREIGN KEY`.
- **Errores Internos**: Fallos inesperados en la memoria o configuración.

### B. Errores de Dominio (Específicos por Módulo)
Cada módulo (ej: `AccountError`, `TransactionError`) captura lógica de negocio específica:
- **Validaciones**: Campos obligatorios (ej: `NameRequired`), formatos inválidos.
- **Contexto**: Envuelve un `MmexError` cuando la persistencia falla durante una operación de ese módulo.

---

## 2. Mapeo de SQLite a `MmexError`

No se deben usar comparaciones de cadenas de texto para identificar errores de base de datos. Se utiliza el `ErrorCode` proporcionado por `rusqlite`.

```rust
// Ejemplo de mapeo semántico en Rust
match error.code {
    ErrorCode::DiskFull => MmexError::DiskFull(msg),
    ErrorCode::DatabaseBusy => MmexError::DatabaseBusy(msg),
    ErrorCode::DatabaseCorrupt => MmexError::DatabaseCorrupt(msg),
    ErrorCode::ConstraintViolation => {
        if msg.contains("UNIQUE") { MmexError::UniqueConstraint(msg) }
        else if msg.contains("FOREIGN KEY") { MmexError::ForeignKeyConstraint(msg) }
        else { MmexError::Database(msg) }
    }
    _ => MmexError::Database(msg),
}
```

---

## 3. Principios de Implementación

1.  **Atomicidad**: Las operaciones complejas deben garantizar que no haya estados parciales en caso de error (uso de transacciones).
2.  **Mensajes Descriptivos**: Siempre se incluye el mensaje original de SQLite (`to_string()`) para facilitar el soporte técnico.
3.  **Result<Option<T>, E>**: Las funciones de búsqueda por ID devuelven `Ok(None)` si el recurso no existe, reservando los errores para fallos reales de ejecución.
4.  **Observabilidad**: Los errores críticos se registran mediante el sistema de logs antes de ser propagados al usuario.

---

## 4. Uso desde FFI (UniFFI)

Los errores están marcados con `#[derive(uniffi::Error)]` para que sean proyectados como excepciones nativas en otros lenguajes:

### Python 🐍
Las excepciones se lanzan de forma estándar y el mensaje de error de Rust es accesible.
```python
try:
    engine.accounts().get_account_by_id(999) # ID inexistente
except Exception as e:
    print(f"Error detectado: {e}")
```

### Kotlin/Android 🤖
Se proyectan como excepciones específicas:
- `MmexException.DiskFull`
- `MmexException.DatabaseBusy`

### Swift/iOS 🍎
Se proyectan como `MmexError`:
- `MmexError.diskFull`
- `MmexError.databaseBusy`

---

## ⚠️ Errores Comunes de la Beta

Durante la fase beta, es posible encontrar errores genéricos del tipo `MmexError::Internal(String)`. Por favor, reporta estos errores incluyendo el mensaje detallado para que podamos categorizarlos correctamente en futuras versiones.
