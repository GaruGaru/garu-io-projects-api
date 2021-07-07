FROM rust:slim-buster AS builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config
WORKDIR /usr/src/
COPY . .
RUN cargo install --path .
RUN ls  /usr/src/

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates
ENV BIN $PROJECT
ENV PATH="/:${PATH}"
COPY --from=builder "/usr/src/target/release/garu-io-projects-api" /garu-io-projects-api
ENTRYPOINT ["garu-io-projects-api"]
