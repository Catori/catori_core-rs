use crate::{Catori, Nil, Path};
use core::marker::PhantomData;

#[derive(Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct BitString<'a, C: Catori, Bit> {
    phantom: PhantomData<C>,
    bit: Bit,
    next: Option<&'a BitString<'a, C, Bit>>, //    Bit{
                                             //        bit:Bit,
                                             //        next: Box<BitString>,
                                             //    },
                                             //    Nil,esdf
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Bit {
    Zero = 0,
    One = 1,
}

pub enum Base10Digit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl Catori for Bit {
    fn length(&self) -> usize {
        1
    }
}

impl Default for Bit {
    fn default() -> Self {
        Bit::Zero
    }
}

impl<'a, CONTEXT, HERE: Catori> Path<BitString<'a, CONTEXT, HERE>> for Bit
where
    CONTEXT: Path<Nil<CONTEXT>>,
{
    type Context = CONTEXT;
    type There = Nil<CONTEXT>;

    fn next(self) -> Self::There {
        Self::There::default()
    }
}
