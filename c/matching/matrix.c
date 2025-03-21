#include "matrix.h"

/**
 * @brief Creates a matrix and initializes it with given data.
 *
 * @param rows Number of rows in the matrix.
 * @param cols Number of columns in the matrix.
 * @param data 2D array containing matrix data.
 * @return Pointer to the created Matrix structure, or NULL on failure.
 */
Matrix* create_matrix(int rows, int cols, float data[rows][cols]) {
    // Allocate memory for the Matrix structure.
    Matrix* matrix = (Matrix*)malloc(sizeof(Matrix));
    if (!matrix) {
        fprintf(stderr, "\t - [ERROR] Memory allocation failed for matrix.\n");
        return NULL;
    }

    matrix->rows = rows;
    matrix->cols = cols;

    // Allocate memory for row pointers.
    matrix->data = (float**)malloc(rows * sizeof(float*));
    if (!matrix->data) {
        fprintf(stderr, "\t - [ERROR] Memory allocation failed for row pointers.\n");
        free(matrix);
        return NULL;
    }

    // Allocate memory for each row and copy data.
    for (int i = 0; i < rows; i++) {
        matrix->data[i] = (float*)malloc(cols * sizeof(float));
        if (!matrix->data[i]) {
            fprintf(stderr, "\t - [ERROR] Memory allocation failed for row %d.\n", i);
            // Free previously allocated memory.
            for (int j = 0; j < i; j++) 
                free(matrix->data[j]);
            free(matrix->data);
            free(matrix);
            return NULL;
        }

        // Copy data from input array.
        for (int j = 0; j < cols; j++) 
            matrix->data[i][j] = data[i][j];
    }
        
    return matrix;
}

/**
 * @brief Frees the allocated memory for a matrix.
 *
 * @param matrix Pointer to the matrix to be freed.
 */
void free_matrix(Matrix* matrix) {
    for (int i = 0; i < matrix->rows; i++)
        free(matrix->data[i]);

    free(matrix->data);
    free(matrix);
}

/**
 * @brief Prints the elements of a matrix.
 *
 * @param matrix Pointer to the matrix to be printed.
 */
void print_matrix(Matrix* matrix) {
    for (int i = 0; i < matrix->rows; i++) {
        for (int j = 0; j < matrix->cols; j++) {
            printf("%f ", matrix->data[i][j]);
        }
        printf("\n");
    }
}

/**
 * @brief Applies inverse scaling to the matrix using its maximum value.
 *
 * @param matrix Pointer to the matrix to be transformed.
 */
void inverse_scale(Matrix* matrix) {
    float max_value = max(matrix);

    for (int i = 0; i < matrix->rows; i++) {
        for (int j = 0; j < matrix->cols; j++) {
            matrix->data[i][j] = -1 * (matrix->data[i][j] - max_value);
        }
    }
}

/**
 * @brief Finds the maximum value in a matrix.
 *
 * @param matrix Pointer to the matrix.
 * @return The maximum value in the matrix.
 */
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

/**
 * @brief Finds the minimum value in a matrix.
 *
 * @param matrix Pointer to the matrix.
 * @return The minimum value in the matrix.
 */
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

/**
 * @brief Extracts a row from the matrix.
 *
 * @param matrix Pointer to the matrix.
 * @param i Index of the row to extract.
 * @return Dynamically allocated array containing the row.
 */
float* get_row(Matrix* matrix, int i) {
    // Dynamically allocate memory for the row.
    float* row = (float*)malloc(matrix->cols * sizeof(float));
    for (int j = 0; j < matrix->cols; j++)
        row[j] = matrix->data[i][j];
    return row;
}

/**
 * @brief Extracts a column from the matrix.
 *
 * @param matrix Pointer to the matrix.
 * @param i Index of the column to extract.
 * @return Dynamically allocated array containing the column.
 */
float* get_column(Matrix* matrix, int i) {
    // Dynamically allocate memory for the column.
    float* column = (float*)malloc(matrix->rows * sizeof(float));
    for (int j = 0; j < matrix->rows; j++)
        column[j] = matrix->data[j][i];
    return column;
}

