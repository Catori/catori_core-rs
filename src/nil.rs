use crate::{Catori, Path};
use core::marker::PhantomData;

///Nil is a zero sized type, but unlike (), it has knowledge of its parent type,
///and is therefore able to function in a path-dependent way.
#[derive(Ord, PartialOrd, Eq, PartialEq, Default, Clone)]
pub struct Nil<CONTEXT>(PhantomData<CONTEXT>);

impl<CONTEXT> Catori for Nil<CONTEXT>
where
    CONTEXT: Path<Nil<CONTEXT>>,
{
    //   type Context = C;
    default fn length(&self) -> usize {
        0
    }
}

impl<CONTEXT> Path<Nil<CONTEXT>> for Nil<CONTEXT>
where
    CONTEXT: Path<Nil<CONTEXT>>,
{
    default fn next(self) -> Self::There {
        Self::There::default()
    }
}
