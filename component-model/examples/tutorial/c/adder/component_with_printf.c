#include "adder.h"
#include <stdio.h>

uint32_t exports_docs_adder_add_add(uint32_t x, uint32_t y)
{
	uint32_t result = x + y;
        // On traditional platforms, printf() prints to stdout, but on Wasm platforms,
        // stdout and the idea of printing to an output stream is
        // introduced and managed by WASI.
        //
        // When building this code with wasi-libc (as a part of wasi-sdk), the printf call
        // below is implemented with code that uses `wasi:cli/stdout` and `wasi:io/streams`.
	printf("%d", result);
	return result;
}
