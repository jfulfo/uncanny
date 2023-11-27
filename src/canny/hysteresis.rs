/*
    Edge hysteresis for Canny edge detection.
 */

use crate::canny::{STRONG_EDGE, WEAK_EDGE};

#[inline]
fn connected_to_strong_edge(matrix: &Vec<Vec<f32>>, i: usize, j: usize) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();
    if i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
        return false;
    }

    let mut connected = false;
    for x in i - 1..i + 2 {
        for y in j - 1..j + 2 {
            if matrix[x][y] == STRONG_EDGE {
                connected = true;
            }
        }
    }

    connected

}

pub(crate) fn hysteresis(input_matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let rows = input_matrix.len();
    let cols = input_matrix[0].len();
    let mut output_matrix = input_matrix.clone();

    for i in 0..rows {
        for j in 0..cols {
            if output_matrix[i][j] == WEAK_EDGE {
                if !connected_to_strong_edge(&input_matrix, i, j) {
                    output_matrix[i][j] = 0.0;
                }
            }
        }
    }

    output_matrix
}