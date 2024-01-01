use ccm_image::SRgbColor;

use ccm_impl::color_reference_charts::XRITE_COLORCHECKER_CLASSIC_2014;
use ccm_impl::ColorCorrectionMatrix;
use eframe::egui::{self, DragValue};
use eframe::epaint::Vec2;

struct ColorCorrectionApp {
    corner_points: Vec<(f64, f64)>,
    grid_resolution: (usize, usize),
    sampling_radius: usize,
    reference_colors: Vec<SRgbColor>,
    color_correction_matrix: ColorCorrectionMatrix,
}

impl Default for ColorCorrectionApp {
    fn default() -> Self {
        ColorCorrectionApp {
            corner_points: vec![(100.0, 100.0), (200.0, 100.0), (200.0, 200.0), (100.0, 200.0)],
            grid_resolution: (6, 4),
            sampling_radius: 1,
            reference_colors: XRITE_COLORCHECKER_CLASSIC_2014.to_vec(),
            color_correction_matrix: ColorCorrectionMatrix::default(),
        }
    }
}

impl eframe::App for ColorCorrectionApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.style_mut().spacing.item_spacing = Vec2::new(3.0, 3.0);
                ui.style_mut().spacing.interact_size = Vec2::new(32.0, 32.0);
                // TODO: Color button border styling is broken

                let (grid_resolution_x, grid_resolution_y) = self.grid_resolution;
                assert_eq!(grid_resolution_x * grid_resolution_y, self.reference_colors.len());

                for y in 0..grid_resolution_y {
                    ui.horizontal(|ui| {
                        for x in 0..grid_resolution_x {
                            let color_index = y * grid_resolution_x + x;
                            ui.color_edit_button_srgb(&mut self.reference_colors[color_index]);
                            // TODO: Color cursor needs to be clipped to the color area
                            // TODO: Color popup needs hex edit at least in sRGB mode
                        }
                    });
                }
            });

            ui.group(|ui| {
                for y in 0..3 {
                    ui.horizontal(|ui| {
                        for x in 0..3 {
                            let value = self.color_correction_matrix.get_mut(y * 3 + x).unwrap();
                            ui.add(DragValue::new(value).speed(0.01));
                        }
                    });
                }
            })
        });

        egui::Window::new("\u{1F527} Settings")
            .default_pos((256.0, 16.0))
            .default_open(false) // TODO: This should be called `.default_collapsed()`
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native("ccm-egui", options, Box::new(|_| Box::<ColorCorrectionApp>::default()))
}
