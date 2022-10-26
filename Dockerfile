FROM rust:1.64-bullseye

ENV ROCKET_ADDRESS=0.0.0.0

WORKDIR /app
RUN curl -sL https://deb.nodesource.com/setup_lts.x | bash -
RUN apt install -y nodejs
COPY package.json package-lock.json esbuild.js ./
COPY assets ./assets/
RUN npm install --frozen-lockfile
RUN npm run build
COPY . .
RUN cargo build --release
EXPOSE 8000
CMD ["cargo", "run", "--release"]
