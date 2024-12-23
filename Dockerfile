FROM rust:alpine AS build-rs
WORKDIR /build
RUN apk add --no-cache musl-dev sqlite-static openssl-dev openssl-libs-static pkgconf libpq-dev 
COPY . .
RUN cargo build --release

FROM scratch
WORKDIR /mercury
COPY --from=build-rs /build/target/release/mercury_land ./

EXPOSE 8080
ENTRYPOINT [ "./mercury_land" ]