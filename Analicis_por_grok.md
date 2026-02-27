**Análisis Exhaustivo de Money Manager Ex (MMEX) como Arquitecto de Software Senior (Rust Multiplataforma)**

He realizado un análisis completo del repositorio principal (`moneymanagerex/moneymanagerex`), sub-módulos (`database`, `general-reports`, `currency`), documentación oficial (`moneymanagerex.org/docs/features/*` y manual de usuario), estructura de código fuente, esquema de base de datos y issues relacionados. MMEX es una aplicación monolítica en **C++17 + wxWidgets** (desktop) con una app Android separada (Java/Kotlin) que comparte el mismo archivo `.mmb/.emb`. No existe app iOS oficial pública, pero la arquitectura permite extensión.

El objetivo se alinea perfectamente: **separar Core Business Logic + Persistence + Parsing + Networking** en un **Rust Core Engine** (crate `mmex-core`) expuesto vía **UniFFI** (para Android/Kotlin y iOS/Swift) y **FFI/cbindgen** (para desktop C++ o migración híbrida). Esto permite un único motor de negocio compartido, preservando compatibilidad 100 % con bases de datos existentes `.mmb` (no encriptadas) y `.emb` (encriptadas AES).

### 1. Inventario de Funcionalidades (Completo y Categorizado)

**Cuentas y Monedas**
- Tipos: Checking/Bank, Term (ocultable), Investment (stocks/shares), Asset (fijos).
- Multi-moneda: Base currency + ilimitadas por cuenta; conversión automática.
- Gestión de currencies personalizadas + histórico de tasas.

**Transacciones**
- Income, Expense, Transfer, Split (múltiples categorías/tags).
- Estados: Unreconciled / Reconciled / Void / Duplicate / Follow-up.
- Attachments (archivos copiados a carpeta `%DATABASE%_attachments`).
- Búsqueda/filtro/sort avanzado, reconciliación bancaria.

**Payees, Categories & Tags**
- Categories jerárquicas ilimitadas (nested).
- Payees con alias.
- Tags múltiples por split transaction.

**Transacciones Recurrentes / Scheduled**
- Reminders automáticos/manuales.
- Generación futura de transacciones (diaria/semanal/mensual/anual).

**Activos (Assets)**
- Tracking separado de cuentas.
- Ajustes automáticos (appreciation/depreciation rate anual).
- Revaluación manual.
- Inclusión en Net Worth.

**Presupuestos y Forecasting**
- Budget mensual/anual (calendar o fiscal year).
- Comparativa real vs. presupuestado.
- Cash flow forecasting.

**Reportes**
- One-click (gráficos via ApexCharts/Chart.js).
- Custom reports (SQL + Lua + HTML/JS via sub-módulo `general-reports`).
- Net Worth, Income/Expense, Budget vs Actual, etc.

**Import/Export**
- Import: QIF, CSV (mapeo flexible de campos, orden libre).
- Export: QIF, CSV, HTML, XML (y JSON vía custom reports o extensiones).
- XML para backups/portabilidad.

**Persistencia y Seguridad**
- SQLite (`.mmb` sin cifrado / `.emb` AES-128/256 via wxSQLite3).
- Attachments en carpeta sincronizable.

**Networking y Sync**
- **BYOC (Bring Your Own Cloud)**: Usuario coloca `.mmb` + carpeta attachments en Dropbox/Google Drive/OneDrive/Syncthing/Nextcloud/Synology. Sin API propietaria.
- Tasas de cambio: Descarga automática desde ExchangeRate-API o JSON estáticos (`moneymanagerex.github.io/currency/data/latest_{BASE}.json`) vía libcurl.

**Otras**
- Portable (USB), multi-idioma (Crowdin), temas, Home Page summary, Stock quotes históricos.

### 2. Análisis de Abstracción para Rust – Clasificación

**Core Logic (100 % a Rust – reglas de negocio puras)**
- Validación de transacciones (balance, splits suman 100 %, fechas lógicas, duplicados).
- Motores de presupuesto y cash-flow (cálculo actual vs. planned, forecasting).
- Conversión multi-moneda y Net Worth (usando tasas históricas).
- Generación de scheduled transactions (lógica de recurrencia).
- Cálculo de asset values (appreciation/depreciation automática).
- Lógica de reports básicos (agregaciones SQL + post-procesado en Rust).
- Reglas de reconciliación y status.

