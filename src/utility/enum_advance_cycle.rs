macro_rules! enum_advance_cycle_derive {
    ($name: ident) => {
        impl $name {
            pub fn enum_prev_cycle(&self) -> Self {
                self.enum_prev()
                    .unwrap_or_else(|| Self::from_repr(Self::COUNT - 1).unwrap())
            }

            pub fn enum_next_cycle(&self) -> Self {
                self.enum_next()
                    .unwrap_or_else(|| Self::from_repr(0).unwrap())
            }
        }
    };
}

pub(crate) use enum_advance_cycle_derive;

#[cfg(test)]
mod test {
    use crate::utility::enum_advance::enum_advance_derive;

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
        enum_advance_cycle_derive!(E);

        let mut e = E::default();
        assert_eq!(e, E::A);
        assert_eq!(e.enum_next_cycle(), E::B);
        e = e.enum_next_cycle();
        assert_eq!(e, E::B);
        assert_eq!(e.enum_next_cycle(), E::C);
        e = e.enum_next_cycle();
        assert_eq!(e, E::C);
        assert_eq!(e.enum_next_cycle(), E::A);

        assert_eq!(e, E::C);
        assert_eq!(e.enum_prev_cycle(), E::B);
        e = e.enum_prev_cycle();
        assert_eq!(e, E::B);
        assert_eq!(e.enum_prev_cycle(), E::A);
        e = e.enum_prev_cycle();
        assert_eq!(e, E::A);
        assert_eq!(e.enum_prev_cycle(), E::C);
    }
}
