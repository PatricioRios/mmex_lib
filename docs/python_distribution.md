# Plan de Distribución Profesional de mmex_lib para Python

Este documento detalla la estrategia para convertir la biblioteca Rust `mmex_lib` en un paquete Python instalable y fácil de usar, permitiendo a los desarrolladores de Python integrarla sin fricción.

## 1. Herramientas Clave

- **Maturin**: El estándar de la industria para construir y publicar paquetes de Python escritos en Rust. Maneja la compilación y el empaquetado de binarios.
- **UniFFI**: Generador de interfaces que permite que el código Rust sea llamado desde Python (y otros lenguajes) manteniendo la seguridad de tipos.
- **Venv (Virtual Environment)**: Herramienta de Python para crear entornos aislados, garantizando que las dependencias de `mmex_lib` no interfieran con otros proyectos.

## 2. Estructura del Proyecto

Se reorganizará el proyecto para cumplir con los estándares de empaquetado de Python:

```text
/
├── pyproject.toml          # Define Maturin como el motor de construcción
├── Cargo.toml              # Metadatos de Rust y configuración de Maturin
├── Makefile                # Comandos abreviados (build, install, test)
├── src/                    # Código fuente en Rust (el "corazón")
└── python/
    └── mmex_lib/
        ├── __init__.py     # Punto de entrada de Python y carga de UniFFI
        └── py.typed        # Marcador para soporte de tipado estático
```

## 3. Experiencia del Desarrollador

### Para Desarrolladores de Python (Uso Ágil)

#### Opción A: Setup Automático (Recomendado)
```bash
# Crea el venv, instala dependencias y compila la librería
make setup
```

#### Opción B: Setup Manual
1. **Crear un entorno virtual**:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   ```

2. **Instalar dependencias de desarrollo**:
   ```bash
   pip install maturin rich
   ```

3. **Instalar la librería en modo desarrollo**:
   ```bash
   maturin develop
   ```

4. **Uso**:
   ```python
   import mmex_lib
   engine = mmex_lib.MmexEngine("my_finance.mmb", None)
   ```

### Para Desarrolladores Core (Makefile)
Se proporciona un `Makefile` para automatizar las tareas repetitivas:
- `make setup`: Prepara el entorno completo (venv + dependencias + librería).
- `make develop`: Recompila e instala la librería tras cambios en Rust.
- `make build`: Compila la librería en modo release.
- `make test`: Ejecuta todos los tests (Rust y Python).

## 4. Notas Importantes

### Sobre el error "externally-managed-environment"
En sistemas modernos (Ubuntu 23.04+, Debian 12+), Python protege el entorno del sistema. **Siempre usa un entorno virtual** para trabajar con `mmex_lib`. El comando `make setup` lo hace automáticamente por ti.

### Sobre patchelf
Maturin puede mostrar una advertencia sobre `patchelf`. Esto no afecta el desarrollo local. Para distribución de binarios, puedes instalarlo con:
```bash
sudo apt install patchelf
```

## 5. Verificación

Para verificar que todo está funcionando correctamente:
```bash
make test-python
```
