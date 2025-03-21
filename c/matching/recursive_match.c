#include "recursive_match.h"
#include "matrix.h"

/**
 * @brief Creates and initializes a Matcher object for matrix matching.
 * 
 * Allocates memory for a Matcher structure and initializes its parameters, 
 * including the matrix, axis, limit flag, and the matches array based on 
 * the axis (0 for rows, 1 for columns). It also calculates the matrix's 
 * minimum value.
 * 
 * @param matrix Pointer to the Matrix structure used for matching.
 * @param axis Axis along which to perform matching (0 for rows, 1 for cols).
 * @param limit Boolean flag to limit matching based on the minimum matrix value.
 * 
 * @return A pointer to the created Matcher structure, or NULL on error.
 */
Matcher* create_matcher(Matrix* matrix, int axis, bool limit) {
    // Allocate memory for the Matcher structure.
    Matcher* matcher = (Matcher*)malloc(sizeof(Matcher));
    if (!matcher) {
        fprintf(stderr, "\t - [ERROR] Memory allocation failed for matcher.\n");
        return NULL;
    }

    int size = 0;
    switch (axis) {
        case 0:
            size = matrix->rows;
            break;
        case 1:
            size = matrix->cols;
            break;
        default:
            fprintf(stderr, "\t - [ERROR] Axis can only be 0 or 1. Got %d\n", axis);
            return NULL;
    }

    // Dynamically allocate memory for matches.
    int* matches = (int*)malloc(size * sizeof(int));
    if (!matches) {
        fprintf(stderr, "\t - [ERROR] Memory allocation failed for matches!\n");
        return NULL;
    }

    for (int i = 0; i < size; i++)
        matches[i] = -1;

    float min_value = min(matrix);

    matcher->matrix = matrix;
    matcher->axis = axis;
    matcher->min_value = min_value;
    matcher->limit = limit;
    matcher->matches = matches;
    matcher->size = size;

    return matcher;
}

/**
 * @brief Performs rematching for a specific element in the matrix.
 * 
 * This function finds the maximum value in the given items (row/column of 
 * the matrix), and attempts to match the current element (index i) with 
 * the best match (index of maximum value). It also handles conflicts by 
 * recursively rematching if necessary. The match may be limited by the 
 * matrix's minimum value.
 * 
 * @param matcher Pointer to the Matcher structure containing matrix and other 
 *                matching parameters.
 * @param i Index of the current element to match.
 * @param items Array of float values representing a row/column of the matrix.
 */
void rematch(Matcher* matcher, int i, float* items) {
    int max_index = 0;
    float max_value = items[0];

    // Find the argmax.
    for (int k = 0; k < matcher->size; k++) {
        if (items[k] > max_value) {
            max_index = k;
            max_value = items[k];
        }
    }

    if (matcher->limit && max_value <= matcher->min_value)
        return;

    // Find the index containing max_index.
    bool contains_max_index = false;
    int j = 0;
    for (int k = 0; k < matcher->size; k++) {
        if (matcher->matches[k] == max_index) {
            contains_max_index = true;
            j = k;
            break;
        }
    }

    if (contains_max_index) {
        float duplicate;
        if (matcher->axis == 0) 
            duplicate = matcher->matrix->data[j][max_index];
        else
            duplicate = matcher->matrix->data[max_index][j];
        
        if (duplicate < max_value) {
            // Unmatch previous because current match is a better fit.
            matcher->matches[j] = -1; 
            // Match the current column/row with the row/column.
            matcher->matches[i] = max_index; 
            
            float *new_items;
            if (matcher->axis == 0)
                new_items = get_row(matcher->matrix, j);
            else
                new_items = get_column(matcher->matrix, j);
            
            // Rematch previous match.
            i = j;
            items = new_items;
        }

        int min_index = 0;
        float min_value = items[0];

        // Find the argmin.
        for (int k = 0; k < matcher->size; k++) {
            if (items[k] < min_value) {
                min_index = k;
                min_value = items[min_index];
            }
        }

        // Reassign the highest values to the minimum - 1 to take
        // the second highest value as the next potential match.
        items[max_index] = min_value - 1.0;

        // If i has not changed to j, rematch current match.
        rematch(matcher, i, items);

    } else {
        matcher->matches[i] = max_index;
    }

}

/**
 * @brief Runs the matching process for all elements along the specified axis.
 * 
 * Iterates through all elements along the specified axis, calling the 
 * rematch function for each. Frees the memory allocated for the row/column 
 * of the matrix after each match operation.
 * 
 * @param matcher Pointer to the Matcher structure containing matrix and 
 *                matching parameters.
 */
void run(Matcher* matcher) {
    for (int i = 0; i < matcher->size; i++) {
        float *items;
        if (matcher->axis == 0) 
            items = get_row(matcher->matrix, i);
        else 
            items = get_column(matcher->matrix, i);
        rematch(matcher, i, items);
        free(items); // Free allocated memory for the items.
    }
}