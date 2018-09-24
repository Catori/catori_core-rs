use crate::{Category};
use crate::nil::Nil;
///A `Path` is a HoTT-like path between spaces, which are `HLists`s
///in the abstract sense, and `Dimensions`s when instantiated at runtime
///
///
///This is inspired by HCons from HList
pub trait Path<H>: Category + Sized + Default {
    type Context: Category;
    type T: Path<Self::T>;
    fn next(self) -> Self::T {
        Self::T::default()
    }
}



//impl

///Any Category that is also instantiatable by Default, is a path to itself
///In other words, a default Category implies its own existence
impl<C: Category + Default> Path<C> for C {
    type Context = C;
    type T = Nil<C>;
}


// impl Path<Peano> for usize {
//     type Context = usize;
//     type T = Succ<usize>;
//     fn next(self) -> Self::T {

//     }
// }

///Within a Context, two Categories (Here and There) are equivalent, and hence equal,
//so long as within Context there is a path from Here to There, as well as from There to Here.
trait Equivalent<Context, H, T>
    : Path<H, Context = Context, T = T> + Path<T, Context = Context, T = H>
where
    Context: Category,
    H: Path<T>,
    T: Path<T>,
{
    fn eq(_: H, _: T) -> bool {
        true
    }
}

///Not all categories have a default value, but a Measure must
pub trait Measure<M: Category + Default>: Path<M> + Default {}
