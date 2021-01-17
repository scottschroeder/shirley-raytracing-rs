use crate::util::{Point, Ray, Vec3};
use anyhow::Result;

const DEFAULT_FOCAL_LENGTH: f64 = 1.0;

#[inline]
fn degrees_to_radians(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

#[derive(Default)]
pub struct CameraBuilder {
    height: Option<usize>,
    width: Option<usize>,
    focal_length: Option<f64>,
    vfov: Option<f64>,
    ratio: Option<AspectRatio>,
}

impl CameraBuilder {
    pub fn vfov(&mut self, vfov: f64) -> &mut Self {
        self.vfov = Some(vfov);
        self
    }
    pub fn focal_length(&mut self, focal_length: f64) -> &mut Self {
        self.focal_length = Some(focal_length);
        self
    }
    pub fn aspect_ratio<A: Into<AspectRatio>>(&mut self, ratio: A) -> &mut Self {
        self.ratio = Some(ratio.into());
        self
    }
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }
    pub fn build(self) -> Result<Camera> {
        let (dimm, ratio) = Dimmensions::from_two_of_three(self.height, self.width, self.ratio)?;
        let theta = degrees_to_radians(self.vfov.unwrap_or(DEFAULT_FOCAL_LENGTH));
        let h = (theta / 2.0).tan();
        let height = 2.0 * h;
        // let height = 1.0;
        let width = ratio.as_float() * height;

        Ok(Camera {
            height,
            width,
            focal_length: self.focal_length.unwrap_or(DEFAULT_FOCAL_LENGTH),
            dimm,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CameraPosition {
    origin: Point,
    w: Vec3,
    u: Vec3,
    v: Vec3,
}

impl CameraPosition {
    pub fn look_at(camera: Point, target: Point, up: Vec3) -> CameraPosition {
        let w = (camera.0 - target.0).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);
        CameraPosition {
            origin: camera,
            w,
            u,
            v,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    height: f64,
    width: f64,
    focal_length: f64,
    pub dimm: Dimmensions,
}

impl Camera {
    pub fn pixel_ray(&self, pos: &CameraPosition, x: f64, y: f64) -> Ray {
        let x_percent = x / (self.dimm.width as f64);
        let y_percent = y / (self.dimm.height as f64);

        let horizontal = pos.u.scale(self.width);
        let vertical = pos.v.scale(self.height);
        let lower_left: Vec3 = pos.origin.0
            - horizontal.scale(0.5)
            - vertical.scale(0.5)
            - pos.w.scale(self.focal_length);

        // let direction = Vec3::new(
        //     (2.0 * x_percent - 1.0) * self.width,
        //     (2.0 * y_percent - 1.0) * self.height,
        //     -self.focal_length,
        // );

        let direction: Vec3 =
            lower_left + horizontal.scale(x_percent) + vertical.scale(y_percent) - pos.origin.0;

        Ray {
            orig: pos.origin,
            direction,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dimmensions {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum AspectRatio {
    Rational(u32, u32),
    Ratio(f64),
}

impl AspectRatio {
    fn as_float(self) -> f64 {
        match self {
            AspectRatio::Rational(n, d) => n as f64 / d as f64,
            AspectRatio::Ratio(f) => f,
        }
    }
}

impl From<(u32, u32)> for AspectRatio {
    fn from(r: (u32, u32)) -> Self {
        AspectRatio::Rational(r.0, r.1)
    }
}

impl From<f64> for AspectRatio {
    fn from(r: f64) -> Self {
        AspectRatio::Ratio(r)
    }
}

impl Dimmensions {
    pub fn from_aspect_ratio<A: Into<AspectRatio>>(width: usize, ratio: A) -> Dimmensions {
        let ratio = ratio.into().as_float();
        let height = ((width as f64) / ratio) as usize;
        Dimmensions { width, height }
    }

    fn from_two_of_three(
        height: Option<usize>,
        width: Option<usize>,
        ratio: Option<AspectRatio>,
    ) -> Result<(Dimmensions, AspectRatio)> {
        Ok(match (height, width, ratio) {
            (None, Some(w), Some(r)) => {
                let h = ((w as f64) / r.as_float()) as usize;
                let img = Dimmensions {
                    width: w,
                    height: h,
                };
                (img, r)
            }
            (Some(h), None, Some(r)) => {
                let w = (h as f64 * r.as_float()) as usize;
                let img = Dimmensions {
                    width: w,
                    height: h,
                };
                (img, r)
            }
            (Some(h), Some(w), None) => {
                let img = Dimmensions {
                    width: w,
                    height: h,
                };
                let r = AspectRatio::Rational(h as u32, w as u32);
                (img, r)
            }
            _ => anyhow::bail!(
                "could not construct dimm: require exactly 2 of (height, width, aspect ratio)"
            ),
        })
    }
}
