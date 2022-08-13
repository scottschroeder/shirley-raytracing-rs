use std::cmp::Ordering;

#[inline]
pub(crate) fn fmin(a: f64, b: f64) -> f64 {
    match a.partial_cmp(&b) {
        Some(Ordering::Less) => a,
        Some(_) => b,
        None => non_nan(a, b),
    }
}

#[inline]
pub(crate) fn fmax(a: f64, b: f64) -> f64 {
    match a.partial_cmp(&b) {
        Some(Ordering::Greater) => a,
        Some(_) => b,
        None => non_nan(a, b),
    }
}

#[inline]
fn non_nan(a: f64, b: f64) -> f64 {
    if a.is_nan() {
        b
    } else {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_nan_first() {
        let a = 4.3;
        let b = 50.1;
        let r = non_nan(a, b);
        assert_eq!(r, a);
    }

    #[test]
    fn non_nan_second() {
        let a = std::f64::NAN;
        let b = 50.1;
        let r = non_nan(a, b);
        assert_eq!(r, b);
    }

    #[test]
    fn non_nan_both() {
        let a = std::f64::NAN;
        let b = std::f64::NAN;
        let r = non_nan(a, b);
        assert!(r.is_nan());
    }

    #[test]
    fn check_min() {
        let a = 4.3;
        let b = 50.1;
        assert_eq!(fmin(a, b), a);
        assert_eq!(fmin(b, a), a);
    }

    #[test]
    fn check_min_same() {
        let a = 4.3;
        assert_eq!(fmin(a, a), a);
    }

    #[test]
    fn check_min_with_nans() {
        let a = 4.3;
        let b = std::f64::NAN;
        assert_eq!(fmin(a, b), a);
        assert_eq!(fmin(b, a), a);
    }
    #[test]
    fn check_min_nan_both() {
        let a = std::f64::NAN;
        let b = std::f64::NAN;
        assert!(fmin(a, b).is_nan())
    }

    #[test]
    fn check_max() {
        let a = 4.3;
        let b = 50.1;
        assert_eq!(fmax(a, b), b);
        assert_eq!(fmax(b, a), b);
    }

    #[test]
    fn check_max_same() {
        let a = 4.3;
        assert_eq!(fmax(a, a), a);
    }

    #[test]
    fn check_max_with_nans() {
        let a = 4.3;
        let b = std::f64::NAN;
        assert_eq!(fmax(a, b), a);
        assert_eq!(fmax(b, a), a);
    }
    #[test]
    fn check_max_nan_both() {
        let a = std::f64::NAN;
        let b = std::f64::NAN;
        assert!(fmax(a, b).is_nan())
    }
}
