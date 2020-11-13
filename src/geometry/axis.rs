use std::fmt;

// These names are probably not right.  The original code uses axes 0, 1, 2 with no documentation
// about what those are supposed to be.
//
// Someday someone with more linear algebra knowledge than I have retained will come here and fix
// this to use the correct names.
#[derive(Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
