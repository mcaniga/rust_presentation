# builder - builds our rust project into binary
FROM clux/muslrust:1.58.0-stable as builder
WORKDIR /volume
COPY . .
RUN cargo build --release

# executor - executes the compiled libary from "builder"
FROM alpine
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/instea_presentation .
WORKDIR /
ENTRYPOINT [ "/instea_presentation" ]