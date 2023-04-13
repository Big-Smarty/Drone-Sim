use crate::*;

#[derive(Component)]
pub struct Drone {
    pub position: na::Vector3<f32>,
    pub rotation: na::Vector3<f32>,
    pub velocity: na::Vector3<f32>,
    pub center_of_mass: na::Vector3<f32>,
    pub center_of_lift: na::Vector3<f32>,
    pub lift_area: f32,
}

struct Body {
    pub center_of_gravity: na::Vector3<f32>,
    pub center_of_lift: na::Vector3<f32>,
    pub lift_area: f32,
    pub model: std::path::Path,
}

struct Flap {
    pub relative_position: na::Vector3<f32>,
    pub relative_rotation: na::Vector3<f32>,
    pub invert: [bool; 3],
    pub center_of_gravity: na::Vector3<f32>,
    pub center_of_lift: na::Vector3<f32>,
    pub lift_area: f32,
    pub model: std::path::Path,
}

struct Engine {
    pub relative_position: na::Vector3<f32>,
    pub invert: [bool; 3],
    pub center_of_gravity: na::Vector3<f32>,
    pub thrust: f32,
    pub model: std::path::Path,
}
