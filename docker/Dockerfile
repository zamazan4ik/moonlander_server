FROM docker.io/rust:1.59.0 as builder
RUN apt-get update && apt-get install -y libdbus-1-dev libusb-1.0-0-dev pkg-config && apt-get clean
WORKDIR /usr/src/moonlander_server
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get clean
COPY --from=builder /usr/local/cargo/bin/moonlander_server /usr/local/bin/moonlander_server
CMD ["moonlander_server"]
