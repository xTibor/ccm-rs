use ccm_image::color_reference_charts::XRITE_COLORCHECKER_CLASSIC_2014;

fn main() {
    let image = image::open("test/test-01.png").unwrap().to_rgb8();

    let corner_points = [(82.0, 83.0), (552.0, 91.0), (543.0, 359.0), (83.0, 358.0)];
    let grid_resolution = (6, 4);

    let color_correction_matrix = ccm_image::calculate_ccm(
        &image,
        &corner_points,
        grid_resolution,
        &XRITE_COLORCHECKER_CLASSIC_2014,
    );

    let output_image = ccm_image::apply_ccm(&image, &color_correction_matrix);
    output_image.save("test/output.png").unwrap();
}
