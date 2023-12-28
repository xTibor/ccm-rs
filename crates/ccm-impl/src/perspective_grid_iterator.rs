use nalgebra::{Matrix3, RowVector3};

#[derive(Debug)]
pub enum PerspectiveGridError {
    IncorrectParameters,
}

#[derive(Debug)]
pub struct PerspectiveGridIterator {
    transform: Matrix3<f64>,
    resolution: (usize, usize),
    next_point_index: usize,
}

impl PerspectiveGridIterator {
    #[rustfmt::skip]
    pub fn new(
        points: [(f64, f64); 4],
        resolution: (usize, usize),
    ) -> Result<PerspectiveGridIterator, PerspectiveGridError> {
        let transform = {
            let [(x0, y0), (x1, y1), (x2, y2), (x3, y3)] = points;

            if ((x0 - x1 + x2 - x3) == 0.0) && ((y0 - y1 + y2 - y3) == 0.0) {
                Matrix3::new(
                    x1 - x0, y1 - y0, 0.0,
                    x2 - x1, y2 - y1, 0.0,
                    x0,      y0,      1.0,
                )
            } else {
                if (((x1 - x2) * (y3 - y2)) - ((x3 - x2) * (y1 - y2))) == 0.0 {
                    return Err(PerspectiveGridError::IncorrectParameters);
                }

                Matrix3::new(
                    x1 - x0 + (((((x0 - x1 + x2 - x3) * (y3 - y2)) - ((y0 - y1 + y2 - y3) * (x3 - x2))) / (((x1 - x2) * (y3 - y2)) - ((x3 - x2) * (y1 - y2)))) * x1),
                    y1 - y0 + (((((x0 - x1 + x2 - x3) * (y3 - y2)) - ((y0 - y1 + y2 - y3) * (x3 - x2))) / (((x1 - x2) * (y3 - y2)) - ((x3 - x2) * (y1 - y2)))) * y1),
                                (((x0 - x1 + x2 - x3) * (y3 - y2)) - ((y0 - y1 + y2 - y3) * (x3 - x2))) / (((x1 - x2) * (y3 - y2)) - ((x3 - x2) * (y1 - y2))),
                    x3 - x0 + (((((y0 - y1 + y2 - y3) * (x1 - x2)) - ((x0 - x1 + x2 - x3) * (y1 - y2))) / (((x1 - x2) * (y3 - y2)) - ((x3 - x2) * (y1 - y2)))) * x3),
                    y3 - y0 + (((((y0 - y1 + y2 - y3) * (x1 - x2)) - ((x0 - x1 + x2 - x3) * (y1 - y2))) / (((x1 - x2) * (y3 - y2)) - ((x3 - x2) * (y1 - y2)))) * y3),
                                (((y0 - y1 + y2 - y3) * (x1 - x2)) - ((x0 - x1 + x2 - x3) * (y1 - y2))) / (((x1 - x2) * (y3 - y2)) - ((x3 - x2) * (y1 - y2))),
                    x0,
                    y0,
                    1.0,
                )
            }
        };

        Ok(PerspectiveGridIterator {
            transform,
            resolution,
            next_point_index: 0,
        })
    }
}

impl Iterator for PerspectiveGridIterator {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let (resolution_x, resolution_y) = self.resolution;

        if self.next_point_index >= resolution_x * resolution_y {
            None
        } else {
            let point_index = self.next_point_index;
            self.next_point_index += 1;

            let (index_x, index_y) = (point_index % resolution_x, point_index / resolution_x);

            let [grid_x, grid_y, grid_z]: [f64; 3] = (RowVector3::from([
                index_x as f64 / (resolution_x - 1) as f64,
                index_y as f64 / (resolution_y - 1) as f64,
                1.0,
            ]) * self.transform)
                .into();

            Some((grid_x / grid_z, grid_y / grid_z))
        }
    }
}
