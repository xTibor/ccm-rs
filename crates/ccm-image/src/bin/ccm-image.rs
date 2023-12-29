use ccm_impl::color_reference_charts::XRITE_COLORCHECKER_CLASSIC_2014;
use ccm_impl::{apply_ccm, calculate_ccm, PerspectiveGridIterator};

fn main() {
    #[rustfmt::skip]
    let colors_detected = [
        [ 92,  56,  31], [161, 101,  63], [ 86,  80,  75], [ 78,  70,  28], [107,  81,  79], [ 90, 112,  77],
        [173,  91,  21], [ 70,  64,  85], [158,  69,  47], [ 79,  46,  48], [124, 117,   9], [165, 102,   8],
        [ 46,  47,  70], [ 71,  98,  29], [142,  54,  31], [184, 131,   1], [146,  66,  68], [ 47,  81,  75],
        [191, 156, 114], [160, 132,  93], [130, 107,  77], [ 97,  78,  53], [ 67,  55,  37], [ 44,  34,  20],
    ];

    let ccm_matrix = calculate_ccm(&colors_detected, &XRITE_COLORCHECKER_CLASSIC_2014);
    println!("{ccm_matrix:}");

    println!("DETECT  | CORR    | REF");

    for (color_detected, color_reference, color_corrected) in colors_detected
        .iter()
        .zip(XRITE_COLORCHECKER_CLASSIC_2014)
        .map(|(color_detected, color_reference)| {
            (color_detected, color_reference, apply_ccm(color_detected, &ccm_matrix))
        })
    {
        #[rustfmt::skip]
        println!(
            "#{:02X}{:02X}{:02X} | #{:02X}{:02X}{:02X} | #{:02X}{:02X}{:02X}",
            color_detected[0], color_detected[1], color_detected[2],
            color_corrected[0], color_corrected[1], color_corrected[2],
            color_reference[0], color_reference[1], color_reference[2],
        );
    }

    for (x, y) in
        PerspectiveGridIterator::new([(13.0, 178.0), (270.0, 207.0), (148.0, 452.0), (15.0, 349.0)], (7, 5)).unwrap()
    {
        println!("{x:}, {y:}");
    }
}
