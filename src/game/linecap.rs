use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Linecap {
    #[default]
    None,
    KillScreenX2,
}

impl Linecap {
    pub fn enum_prev(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 - 1).map(|n| std::mem::replace(self, n))
    }

    pub fn enum_next(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 + 1).map(|n| std::mem::replace(self, n))
    }
}
