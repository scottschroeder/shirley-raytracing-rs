use rand::{rngs::ThreadRng, Rng};

use super::texture::Texture;
use crate::util::{Color, Point, Vec3};

const PERLIN_POINT_COUNT: usize = 256;
const C_SIZE: usize = 2;
const KERNEL_SIZE: usize = C_SIZE * C_SIZE * C_SIZE;

#[derive(Debug)]
struct Perlin {
    ranfloat: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

#[derive(Default)]
struct InterpolationKernel {
    kernel: [Vec3; KERNEL_SIZE],
}

fn kidx(i: usize, j: usize, k: usize) -> usize {
    i * C_SIZE * C_SIZE + j * C_SIZE + k
}

impl InterpolationKernel {
    fn build<F: Fn(usize, usize, usize) -> Vec3>(f: F) -> Self {
        let mut kernel = InterpolationKernel::default();
        for di in 0..C_SIZE {
            for dj in 0..C_SIZE {
                for dk in 0..C_SIZE {
                    kernel.kernel[kidx(di, dj, dk)] = f(di, dj, dk);
                }
            }
        }
        kernel
    }

    fn interp(&self, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        for di in 0..C_SIZE {
            let i = di as f64;
            for dj in 0..C_SIZE {
                let j = dj as f64;
                for dk in 0..C_SIZE {
                    let k = dk as f64;
                    let weight = Vec3::new(u - i, v - j, w - k);

                    accum += (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * self.kernel[kidx(di, dj, dk)].dot(&weight);
                }
            }
        }
        accum
    }
}

impl Perlin {
    fn new() -> Perlin {
        let mut rng = rand::thread_rng();
        let ranfloat = (0..PERLIN_POINT_COUNT)
            .map(|_| Vec3::random_range_with_rng(&mut rng, -1.0, 1.0))
            .collect::<Vec<_>>();

        Perlin {
            ranfloat,
            perm_x: perlin_generate_perm(&mut rng),
            perm_y: perlin_generate_perm(&mut rng),
            perm_z: perlin_generate_perm(&mut rng),
        }
    }

    fn noise(&self, p: Point) -> f64 {
        // p_to_idx(p);
        let xf = p.0.x().floor();
        let yf = p.0.y().floor();
        let zf = p.0.z().floor();

        let u = p.0.x() - xf;
        let v = p.0.y() - yf;
        let w = p.0.z() - zf;

        // Capture the negative floats with twos complmement, before casting to usize
        // this way -1.0 -> -1 -> 255
        let i = xf as i32 as usize;
        let j = yf as i32 as usize;
        let k = zf as i32 as usize;

        let kernel = InterpolationKernel::build(|di, dj, dk| {
            let idx = self.perm_x[(i + di) & 0xFF]
                ^ self.perm_y[(j + dj) & 0xFF]
                ^ self.perm_z[(k + dk) & 0xFF];
            self.ranfloat[idx]
        });
        kernel.interp(u, v, w)
    }
}

fn perlin_generate_perm(rng: &mut ThreadRng) -> Vec<usize> {
    let mut p = (0..PERLIN_POINT_COUNT)
        // .map(|idx| idx as u32)
        .collect::<Vec<_>>();
    permute(rng, &mut p);
    p
}

fn permute<T>(rng: &mut ThreadRng, p: &mut Vec<T>) {
    for idx in (1..p.len()).rev() {
        let target = rng.gen_range(0..idx + 1);
        p.swap(idx, target)
    }
}

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn scale(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::scale(1.0)
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point) -> crate::util::Color {
        let scaled = Point(p.0.scale(self.scale));
        let noise = 0.5 * (1.0 + self.noise.noise(scaled));
        Color(Vec3::new(1.0, 1.0, 1.0).scale(noise))
    }
}
