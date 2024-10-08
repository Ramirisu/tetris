macro_rules! enum_iter_derive {
    ($name: ident) => {
        impl $name {
            pub fn enum_prev(&self) -> Option<Self> {
                <Self as num_traits::FromPrimitive>::from_i64(*self as i64 - 1)
            }

            pub fn enum_next(&self) -> Option<Self> {
                <Self as num_traits::FromPrimitive>::from_i64(*self as i64 + 1)
            }
        }
    };
}

pub(crate) use enum_iter_derive;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
        enum E {
            #[default]
            A,
            B,
            C,
        }

        enum_iter_derive!(E);

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
