FROM rust:1.94.1-slim-bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release --locked \
    && strip target/release/app

FROM debian:bookworm-slim AS runtime

RUN groupadd --gid 1001 app \
    && useradd --uid 1001 --gid app --shell /usr/sbin/nologin --no-create-home app

COPY --from=builder --chown=app:app /app/target/release/app /app/app

USER app
WORKDIR /app
EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
    CMD curl -f http://localhost:8080/healthz || exit 1

CMD ["/app/app"]
