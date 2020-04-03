use crate::{GateType, LogicGate, Observer};
use serde_derive::{Deserialize, Serialize};
use std::sync::Weak;

#[derive(Default)]
pub struct Universe {
    gates: Vec<Weak<LogicGate>>,
    inputs: Vec<Input>,
    observers: Vec<Weak<Observer>>,
    id_counter: GateID,
}

impl Universe {
    pub fn nand(&mut self) -> LogicGate {
        self.gate(GateType::NAnd)
    }

    pub fn xor(&mut self) -> LogicGate {
        self.gate(GateType::XOr)
    }

    fn gate(&mut self, gate_type: GateType) -> LogicGate {
        self.id_counter.0 += 1;
        let gate = LogicGate {
            a: Receiver::None,
            b: Receiver::None,
            gate_type,
            id: self.id_counter.clone(),
        };
        gate
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Receiver {
    None,
    Gate(Box<LogicGate>),
    Observer(Box<Observer>),
}

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Input {
//     True,
//     False,
// }

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GateID(usize);

pub enum Input {
    A,
    B,
}
