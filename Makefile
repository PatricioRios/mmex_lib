.PHONY: build install develop test clean venv setup

# Detectar si estamos en un entorno virtual
VENV_PYTHON = .venv/bin/python
PYTHON = $(if $(wildcard .venv/bin/python),.venv/bin/python,python3)

# Setup inicial (crear venv e instalar dependencias)
setup:
	python3 -m venv .venv
	.venv/bin/pip install maturin rich
	.venv/bin/maturin develop

# Comandos principales
build:
	maturin build

install:
	$(PYTHON) -m pip install .

develop:
	maturin develop

test: test-rust test-python

test-rust:
	cargo test

test-python:
	$(PYTHON) examples/python/tags_example.py
	$(PYTHON) examples/python/categories_example.py
	$(PYTHON) examples/python/accounts_example.py
	$(PYTHON) examples/python/transactions_example.py

clean:
	cargo clean
	rm -rf python/mmex_lib/*.so
	rm -rf python/mmex_lib/__pycache__
	rm -rf target/maturin
	rm -rf build
	rm -rf *.mmb
	rm -rf dist
	rm -rf *.egg-info
