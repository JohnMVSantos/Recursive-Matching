use ndarray::{array, Array2};

mod recursive_match;

fn main() {
    let mut matrix: Array2<f32> = array![
        [0.0, 0.0, 0.0, 0.0, 0.0,],
        [0.20689655, 0.07407407, 0.04761905, 0.0, 0.23076923],
        [0.0, 0.0, 0.38461538, 0.0, 0.0],
        [0.0, 0.0, 0.04347826, 0.5, 0.0],
        [0.5, 0.0, 0.0, 0.0, 1.0]
    ];

    let matches =
        recursive_match::recursive_match(&mut matrix, 1 as usize, true, false);

    println!("Matches: {:?}", matches);
    assert_eq!(vec![1, -1, 2, 3, 4], matches);
}
