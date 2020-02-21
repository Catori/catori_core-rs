use std::marker::PhantomData;

extern crate catori_core;
use catori_core::Free;
//struct Particle<C, H, T>(PhantomData<C>, H, T);
//type Food<C> = Particle<C,(),()>;
use catori_core::*;
#[derive(Ord,PartialOrd,Eq,PartialEq,Default)]
struct Food<C, H: Catori>(PhantomData<C>, H);


#[derive(Ord,PartialOrd,Eq,PartialEq,Default)]
struct Grass {}
//#[derive(Ord,PartialOrd,Eq,PartialEq,Default)]
trait Meat: Free {}

#[derive(Ord,PartialOrd,Eq,PartialEq,Default)]
struct Animal<Food> {
    food: Food,
}
#[derive(Ord,PartialOrd,Eq,PartialEq,Default)]
struct Lion();
#[derive(Ord,PartialOrd,Eq,PartialEq,Default)]
struct Cow();

///In any Context C, the existence of Meat implies a path to Food
///impl<C, H: Category,MEAT:Meat> Path<Food<C, H>> for MEAT {}
///In any Context C, the existence of Grass implies a path to Food
impl<CONTEXT, HERE: Catori> Path<Food<CONTEXT, HERE>> for Grass
    where CONTEXT: Path<Nil<CONTEXT>>
{
    type Context = CONTEXT;
    type There = Nil<CONTEXT>;

    fn next(self) -> Self::There {

        Self::There::default()
    }
}


///The existence of a Lion implies a path to Meat
///impl Path<Meat> for Lion {}
///The existence of a Lion implies the existence of a path from Animal to Meat
///impl Path<Animal<Meat>> for Lion {}
///The existence of a Cow implies the existence of a path from Animal to Grass
impl Path<Animal<Grass>> for Cow {
    type Context = Animal<Grass>;
    type There = Nil<Self::Context>;
    fn next(self) -> Self::There {

        Self::There::default()
    }
}
///The existence of a Cow implies the existence of Grass
impl Path<Cow> for Grass {
    type Context = Animal<Grass>;
    type There = Nil<Self::Context>;
    fn next(self) -> Self::There {

        Self::There::default()
    }}
///The existence of Grass implies a Path to Cow
impl Path<Grass> for Cow {
    type Context = Animal<Grass>;
    type There = Nil<Self::Context>;

    fn next(self) -> Self::There {

        Self::There::default()
    }}
///Lions can acquire Meat

trait Consumer<HERE: Path<THERE>, THERE: Free>: Path<HERE, There = THERE> {
    fn consume(here: HERE, _there: THERE) -> HERE {
        here
    }
}

impl<PATH: Path<HERE>, HERE: Default> Consumer<HERE, <PATH as Path<HERE>>::There> for PATH
    where HERE: Path<<PATH as Path<HERE>>::There>
{
}

trait Eater<FOOD: Free>: Consumer<Self, FOOD>
    where Self: Path<FOOD>
{
    fn eats(self, _food: FOOD) -> Self {
        //Consumer::consume(self,food)
        self
    }
}

trait Getter<FOOD: Default>: Path<FOOD> {
    fn gets(self) -> FOOD {
        FOOD::default()
    }
}



impl Getter<Lion> for Lion {}
///Cows can acquire Grass
impl Getter<Grass> for Cow {}
///Lions can consume Meat
// impl Eater<<Lion as Path<Lion>>::T> for Path<Lion>
// {
// }
///Cows can consume Grass
//impl Eater<<Cow as Path<Cow>>::T> for Cow {}




fn main() {
    let _leo = Lion::default();
    let _milka = Cow::default();
    //let meat: Meat = Meat::default();
    //leo.eats(meat);
    // milka.eats(milka.gets());
    //milka.eats(leo.gets());
    //let lambda: Animal<F> = Animal::default();

    //     let ua = UA{};
    // type a = ua::h;
    //     struct B();
    //   let A = Universe
    //   type AA = Path
    //   type B = Universe<Nil,A,B>;
}
