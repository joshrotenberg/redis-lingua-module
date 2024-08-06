FROM redis:latest as redis
FROM rust:latest as builder

ADD . /build
WORKDIR /build
RUN apt update -qq && apt install -yqq python3 python3-pip clang
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt update -qq \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
RUN rm -rf /var/cache/apt/*
COPY --from=redis /usr/local/bin/redis-* /usr/bin/
COPY --from=builder /build/target/release/libredis_lingua.so /usr/lib/libredis_lingua.so
EXPOSE 6379
CMD ["redis-server", "--loadmodule", "/usr/lib/libredis_lingua.so"]