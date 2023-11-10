/*
* Apply a gaussian filter for the canny algorithm
*/


use crate::canny::convolution::convolve_matrices;
use std::vec::Vec;

const STD_DEV: f64 = 1.0;
pub(crate) const GAUSSIAN_KERNEL_ROWS: usize = 5;
pub(crate) const GAUSSIAN_KERNEL_COLS: usize = 5;

// formula:  1 / (2 * pi * sigma^2) * e^(-((i - u_x)^2 + (y - u_y)^2) / (2 * sigma^2))
fn kernel_index(i: usize, j: usize) -> f64 {
    let x_mean = (GAUSSIAN_KERNEL_ROWS / 2) as i16;
    let y_mean = (GAUSSIAN_KERNEL_COLS / 2) as i16;
    let mut value: f64 = -(((i as i16 - x_mean).pow(2) + (j as i16 - y_mean).pow(2)) as f64) / (2.0 * STD_DEV.powi(2));
    value = value.exp();
    value /= 2.0 * std::f64::consts::PI * STD_DEV.powi(2);
    value
}

fn normalize_kernel(kernel: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let sum: f64 = kernel.iter().flat_map(|row| row.iter()).sum();
    let normalized: Vec<Vec<f64>> = kernel
        .iter()
        .map(|row| row.iter().map(|&element| element / sum).collect())
        .collect();
    normalized 
}

fn gaussian_filter_kernel() -> Vec<Vec<f64>> {
    let mut gaussian_kernel: Vec<Vec<f64>> = Vec::with_capacity(GAUSSIAN_KERNEL_ROWS);

    for _ in 0..GAUSSIAN_KERNEL_ROWS {
        let mut row: Vec<f64> = Vec::with_capacity(GAUSSIAN_KERNEL_COLS);
        for _ in 0..GAUSSIAN_KERNEL_COLS {
            row.push(0.0);
        }
        gaussian_kernel.push(row);
    }

    for i in 0..GAUSSIAN_KERNEL_ROWS {
        for j in 0..GAUSSIAN_KERNEL_COLS {
            gaussian_kernel[i][j] = kernel_index(i, j);
        }
    }

    gaussian_kernel = normalize_kernel(gaussian_kernel);
    gaussian_kernel
}


pub(crate) fn gaussian(input_matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let kernel = gaussian_filter_kernel();
    convolve_matrices(&kernel, &input_matrix)
}

