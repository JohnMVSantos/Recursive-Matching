#ifndef MATRIX_H
#define MATRIX_H

#include <stdio.h>
#include <stdlib.h>

// Structure to represent a matrix
typedef struct {
    int rows;
    int cols;
    float** data;
} Matrix;

// Function prototypes
Matrix* create_matrix(int rows, int cols, float input_data[rows][cols]);
void free_matrix(Matrix* matrix);
void print_matrix(Matrix* matrix);
void inverse_scale(Matrix* matrix);
float max(Matrix* matrix);
float min(Matrix* matrix);
float* get_row(Matrix* matrix, int i);
float* get_column(Matrix* matrix, int i);

#endif