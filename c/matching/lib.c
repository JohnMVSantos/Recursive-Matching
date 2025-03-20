#include "lib.h"
#include "recursive_match.h"
#include "matrix.h"

/**
 * @brief Recursively matches elements in a matrix along a given axis.
 *
 * @param matrix Pointer to the matrix to process.
 * @param axis Axis along which to perform matching (0 for rows, 1 for cols).
 * @param limit Boolean flag to enable or disable match limiting.
 * @param minimum Boolean flag to determine if matching is based on the minimum
 *                values (if true, the matrix is inversely scaled).
 * @return Pointer to an array containing the match results.
 */
int* recursive_match(Matrix* matrix, int axis, bool limit, bool minimum) {
    // Convert matrix to match based on minimum.
    if (minimum)
        inverse_scale(matrix);

    Matcher* matcher = create_matcher(matrix, axis, limit);
    run(matcher);

    return matcher->matches;
}