# rqtll-api

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/RQTLL/rqtll-components/blob/main/assets/branding/logo-main-light.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/RQTLL/rqtll-components/blob/main/assets/branding/logo-main-dark.svg">
  <img alt="RQTLL Logo" src="https://github.com/RQTLL/rqtll-components/blob/main/assets/branding/logo-main-color.svg" width="50px">
</picture>

API gRPC/protobuf que define los contratos entre la IDE (`rqtll-ide` / frontends PySide6) y los backends (`rqtll-rcl-utils`, agentes, daemons) del ecosistema RQTLL.

## Table of Contents
- [Quickstart](#quickstart)
- [Estructura del Repositorio](#estructura-del-repositorio)
- [Cómo contribuir](#cómo-contribuir)
- [Security](#security)
- [License](#license)
- [Maintainers](#maintainers)

## Quickstart

Este crate genera los stubs Rust (clientes y servidores) a partir de los `.proto` en `proto/` usando `tonic-build` en `build.rs`.

### Requirements

- Rust (stable, 1.70+ recommended)
- `protoc` (Protocol Buffers compiler)

### Consumir rqtll-api

Este repositorio puede ser consumido por otros módulos de dos maneras seguras: 

### Dependencia Git en `Cargo.toml`

```toml
rqtll_api = { git = "https://github.com/RQTLL/rqtll-api.git", rev = "<commit-sha>" }
```

### Git Submodule

```bash
# Añadir como submodule
git submodule add https://github.com/RQTLL/rqtll-api.git external/rqtll-api
git submodule update --init --recursive
```

#### Modificación de `Cargo.toml`

```toml
rqtll_api = { path = "external/rqtll-api" }
```

### Build / Generar stubs

```bash
# Desde la raíz del crate
cd rqtll-api
cargo build
```

El comando anterior invocará `build.rs` que compila todos los `.proto` usando `tonic_build` y `prost`.

### Uso desde Rust

`rqtll_api::` contiene los módulos generados; los servicios se encuentran bajo `rqtll_api::rqtll.api.v1` (incluidos por `tonic::include_proto!("rqtll.api.v1")`).

## Estructura del Repositorio

```
./
├── proto/           # Definición de contratos protobuf (services, messages)
├── src/             # Código de arranque/examples que usan los stubs generados
├── build.rs         # Generación de stubs con tonic_build
├── Cargo.toml
└── README.md
```

## Cómo contribuir

Lee `CONTRIBUTING.md` para normas sobre modificación de `.proto`, versionado y pruebas.

En resumen:

- Modifica los `.proto` en la carpeta `proto/` y añade `reserved` cuando retires campos.
- Mantén compatibilidad hacia atrás: no reusar números de campo.
- Ejecuta `cargo build` para regenerar stubs y validar la compilación.
- Abre PR describiendo el cambio y el impacto en clientes.

## Security

Consulta `SECURITY.md` para el proceso de reporte de vulnerabilidades.

## License

Este proyecto está bajo la licencia **MIT**. Consulta el archivo [LICENSE](LICENSE) para más detalles.

## Maintainers
* **adnKSharp** <adnksharp@gmail.com>
