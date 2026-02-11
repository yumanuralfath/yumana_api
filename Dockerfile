# Menggunakan image Rust sebagai base image untuk build
FROM rust:slim

# Install diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Set direktori kerja di dalam container
WORKDIR /usr/src/app

# Menyalin Cargo.toml dan Cargo.lock untuk caching dependensi
COPY Cargo.toml Cargo.lock ./

# Menyalin semua kode sumber dan file migrasi
COPY . .

# Build aplikasi dalam mode release
RUN cargo build --release

# Menggunakan image minimal dengan GLIBC yang lebih baru
FROM debian:bookworm-slim

# Menginstal library runtime yang diperlukan dan diesel CLI
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Set direktori kerja di dalam container
WORKDIR /app

# Menyalin binary aplikasi dan file konfigurasi
COPY --from=builder /usr/src/app/target/release/yumana_api /usr/local/bin/yumana_api
COPY --from=builder /usr/src/app/migrations ./migrations
COPY diesel.toml ./

# Script untuk menjalankan migrasi dan aplikasi
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

EXPOSE 8000

# Menggunakan entrypoint script
ENTRYPOINT ["docker-entrypoint.sh"]
