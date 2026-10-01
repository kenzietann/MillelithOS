FROM debian:trixie-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y live-build ca-certificates

WORKDIR /build

CMD ["/bin/bash"]