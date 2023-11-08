/*
* Apply a gaussian filter for the canny algorithm
*/

use std::vec::Vec;
use image;

const STD_DEV: f64 = 1.0;
const KERNEL_ROWS: u8 = 5;
const KERNEL_COLS: u8 = 5;

fn kernel_index(i: u8, j: u8) -> f64 {
    let x_mean = KERNEL_ROWS / 2;
    let y_mean = KERNEL_COLS / 2;
    let mut value: f64 = -((i - x_mean).powi(2) + (j - y_mean).powi(2) as f64) / (2 * STD_DEV.powi(2));
    value = value.exp();
    value /= 2 * 3.14159 * STD_DEV.powi(2);
    value
}

fn normalize_kernel(kernel: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let sum = kernel.iter().flat_map(|row| row.iter()).sum();
    let normalized: Vec<Vec<f64>> = kernel
        .iter()
        .map(|row| row.iter().map(|&element| element / sum).collect())
        .collect();
    normalized 
}

fn gaussian_filter_kernel() -> Vec<Vec<f64>> {
    let mut gaussian_kernel: Vec<Vec<f64>> = Vec::with_capacity(5);

    for i in 0..KERNEL_ROWS {
        for j in 0..KERNEL_COLS {
            gaussian_kernel[i][j] = kernel_index(i, j);
        }
    }

    gaussian_kernel = normalize_kernel(gaussian_kernel);
    gaussian_kernel
}

fn convolution_at_index(kernel: Vec<Vec<f64>>, matrix: Vec<Vec<f64>>, i: u32, j: u32) -> f64 {
    let mut sum: f64 = 0.0;
    for k in 0..KERNEL_ROWS {
        for l in 0..KERNEL_COLS {
            sum += kernel[k][l] * matrix[i + k as usize][j + l as usize];
        }
    }
    sum
}

fn convolve_matrices(kernel: Vec<Vec<f64>>, matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows = matrix.len() - kernel.len() + 1;
    let cols = matrix[0].len() - kernel[0].len() + 1;
    let convolution: Vec<Vec<f64>> = Vec::with_capacity(rows);

    for i in 0..rows {
        for j in 0..cols {
            convolution[i][j] = convolution_at_index(kernel, matrix, i as u32, j as u32);
        }
    }

    convolution
}

pub(crate) fn gaussian(input_image: &image::RgbImage) -> image::RgbImage {
    let mut output_image = image::RgbImage::new(input_image.width(), input_image.height());
    let kernel = gaussian_filter_kernel();
    let input_matrix = input_image.to_luma8();
    let output_matrix = convolve_matrices(kernel, input_matrix);
    output_image
}

