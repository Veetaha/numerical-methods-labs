use std::fmt;

pub struct Coefs<'a>(pub &'a [f64]);

impl fmt::Display for Coefs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        write!(f, "{:.2E}", self.0[0])?;
        for coef in self.0[1..].iter() {
            write!(f, ", {:.2E}", coef)?;
        }
        write!(f, "]")
    }
}
