FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin discord-ingress-bot
WORKDIR /home/rust/src/discord-ingress-bot/
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN echo $(ls -1 ./target/x86_64-unknown-linux-musl/release/deps/)
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/discord-ingress-bot*
RUN cargo build --release



FROM alpine:latest

LABEL org.opencontainers.image.source https://github.com/discite/discord-ingress-bot

ARG APP=/usr/src/app

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/discord-ingress-bot/target/x86_64-unknown-linux-musl/release/discord-ingress-bot ${APP}/discord-ingress-bot

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./discord-ingress-bot"]