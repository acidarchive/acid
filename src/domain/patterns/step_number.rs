#[derive(Debug)]
pub struct StepNumber(i32);

impl StepNumber {
    pub fn parse(i: i32) -> Result<StepNumber, String> {
        let is_less_than_1 = i < 1;
        let is_greater_than_16 = i > 16;

        if is_greater_than_16 || is_less_than_1 {
            Err(format!("{i} is not a valid step number value. "))
        } else {
            Ok(Self(i))
        }
    }
}

impl AsRef<i32> for StepNumber {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::StepNumber;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_value_less_than_1_is_rejected() {
        let number: i32 = -1;
        assert_err!(StepNumber::parse(number));
    }

    #[test]
    fn a_value_greater_than_16_is_rejected() {
        let number: i32 = 17;
        assert_err!(StepNumber::parse(number));
    }
    #[test]
    fn a_value_in_a_range_between_1_and_16_is_valid() {
        let number: i32 = 8;
        assert_ok!(StepNumber::parse(number));
    }
}
