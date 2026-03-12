FROM ubuntu:24.04 AS build

ARG WASMTIME_VERSION=42.0.1
ARG WASI_SDK_VERSION=27

RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc libc6-dev curl xz-utils ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install wasmtime C API
RUN curl -sL https://github.com/bytecodealliance/wasmtime/releases/download/v${WASMTIME_VERSION}/wasmtime-v${WASMTIME_VERSION}-x86_64-linux-c-api.tar.xz \
    | tar xJ --strip-components=1 -C /usr/local \
    && ldconfig

# Install wasi-sdk (for building the guest component)
RUN curl -sLO https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_SDK_VERSION}/wasi-sdk-${WASI_SDK_VERSION}.0-x86_64-linux.deb \
    && dpkg -i wasi-sdk-${WASI_SDK_VERSION}.0-x86_64-linux.deb \
    && rm wasi-sdk-${WASI_SDK_VERSION}.0-x86_64-linux.deb

# Install wit-bindgen and wasm-tools (pin versions)
ARG WIT_BINDGEN_VERSION=0.53.1
ARG WASM_TOOLS_VERSION=1.245.1
RUN curl -sL https://github.com/bytecodealliance/wit-bindgen/releases/download/v${WIT_BINDGEN_VERSION}/wit-bindgen-${WIT_BINDGEN_VERSION}-x86_64-linux.tar.gz \
    | tar xz --strip-components=1 -C /usr/local/bin --wildcards '*/wit-bindgen' \
    && curl -sL https://github.com/bytecodealliance/wasm-tools/releases/download/v${WASM_TOOLS_VERSION}/wasm-tools-${WASM_TOOLS_VERSION}-x86_64-linux.tar.gz \
    | tar xz --strip-components=1 -C /usr/local/bin --wildcards '*/wasm-tools'

WORKDIR /src

# Fetch guest source from upstream component-docs
ARG COMPONENT_DOCS_REV=7c1b5a9
RUN curl -sL -o world.wit \
      https://raw.githubusercontent.com/bytecodealliance/component-docs/${COMPONENT_DOCS_REV}/component-model/examples/tutorial/wit/adder/world.wit \
    && mkdir -p wit/adder && mv world.wit wit/adder/world.wit
RUN curl -sL -o component.c \
      https://raw.githubusercontent.com/bytecodealliance/component-docs/${COMPONENT_DOCS_REV}/component-model/examples/tutorial/c/adder/component.c

# Build the guest component
RUN wit-bindgen c wit/adder/world.wit
RUN /opt/wasi-sdk/bin/wasm32-wasip2-clang \
    -o adder.wasm \
    -mexec-model=reactor \
    component.c adder.c adder_component_type.o

# Build the host
COPY host.c .
RUN gcc -o /usr/local/bin/adder-host host.c -lwasmtime

# Runtime image
FROM ubuntu:24.04

COPY --from=build /usr/local/lib/libwasmtime.so /usr/local/lib/
RUN ldconfig
COPY --from=build /usr/local/bin/adder-host /usr/local/bin/adder-host
COPY --from=build /src/adder.wasm /opt/adder.wasm

ENTRYPOINT ["adder-host"]
CMD ["1", "2", "/opt/adder.wasm"]
