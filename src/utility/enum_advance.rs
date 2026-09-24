pub(crate) trait EnumAdvance: Copy {
    fn enum_next(&self) -> Option<Self>;
    fn enum_prev(&self) -> Option<Self>;
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        use crate::utility::enum_advance::EnumAdvance;
        use strum::EnumCount;
        use strum_macros::{EnumCount, EnumIter, FromRepr};

        #[derive(
            Debug,
            Default,
            Clone,
            Copy,
            PartialEq,
            Eq,
            FromRepr,
            EnumIter,
            EnumCount,
            tetris_macros::EnumAdvance,
        )]
        enum E {
            #[default]
            A,
            B,
            C,
        }

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
