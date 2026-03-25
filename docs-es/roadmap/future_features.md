# Funcionalidades Futuras Planificadas 🔮

Este documento detalla la visión a largo plazo para `mmex_lib` y las funcionalidades de Money Manager EX Desktop que planeamos integrar en el núcleo de Rust.

## 📈 1. Reportes y Análisis
- **Motor de Reportes Nativo**: Generación de agregados SQL para balances netos (Net Worth), ingresos vs. gastos por periodo y cumplimiento de presupuestos.
- **Formato JSON para UI**: Estructura de datos estandarizada lista para ser consumida por bibliotecas de gráficos (Chart.js, ApexCharts) en Android/iOS/Web.
- **Proyecciones de Flujo de Caja (Cash Flow)**: Motor de cálculo que utilice transacciones programadas para predecir el estado de las cuentas a futuro.

## 📑 2. Gestión Documental y Adjuntos
- **Manejo de Attachments**: Lógica para gestionar la carpeta de archivos adjuntos (`%DATABASE%_attachments`). Copia automática, eliminación sincronizada y mapeo en la base de datos.
- **Sincronización de Archivos**: Notificadores para avisar a la aplicación móvil cuando un adjunto ha sido modificado fuera de la librería.

## 🏗️ 3. Integración Avanzada (FFI)
- **Soporte Swift/iOS**: Generación automática de bindings `.swift` y empaquetado como XCFramework.
- **Soporte WebAssembly (Wasm)**: Compilación del core para navegadores, permitiendo visores de archivos `.mmb` puramente en el cliente.
- **Tauri Integration**: Ejemplo de aplicación de escritorio moderna usando `mmex_lib` con una interfaz de usuario basada en tecnologías web (React/Vue/Svelte).

## 🧩 4. Herramientas de Datos
- **Importadores Portables**: Reimplementación de los parsers de QIF y CSV en Rust (usando `nom` o `pest`) para una importación más rápida y segura que la actual en C++.
- **Exportadores**: Capacidad de exportar a CSV, QIF y XML directamente desde el motor central.
- **Herramientas de Reparación**: Funciones para verificar la integridad referencial de la base de datos y reparar inconsistencias comunes del esquema legacy.

---

Este roadmap está sujeto a cambios basados en las necesidades de la comunidad y de los proyectos que integren `mmex_lib`. Si tienes una sugerencia o quieres liderar una de estas funcionalidades, consulta la [Guía de Contribución](../CONTRIBUTING.md).
