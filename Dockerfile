FROM alpine:edge

COPY . /app
RUN apk add --no-cache rust libsodium-dev cargo && \
    cd /app && \
    cargo build --release && \
    mkdir -p /usr/local/bin && \
    cp /app/target/release/earmms_keyderiv /usr/local/bin/earmms_keyderiv && \
    cd / && \
    rm -rf /app && \
    apk del rust cargo

ENV PORT 80
CMD /usr/local/bin/earmms_keyderiv
