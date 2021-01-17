use nalgebra::Point3;

pub enum EntityType {
    Ball { point: Point3<f32>, radius: f32 },
    Ground { thickness: f32 },
}
