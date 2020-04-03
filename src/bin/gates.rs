use catori_core::{Catori, Free};
use core::fmt::Display;
use lexpr::parse::NilSymbol;
use lexpr::{
    parse::Error as ParseError,
    Cons as ConsCell,
    Number,
    Value as SExprValue,
    Value, //Value::{Bool, Bytes, Char, Cons, Keyword, Nil, Null, Number as ValNumber, String as SString, Symbol, Vector},
};
use log::info;
use serde::de::Unexpected::Unit;
use serde::export::PhantomData;
use std::cmp::Ordering;
use std::collections::hash_set::IntoIter;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::sync::atomic::AtomicBool;

fn main() {
    let nand_gate1 = &mut Gate::default();
    let nand_gate2 = &mut Gate::default();
    let nand_gate3 = &mut Gate::default();

    nand_gate2.q = Some(nand_gate1);
    nand_gate3.q = Some(nand_gate2);
    // dbg!(nand_gate1.length());
    // dbg!(nand_gate2.length());
    // dbg!(nand_gate3.length());
    // // nand_gate.b = Some(Box::new(nand_gate2));
    //  let circuit = nand_gate.
    //  dbg!(nand_gate.truths());
    //dbg!(nand_gate.truths(vec!(true, true)));
    //let truths = all_truths(nand_gate3.clone());
    //    let truths = truths(nand_gate, vec![true, false]);
    //    dbg!(truths);

    dbg!(all_truths(nand_gate2.to_owned()));
}

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub struct Bool(bool);

impl Catori for Bool {
    fn length(&self) -> usize {
        1
    }
}

type Bools = TypeIterator<bool>;

#[derive(Default)]
pub struct TypeIterator<T: Catori> {
    lastval: Option<T>,
}

impl Iterator for TypeIterator<bool> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.lastval = match self.lastval {
            None => Some(false),
            Some(false) => Some(true),
            Some(true) => None,
        };
        self.lastval
    }
}

trait GateType: Sized {}
struct NANDGate();
struct NORGate();

impl GateType for NANDGate {}
impl GateType for NORGate {}

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Gate<'a> {
    a: PhantomData<Gate<'a>>,
    b: Option<&'a Gate<'a>>,
    q: Option<&'a Gate<'a>>,
}

// struct GateIterator<'a>{gate:Gate<'a>}
//
// impl<'a> Iterator for GateIterator<'a> {
//     type Item = &'a Gate<'a>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.gate.q {
//             None => {
//              let item = self.gate.b;
//                 self.gate = item;
//                 item
//             }
//             Some(child) => {
//                 let item = &self.gate;
//                 self.gate = item.b;
//                 item
//             }
//         }
//         Some(self)
//     }
// }

impl<'a> Catori for Gate<'a> {
    fn length(&self) -> usize {
        match &self.q {
            Some(q) => 1 + q.length(),
            None => 1,
        }
    }
    // fn _truths(self, mut inputs: Vec<bool>) -> Vec<bool> {
    //
    // }
}

pub fn all_truths(gate: Gate) -> IntoIter<bool> {
    let mut truths = HashSet::<bool>::default();
    for a in TypeIterator::<bool>::default() {
        for b in TypeIterator::<bool>::default() {
            match gate.q {
                Some(q) => {
                    for truth in &mut all_truths(q.clone()) {
                        truths.insert(truth);
                    }
                }
                None => {
                    // for truth in &mut all_truths(q.clone()) {
                    //     truths.insert(truth);
                    // }
                    // let value = truth(Gate::default(), a, b);
                    //
                    // truths.insert(value);
                }
            }

            //dbg!(a, b, value);
        }
    }
    truths.into_iter()
}

fn truth(gate: Gate, a: bool, b: bool) -> bool {
    use std::sync::atomic::Ordering;
    let a = AtomicBool::new(a);
    a.fetch_nand(b, Ordering::SeqCst);
    a.into_inner()
}

// ///The only thing a gate contains is the truth output values)
// #[derive(Debug, Clone)]
// pub struct Symbol(&'static str);
// #[derive(Debug, Clone)]
// pub struct Gate {
//     symbol: Symbol,
//     expr: String,
//     truth: Vec<TruthTriple>,
// }

// #[derive(Debug, Clone)]
// struct TruthTriple(bool, bool, bool);
//
// #[derive(Debug, Clone)]
// struct TruthTable(Vec<TruthTriple>);
//
// #[derive(Debug, Clone, Default, PartialOrd, Ord, PartialEq, Eq)]
// pub struct Circuit<A: Catori, B: Catori, Q: Catori>(A, B, PhantomData<Q>);

