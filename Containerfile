# Get started with a build env with Rust nightly
FROM docker.io/rustlang/rust:nightly-alpine as builder

RUN apk upgrade --no-cache
RUN apk add --no-cache curl npm libc-dev libressl-dev

RUN npm install -g sass

# Install cargo-leptos
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
# Install sqlx-cli
RUN cargo install sqlx-cli

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Create the database
RUN sqlx database reset -y

# Build the app
WORKDIR /app/web
RUN cargo leptos build --release -vv
WORKDIR /app

FROM docker.io/rustlang/rust:nightly-alpine as runner

# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/brainace_web /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy the database
COPY --from=builder /app/db /app/db
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

# Set any required env variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/app/brainace_web"]
