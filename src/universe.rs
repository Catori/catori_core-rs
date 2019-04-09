use core::marker::PhantomData;

use crate::{Catori, Path};
use crate::nil::Nil;

#[derive(Eq,PartialEq,Ord,PartialOrd,Clone,Default)]
pub struct Dimension<CONTEXT, HERE, THERE>
    where HERE: Path<THERE>
{
    phantom: PhantomData<CONTEXT>,
    h: HERE,
    t: THERE,
}

//  type Qubit = Dimension;


//impl<CONTEXT,H,T> Path<CONTEXT> for Bit<CONTEXT,H,T> {

//}
// struct True();
// struct Fals

// trait Bit : Path<False> {
// }

// #[derive(Eq,PartialEq,Ord,PartialOrd)]
// struct Bool();

// impl Default for Bool {

// }

// impl Bit for Bool{}

// #[derive(Eq,PartialEq,Ord,PartialOrd)]
// enum B {
// True,
// False
// }

// impl Default for Bit{
//     fn default() -> Self {
//         Bit::False
//     }
// }


//type Bit<CONTEXT,H,T> = Dimension<CONTEXT,H,T>;

//impl<BIT:False> Path<BIT> for Bool{}

///A Dimension is the concrete implementation of a Univalent Path
impl<CONTEXT, HERE, THERE> Catori for Dimension<CONTEXT, HERE, THERE>
    where CONTEXT: Path<HERE>,
          HERE: Path<THERE>,
          THERE: Catori
{
}

///A space is a bag of dimensions that are all valid within the same CONTEXT
pub struct Universe<CONTEXT, HERE, THERE>(PhantomData<CONTEXT>, Vec<(HERE, THERE)>) where HERE: Path<THERE>;

trait Wave<WAVE, HERE, THERE>: Path<WAVE> + Path<HERE>
    where WAVE: Path<WAVE> + Path<HERE>,
          HERE: Path<THERE>
{
    type Here;
    type There;
    type Out;
    fn collapse(self) -> Self::Out;
}

impl<WAVE, HERE: Default, THERE, PATH> Wave<WAVE, HERE, THERE> for PATH
    where WAVE: Path<HERE>,
          PATH: Path<HERE> + Path<WAVE>,
          HERE: Path<THERE>
{
    type Here = HERE;
    type There = THERE;
    type Out = Self::Here;
    fn collapse(self) -> Self::Out {
        //  WAVE::H
        unimplemented!()
    }
}

///CONTEXT has a Dimension that provides a path from Here to There
impl<CONTEXT, HERE, THERE> Path<HERE> for Dimension<CONTEXT, HERE, THERE>
    where CONTEXT: Path<HERE>,
          HERE: Path<THERE>,
          THERE: Path<THERE>
{
    type Context = CONTEXT;
    type There = THERE;
    fn next(self) -> THERE {
        self.t
    }
}

//A Qubit is the abstract idea of a Bit
pub trait Qubit<HERE>: Path<HERE> {}

#[derive(Ord,Eq,PartialEq,PartialOrd)]
pub enum Bit<CONTEXT, HERE>
    where CONTEXT: Path<HERE> + Path<Nil<CONTEXT>>,
          HERE: Path<HERE>
{
    False(Nil<CONTEXT>),
    True(HERE),
    Phantom(PhantomData<CONTEXT>),
}

trait BitSeq<CONTEXT, HERE>: Path<HERE, Context = CONTEXT> where CONTEXT: Path<HERE> {}

impl<CONTEXT, HERE> Default for Bit<CONTEXT, HERE>
    where CONTEXT: Path<HERE> + Path<Nil<CONTEXT>>,
          HERE: Path<Nil<HERE>>
{
    fn default() -> Bit<CONTEXT, HERE> {
        Bit::False(Nil::default())
    }
}

impl<CONTEXT, HERE> Path<HERE> for Bit<CONTEXT, HERE>
    where CONTEXT: Path<HERE> + Path<Nil<CONTEXT>>,
          HERE: Path<Nil<HERE>>
{
    type Context = CONTEXT;
    type There = Nil<HERE>;
}

//pub trait Bit{}
// impl Bit for True{}
// impl Bit for False{}

//impl Path<

trait Collider<CONTEXT>
    where <Self as Collider<CONTEXT>>::Here: Path<<Self as Collider<CONTEXT>>::There>
{
    type Here;
    type There;
    fn collide(here: Self::Here, there: Self::There) -> Dimension<CONTEXT, Self::Here, Self::There>;
}

///The default rule for any two things colliding, with the same Context,
///is to have the first item wrap the second. Which is to say that the A
///implies a Path to B.
///Technically, this collision could only happen in the first place *if*
///there already were such a path in the same context, so this is effectively
///a noop. But it sets the stage for specializing other collision types.
///If there is a path between two things, they an collide
impl<CONTEXT, HERE, THERE> Collider<CONTEXT> for Dimension<CONTEXT, HERE, THERE>
    where HERE: Path<THERE>
{
    type Here = HERE;
    type There = THERE;
    fn collide(a: HERE, b: THERE) -> Dimension<CONTEXT, HERE, THERE> {
        Dimension {
            phantom: PhantomData,
            h: a,
            t: b,
        }
    }
}
