///Everything in Catori-space must adhere to this Trait,and must notably be Ordered
///And within all of Catori-space, the only thing
///one can reason about is nested ordering and equality
///No other attributes of a data type are relevant to Catori.
pub trait Catori: Ord {
    fn length(&self) -> usize;
}

///Any non Catori type that is Ord can be automatically promoted to a Catori type
impl<CAT> Catori for CAT where CAT:Ord{
    default fn length(&self) -> usize {1}
}


pub trait Free : Catori + core::default::Default {}
///Blanket implement Catori's Default for any standard type that is Default
/// Conceptually this means that any of Rust's default types are free, and exactly
/// as many of them as are required will be produced
impl<FREE> Free for FREE where FREE:Catori+core::default::Default{}