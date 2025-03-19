#include "matrix.h"

// Function to create a matrix with the given data
Matrix* create_matrix(int rows, int cols, float data[rows][cols]) {
    // Allocate memory for the Matrix structure
    Matrix* matrix = (Matrix*)malloc(sizeof(Matrix));
    if (!matrix) {
        fprintf(stderr, "\t - [ERROR] Memory allocation failed for matrix.\n");
        return NULL;
    }

    matrix->rows = rows;
    matrix->cols = cols;

    // Allocate memory for row pointers
    matrix->data = (float**)malloc(rows * sizeof(float*));
    if (!matrix->data) {
        fprintf(stderr, "\t - [ERROR] Memory allocation failed for row pointers.\n");
        free(matrix);
        return NULL;
    }

    // Allocate memory for each row and copy data
    for (int i = 0; i < rows; i++) {
        matrix->data[i] = (float*)malloc(cols * sizeof(float));
        if (!matrix->data[i]) {
            fprintf(stderr, "\t - [ERROR] Memory allocation failed for row %d.\n", i);
            // Free previously allocated memory
            for (int j = 0; j < i; j++) 
                free(matrix->data[j]);
            free(matrix->data);
            free(matrix);
            return NULL;
        }

        // Copy data from input array
        for (int j = 0; j < cols; j++) 
            matrix->data[i][j] = data[i][j];
    }
        

    return matrix;
}

// Function to free the allocated memory for a matrix
void free_matrix(Matrix* matrix) {
    for (int i = 0; i < matrix->rows; i++)
        free(matrix->data[i]);

    free(matrix->data);
    free(matrix);
}

// Function to print a matrix
void print_matrix(Matrix* matrix) {
    for (int i = 0; i < matrix->rows; i++) {
        for (int j = 0; j < matrix->cols; j++) {
            printf("%f ", matrix->data[i][j]);
        }
        printf("\n");
    }
}

void inverse_scale(Matrix* matrix) {
    float max_value = max(matrix);

    for (int i = 0; i < matrix->rows; i++) {
        for (int j = 0; j < matrix->cols; j++) {
            matrix->data[i][j] = -1 * (matrix->data[i][j] - max_value);
        }
    }
}

// Function to find the maximum value in a matrix
float max(Matrix* matrix) {
    float max_value = matrix->data[0][0];
    for (int i = 0; i < matrix->rows; i++) {
        for (int j = 0; j < matrix->cols; j++) {
            if (matrix->data[i][j] > max_value)
                max_value = matrix->data[i][j];
        }
    }
    return max_value;
}

// Function to find the minimum value in a matrix
float min(Matrix* matrix) {
    float min_value = matrix->data[0][0];
    for (int i = 0; i < matrix->rows; i++) {
        for (int j = 0; j < matrix->cols; j++) {
            if (matrix->data[i][j] < min_value)
                min_value = matrix->data[i][j];
        }
    }
    return min_value;
}

float* get_row(Matrix* matrix, int i) {
    // Dynamically allocate memory for the row
    float* row = (float*)malloc(matrix->cols * sizeof(float));
    for (int j = 0; j < matrix->cols; j++)
        row[j] = matrix->data[i][j];
    return row;
}

float* get_column(Matrix* matrix, int i) {
    // Dynamically allocate memory for the column
    float* column = (float*)malloc(matrix->rows * sizeof(float));
    for (int j = 0; j < matrix->rows; j++)
        column[j] = matrix->data[j][i];
    return column;
}

