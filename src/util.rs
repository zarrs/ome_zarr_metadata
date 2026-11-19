use validatrix::{Accumulator, Validate};

#[derive(Debug, Clone, Copy)]
pub struct ValidNum(pub f64);

impl Validate for ValidNum {
    fn validate_inner(&self, accum: &mut Accumulator) {
        if self.0.is_nan() {
            accum.add_failure("NaN is not allowed");
        } else if self.0.is_infinite() {
            accum.add_failure("infinity is not allowed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validatrix::Validate;

    #[test]
    fn zero_allowed() {
        ValidNum(0.0).validate().unwrap();
    }
}
