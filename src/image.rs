use crate::util::Color;
use std::io;

const PPM_COLOR_SCALE: f64 = 255.999;

pub fn write_ppm_pixel<W: io::Write>(w: &mut W, color: &Color) -> std::io::Result<()> {
    write!(
        w,
        "{} {} {}",
        (PPM_COLOR_SCALE * color.0.x()) as u8,
        (PPM_COLOR_SCALE * color.0.y()) as u8,
        (PPM_COLOR_SCALE * color.0.z()) as u8
    )?;
    Ok(())
}
