# MMEX Lib 🚀

Librería en Rust para interactuar con los datos y la lógica de Money Manager EX (MMEX), con soporte multi-lenguaje mediante UniFFI.

> ⚠️ **ESTADO: BETA**
> Esta librería se encuentra en fase beta. La API puede sufrir cambios significativos. Úsalo con precaución en entornos de producción.

## 🛠 Requisitos Previos

- **Rust**: `cargo` instalado.
- **Python 3.8+**: `python3` instalado.
- **Kotlin/Java**: `kotlinc` y `java` (JRE) instalados.

## 🚀 Inicio Rápido

### Python (Recomendado)

1. **Configuración Automática**:
   ```bash
   make setup
   ```
   Esto creará un entorno virtual, instalará dependencias y compilará la librería.

2. **Uso**:
   ```python
   import mmex_lib
   engine = mmex_lib.MmexEngine("mi_finanza.mmb", None)
   cuentas = engine.accounts().get_all_accounts()
   ```

### Rust

Añade a tu `Cargo.toml`:
```toml
[dependencies]
mmex_lib = { path = "../mmex_lib" } # O vía git
```

Uso:
```rust
use mmex_lib::MmexContext;
use std::path::Path;

let ctx = MmexContext::open(Path::new("mi_finanza.mmb"), None).unwrap();
let cuentas = ctx.accounts().get_all_accounts().unwrap();
```

## 📊 Matriz de Funcionalidades (Beta)

| Módulo | Estado | Descripción |
| :--- | :--- | :--- |
| **Cuentas** | ⚠️ Beta | CRUD de cuentas y cálculo de balances. (Faltan Tests) |
| **Transacciones** | ⚠️ Beta | Gestión de ingresos, gastos y transferencias. (Faltan Tests) |
| **Categorías** | ⚠️ Beta | Gestión de categorías jerárquicas. (Faltan Tests) |
| **Monedas** | ⚠️ Beta | Gestión de monedas y tasas de cambio. |
| **Activos (Assets)** | 🧪 Alpha | Tracking de activos fijos. |
| **Acciones (Stocks)** | 🧪 Alpha | Gestión de portafolio de acciones. |

## 🏗 Desarrollo

Utilizamos un `Makefile` para simplificar las tareas comunes.

```bash
make setup      # Configuración completa
make develop    # Recompilar tras cambios en Rust
make test         # Ejecutar todos los tests (Rust + Python)
```

## 📁 Navegación de Documentación

Toda la documentación detallada se encuentra en la carpeta `docs-es/`:

- [Plan de Documentación](documentation_plan.md)
- [Guía de Contribución](CONTRIBUTING.md)
- [Guía de Manejo de Errores](guides/error_handling.md)
- [Arquitectura del Proyecto](architecture/overview.md)
- [Mapeo de Base de Datos Legacy](architecture/database_mapping.md)
- [Guía de Inicio para Python](guides/getting_started_python.md)
- [Estado del Proyecto y Roadmap](roadmap/status.md)
- [Funcionalidades Futuras](roadmap/future_features.md)

---

## 📄 Licencia

Consulta el archivo `LICENSE` para más detalles.
