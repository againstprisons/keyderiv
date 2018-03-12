FROM alpine:edge

RUN apk add --no-cache \
    rust \
    libsodium-dev

RUN apk add --no-cache cargo

COPY . /app
WORKDIR /app
RUN cargo build --release

ENV PORT 80
CMD /app/target/release/earmms_keyderiv
