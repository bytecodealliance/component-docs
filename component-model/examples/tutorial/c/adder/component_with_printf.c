#include "adder.h"
#include <stdio.h>

uint32_t exports_docs_adder_add_add(uint32_t x, uint32_t y)
{
	uint32_t result = x + y;
	printf("%d", result);
	return result;
}
