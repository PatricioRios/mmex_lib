# Propuesta de Refactorización del Sistema de Errores

Este documento detalla la transición hacia un sistema de errores jerárquico y semántico basado en el dominio, utilizando `thiserror` para la composición y delegación.

## 1. Objetivos
- **Descentralización:** Cada módulo de dominio define sus propios errores específicos.
- **Semántica:** Evitar el uso de errores genéricos cuando existe un error de negocio claro (ej. `DuplicateTagName` vs `DatabaseError`).
- **Composición:** Mantener la capacidad de propagar errores de infraestructura (DB, I/O) a través de un error común.
- **Compatibilidad FFI:** Asegurar que los errores sigan siendo exportables a través de UniFFI/NAPI.

## 2. Arquitectura de Errores

### Capa Base: `MmexError` (Común)
Ubicación: `src/domain/error.rs`
Contiene errores que no pertenecen a un dominio específico sino a la infraestructura o lógica transversal:
- `Database(String)`
- `Crypto(String)`
- `Mapping(String)`
- `Internal(String)`

### Capa de Dominio: `[Entity]Error`
Ubicación: Dentro de cada archivo de dominio (ej. `src/domain/tags.rs`).
Estructura sugerida:
```rust
#[derive(Error, Debug)]
pub enum TagError {
    #[error("Error común de MMEX: {0}")]
    Common(#[from] MmexError),

    #[error("Etiqueta no encontrada: {0}")]
    NotFound(i64),

    #[error("Nombre de etiqueta duplicado: {0}")]
    DuplicateName(String),
}
```

## 3. Hoja de Ruta de Implementación

### Fase 1: Reubicación y Base
1.  Mover `src/error.rs` a `src/domain/error.rs`.
2.  Actualizar `src/domain/mod.rs` para exportar el módulo `error`.
3.  Limpiar `MmexError` de variantes de validación específicas que ahora irán en el dominio.

### Fase 2: Implementación por Módulos (Iterativo)
Para cada módulo (Tags, Payees, Categories, Accounts, Transactions, etc.):
1.  Definir el enum `[Entity]Error` en su archivo `domain`.
2.  Actualizar el Repositorio (Trait en `domain` e Implementación en `infrastructure`) para devolver el nuevo error.
3.  Actualizar el Servicio para devolver el nuevo error.
4.  Mapear errores específicos (ej. convertir `rusqlite::Error::QueryReturnedNoRows` a `EntityError::NotFound` en lugar de un `Database(String)` genérico).

### Fase 3: Capa de Aplicación y FFI
1.  Ajustar `MmexEngine` en `src/api/ffi.rs`. Dado que UniFFI/NAPI suelen preferir un único punto de entrada de errores por objeto, decidiremos si:
    - Aplanamos los errores de dominio en el motor.
    - O mantenemos la jerarquía si el puente FFI lo soporta adecuadamente.

## 4. Beneficios Esperados
- Código más legible en los servicios al no tener que comparar strings para manejar errores.
- Mejor experiencia para los consumidores de la librería (FFI) al recibir errores tipados.
- Mayor facilidad para tests unitarios que verifiquen condiciones de error específicas.

---
**Estado:** COMPLETADO (Marzo 2026)

## Resumen de la Implementación
- Se ha descentralizado el sistema de errores.
- Cada módulo de dominio tiene su propio enum de error.
- Se mantiene la compatibilidad con FFI mediante la conversión de errores de dominio a `MmexError`.
- Se han eliminado las dependencias de strings para detectar errores comunes como "NoRowsFound".
