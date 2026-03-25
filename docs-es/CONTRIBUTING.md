# Guía de Contribución para mmex_lib 🤝

¡Gracias por tu interés en contribuir a `mmex_lib`! Como un proyecto en fase beta que busca ser el núcleo multiplataforma de Money Manager EX, tus aportaciones son fundamentales.

## 🏗 Arquitectura y Estándares

El proyecto sigue una arquitectura **Domain-Driven Design (DDD)**. Antes de realizar cambios, asegúrate de comprender la separación de capas:

1.  **Dominio (`src/domain`)**: Contiene los modelos, tipos y definiciones de repositorios. No debe depender de la base de datos ni de librerías externas de infraestructura.
2.  **Infraestructura (`src/infrastructure`)**: Implementaciones de los repositorios usando `rusqlite` y `sea-query`. Aquí es donde reside el conocimiento del esquema legacy.
3.  **Servicios (`src/services`)**: Lógica de negocio que coordina los repositorios.
4.  **FFI (`src/ffi`)**: Fachada para UniFFI que expone la funcionalidad a Python, Kotlin y Swift.

## 🛠 Proceso de Desarrollo

### 1. Requisitos
- Rust toolchain (última versión estable).
- Python (para tests de integración).
- `maturin` para el empaquetado de Python.

### 2. Flujo de Trabajo
1.  **Crea una rama** para tu funcionalidad o corrección: `git checkout -b feat/nueva-funcionalidad`.
2.  **Escribe código idiomático**: Sigue las convenciones de Rust y utiliza `cargo fmt` antes de enviar tus cambios.
3.  **Manejo de Errores**: Nunca uses `panic!`. Usa `Result<T, MmexError>` y define errores específicos en el módulo de dominio si es necesario.
4.  **Seguridad Financiera**: Siempre usa el tipo `Money` (basado en `Decimal`) para montos monetarios. Nunca uses `f64`.

### 3. Testing (Crítico ⚠️)
Actualmente estamos trabajando en mejorar la cobertura de tests.
- **Unit Tests**: Deben ir en el mismo archivo del módulo, dentro de un bloque `mod tests`.
- **Integration Tests**: Se ubican en `tests/` y deben usar la API pública.
- **Ejecución**:
  ```bash
  cargo test          # Todos los tests de Rust
  make test-python    # Tests de integración con Python
  ```

## 📝 Convenciones de Código

- **Nombramiento**: `snake_case` para funciones/variables, `PascalCase` para tipos/traits.
- **Documentación**: Usa `///` para documentar todos los elementos públicos. Explica el *por qué* y proporciona ejemplos si la lógica es compleja.
- **Commits**: Mensajes claros y descriptivos en inglés o español.

## 🚀 Envío de Cambios

1.  Asegúrate de que `cargo check` y `cargo test` pasen sin errores ni warnings.
2.  Asegúrate de que el código esté formateado (`cargo fmt`).
3.  Envía tu Pull Request con una descripción detallada de los cambios y qué problema resuelven.

---

Al contribuir, aceptas que tu código estará bajo la misma licencia que el proyecto original.
