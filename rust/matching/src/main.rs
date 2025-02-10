use ndarray::array;
use ndarray::{Array1, Array2};
use ndarray_stats::QuantileExt;
use std::ops::Range;


fn recursive_match(
    matrix: &mut Array2<f32>, 
    axis: Option<usize>, 
    limit: Option<bool>,
    minimum: Option<bool>
) -> Array1<i32> {
    let axis = axis.unwrap_or(1); 
    let _limit = limit.unwrap_or(true);  
    let minimum = minimum.unwrap_or(false);

    if axis != 0 && axis != 1 {
        panic!("\t - [WARNING]: Axis can only be 0 or 1. Got {axis}");
    }

    if minimum {
        // The smallest values becomes the largest values.
        let max_value: f32 = *matrix.max().expect("The matrix should not be empty.");
        *matrix -= max_value;
        *matrix *= -1.0;
    }

    // Do not match if the maximum value is less than this value.
    let min_value: &f32 = matrix.min().expect("The matrix should not be empty.");
    // Assigning -1 indicates the column/row has not been matched.
    let matches = Array1::<i32>::zeros(matrix.shape()[axis]) -1;

    fn rematch(_i: usize, _items: &mut ndarray::ArrayViewMut1<f32>) {

    }

    println!("{}", min_value);
    println!("{:?}", matrix);
    println!("{:?}", matches);

    for i in (Range{start: 0, end: matrix.shape()[axis]}) {
        let mut items;
        // Fetching rows.
        if axis == 0 {
            items = matrix.row_mut(i);
        }
        // Fetching columns.
        else {
            items = matrix.column_mut(i);
        }
        rematch(i, &mut items);
    }

    return matches

}

fn main() {
    let mut matrix: Array2<f32> = array![
        [0.0, 0.0, 0.0, 0.0, 0.0,],
        [0.20689655, 0.07407407, 0.04761905, 0.0, 0.23076923],
        [0.0, 0.0, 0.38461538, 0.0, 0.0],
        [0.0, 0.0, 0.04347826, 0.5, 0.0],
        [0.5, 0.0, 0.0, 0.0, 1.0]
    ];
    
    recursive_match(
        &mut matrix,
        None,
        None,
        None
    );
}