// impl Gate {
//     fn new(symbol: Symbol, expr: String, truth: Vec<TruthTriple>) -> Self {
//         Gate { symbol, expr, truth }
//     }
//     fn truth(&self, a: bool, b: bool) -> bool {
//         match (a, b) {
//             (false, false) => self.truth.get(0).unwrap().2,
//             (false, true) => self.truth.get(1).unwrap().2,
//             (true, false) => self.truth.get(2).unwrap().2,
//             (true, true) => self.truth.get(3).unwrap().2,
//         }
//     }
// }

// ///An abstract catori structure with types reflecting the A,B,Q i/o terminology of logic gates
// pub trait Catori: Debug + Clone + Ord {
//     type A;
//     type B;
//     type Q;
//
//     ///take two things and treat them as if they are one
//     fn entangle(self, other: Self::A) -> (Self::A, Self) {
//         (other, self)
//     }
//
//     fn truths(self, values: Vec<Self::A>) -> Vec<Self::Q>;
//
//     fn truth(self, a: Self::A, b: Self::B) -> Self::Q;
// }

// impl<C> Into<Cat<C>> for C {
//     fn into(self) -> Cat<C> {
//         Cat(self)
//     }
// }

// impl<C: Free> From<C> for Gate {
//     fn from(c: C) -> Self {
//         Gate(c)
//     }
// }

// #[derive(Debug, Clone, Default, PartialOrd, Ord, PartialEq, Eq)]
// pub struct Cat<A: Catori>(A);

//pub struct CatIter(BoxCatori>);
// #[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
// pub struct Bool(bool);
// impl Catori for Bool {
//     type A = Bool;
//     type B = Bool;
//     type Q = Bool;
//
//     fn truths(self, values: Vec<Self::A>) -> Vec<Self::Q> {
//         unimplemented!()
//     }
//
//     fn truth(self, a: Self::A, b: Self::B) -> Self::Q {
//         unimplemented!()
//     }
// }

// impl Catori for bool {
//     type A = bool;
//     type B = bool;
//     type Q = bool;
//
//     fn truths(self) -> Vec<Self::Q> {
//         let mut vec = vec![];
//         for &a in [false, true].iter() {
//             for &b in [false, true].iter() {
//                 vec.push(self.clone().truth(a, b))
//             }
//         }
//         vec
//     }
//
//     fn truth(self, a: Self::A, b: Self::B) -> Self::Q {
//         unimplemented!()
//     }
// }

// impl Catori for Circuit<bool, bool, bool> {
//     type A = Cat<bool>;
//     type B = Cat<bool>;
//     type Q = Cat<bool>;
//
//     fn truths(self) -> Vec<Self::Q> {
//         let mut vec = vec![];
//         for &a in [false, true].iter() {
//             for &b in [false, true].iter() {
//                 vec.push(self.clone().truth(Cat(a), Cat(b)))
//             }
//         }
//         vec
//     }
//
//     fn truth(self, a: Self::A, b: Self::B) -> Self::Q {
//         unimplemented!()
//     }
// }

// impl Catori for NANDGate {
//     fn truths(self, mut inputs: Vec<bool>) -> Vec<Self::Q> {
//         let mut vec = vec![];
//         match inputs.pop() {
//             Some(input) => match self.b.clone() {
//                 Some(b) => vec.append(&mut b.truths(inputs)),
//
//                 None => {
//                     for &b in [false, true].iter() {
//                         vec.push(self.clone().truth(b, NANDGate::default().a))
//                     }
//                 }
//             },
//             None => {
//                 for &a in [false, true].iter() {
//                     match self.b.clone() {
//                         Some(b) => vec.append(&mut b.truths(vec![])),
//
//                         None => {
//                             for &b in [false, true].iter() {
//                                 vec.push(self.clone().truth(a, b))
//                             }
//                         }
//                     }
//                 }
//             } //    }
//         }
//         vec
//     }
//
//     fn truth(self, a: bool, b: bool) -> bool {
//         use std::sync::atomic::Ordering;
//         let a = AtomicBool::new(a);
//         a.fetch_nand(b, Ordering::SeqCst);
//         a.into_inner()
//     }
//
//     type A = bool;
//     type B = bool;
//     type Q = bool;
// }

