# Govuk Prototype Rust

[![Tests](https://github.com/baarkerlounger/govuk-prototype-rs/actions/workflows/test.yml/badge.svg)](https://github.com/baarkerlounger/govuk-prototype-rs/actions/workflows/test.yml)

A rusty diesel powered rocket 🚀

A small experiment to see how difficult it would be to start building web services using the [GovUK Design System](https://frontend.design-system.service.gov.uk/) in Rust.

- Uses [Rocket](https://rocket.rs/) as the web framework.
- Uses [Tera](https://tera.netlify.app/) for server side rendered templating.
- Uses [Diesel](https://diesel.rs/) as the ORM.
- Uses [ESBuild](https://esbuild.github.io/) for bundling and minifying JS & CSS and copying images and fonts.

## Infrastructure

Deployed on Render's free tier at https://govuk-prototype-rs.onrender.com/ as a Docker deployment.

Changes that require `npm run build` to be re-run require clearing the build cache before deploying.

## Prerequisites

- [Rust & Cargo](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)
- npm
- Postgres

## Install dependencies

```bash
npm install
npm run build
cargo build
cargo install diesel_cli --no-default-features --features postgres
cargo install cargo-make
```

## Setup database

```bash
cp .env.example .env
cargo make setup_databases
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

```bash
docker build -t govuk-prototype-rs .
```

Running tests or dev server using docker-compose:

```bash
docker-compose build
docker compose run --rm app cargo test
docker-compose run --rm app cargo run
```

## Not yet implemented

- A way to safely run integration tests in transactions (the current approach of dropping the database and recreating when needed is slow, and brittle - it silently fails if the database has an open connection to another program)
- A way to seed data (for test or other databases)
- Any form of authentication
- Request verification/Authenticity tokens


## Thoughts

- Github actions don't yet support caching for cargo build outputs
- Docker generally struggles with cargo. By default it tries to re-compile all dependencies on every code change, which is pretty slow (see above)
- Currently all Gov UK components and patterns need to be used in raw html form. JS and CSS stays up to date with upstream via the npm package but if the markup for those components or patterns change that wouldn't. Ideally there would be a crate wrapping standard components similar to https://github.com/DFE-Digital/govuk-components that could be kept up to date with upstream independently of any individual project. Alternatively having support for running Nunjucks templates in Rust would make it easier to reuse components from other stacks.
- A crate to wrap Gov Notify would be useful
