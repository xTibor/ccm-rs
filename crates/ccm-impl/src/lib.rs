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
//!
//! * New color specifications for ColorChecker SG and Classic Charts
//!   X-Rite, Incorporated
//!   https://www.xrite.com/service-support/new_color_specifications_for_colorchecker_sg_and_classic_charts

mod color;
mod color_correction_matrix;
mod perspective_grid_iterator;

pub use color::SRgbColor;
pub use color_correction_matrix::{apply_ccm, calculate_ccm, ColorCorrectionMatrix};
pub use perspective_grid_iterator::PerspectiveGridIterator;

pub mod color_reference_charts;
