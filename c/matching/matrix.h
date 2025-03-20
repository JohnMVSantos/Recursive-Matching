#ifndef MATRIX_H
#define MATRIX_H

#include <stdio.h>
#include <stdlib.h>

/**
 * @brief Structure to represent a matrix.
 * 
 * @var Matrix::rows
 * Number of rows in the matrix.
 *
 * @var Matrix::cols
 * Number of columns in the matrix.
 *
 * @var Matrix::data
 * Pointer to the matrix data.
 */
typedef struct {
    int rows;       /**< Number of rows in the matrix. */
    int cols;       /**< Number of columns in the matrix. */
    float** data;   /**< Pointer to the matrix data. */
} Matrix;

/** Function Prototypes */
Matrix* create_matrix(int rows, int cols, float input_data[rows][cols]);
void free_matrix(Matrix* matrix);
void print_matrix(Matrix* matrix);
void inverse_scale(Matrix* matrix);
float max(Matrix* matrix);
float min(Matrix* matrix);
float* get_row(Matrix* matrix, int i);
float* get_column(Matrix* matrix, int i);

#endif