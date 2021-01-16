use crate::{
    camera::Dimmensions,
    util::{Color, Vec3},
};
use std::io;

const PPM_COLOR_SCALE: f64 = 255.999;
const PPM_COLOR_MAX: u8 = 255;

pub struct Image {
    pub dimm: Dimmensions,
    pub data: Vec<Color>,
}

impl Image {
    pub fn from_dimm(dimm: Dimmensions) -> Image {
        log::trace!("alloc image buffer");
        Image {
            dimm,
            data: vec![Color(Vec3::new(0.0, 0.0, 0.0)); dimm.width * dimm.height],
        }
    }

    pub fn scanlines_mut(&mut self) -> Vec<&mut [Color]> {
        log::trace!("setup scanlines");
        let mut lines = Vec::with_capacity(self.dimm.height);
        let mut rem = self.data.as_mut_slice();
        for _ in 0..self.dimm.height {
            let (head, cons) = rem.split_at_mut(self.dimm.width);
            lines.push(head);
            rem = cons;
        }
        log::trace!("created {} scanlines", lines.len());
        lines
    }
}

pub fn to_image<P: AsRef<std::path::Path>>(img: &Image, path: P) {
    log::trace!("convert image");
    let mut dst = image::RgbImage::new(img.dimm.width as u32, img.dimm.height as u32);
    for j in 0..img.dimm.height {
        for i in 0..img.dimm.width {
            let c = img.data[img.dimm.width * j + i];

            dst.put_pixel(i as u32, (img.dimm.height - j - 1) as u32, c.to_pixel())
        }
    }
    log::trace!("write png");
    dst.save_with_format(path.as_ref(), image::ImageFormat::Png)
        .unwrap();
}

pub fn write_ppm_image<W: io::Write>(w: &mut W, image: &Image) -> std::io::Result<()> {
    write!(
        w,
        "P3\n{} {}\n{}\n",
        image.dimm.width, image.dimm.height, PPM_COLOR_MAX
    )?;
    for j in (0..image.dimm.height).rev() {
        for i in 0..image.dimm.width {
            let c = image.data[j * image.dimm.width + i];
            write_ppm_pixel(w, &c)?;
        }
    }
    Ok(())
}

pub fn write_ppm_pixel<W: io::Write>(w: &mut W, color: &Color) -> std::io::Result<()> {
    write!(
        w,
        "{} {} {}\n",
        (PPM_COLOR_SCALE * color.0.x()) as u8,
        (PPM_COLOR_SCALE * color.0.y()) as u8,
        (PPM_COLOR_SCALE * color.0.z()) as u8
    )?;
    Ok(())
}
