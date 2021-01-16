pub fn fmin_one(var: f64) -> f64 {
    if let Some(std::cmp::Ordering::Less) = var.partial_cmp(&1.0) {
        var
    } else {
        1.0
    }
}
