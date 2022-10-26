FROM rust:1.64-bullseye

ENV ROCKET_ADDRESS=0.0.0.0

WORKDIR /app
COPY . .
RUN curl -sL https://deb.nodesource.com/setup_lts.x | bash -
RUN apt install -y nodejs
RUN npm install
RUN npm run build
RUN cargo build --release
EXPOSE 8000
CMD ["cargo", "run", "--release"]
