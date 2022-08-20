use serde::{Deserialize, Serialize};

use super::{dielectric::Dielectric, metal::Metal};

#[derive(Serialize, Deserialize)]
pub enum MaterialType {
    Metal(Metal),
    Dielectric(Dielectric),
}
