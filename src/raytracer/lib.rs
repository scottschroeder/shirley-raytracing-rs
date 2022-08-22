pub mod image;
pub mod core {
    mod color;
    pub mod fp;
    pub mod math;
    mod vec3;

    pub use color::Color;
    pub use vec3::{Point, Ray, Vec3, EACH_DIMM};
}

pub mod camera;
pub mod scene;

pub mod skybox;

pub mod geometry {
    pub mod hittable;
    pub mod object;
    pub mod rect;
    pub mod sphere;
}

pub mod bvh {
    pub mod aabb;
    pub mod bbox_tree;
}

pub mod material;
