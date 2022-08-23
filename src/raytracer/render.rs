use rand::Rng;

use crate::{
    bvh::bbox_tree::BboxTreeWorkspace,
    camera::{Camera, CameraPosition},
    core::{Color, Ray},
    material::Material,
    scene::Scene,
};

pub struct Frame<'a> {
    pub camera: &'a Camera,
    pub pos: &'a CameraPosition,
    pub scene: &'a Scene,
}

fn ray_color<R: Rng>(
    rng: &mut R,
    hit_stack: &mut BboxTreeWorkspace,
    incoming: &Ray,
    scene: &Scene,
    mut max_depth: usize,
) -> Color {
    let mut ray = *incoming;
    let mut attenuation = Color::ones();
    let mut emitted = Color::default();

    let mut workspace = scene.workspace_scene(hit_stack);

    while max_depth > 0 {
        if let Some((obj, r)) = workspace.hit_workspace(&ray, 0.001, std::f64::INFINITY) {
            if let Some(e) = obj.material.emitted(&ray, &r) {
                emitted += Color(attenuation.0 * e.0);
            }
            if let Some(scatter) = obj.material.scatter(rng, &ray, &r) {
                attenuation = Color(attenuation.0 * scatter.attenuation.0);
                ray = scatter.direction;
            } else {
                break;
            }
        } else {
            emitted += Color(attenuation.0 * scene.skybox.background(&ray).0);
            break;
        }
        max_depth -= 1
    }
    emitted
}
pub fn render_scanline<R: Rng>(
    frame: &Frame<'_>,
    rng: &mut R,
    samples: usize,
    max_depth: usize,
    hit_stack: &mut BboxTreeWorkspace,
    line_idx: usize,
    buf: &mut [Color],
) {
    for (idx, buf_c) in buf.iter_mut().enumerate() {
        let mut c = Color::default();
        for _ in 0..samples {
            let jitter_idx = idx as f64 + rng.gen::<f64>();
            let jitter_line_idx = line_idx as f64 + rng.gen::<f64>();
            let r = frame
                .camera
                .pixel_ray(rng, frame.pos, jitter_idx, jitter_line_idx);
            c += ray_color(rng, hit_stack, &r, frame.scene, max_depth);
        }
        *buf_c = c
    }
}
