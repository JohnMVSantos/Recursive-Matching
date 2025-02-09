use ndarray::array;
use ndarray::{Array1, Array2};
use ndarray_stats::QuantileExt;
use std::ops::Range;


fn recursive_match(
    matrix: &Array2<f32>, 
    axis: Option<usize>, 
    limit: Option<bool>,
    minimum: Option<bool>
) -> Array1<i32> {
    let axis = axis.unwrap_or(1); 
    let _limit = limit.unwrap_or(true);  
    let _minimum = minimum.unwrap_or(false);

    // Do not match if the maximum value is less than this value.
    let min_value: &f32 = matrix.min().expect("The matrix should not be empty.");
    // Assigning -1 indicates the column/row has not been matched.
    let matches = Array1::<i32>::zeros(matrix.shape()[axis]) -1;

    println!("{}", min_value);
    println!("{:?}", matrix);
    println!("{:?}", matches);

    for i in (Range{start: 0, end: matrix.shape()[axis]}) {
        println!("the value is: {i}");
    }

    return matches

}

fn main() {
    let matrix = array![
        [0.0, 0.0, 0.0, 0.0, 0.0,],
        [0.20689655, 0.07407407, 0.04761905, 0.0, 0.23076923],
        [0.0, 0.0, 0.38461538, 0.0, 0.0],
        [0.0, 0.0, 0.04347826, 0.5, 0.0],
        [0.5, 0.0, 0.0, 0.0, 1.0]
    ];
    
    recursive_match(
        &matrix,
        None,
        None,
        None
    );
}
