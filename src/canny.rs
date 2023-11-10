/*
    Given an image, return a new black and white image
    that is the edges from the Canny algorithm
 */

mod gaussian;
mod gradient;
mod suppression;
mod threshold;
mod hysteresis;
mod matrix_image_converter;
mod convolution;

use image::{RgbImage};

pub(crate) const STRONG_EDGE: f64 = 255.0;
pub(crate) const WEAK_EDGE: f64 = 25.0;
pub(crate) const HIGH_THRESHOLD: f64 = 75.0;
pub(crate) const LOW_THRESHOLD: f64 = 25.0;

/*
    Five steps to Canny edge detection:
    1. Apply Gaussian filter
    2. Find intensity gradients
    3. Apply non-maximum suppression
    4. Apply double threshold to determine potential edges
    5. Track edge by hysteresis
 */
pub(crate) fn canny(input_image: &RgbImage) -> RgbImage {
    let new_height = input_image.height() - gaussian::GAUSSIAN_KERNEL_ROWS as u32 - 1;
    let new_width = input_image.width() - gaussian::GAUSSIAN_KERNEL_COLS as u32 - 1;
    let input_matrix = matrix_image_converter::image_to_matrix(&input_image);
    let mut output_matrix = gaussian::gaussian(input_matrix);
    let (x_gradient, y_gradient) = gradient::gradient(output_matrix);
    output_matrix = suppression::suppress(&x_gradient, &y_gradient);
    output_matrix = threshold::threshold(output_matrix, HIGH_THRESHOLD, LOW_THRESHOLD, STRONG_EDGE, WEAK_EDGE);
    output_matrix = hysteresis::hysteresis(output_matrix);

    matrix_image_converter::matrix_to_image(output_matrix, new_height, new_width)
}
