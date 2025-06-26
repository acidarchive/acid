#[derive(Debug)]
pub struct Tempo(i32);

impl Tempo {
    pub fn parse(i: i32) -> Result<Tempo, String> {
        let is_less_than_0 = i < 0;
        let is_greater_than_999 = i > 999;

        if is_greater_than_999 || is_less_than_0 {
            Err(format!("{i} is not a valid Tempo value. "))
        } else {
            Ok(Self(i))
        }
    }
}

impl AsRef<i32> for Tempo {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Tempo;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_value_less_than_0_is_rejected() {
        let tempo: i32 = -1;
        assert_err!(Tempo::parse(tempo));
    }

    #[test]
    fn a_value_greater_than_999_is_rejected() {
        let tempo: i32 = 1000;
        assert_err!(Tempo::parse(tempo));
    }
    #[test]
    fn a_value_in_a_range_between_0_and_999_is_valid() {
        let tempo: i32 = 128;
        assert_ok!(Tempo::parse(tempo));
    }
}