**Data Persistence (a Rust – centralizado)**
- Gestión completa de SQLite (rusqlite + sqlcipher o SQLite Multiple Ciphers para compatibilidad exacta).
- Esquema completo (tablas: ACCOUNTS, TRANSACTIONS, CATEGORIES, PAYEE, CURRENCYHISTORY, BUDGETSPLIT, ATTACHMENT, CUSTOMFIELD, etc. – ver `database/tables.sql`).
- Migraciones: Portar incremental_upgrade/*.sql a Rust (aplicar en `open_db()` si versión < current).
- Cifrado AES: wxSQLite3 usa AES-128/256 CBC (compatible con SQLCipher v3/v4 en modo compatibility). En Rust usar `rusqlite` compilado contra SQLite Multiple Ciphers o sqlcipher con `PRAGMA cipher_compatibility = 3/4;`.

**Data Parsing (a Rust – parsers puros)**
- QIF parser (lexer + semantic rules).
- CSV flexible importer (user-defined field mapping → struct Transaction).
- Exportadores a QIF/CSV/HTML/XML/JSON.
- JSON para custom reports o API interna.

**Networking (a Rust – opcional, pero recomendado)**
- Cliente HTTP (reqwest) para descargar tasas de cambio (ExchangeRate-API o JSON estáticos).
- **NO** incluir Dropbox/Google Drive SDK (queda en capa plataforma como "BYOC manual"). Solo exponer `sync_status()` o watcher de archivo.

**Fuera de Rust (queda en wrappers por plataforma)**
- UI completa (wxWidgets en desktop, Jetpack Compose/ SwiftUI en móvil).
- Notificaciones locales (OS scheduler).
- Biometría / Keychain / Secure Enclave (para desbloqueo de password DB).
- File picker, attachments viewer, gráficos nativos (si no se usan reports HTML).
- Integraciones específicas: Android ContentProvider para compartir DB, iOS App Group para BYOC.

### 3. Definición de la Capa de Interoperabilidad (API Interna del Core Engine)

**Tecnología recomendada**
- **UniFFI** (principal): genera bindings Kotlin + Swift + Python automáticamente desde `uniffi.toml` + UDL. Ideal para Android/iOS.
- **cbindgen** + C API: para desktop C++ (mantener wxWidgets y llamar `mmex_core.dll/so/dylib`).
- Alternativa moderna: Tauri 2.0 para nuevo desktop (Rust backend + web frontend), pero híbrido posible.

**Diseño de API (ejemplos en Rust/UDL)**
```rust
// mmex-core/src/lib.rs
#[uniffi::export]
pub struct Transaction { /* campos POD + Option<Decimal> */ }

#[uniffi::export]
pub fn open_db(path: String, password: Option<String>) -> Result<DbHandle, Error>;

#[uniffi::export]
pub fn add_transaction(handle: &DbHandle, tx: Transaction) -> Result<i64, Error>;

#[uniffi::export]
pub fn compute_net_worth(handle: &DbHandle, currency: String) -> Result<Decimal, Error>;

#[uniffi::export]
pub fn get_budget_report(handle: &DbHandle, year: i32) -> Result<BudgetReport, Error>;

#[uniffi::export]
pub fn import_csv(handle: &DbHandle, csv_path: String, mapping: Vec<FieldMap>) -> Result<ImportStats, Error>;
```

**Patrón recomendado**
- `DbHandle` = opaque pointer (Arc<Mutex<Connection>>).
- Todos los tipos son serializables (serde + uniffi).
- Errores tipados (enum con variantes específicas).
- Callbacks para progress (import largo, report generation).
- Thread-safe por defecto (tokio o rayon para operaciones pesadas).

**Wrappers**
- Android: Kotlin wrapper + Jetpack DataStore para config.
- iOS: Swift wrapper + Combine/Swift Concurrency.
- Desktop: C++ thin wrapper que llama FFI y alimenta wxWidgets models.

### 4. Evaluación de Viabilidad y Cuellos de Botella Técnicos

**Viabilidad General: Alta (9/10)**
- Esquema DB 100 % portable (ya usado por Android).
- Rust + rusqlite/sqlcipher es más seguro y rápido que C++ wxSQLite3.
- UniFFI reduce boilerplate a cero para móvil.
- Migración incremental: empezar con read-only core, luego write, luego full rewrite desktop.

**Cuellos de Botella Críticos y Mitigaciones**

1. **Compatibilidad Encriptación AES (mayor riesgo)**
    - wxSQLite3 no es idéntico a SQLCipher puro (AES-128 CBC sin HMAC por defecto, o AES-256).
    - **Mitigación**: Usar `SQLite3MultipleCiphers` (soporta explícitamente wxSQLite3 cipher) y compilar rusqlite contra él. Probar con 100+ .emb existentes. Soporte multi-cipher futuro (ya en roadmap MMEX).

2. **Migraciones de Esquema**
    - Incremental_upgrade contiene ~20 scripts.
    - **Mitigación**: Implementar `upgrade_db(version: i32)` en Rust que ejecute exactamente los mismos SQL (copiados del repo database). Version check en `PRAGMA user_version`.

3. **Lógica de Negocio Compleja (preservar comportamiento exacto)**
    - Cálculos de presupuesto, splits, currency history, asset depreciation.
    - **Mitigación**: Unit tests exhaustivos + property-based testing (proptest). Golden tests contra DBs reales de MMEX.

4. **Parsers QIF/CSV (precisión bit-a-bit)**
    - **Mitigación**: Reimplementar en nom o pest + tests con archivos de muestra del repo/issues.

5. **Performance y Tamaño Binario**
    - Rust excelente, pero UniFFI + sqlcipher añade ~2-4 MB.
    - **Mitigación**: Compilación release + stripping; usar rusqlite sin features innecesarias.

6. **Custom Reports (Lua + JS)**
    - **Mitigación**: Exponer raw SQL execution segura (sandbox) o mover Lua a Rust via rlua (opcional fase 2).

**Roadmap Propuesto (Fases)**
1. Core read-only + DB + currency (2-3 meses).
2. Full CRUD + parsers + budget engine (3 meses).
3. UniFFI bindings + Android wrapper (1 mes).
4. Desktop FFI híbrido + iOS (2 meses).
5. Migración completa desktop (opcional, paralelo).

Esta arquitectura convierte MMEX en un producto verdaderamente multiplataforma moderno, con un único source of truth en Rust seguro, rápido y mantenible. El Core Engine será ~15-20 kLOC, extremadamente testable y listo para extensiones futuras (e.g., WebAssembly para reports online).

Si necesitas el código skeleton del crate, UDL completo, o plan detallado de migración de un módulo específico (ej. transaction engine), avísame y lo genero inmediatamente. ¡Listo para implementar!