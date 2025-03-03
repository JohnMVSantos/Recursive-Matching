use ndarray::{Array2, ArrayViewMut, Ix1};
use ndarray_stats::QuantileExt;
use recursive::recursive;
use std::ops::IndexMut;

/// A struct for performing recursive matching on a 2D matrix.
///
/// The `Matcher` struct provides an algorithm for matching rows or columns of
/// a matrix based on the values in the matrix. The matching process can be
/// controlled by an axis, a limit on the matching value, and a minimum threshold
/// for the values to be considered in the match.
///
/// # Fields
///
/// - `matrix`: A mutable reference to a 2D array (`Array2<f32>`) representing
///   the matrix on which the matching is performed.
/// - `axis`: A reference to the axis along which the matching occurs (`0` for
///   rows, `1` for columns).
/// - `min_value`: The minimum value for a match to be considered valid. Matches
///   below this value are discarded when `limit` is `true`.
/// - `limit`: A boolean flag that indicates whether to limit matches to values
///   greater than `min_value`.
/// - `matches`: A `Vec<i32>` representing the indices of the matched rows or
///   columns. A value of `-1` indicates no match.
///
/// # Methods
///
/// ## `new`
///
/// Creates a new `Matcher` instance.
///
/// # Parameters
/// - `matrix`: A mutable reference to a 2D array (`Array2<f32>`) that contains
///   the data to be matched.
/// - `axis`: A reference to the axis to match along (`0` for rows, `1` for
///   columns).
/// - `limit`: A boolean flag indicating whether matches should be limited to
///   values greater than the `min_value`.
///
/// # Return Value
/// Returns a new `Matcher` instance initialized with the provided matrix, axis,
/// and limit flag.
///
/// ## `get_matches`
///
/// Returns the current match results as a `Vec<i32>`.
///
/// # Return Value
/// A `Vec<i32>` representing the matches found by the algorithm. Each index
/// corresponds to a matched value, with `-1` indicating no match.
///
/// ## `rematch`
///
/// A recursive helper method that reattempts to match rows or columns based on
/// the current matrix values.
///
/// # Parameters
/// - `i`: The current index being processed in the matching algorithm.
/// - `items`: A mutable reference to an array view of the current row or column
///   being processed.
///
/// ## `run`
///
/// Runs the matching process across the matrix, invoking the `rematch` method
/// for each row or column, depending on the specified axis.
///
/// # Panics
///
/// The following panics can occur during execution:
///
/// - If the matrix is empty or contains no valid values when calling `min`
///   or `max` functions.
/// - If the `matrix` does not contain valid values (e.g., empty rows or columns).
/// - If `argmax` or `argmin` encounters an empty view.
pub struct Matcher<'a> {
    matrix: &'a mut Array2<f32>,
    axis: &'a usize,
    min_value: f32,
    limit: bool,
    matches: Vec<i32>,
}

impl<'a> Matcher<'a> {
    /// Creates a new `Matcher` instance.
    ///
    /// Initializes the `Matcher` with the provided matrix, axis, and limit flag.
    /// The minimum value for matching is determined by the smallest value in the
    /// matrix, and a vector of `-1` values is used to indicate no matches.
    ///
    /// # Parameters
    /// - `matrix`: A mutable reference to a 2D array (`Array2<f32>`) that will
    ///   be used for matching.
    /// - `axis`: A reference to the axis to match along (either `0` for rows or
    ///   `1` for columns).
    /// - `limit`: A boolean flag indicating whether to limit the matching based
    ///   on the `min_value`.
    ///
    /// # Return Value
    /// Returns a new `Matcher` instance.
    pub fn new(
        matrix: &'a mut Array2<f32>,
        axis: &'a usize,
        limit: bool,
    ) -> Matcher<'a> {
        // Do not match if the maximum value is less than this value.
        let min_value: f32 =
            *matrix.min().expect("The matrix should not be empty.");
        // Assigning -1 indicates the column/row has not been matched.
        let matches: Vec<i32> = vec![-1; matrix.shape()[*axis]];

        Matcher { matrix, axis, min_value, limit, matches }
    }

    /// Returns the current matching results.
    ///
    /// # Return Value
    /// Returns a vector of `i32` values representing the indices of the matched
    /// rows or columns. A value of `-1` indicates no match.
    pub fn get_matches(self) -> Vec<i32> {
        self.matches
    }

    /// A recursive helper function that attempts to rematch rows or columns
    /// during the matching process.
    ///
    /// # Parameters
    /// - `i`: The current index being processed.
    /// - `items`: A mutable reference to a view of the current row or column.
    #[recursive]
    fn rematch<'b>(
        &mut self,
        mut i: usize,
        items: &mut ArrayViewMut<f32, Ix1>,
    ) {
        // The row/column index that best matches the column/row.
        let max_index: usize =
            items.argmax().expect("The matrix should not be empty.");
        let max_value = items[max_index];

        if self.limit && max_value <= self.min_value {
            return;
        }

        // Maintain unique matches.
        if self.matches.contains(&(max_index as i32)) {
            // The row/column that is already matched.
            let j = self
                .matches
                .iter()
                .position(|&x| x == max_index as i32)
                .unwrap();

            let duplicate;
            if *self.axis == 0 {
                duplicate = self.matrix.index_mut((j, max_index));
            } else {
                duplicate = self.matrix.index_mut((max_index, j));
            }

            if *duplicate < max_value {
                self.matches[j] = -1; // Unmatch previous because current match is a better fit.
                self.matches[i] = max_index as i32; // Match the current column/row with the row/column.

                let new_items;
                if *self.axis == 0 {
                    new_items = self.matrix.row(j).to_owned();
                } else {
                    new_items = self.matrix.column(j).to_owned();
                }

                // Rematch previous match.
                i = j;
                items.assign(&new_items);
            }

            // Reassign the highest value to the minimum - 1 to take
            // the second highest value as the next potential match.
            let min_index: usize =
                items.argmin().expect("The matrix should not be empty.");
            let min_value = items[min_index];
            items[max_index] = min_value - 1.0;

            // If i has not changed to j, rematch current match.
            self.rematch(i, items); // Correct call to rematch.
        } else {
            self.matches[i] = max_index as i32; // Add a new match.
        }
    }

    /// Runs the matching algorithm on the entire matrix.
    ///
    /// The function iterates through the matrix, applying the `rematch` method
    /// on each row or column, depending on the axis specified.
    pub fn run(&mut self) {
        let mut matrix = self.matrix.to_owned();
        let end = matrix.shape()[*self.axis];

        for i in 0..end {
            let mut items;
            if *self.axis == 0 {
                items = matrix.row_mut(i);
            } else {
                items = matrix.column_mut(i);
            }
            self.rematch(i, &mut items);
        }
    }
}

