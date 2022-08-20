use serde::{Deserialize, Serialize};
use super::{metal::Metal, dielectric::Dielectric};


#[derive(Serialize, Deserialize)]
pub enum MaterialType {
    Metal(Metal),
    Dielectric(Dielectric),
}
