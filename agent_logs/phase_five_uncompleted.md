# Seguimiento de Fase 5: CRUD Extendido (Hacia Versión Utilizable)
**Estado:** 🔴 EN PROGRESO (Uncompleted)

## Objetivos de la Fase
Implementar las entidades restantes del esquema `tables.sql` para garantizar la gestión total de los datos de Money Manager EX.

## Progreso de Módulos

### 1. Split Transactions (Desgloses) - `SPLITTRANSACTIONS_V1`
- [ ] **Dominio:** Definir `SplitId` y modelo `SplitTransaction`.
- [ ] **Infraestructura:** Implementar `SqlSplitRepository` y mapeo.
- [ ] **Servicio:** Integrar gestión de splits en `TransactionService`.
- [ ] **Validación:** Tests de integración con `tables.sql`.

### 2. Scheduled Bills (Transacciones Programadas) - `BILLSDEPOSITS_V1`
- [ ] **Dominio:** Modelo `ScheduledTransaction`.
- [ ] **Infraestructura:** Repositorio y Mapeo.
- [ ] **Servicio:** `ScheduledService`.

### 3. Assets (Activos) - `ASSETS_V1`
- [ ] **Dominio:** Modelo `Asset`.
- [ ] **Infraestructura:** Repositorio y Mapeo.
- [ ] **Servicio:** `AssetService`.

### 4. Stocks (Inversiones) - `STOCK_V1`
- [ ] **Dominio:** Modelo `Stock`.
- [ ] **Infraestructura:** Repositorio y Mapeo.
- [ ] **Servicio:** `StockService`.

## Registro de Actividad
- **2026-02-25:** Fase inicializada. Comenzando con Split Transactions.
