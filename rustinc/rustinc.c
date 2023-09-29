#include <stdio.h>
#include "rustinc.h"

int main () {
    int a = 10;
    int b = 20;

    int result = addr(a, b);

    printf("Result: %d\n", result);
}