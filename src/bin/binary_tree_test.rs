use log::Log;
use serde::export::PhantomData;
use std::collections::HashMap;
use std::future::Future;
use std::rc::{Rc, Weak};

#[derive(Default)]
pub struct Universe {
    obj_counter: usize,
    //symbols: HashMap<String, Box<Assignable>>,
    gates: Vec<Weak<TwoInputGate>>,
    outputs: Vec<Output>,
    inputs: Vec<Input>,
}

impl Universe {
    // fn new() -> Self {}
    // fn gate(&mut self) -> impl LogicGate {
    //     BinaryGate {
    //         Q: Default::default(),
    //         A: Default::default(),
    //         B: Default::default(),
    //         id: 0,
    //         U: Default::default(),
    //     }
    // }
}

trait LogicGate {
    //  type GateType;
    fn truth(&self, a: TruthGenerator, b: TruthGenerator) -> TruthTable;
}

// impl LogicGate for BinaryGate {
//     fn truth(&self, a_iter: TruthGenerator, b_iter: TruthGenerator) -> TruthTable {
//         for a in a_iter {
//             for b in b_iter {}
//         }
//     }
// }

trait Cat: Sized {
    fn successor(self) -> Option<Self>;
}

// impl Cat for TruthGenerator {
//     fn successor(self) -> Option<Self> {
//         match self.0 {
//             None => Some(F),
//             F => Some(T),
//             T => None,
//         }
//     }
// }
struct TruthGenerator(Option<TruthValue>);

// impl Iterator for TruthGenerator {
//     type Item = TruthValue;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.0 {
//             Some(this) => todo!(), //this..successor(),
//             None => None,
//         }
//     }
// }

pub struct BinaryGate {
    //    Q: OpWeak<NANDCircuit>,
    Q: Output,
    A: Input,
    B: Input,
    id: usize,
    U: Weak<Universe>,
}

#[derive(Clone)]
enum TruthValue {
    F,
    T,
}

use TruthValue::*;

struct TwoInputTruthTable(TruthValue, TruthValue, TruthValue, TruthValue);

const NAND: TwoInputTruthTable = TwoInputTruthTable(T, T, T, F);
const AND: TwoInputTruthTable = TwoInputTruthTable(T, T, T, F);
const OR: TwoInputTruthTable = TwoInputTruthTable(T, T, T, F);
const NOR: TwoInputTruthTable = TwoInputTruthTable(T, T, T, F);
const XOR: TwoInputTruthTable = TwoInputTruthTable(T, T, T, F);
const XNOR: TwoInputTruthTable = TwoInputTruthTable(T, T, T, F);

struct OneInputGate(TruthValue, TruthValue);

struct TwoInputGate(TruthValue, TruthValue, TruthValue, TruthValue);

const Buffer: OneInputGate = OneInputGate(F, T);
const NOT: OneInputGate = OneInputGate(T, F);

// const NAND: TwoInputGate<NANDGate> = TwoInputGate(NANDGate);
// const AND: TwoInputGate<ANDGate> = TwoInputGate(ANDGate);
// const OR: TwoInputGate<ORGate> = TwoInputGate(F, T, T, T);
// const NOR: TwoInputGate<NORGate> = TwoInputGate(T, F, F, F);
// const XOR: TwoInputGate<XORGate> = TwoInputGate(F, F, T, T);
// const XNOR: TwoInputGate<XNORGate> = TwoInputGate(T, F, F, T);

// impl LogicGate for TwoInputGate {
//     fn truth(&self, a: TruthGenerator, b: TruthGenerator) -> TruthGenerator {
//         use TruthValue::*;
//         match (a.0, b.0) {
//             (F, F) => self.0.clone(),
//             (F, T) => self.1.clone(),
//             (T, F) => self.2.clone(),
//             (T, T) => self.3.clone(),
//         }
//     }
// }
enum Input {
    Variable,
    Fixed(TruthValue),
    Gate(Box<TwoInputGate>),
}

enum Output {
    Unknown,
    Fixed(TruthValue),
    TruthTable(TruthTable),
}

impl Default for Output {
    fn default() -> Self {
        Output::Unknown
    }
}

#[derive(Clone)]
struct TruthTable();

impl Default for Input {
    fn default() -> Self {
        Input::Variable
    }
}

impl Default for TruthValue {
    fn default() -> Self {
        TruthValue::F
    }
}

//
//
//
// trait NOT {}
// trait NOR {}
// trait XOR {}
//
// impl LogicGate for NOT {
//     type GateType = BinaryGate<NAND>;
//
//     fn truth(a: TruthValue, b: TruthValue) -> TruthValue {
//         unimplemented!()
//     }
// }
//
//

//
// impl LogicGate for LogicGate<GateType=NAND> {
//     type GateType = BinaryGate<NAND>;
//
//     fn truth(a: TruthValue, b: TruthValue) -> TruthValue {
//         use TruthValue::*;
//         match (a, b) {
//             (F, F) => T,
//             (F, T) => T,
//             (T, F) => T,
//             (T, T) => F,
//         }
//     }
// }
//
// impl Universe {
//     ///A Gate builder. The only way to create new gates
//     /// Creates a default gate with no inputs or outputs, and assigns a universally unique id
//     /// Returns a boxed refernce counted value so that the universe can hold a weak reference to
//     /// everything
//     fn gate(&mut self) -> Box<Rc<impl LogicGate>> {
//         let gate = Gate::NAND(BinaryGate {
//             Q: Default::default(),
//             A: Default::default(),
//             B: Default::default(),
//             id: 0,
//             U: Default::default(),
//         });
//
//         let rc_gate = Rc::new(gate);
//         self.gates.push(Rc::downgrade(&rc_gate));
//         Box::new(rc_gate)
//     }
// }
//
//
// trait Generator<GATETYPE> {
//     fn gate(&mut self) -> BinaryGate<GATETYPE> {
//         BinaryGate {
//             Q: Default::default(),
//             A: Default::default(),
//             B: Default::default(),
//             id: 0,
//             U: Default::default(),
//         }
//     }
// }
//
// trait Assignable {}
//
// trait Identifiable {
//     fn id(&self) -> usize;
// }
//
// // impl Assignable for LogicGate {}
// impl Assignable for Input {}
// impl Assignable for Output {}
//
// // impl Identifiable for LogicGate{
// //     fn id(&self) -> usize {
// //         self.
// //     }
// // }
// // impl Identifiable for Input{}
// // impl Identifiable for Output{}
//
// trait GateType: Sized {
//     // NAND,
//     // OR,
// }
//

// impl LogicGate for BinaryGate<NAND> {
//     type GateType = NAND
//
//     fn truth(a: TruthValue, b: TruthValue) -> TruthValue {
//         match self. {
//             (Gate::NAND(a),Gate::NAND(b)) => true
//         }
//     }
//     // fn convert_tree(mut self) -> Self {
//     //     todo!()
//     // }
//     //
//     // fn print_in_order(&self) {
//     //     todo!()
//     // }
//     //
//     // fn assign_A(&mut self, a: Input) {
//     //     self.A = a
//     // }
//     // fn assign_B(&mut self, b: Input) {
//     //     self.B = b
//     // }
// }

fn main() {
    let mut u = Universe::default();
    // let a = u.gate();
    // let b = u.gate();
    // l
    // let mut root = LogicGate::default();
    // root.assign_A(True.clone());
}
