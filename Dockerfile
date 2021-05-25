FROM rust:1.52
ARG _DISCORD_PUBLIC_KEY
WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .
ENV DISCORD_PUBLIC_KEY $_DISCORD_PUBLIC_KEY
CMD ["songxinran"]