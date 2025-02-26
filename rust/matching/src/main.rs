use ndarray::{array, Array2, ArrayViewMut, Ix1};
use ndarray_stats::QuantileExt;
use recursive::recursive;
use std::ops::IndexMut;

struct Matcher<'a> {
    matrix: &'a mut Array2<f32>,
    axis: &'a usize,
    min_value: f32,
    limit: bool,
    matches: Vec<i32>,
}

impl<'a> Matcher<'a> {
    fn new(
        matrix: &'a mut Array2<f32>,
        axis: &'a usize,
        limit: bool,
    ) -> Matcher<'a> {
        // Do not match if the maximum value is less than this value.
        let min_value: f32 =
            *matrix.min().expect("The matrix should not be empty.");
        // Assigning -1 indicates the column/row has not been matched.
        let matches: Vec<i32> = vec![-1; matrix.shape()[*axis]];

        Matcher {
            matrix,
            axis,
            min_value,
            limit,
            matches,
        }
    }

    fn get_matches(self) -> Vec<i32> {
        self.matches
    }

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

    fn run(&mut self) {
        let mut matrix = self.matrix.to_owned();
        let end = matrix.shape()[*self.axis]; 

        for i in 0..end {
            let mut items;
            if *self.axis == 0 {
                items = matrix.row_mut(i);
            }
            else {
                items = matrix.column_mut(i);
            }
            self.rematch(i, &mut items);
        }
    }
}

fn recursive_match(
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

    let mut matcher = Matcher::new(matrix, &axis, limit);

    matcher.run();

    let matches = matcher.get_matches();

    return matches;
}

fn main() {
    let mut matrix: Array2<f32> = array![
        [0.0, 0.0, 0.0, 0.0, 0.0,],
        [0.20689655, 0.07407407, 0.04761905, 0.0, 0.23076923],
        [0.0, 0.0, 0.38461538, 0.0, 0.0],
        [0.0, 0.0, 0.04347826, 0.5, 0.0],
        [0.5, 0.0, 0.0, 0.0, 1.0]
    ];

    let matches = recursive_match(&mut matrix, 1 as usize, true, false);

    println!("Matches: {:?}", matches);
}
