use crate::{Category,Path};
use std::marker::PhantomData;

///Nil is a zero sized type, but unlike (), it has knowledge of its parent type,
///and is therefore able to function in a path-dependent way.
#[derive(Ord,PartialOrd,Eq,PartialEq,Default,Clone)]
pub struct Nil<C>(PhantomData<C>);

impl<C> Category for Nil<C> where C:Path<Nil<C>> {
 //   type Context = C;
    fn length(&self) -> usize {
        0
    }
}

impl<C> Path<Nil<C>> for Nil<C> where C:Path<Nil<C>>{
    fn next(self) -> Self::T {
        Self::T::default()
    }
}
