# mmex_lib: Backlog de Implementación

Este documento detalla la hoja de ruta para la implementación completa de la lógica de Money Manager EX (basada en el esquema definido en `tables.sql`), priorizando entidades con menor acoplamiento para construir un dominio robusto desde la base.

## Fase 1: Entidades de Catálogo (Baja Relación) [COMPLETADA]
- [x] **Módulo de Tags (Etiquetas)** - `TAG_V1`
- [x] **Módulo de Payees (Beneficiarios)** - `PAYEE_V1`
- [x] **Módulo de Currencies (Monedas)** - `CURRENCYFORMATS_V1`

## Fase 2: Entidades Jerárquicas [COMPLETADA]
- [x] **Módulo de Categorías** - `CATEGORY_V1`

## Fase 3: Refinamiento de Cuentas [COMPLETADA]
- [x] **Evolución de Account** - `ACCOUNTLIST_V1`

## Fase 4: El Núcleo de Transacciones (Alta Relación) [COMPLETADA]
- [x] **Módulo de Transacciones** - `CHECKINGACCOUNT_V1`
- [x] **Gestión de Tags en Transacciones** - `TAGLINK_V1`

## Fase 5: CRUD Extendido (Hacia Versión Utilizable)
Implementación de las entidades restantes del esquema para permitir la gestión total del archivo .mmb.

- [ ] **Split Transactions (Desgloses)** - `SPLITTRANSACTIONS_V1`
    - [ ] Domain: `SplitTransaction` model.
    - [ ] Infrastructure: Repositorio y Mapeo.
    - [ ] Service: Integración con `TransactionService`.
- [ ] **Scheduled Bills (Transacciones Programadas)** - `BILLSDEPOSITS_V1`
    - [ ] Domain: `ScheduledTransaction` model.
    - [ ] Infrastructure: Repositorio y Mapeo.
    - [ ] Service: `ScheduledService`.
- [ ] **Assets (Activos)** - `ASSETS_V1`
    - [ ] Domain: `Asset` model.
    - [ ] Infrastructure: Repositorio y Mapeo.
    - [ ] Service: `AssetService`.
- [ ] **Stocks (Inversiones)** - `STOCK_V1`
    - [ ] Domain: `Stock` model.
    - [ ] Infrastructure: Repositorio y Mapeo.
    - [ ] Service: `StockService`.

## Fase 6: Lógica de Negocio Avanzada
- [ ] **Cálculo de Balances**
    - [ ] Lógica para calcular saldos actuales sumando transacciones al balance inicial.
- [ ] **Transferencias entre Cuentas**
    - [ ] Lógica para manejar el par de transacciones vinculadas (Gasto/Ingreso).

## Fase 7: Metadata & Settings
- [ ] **Módulo de Metadatos** - `INFOTABLE_V1`
    - [ ] Lectura de versión de base de datos (`DATAVERSION`).
- [ ] **Módulo de Configuración** - `SETTING_V1`
    - [ ] Gestión de preferencias de usuario persistidas en la DB.

## Fase 8: Validación e Integridad
- [ ] **Validación de Integridad SQLCipher**
    - [ ] Pruebas de error en descifrado y recuperación de datos corruptos.

---
**Nota de Diseño:** Cada módulo debe incluir sus propios tests unitarios en `domain` y tests de integración en `tests/` (usando `tables.sql`) antes de marcarse como completado.
