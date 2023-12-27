pub type SRgbColor = [u8; 3];

pub(crate) fn srgb_to_linear(srgb_color: &SRgbColor) -> LinearColor {
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

pub(crate) fn linear_to_srgb(linear_color: &LinearColor) -> SRgbColor {
    linear_color.map(|value| {
        let value = if value <= 0.0031308 {
            value * 12.92
        } else {
            1.055 * value.powf(1.0 / 2.4) - 0.055
        };

        (value.clamp(0.0, 1.0) * 255.0) as u8
    })
}
