FROM rust:1.56 as builder
RUN rustup target add wasm32-unknown-unknown \
    && curl -fsSL https://deb.nodesource.com/setup_14.x | bash - \
    && apt-get install -y nodejs
WORKDIR /
COPY . .
RUN cargo build --release

WORKDIR /frontend
RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.14.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
RUN npx tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify && ./trunk build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /target/release/faces ${APP}/faces
COPY --from=builder /frontend/dist ${APP}/frontend/dist

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./faces"]
