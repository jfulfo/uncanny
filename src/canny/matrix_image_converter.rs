/*
    Utils for converting between image and matrix
 */

use image::RgbImage;
use rayon::prelude::*;

pub(crate) fn matrix_to_image(matrix: Vec<Vec<f32>>, height: u32, width: u32) -> RgbImage {
    let mut image = RgbImage::new(width, height);

    // Accessing the underlying buffer directly
    let buffer = image.as_mut();
    buffer.par_chunks_mut(3).enumerate().for_each(|(index, pixel)| {
        let x = (index as u32) % width;
        let y = (index as u32) / width;
        let value = matrix[x as usize][y as usize] as u8;
        pixel.copy_from_slice(&[value, value, value]);
    });

    image
}

pub(crate) fn image_to_matrix(input_image: &RgbImage) -> Vec<Vec<f32>> {
    let width = input_image.width() as usize;
    let height = input_image.height() as usize;

    let mut matrix: Vec<Vec<f32>> = vec![vec![0.0; height]; width];

    matrix.par_iter_mut().enumerate().for_each(|(x, col)| {
        for y in 0..height {
            let pixel = input_image.get_pixel(x as u32, y as u32);
            col[y] = pixel[0] as f32;
        }
    });

    matrix
}
