FROM rust:alpine AS build

COPY . /app

WORKDIR /app

RUN cargo build --release

FROM alpine

COPY --from=build /app/target/release/fronius-prometheus-exporter /

USER nobody

EXPOSE 9123

CMD [ "/fronius-prometheus-exporter" ]
