#ifndef RECURSIVE_MATCH_H
#define RECURSIVE_MATCH_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include "matrix.h"

/**
 * @file recursive_match.h
 * @brief Header file for recursive matching functionality.
 * 
 * This header defines the data structures and function prototypes necessary 
 * to perform matching operations on a matrix. The primary structure is the 
 * `Matcher` that holds the matrix, matching parameters, and an array of matches.
 * The functions declared in this file handle the creation of a matcher, 
 * performing rematching operations, and running the overall matching process.
 */

/**
 * @struct Matcher
 * @brief Structure to represent the matcher for matrix matching operations.
 * 
 * The `Matcher` structure stores the matrix to match against, the axis of 
 * matching (rows or columns), the minimum matrix value for limiting matches, 
 * a boolean flag to indicate if the matching should be limited, an array 
 * representing the matches, and the size of the matches.
 * 
 * @var Matcher::matrix
 * The matrix on which the matching operations are performed.
 * 
 * @var Matcher::axis
 * The axis along which to perform matching. 0 for rows, 1 for columns.
 * 
 * @var Matcher::min_value
 * The minimum value in the matrix, used to limit matches if the `limit` flag 
 * is set to true.
 * 
 * @var Matcher::limit
 * A boolean flag that determines if matching should be limited by the 
 * `min_value`.
 * 
 * @var Matcher::matches
 * An array of integers representing the current matches for each row/column.
 * 
 * @var Matcher::size
 * The size of the matches array, which corresponds to the number of rows or 
 * columns in the matrix.
 */
typedef struct {
    Matrix* matrix;  /**< Pointer to the matrix for matching. */
    int axis;        /**< The axis to match along (0 for rows, 1 for columns). */
    float min_value; /**< The minimum value in the matrix. */
    bool limit;      /**< Whether to limit matching based on the minimum value. */
    int* matches;    /**< Array of integer matches for each row/column. */
    int size;        /**< The number of rows or columns in the matrix. */
} Matcher;

/** Function Prototypes */
Matcher* create_matcher(Matrix* matrix, int axis, bool limit);
void rematch(Matcher* matcher, int i, float* items);
void run(Matcher* matcher);

#endif
