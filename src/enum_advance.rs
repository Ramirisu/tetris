macro_rules! enum_advance_derive {
    ($name: ident) => {
        impl $name {
            pub fn enum_prev(&self) -> Option<Self> {
                let v = *self as usize;
                if v == 0 { None } else { Self::from_repr(v - 1) }
            }

            pub fn enum_next(&self) -> Option<Self> {
                let v = *self as usize;
                if v == Self::COUNT - 1 {
                    None
                } else {
                    Self::from_repr(v + 1)
                }
            }
        }
    };
}

pub(crate) use enum_advance_derive;

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        use strum::EnumCount;
        use strum_macros::{EnumCount, EnumIter, FromRepr};

        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
        enum E {
            #[default]
            A,
            B,
            C,
        }

        enum_advance_derive!(E);

        let mut e = E::default();
        assert_eq!(e, E::A);
        assert_eq!(e.enum_next(), Some(E::B));
        e = e.enum_next().unwrap();
        assert_eq!(e, E::B);
        assert_eq!(e.enum_next(), Some(E::C));
        e = e.enum_next().unwrap();
        assert_eq!(e, E::C);
        assert_eq!(e.enum_next(), None);

        assert_eq!(e, E::C);
        assert_eq!(e.enum_prev(), Some(E::B));
        e = e.enum_prev().unwrap();
        assert_eq!(e, E::B);
        assert_eq!(e.enum_prev(), Some(E::A));
        e = e.enum_prev().unwrap();
        assert_eq!(e, E::A);
        assert_eq!(e.enum_prev(), None);
    }
}
