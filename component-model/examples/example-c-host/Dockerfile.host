FROM ubuntu:24.04 AS build

ARG WASMTIME_VERSION=42.0.1
ARG TARGETARCH

RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc libc6-dev curl xz-utils ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install wasmtime C API
RUN set -eux; \
    case "${TARGETARCH}" in \
        amd64) WASMTIME_ARCH=x86_64 ;; \
        arm64) WASMTIME_ARCH=aarch64 ;; \
        *) echo "Unsupported TARGETARCH: ${TARGETARCH}" >&2; exit 1 ;; \
    esac; \
    curl -sL "https://github.com/bytecodealliance/wasmtime/releases/download/v${WASMTIME_VERSION}/wasmtime-v${WASMTIME_VERSION}-${WASMTIME_ARCH}-linux-c-api.tar.xz" \
    | tar xJ --strip-components=1 -C /usr/local; \
    ldconfig

WORKDIR /src

# Build the host
COPY host.c .
RUN gcc -o /usr/local/bin/adder-host host.c -lwasmtime

# Runtime image
FROM ubuntu:24.04

COPY --from=build /usr/local/lib/libwasmtime.so /usr/local/lib/
RUN ldconfig
COPY --from=build /usr/local/bin/adder-host /usr/local/bin/adder-host

ENTRYPOINT ["adder-host"]
CMD ["1", "2", "/component/add.wasm"]