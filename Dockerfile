FROM node:latest AS build-npm
WORKDIR /build
COPY . .
RUN npm i -g vue-tsc vite \
&& npm i \
&& npm run build

FROM rust:latest AS build-rs
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:12-slim
WORKDIR /mercury
COPY --from=build-npm /build/dist ./dist
COPY --from=build-rs /build/target/release/mercury_land ./

EXPOSE 8080
ENTRYPOINT [ "./mercury/mercury_land" ]