# Estado del Proyecto y Roadmap: mmex_lib (Fase Beta) 🚀

Este documento detalla el estado actual de los módulos de la librería, los criterios para alcanzar la versión 1.0 estable y las funcionalidades planificadas a corto y largo plazo.

## 1. Definición de "Beta" para mmex_lib

Actualmente en la versión `0.1.0-beta.x`, lo que implica:
- **API en Evolución**: Los nombres de funciones y tipos pueden cambiar para mejorar la ergonomía.
- **Cobertura Incompleta**: Algunos módulos secundarios de MMEX (ej. Stocks, Inversiones) están en fase inicial de desarrollo.
- **Optimización en Curso**: El rendimiento en bases de datos masivas (10,000+ transacciones) está siendo evaluado y mejorado.

---

## 2. Matriz de Estabilidad de Módulos

| Módulo | Estabilidad | Características Implementadas | Próximos Pasos |
| :--- | :--- | :--- | :--- |
| **Cuentas (Accounts)** | ⚠️ Beta | CRUD completo, cálculo de balances. | Implementar Unit Tests y balances históricos. |
| **Transacciones (Transactions)** | ⚠️ Beta | Ingresos, Gastos, Transferencias simples. | Implementar Unit Tests y Splits avanzados. |
| **Categorías (Categories)** | ⚠️ Beta | Jerarquías, CRUD, asociaciones. | Implementar Unit Tests y validación de ciclos. |
| **Beneficiarios (Payees)** | ⚠️ Beta | CRUD, alias, vinculación. | Implementar Unit Tests y mapeo inteligente. |
| **Monedas (Currencies)** | ⚠️ Beta | Tipos base, tasas manuales. | Implementar Unit Tests y descarga automática. |
| **Transacciones Programadas** | ⚠️ Beta | Listado, creación básica. | Motor de generación automática. |
| **Etiquetas (Tags)** | ⚠️ Beta | CRUD, asociación a splits. | Filtrado avanzado por múltiples tags. |
| **Activos (Assets)** | 🧪 Alpha | Modelado básico, listado. | Cálculo automático de apreciación/depreciación. |
| **Acciones (Stocks)** | 🧪 Alpha | Modelado básico, listado. | Descarga de cotizaciones históricas. |

---

## 3. Criterios para la v1.0 (Estable)

Para que `mmex_lib` sea considerada estable, debe cumplir con los siguientes hitos:
1.  **Paridad de Negocio**: Soporte completo para todos los tipos de transacciones de MMEX Desktop.
2.  **Estabilidad de la API FFI**: La interfaz UniFFI debe estar congelada y documentada.
3.  **Cobertura de Tests**: Mínimo de 80% de cobertura en la capa de servicios y 100% en los mapeos de base de datos.
4.  **Multiplataforma Verificada**: Tests automatizados corriendo exitosamente en Linux, Android (Kotlin) e iOS (Swift).
5.  **Soporte SQLCipher Completo**: Compatibilidad garantizada con archivos `.mmb` (sin cifrado) y `.emb` (cifrados AES).

---

## 4. Roadmap a Corto Plazo (Q2 2026)

- [ ] **Finalización de Transacciones Programadas**: Implementar el motor que genera automáticamente transacciones en la base de datos según su recurrencia.
- [ ] **Networking Core**: Añadir capacidad (opcional) en Rust para descargar tasas de cambio desde APIs públicas.
- [ ] **Soporte de Adjuntos (Attachments)**: Lógica para gestionar la carpeta de archivos adjuntos asociada a la base de datos.
- [ ] **Optimización de Consultas**: Implementar caché de lectura para mejorar la velocidad de respuesta en listados largos.

---

## 5. Roadmap a Largo Plazo (Futuro)

- **Soporte de Reportes**: Motor de reportes nativo en Rust (Net Worth, Income vs Expense) que genere datos listos para gráficos (JSON).
- **Importadores/Exportadores**: Portar los parsers de QIF y CSV de C++ a Rust para una mayor velocidad y seguridad.
- **Tauri Integration**: Ejemplo de aplicación de escritorio híbrida usando `mmex_lib` como backend nativo.
