/*
    Lower bound cutoff suppression for Canny edge detection
 */

use std::f32::consts::PI;
use rayon::prelude::*;

enum Direction {
    Horizontal,
    Vertical,
    Diagonal45,
    Diagonal135
}

#[inline]
fn direction(gradient_x: f32, gradient_y: f32) -> f32 {
    (gradient_y / gradient_x).atan()
}

#[inline]
fn hypot(gradient_x: f32, gradient_y: f32) -> f32 {
    (gradient_x.powi(2) + gradient_y.powi(2)).sqrt()
}

fn angle_to_direction(theta: f32) -> Direction {
    let theta = theta.abs();
    if theta < PI / 8.0 {
        Direction::Horizontal
    } else if theta < 3.0 * PI / 8.0 {
        Direction::Diagonal45
    } else if theta < 5.0 * PI / 8.0 {
        Direction::Vertical
    } else if theta < 7.0 * PI / 8.0 {
        Direction::Diagonal135
    } else {
        Direction::Horizontal
    }
}

/*
    Compare the edge strength of the current pixel with the edge strength of the pixel in the positive and negative gradient directions.
    If the edge strength of the current pixel is the largest compared to the other pixels in the mask with the same direction
    (e.g., a pixel that is pointing in the y-direction will be compared to the pixel above and below it in the vertical axis),
    the value will be preserved.
    Otherwise, the value will be suppressed.
 */
pub(crate) fn suppress(x_gradient: &[Vec<f32>], y_gradient: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let rows = x_gradient.len();
    let cols = x_gradient[0].len();
    let mut suppressed_image = vec![vec![0.0; cols]; rows];

    suppressed_image.par_iter_mut().enumerate().for_each(|(i, row)| {
        if i == 0 || i == rows - 1 { return; } // Skip the first and last row

        for j in 1..(cols - 1) {
            let angle = direction(x_gradient[i][j], y_gradient[i][j]);
            let gradient = hypot(x_gradient[i][j], y_gradient[i][j]);

            let (i_offset, j_offset) = match angle_to_direction(angle) {
                Direction::Horizontal => (0, 1),
                Direction::Vertical => (1, 0),
                Direction::Diagonal45 => (1, 1),
                Direction::Diagonal135 => (1, -1),
            };

            let i_pos = i.checked_add(i_offset);
            let i_neg = i.checked_sub(i_offset);
            let j_pos = j.checked_add(j_offset as usize);
            let j_neg = j.checked_sub(j_offset as usize);

            if let (Some(i_pos), Some(j_pos), Some(i_neg), Some(j_neg)) = (i_pos, j_pos, i_neg, j_neg) {
                if gradient > hypot(x_gradient[i_neg][j_neg], y_gradient[i_neg][j_neg]) &&
                    gradient > hypot(x_gradient[i_pos][j_pos], y_gradient[i_pos][j_pos]) {
                    row[j] = gradient;
                }
            }
        }
    });

    suppressed_image
}