# mmex_lib: Backlog de Implementación

Este documento detalla la hoja de ruta para la implementación completa de la lógica de Money Manager EX (basada en el esquema definido en `tables.sql`), priorizando entidades con menor acoplamiento para construir un dominio robusto desde la base.

## Fase 1: Entidades de Catálogo (Baja Relación) [COMPLETADA]
Estas entidades son atómicas y sirven de referencia para el resto del sistema.

- [x] **Módulo de Tags (Etiquetas)** - `TAGS_V1`
    - [x] Domain: `TagId`, `Tag` model.
    - [x] Infrastructure: `SqlTagRepository`, `TagMapper`.
    - [x] Service: `TagService` (CRUD básico).
- [x] **Módulo de Payees (Beneficiarios)** - `PAYEE_V1`
    - [x] Domain: `PayeeId`, `Payee` model.
    - [x] Infrastructure: `SqlPayeeRepository`, `PayeeMapper`.
    - [x] Service: `PayeeService`.
- [x] **Módulo de Currencies (Monedas)** - `CURRENCYFORMATS_V1`
    - [x] Domain: `CurrencyId`, `Currency` model (símbolos, decimales).
    - [x] Infrastructure: Repositorio para lectura de formatos.

## Fase 2: Entidades Jerárquicas
- [x] **Módulo de Categorías** - `CATEGORY_V1`
    - [x] Domain: Lógica de relación Padre-Hijo (jerarquía).
    - [x] Infrastructure: Mapeo de subcategorías.
    - [x] Service: Búsqueda recursiva y validación de niveles.

## Fase 3: Refinamiento de Cuentas
- [ ] **Evolución de Account** - `ACCOUNTLIST_V1`
    - [ ] Domain: Vincular `Account` con `Currency`.
    - [ ] Lógica de tipos de cuenta (Checking, Credit Card, Term, etc.).

## Fase 4: El Núcleo de Transacciones (Alta Relación)
Esta fase une todas las entidades anteriores.

- [ ] **Módulo de Transacciones** - `CHECKINGACCOUNT_V1`
    - [ ] Domain: `Transaction` model vinculado a `AccountId`, `PayeeId`, `CategoryId`.
    - [ ] Infrastructure: Mapeo de campos legacy (nombres de columnas complejos).
    - [ ] **Gestión de Tags en Transacciones** - `TRANSTAGS_V1`
        - [ ] Lógica de relación muchos-a-muchos entre Transacciones y Tags.

## Fase 5: Lógica de Negocio Avanzada
- [ ] **Cálculo de Balances**
    - [ ] Lógica para calcular saldos por cuenta, categoría y periodo.
- [ ] **Transferencias entre Cuentas**
    - [ ] Lógica para manejar transacciones duales (Gasto en A / Ingreso en B).
- [ ] **Transacciones Programadas** - `REPEATINGTRANSACTIONS_V1`

## Fase 6: Infraestructura de Soporte
- [ ] **Módulo de Metadatos** - `INFOTABLE_V1`
    - [ ] Lectura de versión de base de datos y configuración del usuario.
- [ ] **Validación de Integridad SQLCipher**
    - [ ] Pruebas de error en descifrado y recuperación.

---
**Nota de Diseño:** Cada módulo debe incluir sus propios tests unitarios en `domain` y tests de integración en `tests/` antes de marcarse como completado.
