# Ejemplos de uso de mmex_lib 🧪

Esta carpeta contiene ejemplos prácticos de cómo integrar y utilizar la librería `mmex_lib` en diferentes entornos y lenguajes.

## 📁 Estructura de la Carpeta

- `rust/`: Ejemplos puros de uso de la librería desde Rust.
- `python/`: Scripts de integración de Python utilizando `MmexEngine`.
- `kotlin/`: (En desarrollo) Ejemplos de integración para aplicaciones Android.

## 🚀 Cómo ejecutar los ejemplos

### Desde Rust

Puedes ejecutar los ejemplos de Rust utilizando `cargo run --example <nombre_del_ejemplo>`.

```bash
cargo run --example example_account
cargo run --example example_1
```

### Desde Python

Para ejecutar los ejemplos de Python, primero asegúrate de haber configurado el entorno virtual y compilado la librería:

```bash
make setup
source .venv/bin/activate
python examples/python/example.py
```

## 📝 Descripción de los ejemplos

| Archivo | Lenguaje | Descripción |
| :--- | :--- | :--- |
| `example_account.rs` | Rust | Cómo crear y listar cuentas utilizando `MmexContext`. |
| `example_1.rs` | Rust | Operaciones básicas de base de datos en memoria. |
| `python/example.py` | Python | Ejemplo básico de apertura de base de datos y listado de transacciones. |

---

## 💡 Contribuir con ejemplos
Si has desarrollado un caso de uso interesante, ¡te invitamos a compartirlo! Crea un Pull Request añadiendo tu ejemplo a la carpeta correspondiente.
