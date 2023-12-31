use nalgebra::{Dyn, Matrix3, OMatrix, OVector, RowVector3, U3};

use crate::color::{linear_to_srgb, srgb_to_linear, SRgbColor};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

const CCM_MAX_ITERATIONS: usize = 128;

const CCM_CONVERGENCE_THRESHOLD: f64 = 0.00001;

const CCM_ERROR_EPSILON: f64 = 0.1;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type ColorCorrectionMatrix = Matrix3<f64>;

pub fn calculate_ccm(image_colors: &[SRgbColor], reference_colors: &[SRgbColor]) -> ColorCorrectionMatrix {
    type CMatrix = OMatrix<f64, Dyn, U3>;
    type EMatrix = OMatrix<f64, Dyn, Dyn>;
    type EDiagonal = OVector<f64, Dyn>;

    let colors_to_matrix = |colors: &[SRgbColor]| {
        CMatrix::from_rows(
            &colors
                .iter()
                .map(srgb_to_linear)
                .map(RowVector3::from)
                .collect::<Vec<_>>(),
        )
    };

    let matrix_image_colors = colors_to_matrix(image_colors);
    let matrix_reference_colors = colors_to_matrix(reference_colors);

    let mut matrix = (matrix_image_colors.transpose() * &matrix_image_colors)
        .try_inverse()
        .unwrap()
        * matrix_image_colors.transpose()
        * &matrix_reference_colors;

    for _ in 0..CCM_MAX_ITERATIONS {
        let cost_matrix = {
            let cost_vector = {
                let iter_image_colors = image_colors.iter().map(srgb_to_linear).map(RowVector3::from);
                let iter_reference_colors = reference_colors.iter().map(srgb_to_linear).map(RowVector3::from);

                iter_image_colors
                    .zip(iter_reference_colors)
                    .map(|(color_detected, color_reference)| {
                        color_reference.metric_distance(&(color_detected * matrix))
                    })
                    .map(|error_value| 1.0 / (error_value + CCM_ERROR_EPSILON))
                    .collect::<Vec<_>>()
            };

            let processed_cost_vector = {
                let cost_sum = cost_vector
                    .iter()
                    .map(|cost_value| cost_value.powf(2.0))
                    .sum::<f64>()
                    .sqrt();

                cost_vector
                    .iter()
                    .map(|cost_value| (cost_value / cost_sum).powf(2.0))
                    .collect::<Vec<_>>()
            };

            EMatrix::from_diagonal(&EDiagonal::from(processed_cost_vector))
        };

        let prev_matrix = matrix;

        matrix = (matrix_image_colors.transpose() * &cost_matrix * &matrix_image_colors)
            .try_inverse()
            .unwrap()
            * matrix_image_colors.transpose()
            * &cost_matrix
            * &matrix_reference_colors;

        {
            let matrix_convergence = (prev_matrix - matrix).abs();

            if matrix_convergence.iter().all(|diff| *diff < CCM_CONVERGENCE_THRESHOLD) {
                break;
            }
        }
    }

    matrix
}

pub fn apply_ccm(srgb_color: &SRgbColor, matrix: &ColorCorrectionMatrix) -> SRgbColor {
    let linear_color = srgb_to_linear(srgb_color);
    let corrected_color = RowVector3::from(linear_color) * matrix;
    linear_to_srgb(&corrected_color.into())
}
