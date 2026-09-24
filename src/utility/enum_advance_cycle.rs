use crate::utility::enum_advance::EnumAdvance;

pub(crate) trait EnumAdvanceCycle: EnumAdvance {
    fn enum_next_cycle(&self) -> Self;
    fn enum_prev_cycle(&self) -> Self;
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use crate::utility::enum_advance_cycle::EnumAdvanceCycle;
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
            tetris_macros::EnumAdvanceCycle,
        )]
        enum E {
            #[default]
            A,
            B,
            C,
        }

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
