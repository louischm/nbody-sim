use nalgebra::Vector3;

pub struct Node {
    pub center: Vector3<f64>,
    pub half_size: f64,

    pub mass: f64,
    pub center_of_mass: Vector3<f64>,

    pub body: Option<usize>,
    pub children: Option<[Box<Node>; 4]>,
}