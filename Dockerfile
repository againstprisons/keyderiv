FROM alpine:3.6

VOLUME /config

RUN apk add --no-cache python3-dev bash ca-certificates git
RUN python3 -m ensurepip

# install cffi manually since pip doesn't like installing it while
# installing the application for some reason
RUN apk add --no-cache gcc make musl-dev libffi-dev
RUN python3 -m pip install cffi

# install libsodium for incoming application installation
RUN apk add --no-cache libsodium-dev

# install the application
COPY . /earmms_keyderiv
RUN python3 -m pip install /earmms_keyderiv

ENV EARMMS_KEYDERIV_CONFIG=/config/config.py
CMD gunicorn -b 0.0.0.0:80 earmms_keyderiv.app:app

