# Contributing to Rusty API Lambda

¡Gracias por tu interés en contribuir! Este proyecto es parte de una comparación de rendimiento entre diferentes runtimes de AWS Lambda.

## 🚀 Cómo Contribuir

### Reportar Issues

Si encuentras un problema o tienes una sugerencia:

1. Verifica que no exista un issue similar
2. Crea un nuevo issue con una descripción clara
3. Incluye ejemplos de código si es relevante

### Enviar Pull Requests

1. **Fork** el repositorio
2. **Crea** una rama para tu feature (`git checkout -b feature/amazing-feature`)
3. **Commit** tus cambios (`git commit -m 'Add amazing feature'`)
4. **Push** a tu rama (`git push origin feature/amazing-feature`)
5. **Abre** un Pull Request

### Guías de Estilo

#### Código Rust

- Usa Rust 2021 edition features
- Sigue las convenciones de Rust (rustfmt)
- Usa clippy para linting
- Documenta funciones públicas con doc comments
- Mantén funciones pequeñas y enfocadas
- Prefiere ownership sobre cloning cuando sea posible

```rust
/// Process a single item from DynamoDB.
///
/// # Arguments
///
/// * `item` - HashMap containing the DynamoDB item
///
/// # Returns
///
/// Formatted string representation of the item
pub fn process_item(item: HashMap<String, String>) -> String {
    // Implementation here
}
```

#### Commits

- Usa mensajes descriptivos en presente
- Referencia issues cuando sea relevante
- Ejemplos:
  - ✅ "Add error handling for S3 operations"
  - ✅ "Fix #123: Update DynamoDB client initialization"
  - ❌ "Fixed stuff"
  - ❌ "WIP"

### Testing

Antes de enviar un PR:

```bash
# Formatear código
cargo fmt

# Linting
cargo clippy -- -D warnings

# Tests unitarios
cargo test

# Tests de integración
cargo test --test integration_test

# Build para Lambda
make build
```

### Arquitectura

Este proyecto sigue **Hexagonal Architecture** (Ports & Adapters):

```
src/
├── main.rs              # Lambda handler
├── lib.rs              # Library root
├── application/        # Application layer (use cases)
│   ├── mod.rs
│   └── service.rs
├── domain/             # Domain layer (business logic)
│   ├── mod.rs
│   ├── models.rs       # Domain models
│   ├── ports.rs        # Port interfaces
│   └── mocks.rs        # Mock implementations
└── infrastructure/     # Infrastructure layer (adapters)
    ├── mod.rs
    ├── dynamo.rs       # DynamoDB adapter
    └── s3.rs           # S3 adapter
```

**Principios a seguir:**
- Domain no debe depender de infrastructure
- Application orquesta domain e infrastructure
- Infrastructure implementa ports definidos en domain

### Optimizaciones de Rendimiento

Si contribuyes con optimizaciones:

1. Documenta el cambio y por qué mejora el rendimiento
2. Incluye benchmarks antes/después si es posible
3. Verifica que no rompe la funcionalidad existente
4. Considera el impacto en cold start y binary size

### Dependencias

- Evita dependencias pesadas que aumenten cold start
- Prefiere crates con pocas dependencias transitivas
- Usa features para incluir solo lo necesario
- Revisa el tamaño del binario compilado (`ls -lh target/lambda/bootstrap`)

### Areas de Mejora

Contribuciones bienvenidas en:

- ⚡ Optimizaciones de cold start
- 📊 Mejoras en logging y observability
- 🧪 Tests unitarios y de integración
- 📝 Documentación y ejemplos
- 🔒 Mejoras de seguridad
- 🏗️ Patrones de arquitectura hexagonal
- 🎯 Benchmarks y comparaciones de rendimiento

## 📋 Checklist para PRs

- [ ] El código sigue las guías de estilo (cargo fmt)
- [ ] Pasa clippy sin warnings (cargo clippy)
- [ ] La documentación está actualizada
- [ ] Los tests pasan (cargo test)
- [ ] El commit tiene un mensaje descriptivo
- [ ] No hay código comentado innecesario
- [ ] Las variables de entorno están documentadas
- [ ] `Cargo.toml` está actualizado si hay nuevas deps
- [ ] El binario compila para Lambda (make build)

## 🏗️ Build y Deploy

```bash
# Build local
cargo build

# Build para Lambda (cross-compilation)
make build

# Deploy (requiere configuración AWS)
make deploy

# Run tests
make test
```

## 🔧 Herramientas Recomendadas

- **rustup**: Para gestionar versiones de Rust
- **cargo-lambda**: Para desarrollo local de Lambdas
- **cargo-watch**: Para auto-rebuild durante desarrollo
- **rust-analyzer**: Language server para IDEs

```bash
# Instalar herramientas útiles
cargo install cargo-lambda
cargo install cargo-watch
cargo install cargo-edit
```

## ❓ Preguntas

Si tienes preguntas, abre un issue con la etiqueta `question`.

## 📜 Código de Conducta

- Sé respetuoso y constructivo
- Acepta feedback con mentalidad abierta
- Enfócate en el código, no en las personas
- Valora la claridad y simplicidad sobre la "inteligencia"

## 🦀 Recursos de Rust

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [AWS Lambda Rust Runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
- [Hexagonal Architecture en Rust](https://alexis-lozano.com/hexagonal-architecture-in-rust/)

¡Gracias por hacer este proyecto mejor! 🦀🚀
