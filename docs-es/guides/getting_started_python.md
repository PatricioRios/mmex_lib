# Inicio Rápido con Python (mmex_lib) 🐍

Esta guía te ayudará a integrar `mmex_lib` en tus proyectos de Python para interactuar con bases de datos de Money Manager EX (.mmb).

## 🛠 Requisitos Previos

1. **Python 3.8+** instalado.
2. **Entorno Virtual**: Se recomienda encarecidamente usar entornos virtuales para evitar conflictos con el sistema.

## 🚀 Instalación

`mmex_lib` está disponible en PyPI, lo que hace que su instalación sea muy sencilla.

### Paso 1: Crear un entorno virtual

```bash
python3 -m venv .venv
source .venv/bin/activate
```

### Paso 2: Instalar el paquete

```bash
pip install mmex-lib
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

## 🛠 Solución de Problemas y Desarrollo

Si deseas contribuir al proyecto o compilarlo desde el código fuente utilizando `maturin` y `cargo`, por favor consulta la [sección de Desarrollo en el README principal](../../README.md#desarrollo-y-contribución).
