FROM docker.io/library/rust:1.69 as build
RUN mkdir /app
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM docker.io/library/almalinux:9
RUN mkdir /app
WORKDIR /app
COPY  --from=build /app/target/release/http-nats-obj /app/
 
