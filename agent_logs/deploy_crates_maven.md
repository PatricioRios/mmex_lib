# Plan de Configuración de Deploy: Crates.io y Maven Central

## 1. Actualización de `.gitignore`

Reemplazar contenido por exclusiones estándar de Rust, Python, Kotlin/Java, Gradle, macOS, Windows e IDEs. `Cargo.lock` se mantiene.

## 2. Actualización de `Cargo.toml`

Añadir metadatos para crates.io: license (MIT), repository, homepage, documentation, readme, keywords, categories, exclude.

## 3. Configuración Gradle en `kotlin/`

Crear `kotlin/settings.gradle.kts` y `kotlin/build.gradle.kts` con:
- Plugin kotlin("jvm"), maven-publish, signing
- Grupo `com.github.patriciorios`
- Publicación a Sonatype OSSRH (Maven Central)
- Firma PGP con credenciales desde env vars
- Source sets apuntando a bindings generados por UniFFI

## 4. Actualización de `.github/workflows/publish.yml`

Añadir jobs:
- `publish-crates`: cargo publish con CARGO_REGISTRY_TOKEN
- `publish-maven`: build Rust → generar bindings Kotlin → ./gradlew publish

## Secretos necesarios en GitHub

| Secreto | Descripción |
|---------|-------------|
| `CARGO_REGISTRY_TOKEN` | Token de crates.io |
| `OSSRH_USERNAME` | Usuario de Sonatype OSSRH |
| `OSSRH_PASSWORD` | Password de Sonatype OSSRH |
| `SIGNING_KEY_ID` | ID de la clave PGP |
| `SIGNING_KEY` | Clave privada PGP (ASCII armored) |
| `SIGNING_PASSWORD` | Password de la clave PGP |
