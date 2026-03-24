# Mapeo de Base de Datos Legacy a Rust 🗺️

Este documento técnico explica la estrategia utilizada en `mmex_lib` para interactuar con las bases de datos de Money Manager EX (MMEX) de forma segura y eficiente.

## 1. El Esquema Legacy (`tables.sql`)

La base de datos de MMEX es SQLite con un esquema histórico que ha evolucionado durante más de una década. La referencia absoluta para las tablas es el archivo `tables.sql` ubicado en la raíz del proyecto.

### Desafíos Principales:
- **Nombres de Tablas y Columnas**: Uso de sufijos `_V1` y nombres en mayúsculas (ej: `ACCOUNTLIST_V1`, `CHECKINGACCOUNT_V1`).
- **IDs de Tipo `i64`**: Soporte para bases de datos de gran volumen.
- **Sin Esquema Estricto**: Muchas columnas son opcionales o requieren interpretación según el contexto.

---

## 2. Estrategia de Mapeo: `sea-query` 🚀

Para evitar concatenar cadenas SQL (riesgo de inyección y errores tipográficos) y no depender de un ORM pesado, utilizamos **`sea-query`**.

### Flujo de Consulta:
1.  **Definición de Tablas**: Se definen `Iden` (Identificadores) en Rust que coinciden con los nombres de las tablas legacy.
2.  **Construcción Dinámica**: Se construyen las consultas SQL programáticamente.
3.  **Ejecución con `rusqlite`**: Se ejecutan las consultas a través de la conexión SQLite.
4.  **Mapeo a Dominio**: Los resultados se transforman en estructuras de dominio limpias de Rust.

### Ejemplo de Consulta:
```rust
// Uso de Alias para representar la tabla legacy
let table = Alias::new("ACCOUNTLIST_V1");

// Construcción de la consulta con sea-query
let (sql, values) = Query::select()
    .columns([
        "ACCOUNTID",
        "ACCOUNTNAME",
        "STATUS",
        "INITIALBAL",
    ])
    .from(table)
    .and_where(Expr::col(Alias::new("STATUS")).eq("Open"))
    .build_rusqlite(SqliteQueryBuilder);
```

---

## 3. Correspondencia de Tablas y Modelos

| Tabla Legacy (`tables.sql`) | Concepto de Dominio (Rust) | Módulo de Infraestructura |
| :--- | :--- | :--- |
| `ACCOUNTLIST_V1` | `Account` | `repositories.rs` |
| `CHECKINGACCOUNT_V1` | `Transaction` | `transactions_repository.rs` |
| `CATEGORY_V1` | `Category` | `categories_repository.rs` |
| `CURRENCYFORMATS_V1` | `Currency` | `currencies_repository.rs` |
| `SPLITTRANSACTIONS_V1` | `SplitTransaction` | `splits_repository.rs` |
| `BILLSDEPOSITS_V1` | `ScheduledTransaction` | `scheduled_repository.rs` |
| `ASSETS_V1` | `Asset` | `assets_repository.rs` |
| `STOCK_V1` | `Stock` | `stocks_repository.rs` |

---

## 4. Tipos de Datos Especiales

### IDs (Newtypes)
Para evitar confundir IDs de diferentes entidades, usamos el patrón **Newtype**. Internamente, estos tipos contienen el valor `v1` que corresponde al `i64` de la base de datos:
- `AccountId(i64)` -> Acceso vía `id.v1`
- `TransactionId(i64)` -> Acceso vía `id.v1`

### Monedas (`Money`)
Nunca usamos `f64` para montos de dinero para evitar errores de precisión. Usamos `rust_decimal::Decimal` envuelto en un tipo `Money`. El valor real se almacena en el campo `.v1`.

### Fechas
Usamos `chrono::NaiveDate` envuelto en `MmexDate`. Para persistencia, se formatea como cadena ISO `%Y-%m-%d` que es lo que MMEX espera.

---

## 5. Implementación en `Infrastructure`

Toda la lógica de mapeo reside en la capa de infraestructura (`src/infrastructure/`). Los repositorios (ej: `SqlAccountRepository`, `SqlTransactionRepository`) implementan los traits definidos en el dominio. Estos repositorios son los únicos que conocen los nombres de las columnas y tablas legacy, utilizando `DbExecutor` para realizar las operaciones de forma segura.
