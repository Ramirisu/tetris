use num_traits::FromPrimitive;

#[derive(Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum DropSpeed {
    Level = 0,
    Locked,
}

impl DropSpeed {
    pub fn enum_prev(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 - 1).map(|n| std::mem::replace(self, n))
    }

    pub fn enum_next(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 + 1).map(|n| std::mem::replace(self, n))
    }
}
