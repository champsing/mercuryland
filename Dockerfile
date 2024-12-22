FROM rust:latest AS build-rs
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:12-slim
WORKDIR /mercury
COPY --from=build-rs /build/target/release/mercury_land ./

EXPOSE 8080
ENTRYPOINT [ "./mercury_land" ]