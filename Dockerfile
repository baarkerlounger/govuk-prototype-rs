FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
RUN curl -sL https://deb.nodesource.com/setup_lts.x | bash -
RUN apt install -y nodejs
COPY package.json package-lock.json esbuild.js ./
COPY assets ./assets/
RUN npm install --frozen-lockfile
RUN npm run build
# Build application
COPY . .
RUN cargo build --release --bin govuk-prototype-rs

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/govuk-prototype-rs /usr/local/bin
RUN mkdir assets && mkdir templates
COPY --from=builder app/assets/ assets/
COPY --from=builder app/templates/ templates/
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/govuk-prototype-rs"]
