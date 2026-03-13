# Estrategia de Manejo de Errores en mmex_lib

Este documento describe la arquitectura y las mejores prácticas para el manejo de errores en la biblioteca `mmex_lib`.

## 1. Jerarquía de Errores

La biblioteca utiliza una estructura de errores en capas para separar los fallos de infraestructura de los fallos de lógica de negocio.

### A. MmexError (Infraestructura y Errores Globales)
Definido en `src/domain/error.rs`, captura fallos transversales:
- **Errores de Base de Datos**: Mapeo semántico de `rusqlite::ErrorCode` (Disco lleno, Base de datos bloqueada, Corrupción).
- **Restricciones de Integridad**: Errores comunes de SQLite como `UNIQUE` o `FOREIGN KEY`.
- **Errores Internos**: Fallos inesperados o de memoria.

### B. Errores de Dominio (Específicos por Módulo)
Cada módulo (ej: `TagError`, `AccountError`) captura lógica de negocio:
- **Validaciones**: Campos obligatorios, formatos inválidos.
- **Contexto**: Envuelve un `MmexError` cuando la persistencia falla durante una operación específica.

## 2. Mapeo de SQLite a MmexError

No se deben usar comparaciones de cadenas de texto para identificar errores de base de datos. Se utiliza el `ErrorCode` proporcionado por `rusqlite`.

```rust
// Ejemplo de mapeo semántico
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

## 3. Principios de Implementación

1. **Atomicidad**: Las operaciones complejas en el repositorio deben garantizar que no haya estados parciales en caso de error (uso de transacciones donde sea necesario).
2. **Preservación de Mensajes**: Siempre se debe incluir el mensaje original de SQLite (`to_string()`) dentro del error de la biblioteca para facilitar el soporte técnico.
3. **Result<Option<T>, E>**: Las funciones de búsqueda por ID deben devolver `Ok(None)` si el recurso no existe, reservando los errores para fallos reales de ejecución o integridad.
4. **Observabilidad**: Los errores críticos deben ser registrados mediante el sistema de logs (`tracing`) antes de ser propagados al llamador.

## 4. Uso desde FFI (UniFFI)

Los errores están marcados con `#[derive(uniffi::Error)]` para que sean proyectados como excepciones nativas en:
- **Kotlin**: `MmexException.DiskFull`
- **Swift**: `MmexError.diskFull`
