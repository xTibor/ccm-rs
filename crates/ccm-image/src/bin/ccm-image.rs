use ccm_impl::color_reference_charts::XRITE_COLORCHECKER_CLASSIC_2014;
use ccm_impl::{apply_ccm, calculate_ccm, PerspectiveGridIterator};

fn main() {
    let image = image::open("test/test-01.png").unwrap().to_rgb8();
    let mut output_image = image.clone();

    let colors_detected =
        PerspectiveGridIterator::new([(82.0, 83.0), (552.0, 91.0), (543.0, 359.0), (83.0, 358.0)], (6, 4))
            .unwrap()
            .map(|(x, y)| {
                // TODO: Sample average N-pixel neighborhood
                output_image.get_pixel(x as u32, y as u32).0
            })
            .collect::<Vec<_>>();

    let ccm_matrix = calculate_ccm(&colors_detected, &XRITE_COLORCHECKER_CLASSIC_2014);

    output_image.pixels_mut().for_each(|pixel| {
        pixel.0 = apply_ccm(&pixel.0, &ccm_matrix);
    });

    output_image.save("test/output.png").unwrap();
}
