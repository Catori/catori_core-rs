use std;

///Everything in Catori-space must be a category
///And within all of Catori-space, the only thing
///one can reason about is nested ordering and equality
///No other attributes of a data type are relevant to Catori.
pub trait Category: Ord + Eq {
    fn length(&self) -> usize {
        1
    }
}

///Any type that is both Ord and Eq is automatically promoted to a Category
impl<CAT> Category for CAT where CAT:Ord+Eq{}


pub trait Default : Category + std::default::Default {}

impl<DEFAULT> Default for DEFAULT where DEFAULT:Category+std::default::Default{}