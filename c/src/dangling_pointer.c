#include <stdio.h>


int* getArray() {

    int array[3] = {1, 2, 3};

    return array;

}


void printArray(int* arr) {

    for (int i = 0; i < 3; i++) {

        printf("%d ", arr[i]);

    }

    printf("\n");

}


int main() {

    int* danglingPointer = getArray();

    printArray(danglingPointer); // Using dangling pointer


    return 0;

}