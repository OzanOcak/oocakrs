# oocakrs

Rust + Axum backend (incremental port of the Node/Express backend to Rust).

## Stack

- **Axum** 0.8 + **Tokio** — HTTP server
- **SQLx** 0.8 (Postgres, runtime queries) — DB access
- **tower-http** — CORS + security headers
- **dotenvy** — env loading
- **tracing** — logging

## Layout
