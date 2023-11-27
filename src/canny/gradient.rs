/*
    Calculate gradients for canny algorithm
 */



use crate::canny::convolution::convolve_matrices;

use std::vec::Vec;

pub(crate) fn gradient(input_matrix: Vec<Vec<f32>>) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let sobel_x: Vec<Vec<f32>> = vec![
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 0.0, 2.0],
        vec![-1.0, 0.0, 1.0]
    ];
    let sobel_y: Vec<Vec<f32>> = vec![
        vec![-1.0, -2.0, -1.0],
        vec![0.0, 0.0, 0.0],
        vec![1.0, 2.0, 1.0]
    ];
    let x_gradient = convolve_matrices(&sobel_x, &input_matrix);
    let y_gradient = convolve_matrices(&sobel_y, &input_matrix);
    (x_gradient, y_gradient)
}