/*
    Apply double threshold to determine potential edges
 */


pub(crate) fn threshold(input_matrix: Vec<Vec<f64>>, high_threshold: f64, low_threshold: f64, strong_edge: f64, weak_edge: f64) -> Vec<Vec<f64>> {
    let rows = input_matrix.len();
    let cols = input_matrix[0].len();
    let mut output_matrix = vec![vec![0.0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let pixel_value = input_matrix[i][j];
            if pixel_value >= high_threshold {
                output_matrix[i][j] = strong_edge;
            } else if pixel_value >= low_threshold {
                output_matrix[i][j] = weak_edge;
            }
        }
    }

    output_matrix
}