use crate::{Catori};
use crate::nil::Nil;
///A `Path` represets the abstract notion that there is a path from Here to There,
/// or in other words, given a Here, an implementor of a Path guarantees that if it is traversed
/// from Here, then eventually a There will be reached
///This is inspired by HCons from HList
pub trait Path<HERE>: Catori + Sized + Default {
    type Context: Catori;
    type There: Path<Self::There>;
    fn next(self) -> Self::There {

        Self::There::default()
    }
}



//impl

///Any Category that is also instantiatable by Default, is a path to itself
///In other words, a default Category implies its own existence
impl<Context: Catori + Default> Path<Context> for Context {
    type Context = Context;
    type There = Nil<Context>;
}


// impl Path<Peano> for usize {
//     type Context = usize;
//     type There = Succ<usize>;
//     fn next(self) -> Self::There {

//     }
// }

///Within a Context, two Categories (Here and There) are equivalent, and hence equal,
//so long as within Context there is a path from Here to There, as well as from There to Here.
trait Equivalent<Context, Here, There>
    : Path<Here, Context = Context, There = There> + Path<There, Context = Context, There = Here>
where
    Context: Catori,
    Here: Path<There>,
    There: Path<There>,
{
    fn eq(_: Here, _: There) -> bool {
        true
    }
}

///Not all categories have a default value, but a Measure must
pub trait Measure<M: Catori + Default>: Path<M> + Default {}
