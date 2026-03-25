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
1.  [x] Crear `docs-es/documentation_plan.md` (Completado ✅).
2.  [x] Actualizar `Cargo.toml` y `pyproject.toml` a `0.1.0-beta.1` (Completado ✅).
3.  [x] Crear el `README.md` renovado en español que sirva de portal (Completado ✅).

### Paso 2: Guías Críticas
1.  [x] Redactar `docs-es/guides/getting_started_python.md` (Completado ✅).
2.  [x] Actualizar y traducir `docs-es/guides/error_handling.md` (Completado ✅).
3.  [x] Documentar el mapeo de base de datos (`docs-es/architecture/database_mapping.md`) (Completado ✅).
4.  [x] Redactar la arquitectura general (`docs-es/architecture/overview.md`) (Completado ✅).

### Paso 3: Documentación del Código (RustDoc)
1.  [x] Audit de comentarios `///` en `src/domain/` (Completado ✅).
2.  [x] Audit de comentarios `///` en `src/services/` (Completado ✅).
3.  [x] Añadir ejemplos de código probables (doctests) en la capa `api` (Completado ✅).

### Paso 4: Roadmap y Cierre
1.  [x] Crear `docs-es/roadmap/status.md` con la matriz de estabilidad (Completado ✅).
2.  [x] Revisar y actualizar los ejemplos en `examples/` (Completado ✅).
3.  [x] Crear `docs-es/CONTRIBUTING.md` (Completado ✅).
4.  [x] Crear `docs-es/roadmap/future_features.md` (Completado ✅).

---

## 💡 Principios de la Documentación
- **Fácil de encontrar:** Estructura de carpetas intuitiva.
- **Fácil de leer:** Uso de Mermaid para diagramas de arquitectura.
- **Verificable:** Los ejemplos de código deben ser probados automáticamente si es posible.
- **Sincronizada:** La documentación debe actualizarse junto con el código.
