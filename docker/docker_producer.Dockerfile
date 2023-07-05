FROM rust:1.70 AS builder

COPY ./kafa_cons_prod ./kafka_cons_prod
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/docker ./target/release/docker
CMD ["/target/release/docker"]