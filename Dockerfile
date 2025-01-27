FROM rust:1.55 as builder

RUN USER=root cargo new --bin scriven 
WORKDIR ./scriven
COPY ./Cargo.toml ./Cargo.toml
RUN rustup default nightly
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /scriven/target/release/scriven ${APP}/scriven
COPY --from=builder /scriven/templates ${APP}/templates
COPY --from=builder /scriven/static ${APP}/static

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./scriven"]
