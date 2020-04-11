FROM rust:slim-stretch as builder
WORKDIR /usr/src/
COPY . .
RUN cargo install --path .
RUN ls  /usr/src/

FROM debian:buster-slim
ENV BIN $PROJECT
ENV PATH="/:${PATH}"
COPY --from=builder "/usr/src/target/release/garu-io-projects-api" /garu-io-projects-api
ENTRYPOINT ["garu-io-projects-api"]