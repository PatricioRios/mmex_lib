# Plan de Implementación de Managers FFI

Este documento detalla el plan para implementar los gestores FFI (Foreign Function Interface) faltantes en `src/ffi/`, exponiendo la funcionalidad de los servicios definidos en `src/services/` para su uso a través de UniFFI.

## 1. Gestores a Implementar

Se crearán los siguientes archivos en `src/ffi/`:

### `src/ffi/assets.rs` (`AssetManager`)
- **get_all**: Obtiene todos los activos.
- **get_by_id**: Obtiene un activo por ID.
- **create**: Crea un nuevo activo.
- **update**: Actualiza un activo existente.
- **delete**: Elimina un activo.
- **get_all_json**: Exporta la lista de activos a JSON.

### `src/ffi/categories.rs` (`CategoryManager`)
- **get_all**: Obtiene todas las categorías.
- **get_by_id**: Obtiene una categoría por ID.
- **get_subcategories**: Obtiene subcategorías de un padre.
- **create**: Crea una nueva categoría.
- **update**: Actualiza una categoría.
- **delete**: Elimina una categoría.
- **get_all_json**: Exporta a JSON.

### `src/ffi/currencies.rs` (`CurrencyManager`)
- **get_all**: Obtiene todas las monedas.
- **get_by_id**: Obtiene moneda por ID.
- **get_by_symbol**: Busca moneda por símbolo.
- **create**: Registra una nueva moneda.
- **update**: Actualiza datos de moneda.
- **delete**: Elimina moneda.
- **get_all_json**: Exporta a JSON.

### `src/ffi/payees.rs` (`PayeeManager`)
- **get_all**: Obtiene todos los beneficiarios.
- **get_by_id**: Obtiene beneficiario por ID.
- **create**: Crea un nuevo beneficiario.
- **update**: Actualiza beneficiario.
- **delete**: Elimina beneficiario.
- **get_all_json**: Exporta a JSON.

### `src/ffi/scheduled.rs` (`ScheduledManager`)
- **get_all**: Obtiene transacciones programadas.
- **get_by_id**: Obtiene por ID.
- **create**: Crea transacción programada.
- **update**: Actualiza transacción programada.
- **delete**: Elimina transacción programada.
- **get_all_json**: Exporta a JSON.

### `src/ffi/stocks.rs` (`StockManager`)
- **get_all**: Obtiene todos los valores/acciones.
- **get_by_id**: Obtiene por ID.
- **create**: Crea un nuevo valor.
- **update**: Actualiza valor.
- **delete**: Elimina valor.
- **get_all_json**: Exporta a JSON.

### `src/ffi/transactions.rs` (`TransactionManager`)
- **Gestión de Transacciones**: CRUD completo (`get_all`, `get_by_id`, `create`, `update`, `delete`).
- **Gestión de Etiquetas**: `get_tags`, `link_tag`, `unlink_tag`.
- **Gestión de Desgloses (Splits)**: `get_splits`, `add_split`, `update_split`, `delete_split`.
- **JSON**: Exportación de transacciones y desgloses.

## 2. Integración y Registro

### `src/ffi/mod.rs`
- Registrar los nuevos módulos: `assets`, `categories`, `currencies`, `payees`, `scheduled`, `stocks`, `transactions`.

### `src/ffi/engine.rs` (`MmexEngine`)
- Añadir métodos para acceder a cada nuevo manager:
  - `assets()` -> `Arc<AssetManager>`
  - `categories()` -> `Arc<CategoryManager>`
  - `currencies()` -> `Arc<CurrencyManager>`
  - `payees()` -> `Arc<PayeeManager>`
  - `scheduled()` -> `Arc<ScheduledManager>`
  - `stocks()` -> `Arc<StockManager>`
  - `transactions()` -> `Arc<TransactionManager>`

## 3. Verificación

1. **Compilación**: Ejecutar `cargo check` para validar tipos y exportaciones UniFFI.
2. **Pruebas**: (Opcional) Crear tests de integración si es posible para validar la exposición FFI.
