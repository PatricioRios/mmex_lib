# Seguimiento de Fase 2: Entidades Jerárquicas
**Estado:** 🔴 EN PROGRESO (Uncompleted)

## Objetivos de la Fase
Implementar entidades que poseen relaciones internas o dependencias entre sí, como la jerarquía de categorías.

## Progreso de Módulos

### 1. Módulo de Categorías - `CATEGORY_V1`
- [x] **Dominio:** Definir `CategoryId` y modelo `Category` con soporte para jerarquía.
- [x] **Infraestructura:** Implementar `SqlCategoryRepository` y `CategoryMapper`.
- [x] **Servicio:** Implementar `CategoryService` con lógica de navegación jerárquica.
- [x] **Validación:** Tests de integración que verifiquen relaciones padre-hijo.

## Registro de Actividad
- **2026-02-25:** Fase inicializada. Implementado el módulo de Categorías con soporte para relaciones Padre-Hijo (-1 como raíz). Verificado con tests de integración.
