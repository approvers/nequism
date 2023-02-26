FROM rust:slim-buster as build

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

LABEL org.opencontainers.image.source=="https://github.com/approvers/nequism"

COPY --from=build /target/release/nequism /

CMD [ "/nequism" ]
