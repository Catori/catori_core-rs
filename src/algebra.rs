use crate::{Dimension, Path, Nil, Catori, Measure};
use core::ops::Add;
use core::marker::PhantomData;

trait Semigroup<HERE>
    where HERE: Path<<Self as Semigroup<HERE>>::There>
{
    type Context;
    type There;
    fn combine(here: HERE, there: Self::There) -> HERE;
}

trait Monoid<HERE>: Semigroup<HERE>
    where HERE: Path<<Self as Semigroup<HERE>>::There>
{
    type Context;
    type Empty;
}


impl<CONTEXT, HERE, THERE> Semigroup<HERE> for Dimension<CONTEXT, HERE, THERE>
    where HERE: Path<HERE> + Path<THERE> + Add<Output = HERE>,
          THERE: Path<HERE>
{
    type Context = CONTEXT;
    type There = HERE;
    fn combine(x: HERE, y: HERE) -> HERE {
        x + y
    }
}


impl<CONTEXT, HERE> Monoid<HERE> for Dimension<CONTEXT, HERE, HERE>
    where HERE: Path<HERE> + Add<Output = HERE>
{
    type Context = CONTEXT;
    type Empty = Nil<CONTEXT>;
}

pub trait Peano<MEASURE> {}
pub trait NonZero<MEASURE>: Peano<MEASURE> {}
pub trait NonNeg<MEASURE>: Peano<MEASURE> {}
pub trait NonPos<MEASURE>: Peano<MEASURE> {}

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
impl<M> Measure<M> for M where M: Path<M> + NonNeg<M> + Catori + Default {}

// impl Peano<usize> for usize {}
// impl NonNeg<usize> for usize {}
// impl NonPos<usize> for usize {}
