FROM rust:1.68.2

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN cargo build --release
ENTRYPOINT [ "./target/release/mailcolobus" ]