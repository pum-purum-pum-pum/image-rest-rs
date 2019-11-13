#https://habr.com/ru/post/414109/
FROM rust:1.39 as build

WORKDIR /app/build
COPY . .

RUN cargo build --release

FROM ubuntu:18.04
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    ca-certificates
RUN groupadd -r sample && useradd -r -g sample sample
USER sample
COPY --from=build /app/build/target/release/image_rest /usr/local/bin/
ENTRYPOINT ["image_rest"]