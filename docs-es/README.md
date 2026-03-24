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
| **Cuentas** | ✅ Estable | CRUD de cuentas y cálculo de balances. |
| **Transacciones** | ✅ Estable | Gestión de ingresos, gastos y transferencias. |
| **Categorías** | ✅ Estable | Gestión de categorías jerárquicas. |
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
- [Guía de Manejo de Errores](guides/error_handling.md) (Próximamente)
- [Arquitectura del Proyecto](architecture/overview.md) (Próximamente)
- [Guía de Inicio para Python](guides/getting_started_python.md) (Próximamente)

---

## 📄 Licencia

Consulta el archivo `LICENSE` para más detalles.
