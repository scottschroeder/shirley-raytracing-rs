use crate::{
    camera::Dimmensions,
    util::{Color, Vec3},
};
use std::io;

const PPM_COLOR_SCALE: f64 = 255.999;

pub struct Image {
    pub dimm: Dimmensions,
    pub data: Vec<Vec<Color>>,
    pub samples: usize,
}

impl Image {
    pub fn from_dimm(dimm: Dimmensions) -> Image {
        log::trace!("alloc image buffer");
        Image {
            dimm,
            samples: 1,
            data: vec![vec![Color(Vec3::new(0.0, 0.0, 0.0)); dimm.width]; dimm.height],
        }
    }

    pub fn scanlines_mut(&mut self) -> &mut [Vec<Color>] {
        self.data.as_mut_slice()
    }
}

pub fn to_image<P: AsRef<std::path::Path>>(img: &Image, path: P) {
    log::trace!("convert image");
    let mut dst = image::RgbImage::new(img.dimm.width as u32, img.dimm.height as u32);
    for j in 0..img.dimm.height {
        for (i, c) in img.data[j].iter().enumerate() {
            let c = Color(c.0.scale(1.0 / (img.samples as f64)));
            dst.put_pixel(i as u32, (img.dimm.height - j - 1) as u32, c.to_pixel())
        }
    }
    log::trace!("write png");
    dst.save_with_format(path.as_ref(), image::ImageFormat::Png)
        .unwrap();
}

// pub fn write_ppm_image<W: io::Write>(w: &mut W, image: &Image) -> std::io::Result<()> {
//     write!(
//         w,
//         "P3\n{} {}\n{}\n",
//         image.dimm.width, image.dimm.height, PPM_COLOR_MAX
//     )?;
//     for j in (0..image.dimm.height).rev() {
//         for i in 0..image.dimm.width {
//             let c = image.data[j * image.dimm.width + i];
//             write_ppm_pixel(w, &c)?;
//         }
//     }
//     Ok(())
// }

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
