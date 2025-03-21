#include <stdio.h>
#include <stdbool.h>
#include "matrix.h"
#include "lib.h"

/**
 * Entrypoint script to show usage of the library.
 */
int main() {
    
    float data[5][5] = {
        {0.0, 0.0, 0.0, 0.0, 0.0,},
        {0.20689655, 0.07407407, 0.04761905, 0.0, 0.23076923},
        {0.0, 0.0, 0.38461538, 0.0, 0.0},
        {0.0, 0.0, 0.04347826, 0.5, 0.0},
        {0.5, 0.0, 0.0, 0.0, 1.0}
    };

    Matrix* matrix = create_matrix(5, 5, data);
    if (!matrix)
        return 1; // Error occured.

    int* matches = recursive_match(matrix, 0, true, false);

    printf("Matches: [");
    for (int i = 0; i < matrix->rows; i++)
        printf("%d ", matches[i]);
    printf("]\n");

    // Free allocated memory.
    free_matrix(matrix);

    return 0;
}