# Seguimiento de Fase 5: CRUD Extendido (Hacia Versión Utilizable)
**Estado:** ✅ COMPLETADA (Completed)

## Objetivos de la Fase
Implementar las entidades restantes del esquema `tables.sql` para garantizar la gestión total de los datos de Money Manager EX.

## Progreso de Módulos

### 1. Split Transactions (Desgloses) - `SPLITTRANSACTIONS_V1`
- [x] **Dominio:** Definir `SplitId` y modelo `SplitTransaction`.
- [x] **Infraestructura:** Implementar `SqlSplitRepository` y mapeo.
- [x] **Servicio:** Integrar gestión de splits en `TransactionService`.
- [x] **Validación:** Tests de integración con `tables.sql`.

### 2. Scheduled Bills (Transacciones Programadas) - `BILLSDEPOSITS_V1`
- [x] **Dominio:** Modelo `ScheduledTransaction`.
- [x] **Infraestructura:** Repositorio y Mapeo.
- [x] **Servicio:** `ScheduledService`.

### 3. Assets (Activos) - `ASSETS_V1`
- [x] **Dominio:** Modelo `Asset`.
- [x] **Infraestructura:** Repositorio y Mapeo.
- [x] **Servicio:** `AssetService`.

### 4. Stocks (Inversiones) - `STOCK_V1`
- [x] **Dominio:** Modelo `Stock`.
- [x] **Infraestructura:** Repositorio y Mapeo.
- [x] **Servicio:** `StockService`.

## Registro de Actividad
- **2026-02-25:** Fase inicializada. Implementado el módulo de Split Transactions con soporte para desgloses de categorías e importes.
- **2026-02-25:** Implementado el módulo de Scheduled Transactions basándonos en BILLSDEPOSITS_V1.
- **2026-02-25:** Implementado el módulo de Assets (Activos) basado en ASSETS_V1.
- **2026-02-25:** Finalizada la Fase 5 con la implementación del módulo de Stocks (Inversiones). Verificado con tests de integración.
