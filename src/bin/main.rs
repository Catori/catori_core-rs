use std::marker::PhantomData;
use log::{error,warn,info};

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



//
// fn main() {
//     let _leo = Lion::default();
//     let _milka = Cow::default();
//     //let meat: Meat = Meat::default();
//     //leo.eats(meat);
//     // milka.eats(milka.gets());
//     //milka.eats(leo.gets());
//     //let lambda: Animal<F> = Animal::default();
//
//     //     let ua = UA{};
//     // type a = ua::h;
//     //     struct B();
//     //   let A = Universe
//     //   type AA = Path
//     //   type B = Universe<Nil,A,B>;
// }

//use serde_lexpr::{from_str, to_string};
use lexpr::{sexp,Value,Cons};


// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct Observation(, String, u8);

struct Observation {
    lens: Cons,
    path: Cons,
}

fn main() {
    // let v1: Vec<u32> = from_str("(1 2 3)").unwrap();
    // assert_eq!(v1, vec![1, 2, 3]);
    // assert_eq!(to_string(&v1).unwrap(), "(1 2 3)".to_string());
    // let v2: Vec<u32> = from_str("#(1 2 3)").unwrap();
    // assert_eq!(v1, v2);
    // assert_eq!(to_string(&v2).unwrap(), "(1 2 3)".to_string());
    // dbg!(v1);
    // dbg!(v2);
    //
    //
    // let address = sexp!(((name . "Jane Doe") (street . "4026 Poe Lane")));
    // dbg!(address);
    //
    //
    // let names = Value::list(vec!["Alice", "Bob", "Mallory"]);
    // println!("The bad guy is {}", names[2].as_str().unwrap());
    let string = "(? _ 3)";
    let catexp = lexpr::from_str(&string).expect("parsing failed");

    match catexp[0].as_symbol().unwrap() {
        "?" => Observation{lens: *catexp[1].as_cons().unwrap(), path: *catexp[2].as_cons().unwrap()},
        other=> {todo!("{:?} not supported. Can only observe with ?", other);panic!()}
    }
   // let catexp:Cons = from_str("(?)").unwrap();
    dbg!(catexp);
}