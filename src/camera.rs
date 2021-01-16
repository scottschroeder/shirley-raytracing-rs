use crate::util::{Point, Ray, Vec3};
use anyhow::Result;

const VIEWPORT_HEIGHT: f64 = 2.0;
const DEFAULT_FOCAL_LENGTH: f64 = 1.0;

#[derive(Default)]
pub struct CameraBuilder {
    position: Option<Point>,
    height: Option<usize>,
    width: Option<usize>,
    focal_length: Option<f64>,
    ratio: Option<AspectRatio>,
}

impl CameraBuilder {
    pub fn aspect_ratio<A: Into<AspectRatio>>(&mut self, ratio: A) -> &mut Self {
        self.ratio = Some(ratio.into());
        self
    }
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }
    pub fn build(self) -> Result<Camera> {
        let (dimm, ratio) = Image::from_two_of_three(self.height, self.width, self.ratio)?;
        let width = ratio.as_float() * VIEWPORT_HEIGHT;
        Ok(Camera {
            position: self.position.unwrap_or_else(|| Point::default()),
            width,
            focal_length: self.focal_length.unwrap_or(DEFAULT_FOCAL_LENGTH),
            dimm,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    position: Point,
    width: f64,
    focal_length: f64,
    pub dimm: Image,
}

impl Camera {
    pub fn pixel_ray(&self, x: usize, y: usize) -> Ray {
        let u = (x as f64) / (self.dimm.width as f64);
        let v = (y as f64) / (self.dimm.height as f64);

        let direction = Vec3::new(
            (2.0 * u - 1.0) * self.width,
            (2.0 * v - 1.0) * VIEWPORT_HEIGHT,
            -self.focal_length,
        );

        Ray {
            orig: self.position,
            direction,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Image {
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

impl Image {
    pub fn from_aspect_ratio<A: Into<AspectRatio>>(width: usize, ratio: A) -> Image {
        let ratio = ratio.into().as_float();
        let height = ((width as f64) / ratio) as usize;
        Image { width, height }
    }

    fn from_two_of_three(
        height: Option<usize>,
        width: Option<usize>,
        ratio: Option<AspectRatio>,
    ) -> Result<(Image, AspectRatio)> {
        Ok(match (height, width, ratio) {
            (None, Some(w), Some(r)) => {
                let h = ((w as f64) / r.as_float()) as usize;
                let img = Image {
                    width: w,
                    height: h,
                };
                (img, r)
            }
            (Some(h), None, Some(r)) => {
                let w = (h as f64 * r.as_float()) as usize;
                let img = Image {
                    width: w,
                    height: h,
                };
                (img, r)
            }
            (Some(h), Some(w), None) => {
                let img = Image {
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
