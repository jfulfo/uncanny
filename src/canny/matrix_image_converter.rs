/*
    Utils for converting between image and matrix
 */

use image::{RgbImage};

pub(crate) fn matrix_to_image(matrix: Vec<Vec<f64>>, height: u32, width: u32) -> RgbImage {
    let mut image = RgbImage::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let value = matrix[x as usize][y as usize];
        *pixel = image::Rgb([value as u8, value as u8, value as u8]);
    }
    image
}

pub(crate) fn image_to_matrix(input_image: &RgbImage) -> Vec<Vec<f64>> {
    let mut matrix: Vec<Vec<f64>> = Vec::with_capacity(input_image.width() as usize);

    for _ in 0..input_image.width() {
        let mut row: Vec<f64> = Vec::with_capacity(input_image.height() as usize);
        for _ in 0..input_image.height() {
            row.push(0.0);
        }
        matrix.push(row);
    }

    for (x, y, pixel) in input_image.enumerate_pixels() {
        matrix[x as usize][y as usize] = pixel[0] as f64;
    }
    matrix
}
