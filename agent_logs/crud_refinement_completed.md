# Seguimiento: Refinamiento CRUD (Edición y Borrado)
**Estado:** 🔴 EN PROGRESO (Uncompleted)

## Objetivos
Implementar los métodos `update` y `delete` en todas las capas para garantizar una gestión de datos completa.

## Progreso por Entidad

### 1. Tags (Etiquetas)
- [x] **Dominio:** Añadir `update` y `delete` al trait `TagRepository`.
- [x] **Infraestructura:** Implementar lógica de borrado y actualización en `SqlTagRepository`.
- [x] **Servicio:** Exponer `update_tag` y `delete_tag`.
- [x] **Validación:** Tests de integración.

### 2. Payees (Beneficiarios)
- [x] **Dominio:** Añadir `update` y `delete` al trait `PayeeRepository`.
- [x] **Infraestructura:** Implementar lógica de borrado y actualización en `SqlPayeeRepository`.
- [x] **Servicio:** Exponer `update_payee` y `delete_payee`.
- [x] **Validación:** Tests de integración.

### 3. Currencies (Monedas)
- [x] **Dominio:** Añadir `insert`, `update` y `delete` al trait `CurrencyRepository`.
- [x] **Infraestructura:** Implementar CRUD completo en `SqlCurrencyRepository`.
- [x] **Servicio:** Exponer métodos de gestión en `CurrencyService`.
- [x] **Validación:** Tests de integración.

### 4. Categories (Categorías)
- [x] **Dominio:** Añadir `insert`, `update` y `delete` al trait `CategoryRepository`.
- [x] **Infraestructura:** Implementar CRUD completo en `SqlCategoryRepository`.
- [x] **Servicio:** Exponer métodos de gestión jerárquica en `CategoryService`.
- [x] **Validación:** Tests de integración.

### 5. Accounts (Cuentas)
- [x] **Dominio:** Añadir `insert`, `update` y `delete` al trait `AccountRepository`.
- [x] **Infraestructura:** Implementar CRUD completo en `SqlAccountRepository`.
- [x] **Servicio:** Exponer métodos de gestión en `AccountService`.
- [x] **Validación:** Tests de integración.

### 6. Transactions (Transacciones)
- [x] **Dominio:** Añadir `insert`, `update` y `delete` al trait `TransactionRepository`.
- [x] **Infraestructura:** Implementar CRUD completo en `SqlTransactionRepository`.
- [x] **Servicio:** Exponer métodos de gestión y limpieza de vínculos en `TransactionService`.
- [x] **Validación:** Tests de integración con esquema multi-tabla.

## Registro de Actividad
- **2026-02-25:** Plan iniciado. Refinados los módulos de Tags, Payees, Currencies, Categories y Accounts.
- **2026-02-25:** Finalizado refinamiento de Transactions. Toda la librería soporta ahora operaciones CRUD completas bajo arquitectura DDD.