// #[derive(Clone, Debug, Default)]
// pub struct SymbolTable(HashMap<String, NANDGate>);
//
// impl SymbolTable {
//     fn add(&mut self, gate: Gate) -> &mut Self {
//         self.0.insert(gate.symbol.0.to_string(), gate);
//         self
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Path {
//     expr: SExprValue,
//     symbols: SymbolTable,
// }
//
// impl Path {
//     pub fn new(gate: Gate) -> Self {
//         Path {
//             expr: lexpr::from_str(&gate.expr).unwrap(),
//             symbols: SymbolTable({
//                 let mut m = HashMap::new();
//                 m.insert(gate.expr.clone(), gate);
//                 m
//             }),
//         }
//     }
//     pub fn eval(self, symbols: SymbolTable) -> Vec<bool> {
//         use SExprValue::*;
//         match self.expr {
//             Symbol(sym) => todo!("{}", sym),
//             Null | Nil => todo!(),
//             Bool(b) => todo!(),
//             Number(n) => todo!(),
//             Char(c) => todo!(),
//             String(s) => todo!(),
//             Keyword(k) => todo!(),
//             Bytes(b) => todo!(),
//             Cons(cons) => match cons.car() {
//                 Symbol(sym) => {
//                     for internal_sym in self.symbols.0 {
//                         dbg!(&internal_sym);
//                     }
//                     todo!()
//                     //                    let gate = self.1.(0.
//                 }
//                 _ => todo!(),
//             },
//             Vector(v) => todo!(), //_ => panic!("{}: sexpr must start with a recognized symbol", self.0),
//         }
//     }
// }

// pub fn truths(gate: NANDGate, mut truths: Vec<bool>) -> Vec<bool> {
//     match truths.pop() {
//         Some(a) => match truths.pop() {
//             Some(b) => vec![todo!()],
//             None => truths(vec),
//         },
//         None => all_truths(gate),
//     }
// }

// fn main() {
//     let mut nand_gate = NANDGate::default();
//     let mut nand_gate2 = NANDGate::default();
//     //nand_gate2.b = Some(Box::new(NANDGate::default()));
//     // nand_gate.b = Some(Box::new(nand_gate2));
//     //  let circuit = nand_gate.
//     //  dbg!(nand_gate.truths());
//     //dbg!(nand_gate.truths(vec!(true, true)));
//     let truths = all_truths(nand_gate2);
//     //    let truths = truths(nand_gate, vec![true, false]);
//     dbg!(truths);
// dbg!(nand_gate.truth());
// let nand_gate: Gate = Gate {
//     symbol: Symbol("NAND"),
//     expr: "(NAND)".to_string(),
//     truth: vec![
//         TruthTriple(false, false, true),
//         TruthTriple(false, true, true),
//         TruthTriple(true, false, true),
//         TruthTriple(true, true, false),
//     ],
// };
// let not_gate: Gate = Gate {
//     symbol: Symbol("NOT"),
//     expr: "(NAND a a)".to_string(),
//     truth: vec![TruthTriple(false, false, true), TruthTriple(true, true, false)],
// };
//
// let nor_gate: Gate = Gate {
//     symbol: Symbol("NOR"),
//     expr: "(NOT (NAND (NOT a) (NOT b)))".to_string(),
//     truth: vec![
//         TruthTriple(false, false, true),
//         TruthTriple(false, true, false),
//         TruthTriple(true, false, false),
//         TruthTriple(true, true, false),
//     ],
// };
// let and_gate: Gate = Gate {
//     symbol: Symbol("AND"),
//     expr: "(NOT (NAND a b))".to_string(),
//     truth: vec![
//         TruthTriple(false, false, false),
//         TruthTriple(false, true, false),
//         TruthTriple(true, false, false),
//         TruthTriple(true, true, true),
//     ],
// };
// let or_gate: Gate = Gate {
//     symbol: Symbol("OR"),
//     expr: " (NAND ((NOT a) (NOT a)))".to_string(),
//     truth: vec![
//         TruthTriple(false, false, false),
//         TruthTriple(false, true, true),
//         TruthTriple(true, false, false),
//         TruthTriple(true, true, false),
//     ],
// };
//
// let mut symbols = SymbolTable::default();
// symbols.add((nand_gate.clone()));
// let nand_universe = Path::new(nand_gate);

// dbg!(NAndGate.truth(true, false));
// dbg!(NAndGate.truth(true, true));
// dbg!(NOrGate.truth(true, false));

// let mut sexprs = vec![];

//    let path = Universe::new(lexpr::from_str("(let NOT (NAND A A))").unwrap(), SymbolTable::default());
//    dbg!(nand_universe.clone());

//assert_eq!(nand_universe,)
//    dbg!(nand_universe.eval(SymbolTable::default().add()));
//}
