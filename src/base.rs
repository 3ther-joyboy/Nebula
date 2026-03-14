pub mod sircle;

pub struct Math;
impl Math {
    pub fn distance(point_a: &[f32;2],point_b: &[f32;2]) -> f32 {
        let a = (point_a[0]-point_b[0]).abs();
        let b = (point_a[1]-point_b[1]).abs();
        (a*a+b*b).sqrt()
    }
    pub fn add_vec(a: &[f32;2],b: &[f32;2]) -> [f32;2] {
        [a[0] + b[0],a[1] + b[1]]
    }
}
