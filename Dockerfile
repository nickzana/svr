FROM rust:alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/svr
COPY . .
RUN cargo build --release

FROM alpine:edge
RUN apk add --no-cache yt-dlp
COPY --from=builder /usr/src/svr/target/release/svr /usr/local/bin/svr

CMD ["/usr/local/bin/svr"]
