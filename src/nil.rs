use crate::Catori;

///Nil is a zero sized type, but unlike (), it has knowledge of its parent type,
///and is therefore able to function in a path-dependent way.
#[derive(Ord, PartialOrd, Eq, PartialEq, Default, Clone)]
pub struct Nil();

impl Catori for Nil {
    //   type Context = C;
    default fn length(&self) -> usize {
        0
    }
}
