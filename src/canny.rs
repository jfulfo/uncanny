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

use image::RgbImage;
use std::time::Instant;

pub(crate) const STRONG_EDGE: f32 = 255.0;
pub(crate) const WEAK_EDGE: f32 = 25.0;
pub(crate) const HIGH_THRESHOLD: f32 = 75.0;
pub(crate) const LOW_THRESHOLD: f32 = 25.0;


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
    let start = Instant::now();
    let input_matrix = matrix_image_converter::image_to_matrix(&input_image);
    println!("Image to matrix: {:?}", start.elapsed());
    let mut output_matrix = gaussian::gaussian(input_matrix);
    println!("Gaussian: {:?}", start.elapsed());
    let (x_gradient, y_gradient) = gradient::gradient(output_matrix);
    println!("Gradient: {:?}", start.elapsed());
    output_matrix = suppression::suppress(&x_gradient, &y_gradient);
    println!("Suppression: {:?}", start.elapsed());
    output_matrix = threshold::threshold(output_matrix, HIGH_THRESHOLD, LOW_THRESHOLD, STRONG_EDGE, WEAK_EDGE);
    println!("Threshold: {:?}", start.elapsed());
    output_matrix = hysteresis::hysteresis(output_matrix);
    println!("Hysteresis: {:?}", start.elapsed());

    let img: RgbImage = matrix_image_converter::matrix_to_image(output_matrix, new_height, new_width);
    println!("Matrix to image: {:?}", start.elapsed());
    img
}
