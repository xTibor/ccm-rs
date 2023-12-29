use ccm_impl::color_reference_charts::COLOR_REFERENCE_CHARTS;

fn main() {
    for (ref_name, (ref_width, ref_height), ref_colors) in COLOR_REFERENCE_CHARTS {
        println!("{ref_name:}");
        for y in 0..*ref_height {
            for x in 0..*ref_width {
                let [r, g, b] = ref_colors[y * ref_width + x];
                print!("#{:02X}{:02X}{:02X} ", r, g, b);
            }
            println!();
        }
        println!();
    }
}
