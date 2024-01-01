use ccm_impl::color::srgb_average;
use ccm_impl::PerspectiveGridIterator;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb};

pub use ccm_impl::color::SRgbColor;
pub use ccm_impl::{color_reference_charts, ColorCorrectionMatrix};

pub fn calculate_ccm<I>(
    image: &I,
    corner_points: &[(f64, f64); 4],
    grid_resolution: (usize, usize),
    sampling_radius: usize,
    reference_colors: &[SRgbColor],
) -> Option<ColorCorrectionMatrix>
where
    I: GenericImageView<Pixel = Rgb<u8>>,
{
    let (grid_resolution_x, grid_resolution_y) = grid_resolution;
    assert_eq!(reference_colors.len(), grid_resolution_x * grid_resolution_y);

    // Cannot use `GenericImageView::in_bounds` with float and negative coordinates
    // that `PerspectiveGridIterator` may emit.
    #[rustfmt::skip]
    let in_bounds = |x, y| {
        (x >= 0.0) && (x < image.width() as f64) &&
        (y >= 0.0) && (y < image.height() as f64)
    };

    let image_colors = PerspectiveGridIterator::new(corner_points, grid_resolution)?
        .map(|(grid_x, grid_y)| {
            if !in_bounds(grid_x, grid_y) {
                return None;
            }

            let mut sampled_colors = Vec::new();

            for sample_dy in -(sampling_radius as isize)..=(sampling_radius as isize) {
                for sample_dx in -(sampling_radius as isize)..=(sampling_radius as isize) {
                    let sample_rx = (sample_dx as f64) / (sampling_radius as f64);
                    let sample_ry = (sample_dy as f64) / (sampling_radius as f64);

                    if sample_rx * sample_rx + sample_ry * sample_ry > 1.0 {
                        continue;
                    }

                    let sample_x = (grid_x as isize + sample_dx) as u32;
                    let sample_y = (grid_y as isize + sample_dy) as u32;

                    if in_bounds(sample_x as f64, sample_y as f64) {
                        let sampled_color = image.get_pixel(sample_x, sample_y).0;
                        sampled_colors.push(sampled_color);
                    }
                }
            }

            srgb_average(&sampled_colors)
        })
        .collect::<Option<Vec<SRgbColor>>>()?;

    assert_eq!(reference_colors.len(), image_colors.len());
    let matrix = ccm_impl::calculate_ccm(&image_colors, reference_colors);

    Some(matrix)
}

pub fn apply_ccm<I>(image: &I, matrix: &ColorCorrectionMatrix) -> ImageBuffer<Rgb<u8>, Vec<u8>>
where
    I: GenericImageView<Pixel = Rgb<u8>>,
{
    let (width, height) = image.dimensions();
    let mut result = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let src_color = image.get_pixel(x, y);
            let dst_color = Rgb(ccm_impl::apply_ccm(&src_color.0, matrix));
            result.put_pixel(x, y, dst_color);
        }
    }

    result
}

pub fn apply_ccm_in_place<I>(image: &mut I, matrix: &ColorCorrectionMatrix)
where
    I: GenericImage<Pixel = Rgb<u8>>,
{
    let (width, height) = image.dimensions();

    for y in 0..height {
        for x in 0..width {
            let src_color = image.get_pixel(x, y);
            let dst_color = Rgb(ccm_impl::apply_ccm(&src_color.0, matrix));
            image.put_pixel(x, y, dst_color);
        }
    }
}
