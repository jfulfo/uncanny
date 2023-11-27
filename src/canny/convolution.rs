/*
    Naively convolves a kernel with a matrix.
 */

use rayon::prelude::*;

pub(crate) fn convolve_matrices(kernel: &[Vec<f32>], matrix: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let kernel_rows = kernel.len();
    let kernel_cols = kernel[0].len();
    let rows = matrix.len() - kernel_rows + 1;
    let cols = matrix[0].len() - kernel_cols + 1;

    let mut result = vec![vec![0.0; cols]; rows];

    // hatred
    result.par_iter_mut().enumerate().for_each(|(i, result_row)| {
        for (j, result_cell) in result_row.iter_mut().enumerate().take(cols) {
            *result_cell = kernel.iter().take(kernel_rows).zip(matrix[i..].iter())
                .flat_map(|(kernel_row, matrix_row)| {
                    kernel_row.iter().take(kernel_cols).zip(matrix_row[j..].iter())
                        .map(|(&k, &m)| k * m)
                })
                .sum();
        }
    });
    result
}