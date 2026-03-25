# Inicio Rápido con Python (mmex_lib) 🐍

Esta guía te ayudará a integrar `mmex_lib` en tus proyectos de Python para interactuar con bases de datos de Money Manager EX (.mmb).

## 🛠 Requisitos Previos

1. **Python 3.8+** instalado.
2. **Entorno Virtual**: Se recomienda encarecidamente usar entornos virtuales para evitar conflictos con el sistema.

### Herramientas Utilizadas:
- **Maturin**: El estándar para construir y publicar paquetes Python escritos en Rust.
- **UniFFI**: Generador de interfaces que permite llamar a Rust desde Python manteniendo la seguridad de tipos.

## 🚀 Configuración del Proyecto

### Paso 1: Clonar e Instalar (Modo Desarrollo)

Si tienes el código fuente de `mmex_lib`, puedes usar el `Makefile` para una configuración rápida:

```bash
make setup
```

Este comando:
1. Crea un entorno virtual (`.venv`).
2. Instala `maturin` (necesario para compilar el core de Rust).
3. Compila e instala la librería en el entorno virtual.

### Paso 2: Activar el entorno virtual

```bash
source .venv/bin/activate
```

---

## 💻 Ejemplo de Uso Básico

Aquí tienes un script básico para abrir una base de datos y listar todas las cuentas:

```python
import mmex_lib
import os

# 1. Crear una instancia del motor
# Parámetros: ruta al archivo .mmb, clave (None si no está cifrado)
db_path = "mi_finanza.mmb"
engine = mmex_lib.MmexEngine(db_path, None)

# 2. Acceder al gestor de cuentas
account_manager = engine.accounts()

# 3. Obtener todas las cuentas
cuentas = account_manager.get_all_accounts()

print(f"Encontradas {len(cuentas)} cuentas:")
for cuenta in cuentas:
    print(f"- {cuenta.name} (Balance Inicial: {cuenta.initial_balance})")
```

---

## 📂 Operaciones Comunes

### Obtener el Balance Actual de una Cuenta

```python
from mmex_lib import AccountId

# Supongamos que conocemos el ID de la cuenta
account_id = 1
balance_info = engine.accounts().get_account_balance(account_id)

print(f"Cuenta ID: {balance_info.account_id}")
print(f"Balance Actual: {balance_info.current_balance}")
```

### Listar Transacciones

```python
transactions = engine.transactions().get_all_transactions()
for tx in transactions:
    print(f"Fecha: {tx.trans_date} | Monto: {tx.amount} | Tipo: {tx.trans_code}")
```

---

## ⚠️ Manejo de Errores

La librería lanza excepciones nativas de Python ante fallos en la base de datos o lógica de negocio.

```python
try:
    engine = mmex_lib.MmexEngine("archivo_corrupto.mmb", None)
except Exception as e:
    print(f"Error al abrir la base de datos: {e}")
```

---

## 🛠 Solución de Problemas

### Error "externally-managed-environment"
Este error ocurre en sistemas Linux modernos al intentar instalar paquetes con `pip` globalmente.
**Solución**: Usa siempre el entorno virtual creado con `make setup`.

### Errores de compilación de Rust
Asegúrate de tener instalado el toolchain de Rust (`rustc` y `cargo`). Puedes instalarlo desde [rustup.rs](https://rustup.rs/).

### Advertencia de patchelf
Maturin puede mostrar una advertencia sobre `patchelf`. Esto no afecta el desarrollo local. Para la distribución de binarios (wheels), puedes instalarlo con:
```bash
sudo apt install patchelf
```
