#[derive(Debug)]
pub struct BPM(i32);

impl BPM {
    pub fn parse(i: i32) -> Result<BPM, String> {
        let is_less_than_0 = i < 0;
        let is_greater_than_999 = i > 999;

        if is_greater_than_999 || is_less_than_0 {
            Err(format!("{} is not a valid BPM value. ", i))
        } else {
            Ok(Self(i))
        }
    }
}

impl AsRef<i32> for BPM {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::BPM;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_value_less_than_0_is_rejected() {
        let bpm: i32 = -1;
        assert_err!(BPM::parse(bpm));
    }

    #[test]
    fn a_value_greater_than_999_is_rejected() {
        let bpm: i32 = 1000;
        assert_err!(BPM::parse(bpm));
    }
    #[test]
    fn a_value_in_a_range_between_0_and_999_is_valid() {
        let bpm: i32 = 128;
        assert_ok!(BPM::parse(bpm));
    }
}
