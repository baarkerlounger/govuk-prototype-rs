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
