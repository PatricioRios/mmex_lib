# Getting Started with Python (`mmex_lib`) 🐍

This guide will help you integrate `mmex_lib` into your Python projects to interact with Money Manager EX (.mmb) databases.

## 🛠 Prerequisites

1. **Python 3.8+** installed.
2. **Virtual Environment**: It is highly recommended to use virtual environments to avoid system conflicts.

## 🚀 Installation

`mmex_lib` is available on PyPI, making it extremely easy to install.

### Step 1: Create a virtual environment

```bash
python3 -m venv .venv
source .venv/bin/activate
```

### Step 2: Install the package

```bash
pip install mmex-lib
```

---

## 💻 Basic Usage Example

Here is a basic script to open a database and list all accounts:

```python
import mmex_lib
import os

# 1. Create an instance of the engine
# Parameters: path to the .mmb file, key (None if not encrypted)
db_path = "my_finance.mmb"
engine = mmex_lib.MmexEngine(db_path, None)

# 2. Access the account manager
account_manager = engine.accounts()

# 3. Get all accounts
accounts = account_manager.get_all_accounts()

print(f"Found {len(accounts)} accounts:")
for account in accounts:
    print(f"- {account.name} (Initial Balance: {account.initial_balance})")
```

---

## 📂 Common Operations

### Get Current Account Balance

```python
# Assuming we know the account ID
account_id = 1
balance_info = engine.accounts().get_account_balance(account_id)

print(f"Account ID: {balance_info.account_id}")
print(f"Current Balance: {balance_info.current_balance}")
```

### List Transactions

```python
transactions = engine.transactions().get_all_transactions()
for tx in transactions:
    print(f"Date: {tx.trans_date} | Amount: {tx.amount} | Type: {tx.trans_code}")
```

---

## ⚠️ Error Handling

The library raises native Python exceptions for database or business logic failures.

```python
try:
    engine = mmex_lib.MmexEngine("corrupt_file.mmb", None)
except Exception as e:
    print(f"Error opening database: {e}")
```

---

## 🛠 Troubleshooting & Development

If you want to contribute to the project or build it from source using `maturin` and `cargo`, please refer to the [Development section in the main README](../../README.md#development--contributing).
