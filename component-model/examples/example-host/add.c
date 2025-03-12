#include "example.h"
#include <stdio.h>

int32_t exports_example_add(int32_t x, int32_t y)
{
    int32_t result = x + y;
    printf("%d", result);
    return result;
}