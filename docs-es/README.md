# MMEX Lib 🚀

Librería en Rust para interactuar con los datos y la lógica de Money Manager EX (MMEX), con soporte multi-lenguaje mediante UniFFI.

> ⚠️ **ESTADO: BETA**
> Esta librería se encuentra en fase beta. La API puede sufrir cambios significativos. Úsalo con precaución en entornos de producción.

## 🌍 Soporte de Lenguajes

| Lenguaje | Gestor de Paquetes | Estado |
| :--- | :--- | :--- |
| **Python** (3.8+) | PyPI | ✅ Disponible (`pip install mmex-lib`) |
| **Rust** | crates.io | ⏳ Próximamente... |
| **Kotlin/JVM** | Maven Central | ⏳ Próximamente... |

## 🚀 Inicio Rápido (Python)

La forma más fácil de usar `mmex_lib` es mediante Python, instalándolo directamente desde PyPI.

### Instalación

```bash
pip install mmex-lib
```

### Uso

```python
import mmex_lib

engine = mmex_lib.MmexEngine("mi_finanza.mmb", None)
cuentas = engine.accounts().get_all_accounts()

for cuenta in cuentas:
    print(f"- {cuenta.name} (Balance Inicial: {cuenta.initial_balance})")
```

Para más detalles sobre la instalación y el uso, consulta la [Guía de Inicio para Python](guides/getting_started_python.md).

## 📊 Matriz de Funcionalidades (Beta)

| Módulo | Estado | Descripción |
| :--- | :--- | :--- |
| **Cuentas** | ⚠️ Beta | CRUD de cuentas y cálculo de balances. |
| **Transacciones** | ⚠️ Beta | Gestión de ingresos, gastos y transferencias. |
| **Categorías** | ⚠️ Beta | Gestión de categorías jerárquicas. |
| **Monedas** | ⚠️ Beta | Gestión de monedas y tasas de cambio. |
| **Activos (Assets)** | 🧪 Alpha | Tracking de activos fijos. |
| **Acciones (Stocks)** | 🧪 Alpha | Gestión de portafolio de acciones. |

## 🏗 Desarrollo y Contribución

Si deseas compilar la librería desde el código fuente (para desarrollo o para usarla en Rust), por favor consulta las siguientes guías:

- [Guía de Contribución](CONTRIBUTING.md)
- [Arquitectura del Proyecto](architecture/overview.md)

### Configuración para desarrollo (Python/Rust)

```bash
make setup      # Configuración completa (venv + deps + compilar lib)
make develop    # Recompilar tras cambios en Rust
make test         # Ejecutar todos los tests (Rust + Python)
```

## 📁 Navegación de Documentación

Toda la documentación detallada técnica está aquí:

- [Plan de Documentación](documentation_plan.md)
- [Guía de Manejo de Errores](guides/error_handling.md)
- [Mapeo de Base de Datos Legacy](architecture/database_mapping.md)
- [Estado del Proyecto y Roadmap](roadmap/status.md)
- [Funcionalidades Futuras](roadmap/future_features.md)

---

## 📄 Licencia

Consulta el archivo `LICENSE` para más detalles.
