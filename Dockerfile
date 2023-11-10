FROM rust:latest

WORKDIR .

COPY Cargo.toml config.toml ./

COPY src ./src

ARG BUILD_MODE=release

# Build the application based on the build mode
RUN if [ "$BUILD_MODE" = "debug" ]; then \
    cargo build; \
else \
    cargo build --release; \
fi

EXPOSE 8080

ENV RUST_BACKTRACE "1"

CMD if [ "$BUILD_MODE" = "debug" ]; then ./target/debug/ccip_server; else ./target/release/ccip_server; fi
