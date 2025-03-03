mod recursive_match;

use ndarray::Array2;
use ndarray_stats::QuantileExt;

/// Runs the recursive algorithm to generate the assignments.
///
/// This function processes a given 2D matrix and generates assignments
/// based on the recursive algorithm applied to it. The algorithm can
/// operate along a specified axis, either axis 0 or 1, and can adjust
/// the matrix values to match a certain condition.
///
/// # Parameters
///
/// - `matrix`: A mutable reference to a 2D array (`Array2<f32>`)
///             containing the values to be processed.
/// - `axis`: An unsigned integer (`usize`) representing the
///           axis along which the matching will be performed.
///           If `axis == 0`, the matching occurs along rows.
///           If `axis == 1`, the matching occurs along columns.
///           The function will panic if the axis is neither 0 nor 1.
/// - `limit`: A boolean indicating whether to impose
///            a limit on the matching process.
/// - `minimum`: A boolean that, if true, reverses the matrix
///              values by first finding the maximum value
///              and then inverting the matrix values
///              (i.e., subtracting the matrix from the maximum value and
///              multiplying by -1). If false, the matrix remains unchanged.
///
/// # Return Value
///
/// Returns a `Vec<i32>` representing the generated assignments
/// based on the recursive matching algorithm. The returned vector
/// contains the indices or assignments based on the processed matrix.
///
/// # Examples
/// ```
/// use ndarray::{array, Array2};
/// use recursive_matching::recursive_match;
/// 
/// let mut matrix: Array2<f32> = array![
///     [0.0, 0.0, 0.0, 0.0, 0.0,],
///     [0.20689655, 0.07407407, 0.04761905, 0.0, 0.23076923],
///     [0.0, 0.0, 0.38461538, 0.0, 0.0],
///     [0.0, 0.0, 0.04347826, 0.5, 0.0],
///     [0.5, 0.0, 0.0, 0.0, 1.0]
/// ];
/// let matches = recursive_match(&mut matrix, 1 as usize, true, false);
/// assert_eq!(vec![1, -1, 2, 3, 4], matches);
/// ```
///
/// # Panics
///
/// This function will panic in the following cases:
///
/// - If the `axis` is neither 0 nor 1. For example, passing `axis = 2`
///   will trigger a panic with a warning message.
/// - If the `matrix` is empty, the `max` function will return `None`,
///   and the code will panic with the message "The matrix should not be empty."
///
/// # Errors
///
/// There are no explicit error returns, but the algorithm
/// relies on the assumption that the matrix is not empty.
/// If the matrix is empty or contains invalid values, the algorithm will panic.
///
/// # Safety
///
/// This function is not marked as unsafe, but the
/// correctness of the algorithm depends on the assumptions:
/// - The matrix must not be empty; otherwise, it will panic.
/// - The `axis` must be either 0 or 1. If it is not, a panic will occur.
///
/// # Notes
///
/// - If `minimum` is `true`, the matrix values are inverted based
///   on the maximum value, which may affect the results of the algorithm.
///
pub fn recursive_match(
    matrix: &mut Array2<f32>,
    axis: usize,
    limit: bool,
    minimum: bool,
) -> Vec<i32> {
    if axis != 0 && axis != 1 {
        panic!("\t - [WARNING]: Axis can only be 0 or 1. Got {axis}");
    }

    if minimum {
        // The smallest values becomes the largest values.
        let max_value: f32 =
            *matrix.max().expect("The matrix should not be empty.");
        *matrix -= max_value;
        *matrix *= -1.0;
    }

    let mut matcher = recursive_match::Matcher::new(matrix, &axis, limit);

    matcher.run();

    let matches = matcher.get_matches();

    return matches;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_recursive_match() {
        
        let mut matrix: Array2<f32> = array![
            [0.0, 0.0, 0.0, 0.0, 0.0,],
            [0.20689655, 0.07407407, 0.04761905, 0.0, 0.23076923],
            [0.0, 0.0, 0.38461538, 0.0, 0.0],
            [0.0, 0.0, 0.04347826, 0.5, 0.0],
            [0.5, 0.0, 0.0, 0.0, 1.0]
        ];

        let matches =
            recursive_match(&mut matrix, 1 as usize, true, false);

        assert_eq!(vec![1, -1, 2, 3, 4], matches);
    }
}