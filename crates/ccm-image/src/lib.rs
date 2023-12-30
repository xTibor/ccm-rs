use ccm_impl::PerspectiveGridIterator;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb};

pub use ccm_impl::{color_reference_charts, ColorCorrectionMatrix, SRgbColor};

pub fn calculate_ccm<I>(
    image: &I,
    corner_points: &[(f64, f64); 4],
    grid_resolution: (usize, usize),
    reference_colors: &[SRgbColor],
) -> ColorCorrectionMatrix
where
    I: GenericImageView<Pixel = Rgb<u8>>,
{
    let (grid_resolution_x, grid_resolution_y) = grid_resolution;
    assert_eq!(reference_colors.len(), grid_resolution_x * grid_resolution_y);

    let image_colors = PerspectiveGridIterator::new(corner_points, grid_resolution)
        .unwrap()
        .map(|(x, y)| {
            // TODO: Sample average N-pixel neighborhood
            image.get_pixel(x as u32, y as u32).0
        })
        .collect::<Vec<_>>();

    ccm_impl::calculate_ccm(&image_colors, reference_colors)
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
