/*
    Naively convolves a kernel with a matrix.
 */

fn convolution_at_index(kernel: &[Vec<f64>], matrix: &[Vec<f64>], i: usize, j: usize) -> f64 {
    let mut sum: f64 = 0.0;
    for (k, kernel_row) in kernel.iter().enumerate() {
        for (l, &kernel_val) in kernel_row.iter().enumerate() {
            sum += kernel_val * matrix[i + k][j + l];
        }
    }
    sum
}

pub(crate) fn convolve_matrices(kernel: &[Vec<f64>], matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let kernel_rows = kernel.len();
    let kernel_cols = kernel[0].len();
    let rows = matrix.len() - kernel_rows + 1;
    let cols = matrix[0].len() - kernel_cols + 1;

    (0..rows).map(|i| {
        (0..cols).map(|j| {
            convolution_at_index(kernel, matrix, i, j)
        }).collect()
    }).collect()
}