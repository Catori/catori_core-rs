use crate::{Dimension, Path, Nil, Category, Measure};
use std::ops::Add;
use std::marker::PhantomData;

trait Semigroup<Here>
    where Here: Path<<Self as Semigroup<Here>>::There>
{
    type Context;
    type There;
    fn combine(here: Here, there: Self::There) -> Here;
}

trait Monoid<Here>: Semigroup<Here>
    where Here: Path<<Self as Semigroup<Here>>::There>
{
    type Context;
    type Empty;
}


impl<Context, Here, There> Semigroup<Here> for Dimension<Context, Here, There>
    where Here: Path<Here> + Path<There> + Add<Output = Here>,
          There: Path<Here>
{
    type Context = Context;
    type There = Here;
    fn combine(x: Here, y: Here) -> Here {
        x + y
    }
}


impl<Context, Here> Monoid<Here> for Dimension<Context, Here, Here>
    where Here: Path<Here> + Add<Output = Here>
{
    type Context = Context;
    type Empty = Nil<Context>;
}

pub trait Peano<Measure> {}
pub trait NonZero<M>: Peano<M> {}
pub trait NonNeg<M>: Peano<M> {}
pub trait NonPos<M>: Peano<M> {}

#[derive(Copy, Clone)]
pub struct Succ<N: NonNeg<N>> {
    _marker: PhantomData<N>,
}

pub type P1 = Succ<Nil<()>>;
pub type P2 = Succ<P1>;
pub type P3 = Succ<P2>;
pub type P4 = Succ<P3>;
pub type P5 = Succ<P4>;
pub type P6 = Succ<P5>;
pub type P7 = Succ<P6>;
pub type P8 = Succ<P7>;
pub type P9 = Succ<P8>;

#[derive(Copy, Clone)]
pub struct Pred<N: NonPos<N>> {
    _marker: PhantomData<N>,
}

pub type N1 = Pred<Nil<()>>;
pub type N2 = Pred<N1>;
pub type N3 = Pred<N2>;
pub type N4 = Pred<N3>;
pub type N5 = Pred<N4>;
pub type N6 = Pred<N5>;
pub type N7 = Pred<N6>;
pub type N8 = Pred<N7>;
pub type N9 = Pred<N8>;

impl<Rhs> Add<Rhs> for Nil<()>
    where Rhs: Peano<Rhs>
{
    type Output = Rhs;
    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<Lhs, Rhs> Add<Rhs> for Succ<Lhs>
    where Lhs: NonNeg<Lhs> + Add<Rhs>,
          Rhs: NonNeg<Rhs>,
          <Lhs as Add<Rhs>>::Output: NonNeg<<Lhs as Add<Rhs>>::Output>
{
    type Output = Succ<<Lhs as Add<Rhs>>::Output>;
    #[allow(unused_variables)]
    fn add(self, rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<Lhs, Rhs> Add<Rhs> for Pred<Lhs>
    where Lhs: NonPos<Lhs> + Add<Rhs>,
          Rhs: NonPos<Rhs>,
          <Lhs as Add<Rhs>>::Output: NonPos<<Lhs as Add<Rhs>>::Output>
{
    type Output = Pred<<Lhs as Add<Rhs>>::Output>;
    fn add(self, _rhs: Rhs) -> Self::Output {
        unreachable!()
    }
}

impl<Lhs, Rhs> Add<Succ<Rhs>> for Pred<Lhs>
    where Lhs: NonPos<Lhs> + Add<Rhs>,
          Rhs: NonNeg<Rhs>
{
    type Output = <Lhs as Add<Rhs>>::Output;
    fn add(self, _rhs: Succ<Rhs>) -> Self::Output {
        unreachable!()
    }
}

impl<Lhs, Rhs> Add<Pred<Rhs>> for Succ<Lhs>
    where Lhs: NonNeg<Lhs> + Add<Rhs>,
          Rhs: NonPos<Rhs>
{
    type Output = <Lhs as Add<Rhs>>::Output;
    #[allow(unused_variables)]
    fn add(self, rhs: Pred<Rhs>) -> Self::Output {
        unreachable!()
    }
}
///usize is a good measure...but not the only good measure
///This should be generic over Natural numbers
//impl<M> Measure<Peano<M>> for M{}
impl<M> Measure<M> for M where M: Path<M> + NonNeg<M> + Category + Default {}

// impl Peano<usize> for usize {}
// impl NonNeg<usize> for usize {}
// impl NonPos<usize> for usize {}
