//! A color correction crate implementing the robust least-squares algorithm.
//!
//! References:
//! * NTIA Technical Memorandum TM-04-406
//!   Color Correction Matrix for Digital Still and Video Imaging Systems
//!   Stephen Wolf, 2003
//!   https://its.ntia.gov/umbraco/surface/download/publication?reportNumber=TM-04-406.pdf
//!
//! * A Sample-Based Color Correction Method for Laparoscopic Images
//!   Longfei Wang, Qi Li, Haozhe Yang, Jia Huang, Kai Xu, 2020
//!   https://www.researchgate.net/publication/348091082_A_Sample-Based_Color_Correction_Method_for_Laparoscopic_Images
//!
//! * Average measurement of all ColorChecker Charts
//!   X­-Rite Incorporated, 2009
//!   https://xritephoto.com/documents/literature/en/ColorData-1p_EN.pdf
//!
//! * SpyderCheckr Color Data
//!   Datacolor, 2016
//!   https://web.archive.org/web/20171116080802/http://www.datacolor.com/wp-content/uploads/2016/08/SpyderCheckr_Color_Data.pdf

use nalgebra::{Dyn, Matrix3, OMatrix, OVector, RowVector3, U3};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type SRgbColor = [u8; 3];

fn srgb_to_linear(srgb_color: &SRgbColor) -> LinearColor {
    srgb_color.map(|value| {
        let value = (value as f64) / 255.0;

        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    })
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

type LinearColor = [f64; 3];

fn linear_to_srgb(linear_color: &LinearColor) -> SRgbColor {
    linear_color.map(|value| {
        let value = if value <= 0.0031308 {
            value * 12.92
        } else {
            1.055 * value.powf(1.0 / 2.4) - 0.055
        };

        (value.clamp(0.0, 1.0) * 255.0) as u8
    })
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[rustfmt::skip]
pub const COLOR_REFERENCE_CHARTS: &[(&str, (usize, usize), &[SRgbColor])] = &[
    ("xrite",        (6, 4), &COLORS_XRITE_COLORCHECKER    ),
    ("charttu",      (6, 4), &COLORS_CHARTTU_COLORCHECKER  ),
    ("spydercheckr", (8, 6), &COLORS_DATACOLOR_SPYDERCHECKR),
];

#[rustfmt::skip]
pub const COLORS_XRITE_COLORCHECKER: [SRgbColor; 24] = [
    [115,  82,  68], [194, 150, 130], [ 98, 122, 157], [ 87, 108,  67], [133, 128, 177], [103, 189, 170],
    [214, 126,  44], [ 80,  91, 166], [193,  90,  99], [ 94,  60, 108], [157, 188,  64], [224, 163,  46],
    [ 56,  61, 150], [ 70, 148,  73], [175,  54,  60], [231, 199,  31], [187,  86, 149], [  8, 133, 161],
    [243, 243, 242], [200, 200, 200], [160, 160, 160], [122, 122, 121], [ 85,  85,  85], [ 52,  52,  52],
];

#[rustfmt::skip]
pub const COLORS_CHARTTU_COLORCHECKER: [SRgbColor; 24] = [
    [117,  82,  68], [198, 144, 129], [ 93, 121, 157], [ 92, 109,  65], [133, 128, 177], [ 97, 189, 172],
    [230, 129,  55], [ 73,  91, 167], [195,  81,  97], [ 96,  60, 106], [161, 189,  64], [227, 161,  37],
    [ 39,  61, 144], [ 65, 150,  75], [181,  55,  60], [236, 200,  25], [192,  82, 149], [  1, 134, 166],
    [241, 242, 236], [201, 203, 203], [161, 163, 163], [120, 120, 121], [ 86,  86,  87], [ 50,  51,  52],
];

// sRGB channel values for Patch 2G (Blueprint) are incorrect in the Datacolor docs.
// I fixed it by converting the corresponding Lab color into sRGB for that color patch.
#[rustfmt::skip]
pub const COLORS_DATACOLOR_SPYDERCHECKR: [SRgbColor; 48] = [
    [210, 121, 117], [218, 203, 201], [237, 206, 186], [241, 233, 229], [249, 242, 238], [  0, 127, 159], [222, 118,  32], [ 98, 187, 166],
    [216, 179,  90], [203, 205, 196], [211, 175, 133], [229, 222, 220], [202, 198, 195], [192,  75, 145], [ 58,  88, 159], [126, 125, 174],
    [127, 175, 120], [206, 203, 208], [193, 149,  91], [182, 178, 176], [161, 157, 154], [245, 205,   0], [195,  79,  95], [ 82, 106,  60],
    [ 66, 157, 179], [ 66,  57,  58], [139,  93,  61], [139, 136, 135], [122, 118, 116], [186,  26,  51], [ 83,  58, 106], [ 87, 120, 155],
    [116, 147, 194], [ 54,  61,  56], [ 74,  55,  46], [100,  99,  97], [ 80,  80,  78], [ 57, 146,  64], [157, 188,  54], [197, 145, 125],
    [190, 121, 154], [ 63,  60,  69], [ 57,  54,  56], [ 63,  61,  62], [ 43,  41,  43], [ 25,  55, 135], [238, 158,  25], [112,  76,  60],
];

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

const CCM_MAX_ITERATIONS: usize = 128;

const CCM_CONVERGENCE_THRESHOLD: f64 = 0.00001;

const CCM_ERROR_EPSILON: f64 = 0.1;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

pub type ColorCorrectionMatrix = Matrix3<f64>;

pub fn calculate_ccm(colors_detected: &[SRgbColor], colors_reference: &[SRgbColor]) -> ColorCorrectionMatrix {
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

    let matrix_detected = colors_to_matrix(colors_detected);
    let matrix_reference = colors_to_matrix(colors_reference);

    let mut ccm_matrix = (matrix_detected.transpose() * &matrix_detected).try_inverse().unwrap()
        * matrix_detected.transpose()
        * &matrix_reference;

    for _ in 0..CCM_MAX_ITERATIONS {
        let cost_matrix = {
            let cost_vector = {
                let iter_detected = colors_detected.iter().map(srgb_to_linear).map(RowVector3::from);
                let iter_reference = colors_reference.iter().map(srgb_to_linear).map(RowVector3::from);

                iter_detected
                    .zip(iter_reference)
                    .map(|(color_detected, color_reference)| {
                        color_reference.metric_distance(&(color_detected * ccm_matrix))
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

        let prev_ccm_matrix = ccm_matrix;

        ccm_matrix = (matrix_detected.transpose() * &cost_matrix * &matrix_detected)
            .try_inverse()
            .unwrap()
            * matrix_detected.transpose()
            * &cost_matrix
            * &matrix_reference;

        {
            let ccm_convergence = (prev_ccm_matrix - ccm_matrix).abs();

            if ccm_convergence.iter().all(|diff| *diff < CCM_CONVERGENCE_THRESHOLD) {
                break;
            }
        }
    }

    ccm_matrix
}

pub fn apply_ccm(srgb_color: &SRgbColor, ccm_matrix: &ColorCorrectionMatrix) -> SRgbColor {
    let linear_color = srgb_to_linear(srgb_color);
    let corrected_color = RowVector3::from(linear_color) * ccm_matrix;
    linear_to_srgb(&corrected_color.into())
}
