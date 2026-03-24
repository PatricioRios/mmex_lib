# Plan de Documentación: mmex_lib (Fase Beta)

Este documento detalla la estrategia integral para la documentación del proyecto `mmex_lib`. El objetivo es proporcionar claridad técnica para los desarrolladores del core y facilidad de integración para los usuarios de otros lenguajes (Python, Kotlin, Swift).

## 📊 Estado del Proyecto: BETA
La librería se encuentra actualmente en fase **Beta**. Esto significa:
- La API es funcional pero está sujeta a cambios (Breaking Changes).
- Faltan algunos módulos secundarios por implementar (ej. Inversiones avanzadas).
- Se requiere feedback de usuarios externos para estabilizar la v1.0.

---

## 🗺️ Estructura de Documentación (`docs-es/`)

### 1. Identidad y Acceso Rápido (Raíz)
- `README.md` (Principal): Visión general, matriz de funcionalidades y "Quick Start".
- `CHANGELOG.md`: Registro de cambios y breaking changes de la beta.
- `CONTRIBUTING.md`: Guía de estilo Rust, uso de `sea-query` y flujos de PR.

### 2. Arquitectura y Diseño (`docs-es/architecture/`)
- `overview.md`: Explicación de las capas DDD (Domain, Infrastructure, Services, API, FFI).
- `database_mapping.md`: Cómo mapeamos el esquema legacy de `tables.sql` a Rust usando `sea-query`.
- `ffi_bridge.md`: Funcionamiento de UniFFI y la gestión de memoria/hilos con `Arc<Mutex>`.

### 3. Guías de Usuario (`docs-es/guides/`)
- `getting_started_rust.md`: Instalación vía Cargo y uso de `MmexContext`.
- `getting_started_python.md`: Configuración con `maturin` y uso de `MmexEngine`.
- `error_handling.md`: Jerarquía de errores y cómo se proyectan a otros lenguajes.
- `testing_guide.md`: Cómo escribir tests de integración con bases de datos en memoria.

### 4. Roadmap y Estabilidad (`docs-es/roadmap/`)
- `status.md`: Definición de "Beta", qué módulos son estables y qué falta.
- `future_features.md`: Próximas integraciones (Swift/iOS, adjuntos, optimizaciones SQL).

### 5. Ejemplos (`examples/`)
- `examples/README.md`: Índice de ejemplos con descripción y comandos de ejecución.
- `rust/`: Ejemplos puros de lógica de negocio.
- `python/`: Scripts de integración real.

---

## 🛠️ Plan de Acción (Cronograma de Ejecución)

### Paso 1: Cimientos (Inmediato)
1.  [ ] Crear `docs-es/documentation_plan.md` (Completado ✅).
2.  [ ] Actualizar `Cargo.toml` y `pyproject.toml` a `0.1.0-beta.1`.
3.  [ ] Crear el `README.md` renovado en español que sirva de portal.

### Paso 2: Guías Críticas
1.  [ ] Redactar `docs-es/guides/getting_started_python.md` (Prioridad alta para usuarios).
2.  [ ] Actualizar y traducir `docs-es/guides/error_handling.md`.
3.  [ ] Documentar el mapeo de base de datos (`docs-es/architecture/database_mapping.md`).

### Paso 3: Documentación del Código (RustDoc)
1.  [ ] Audit de comentarios `///` en `src/domain/`.
2.  [ ] Audit de comentarios `///` en `src/services/`.
3.  [ ] Añadir ejemplos de código probables (doctests) en la capa `api`.

### Paso 4: Roadmap y Cierre
1.  [ ] Crear `docs-es/roadmap/status.md` con la matriz de estabilidad.
2.  [ ] Revisar y actualizar los ejemplos en `examples/`.

---

## 💡 Principios de la Documentación
- **Fácil de encontrar:** Estructura de carpetas intuitiva.
- **Fácil de leer:** Uso de Mermaid para diagramas de arquitectura.
- **Verificable:** Los ejemplos de código deben ser probados automáticamente si es posible.
- **Sincronizada:** La documentación debe actualizarse junto con el código.
