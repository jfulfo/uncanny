/*
    Given an image, return a new black and white image
    that is the edges from the Canny algorithm
 */

mod gaussian;
mod gradient;
mod suppression;
mod threshold;
mod hysteresis;

use image::{ImageBuffer, RgbImage};

/*
    Five steps to Canny edge detection:
    1. Apply Gaussian filter
    2. Find intensity gradients
    3. Apply non-maximum suppression
    4. Apply double threshold to determine potential edges
    5. Track edge by hysteresis
 */
pub(crate) fn canny(input_image: &RgbImage) -> RgbImage {
    let mut output_image = ImageBuffer::new(input_image.width(), input_image.height());

    let gaussian_image = gaussian::gaussian(input_image);

    output_image 
}
