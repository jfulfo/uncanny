/*
* Apply a gaussian filter for the canny algorithm
*/


use crate::canny::convolution::convolve_matrices;
use std::vec::Vec;

const STD_DEV: f32 = 1.0;
pub(crate) const GAUSSIAN_KERNEL_ROWS: usize = 5;
pub(crate) const GAUSSIAN_KERNEL_COLS: usize = 5;
const X_MEAN: i16 = (GAUSSIAN_KERNEL_ROWS / 2) as i16;
const Y_MEAN: i16 = (GAUSSIAN_KERNEL_COLS / 2) as i16;
const DENOMINATOR: f32 = 2.0 * STD_DEV; // 2 * sigma^2
const NORMALIZER: f32 = 2.0 * std::f32::consts::PI * STD_DEV; // 2 * pi * sigma^2

// formula:  1 / (2 * pi * sigma^2) * e^(-((i - u_x)^2 + (y - u_y)^2) / (2 * sigma^2))
#[inline]
fn kernel_index(i: usize, j: usize) -> f32 {
    let mut value: f32 = -(((i as i16 - X_MEAN).pow(2) + (j as i16 - Y_MEAN).pow(2)) as f32) / DENOMINATOR;
    value = value.exp();
    value /= NORMALIZER;
    value
}

fn normalize_kernel(kernel: &mut Vec<Vec<f32>>) {
    let sum: f32 = kernel.iter().flat_map(|row| row.iter()).sum();
    for i in 0..kernel.len() {
        for j in 0..kernel[i].len() {
            kernel[i][j] /= sum;
        }
    }
}

fn gaussian_filter_kernel() -> Vec<Vec<f32>> {
    let mut gaussian_kernel: Vec<Vec<f32>> = vec![vec![0.0; GAUSSIAN_KERNEL_COLS]; GAUSSIAN_KERNEL_ROWS];

    for i in 0..GAUSSIAN_KERNEL_ROWS {
        for j in 0..GAUSSIAN_KERNEL_COLS {
            gaussian_kernel[i][j] = kernel_index(i, j);
        }
    }

    normalize_kernel(&mut gaussian_kernel);
    gaussian_kernel
}


pub(crate) fn gaussian(input_matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let kernel = gaussian_filter_kernel();
    convolve_matrices(&kernel, &input_matrix)
}

