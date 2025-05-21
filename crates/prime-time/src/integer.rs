pub trait Integer {
    fn is_prime(self) -> bool;
}

impl Integer for f64 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn is_prime(self) -> bool {
        if self != self.round() {
            return false;
        }
        let int = self as i64;
        if int < 4 {
            int > 1
        } else if int % 2 == 0 || int % 3 == 0 {
            false
        } else {
            let max_p = (int as f64).sqrt().ceil() as i64;
            !(5..=max_p)
                .step_by(6)
                .any(|p| int % p == 0 || int % (p + 2) == 0)
        }
    }
}
