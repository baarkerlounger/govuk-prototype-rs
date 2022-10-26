# Govuk-prototype-rs

A small experiment to see how difficult it would be to start building web services using the [GovUK Design System](https://frontend.design-system.service.gov.uk/) in Rust.

What's really missing here is a crate to wrap the design system components that can be kept up to date independently of any service. Similar to https://github.com/DFE-Digital/govuk-components

- Uses [Rocket](https://rocket.rs/) as the web framework.
- Uses [Tera](https://tera.netlify.app/) for server side rendered templating.
- Uses [ESBuild](https://esbuild.github.io/) for bundling and minifying JS & CSS and copying images and fonts.

## Prerequisites

- [Rust & Cargo](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)
- npm

## Install dependencies

```bash
npm install
npm run build
cargo build
```

## Run server

```bash
cargo run
```

## Run tests

```bash
cargo test
```

## Docker

Rust + Docker still feels a little rough around the edges. This uses [cargo chef](https://github.com/LukeMathWalker/cargo-chef) to build rust dependencies separately from compiling our own source code so we can cache that layer.

The final image is built without the rust and node toolchains so we need to copy the compiled and copied front end assets and templates (which are statically linked by cargo) into the image as well as the rust binary.

The final image comes in at 96.7MB.

```bash
docker build -t govuk-prototype-rs .
docker run -p 8000:8000 --rm --name govuk-prototype-rs-1 govuk-prototype-rs
```
