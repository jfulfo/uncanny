/*
    Apply double threshold to determine potential edges
 */

use rayon::prelude::*;

pub(crate) fn threshold(input_matrix: Vec<Vec<f32>>, high_threshold: f32, low_threshold: f32, strong_edge: f32, weak_edge: f32) -> Vec<Vec<f32>> {
    let rows = input_matrix.len();
    let cols = input_matrix[0].len();

    let mut output_matrix = vec![vec![0.0; cols]; rows];

    output_matrix.par_iter_mut().enumerate().for_each(|(i, output_row)| {
        for j in 0..cols {
            let pixel_value = input_matrix[i][j];
            if pixel_value >= high_threshold {
                output_row[j] = strong_edge;
            } else if pixel_value >= low_threshold {
                output_row[j] = weak_edge;
            }
        }
    });

    output_matrix
}
