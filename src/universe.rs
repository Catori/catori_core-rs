use std::marker::PhantomData;


use {Category, Path};
use nil::Nil;

#[derive(Eq,PartialEq,Ord,PartialOrd,Clone,Default)]
pub struct Dimension<CONTEXT, H, T>
    where H: Path<T>
{
    _phantom: PhantomData<CONTEXT>,
    h: H,
    t: T,
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
impl<CONTEXT, H, T> Category for Dimension<CONTEXT, H, T>
    where CONTEXT: Path<H>,
          H: Path<T>,
          T: Category
{
}

///A space is a bag of dimensions that are all valid within the same CONTEXT
pub struct Universe<CONTEXT, H, T>(PhantomData<CONTEXT>, Vec<(H, T)>) where H: Path<T>;

trait Wave<WAVE, H, T>: Path<WAVE> + Path<H>
    where WAVE: Path<WAVE> + Path<H>,
          H: Path<T>
{
    type H;
    type T;
    type Out;
    fn collapse(self) -> Self::Out;
}

impl<WAVE, H: Default, T, PATH> Wave<WAVE, H, T> for PATH
    where WAVE: Path<H>,
          PATH: Path<H> + Path<WAVE>,
          H: Path<T>
{
    type H = H;
    type T = T;
    type Out = Self::H;
    fn collapse(self) -> Self::Out {
        //  WAVE::H
        unimplemented!()
    }
}

///CONTEXT has a Dimension that provides a path from Here to There
impl<CONTEXT, H, T> Path<H> for Dimension<CONTEXT, H, T>
    where CONTEXT: Path<H>,
          H: Path<T>,
          T: Path<T>
{
    type Context = CONTEXT;
    type T = T;
    fn next(self) -> T {
        self.t
    }
}

//A Qubit is the abstract idea of a Bit
pub trait Qubit<H>: Path<H> {}

#[derive(Ord,Eq,PartialEq,PartialOrd)]
enum Bit<C, H>
    where C: Path<H> + Path<Nil<C>>,
          H: Path<H>
{
    False(Nil<C>),
    True(H),
    _phantom(PhantomData<C>),
}

trait BitSeq<C, H>: Path<H, Context = C> where C: Path<H> {}

impl<C, H> Default for Bit<C, H>
    where C: Path<H> + Path<Nil<C>>,
          H: Path<Nil<H>>
{
    fn default() -> Bit<C, H> {
        Bit::False(Nil::default())
    }
}

impl<C, H> Path<H> for Bit<C, H>
    where C: Path<H> + Path<Nil<C>>,
          H: Path<Nil<H>>
{
    type Context = C;
    type T = Nil<H>;
}

//pub trait Bit{}
// impl Bit for True{}
// impl Bit for False{}

//impl Path<

trait Collider<CONTEXT>
    where <Self as Collider<CONTEXT>>::H: Path<<Self as Collider<CONTEXT>>::T>
{
    type H;
    type T;
    fn collide(here: Self::H, there: Self::T) -> Dimension<CONTEXT, Self::H, Self::T>;
}

///The default rule for any two things colliding, with the same Context,
///is to have the first item wrap the second. Which is to say that the A
///implies a Path to B.
///Technically, this collision could only happen in the first place *if*
///there already were such a path in the same context, so this is effectively
///a noop. But it sets the stage for specializing other collision types.
///If there is a path between two things, they an collide
impl<CONTEXT, H, T> Collider<CONTEXT> for Dimension<CONTEXT, H, T>
    where H: Path<T>
{
    type H = H;
    type T = T;
    fn collide(a: H, b: T) -> Dimension<CONTEXT, H, T> {
        Dimension {
            _phantom: PhantomData,
            h: a,
            t: b,
        }
    }
}
