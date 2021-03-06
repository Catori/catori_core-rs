use serde_derive::{Deserialize, Serialize};
use serde_lexpr::{from_str, to_string};

use crate::Receiver::{Gate, Observer};
use crate::{GateID, Receiver};
use lexpr::{
    parse::Error as ParseError,
    Cons as ConsCell, Number, Value as SExprValue, Value,
    Value::{
        Bool, Bytes, Char, Cons, Keyword, Nil, Null, Number as ValNumber, String as SString,
        Symbol, Vector,
    },
};
use std::convert::TryInto;
use std::fmt::{Display, Error, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct LogicGate {
    pub a: Receiver,
    pub b: Receiver,
    pub gate_type: GateType,
    pub id: GateID,
}

impl Display for LogicGate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl LogicGate {
    pub fn set_a(mut self, a_gate: LogicGate) -> Self {
        self.a = Receiver::Gate(Box::new(a_gate));
        self
    }

    pub fn set_b(mut self, b_gate: LogicGate) -> Self {
        self.b = Receiver::Gate(Box::new(b_gate));
        self
    }

    pub fn get_open_input_ids(self, ids: &mut Vec<GateID>) -> Vec<GateID> {
        //  dbg!(&gate);

        match self.a {
            // Receiver::Gate(g) => g.get_open_input_ids(ids),
            Receiver::None => {
                ids.push(self.id.clone());
            }
            Receiver::Observer(o) => {}
            Receiver::Gate(g) => {
                g.get_open_input_ids(ids);
            }
        }

        match self.b {
            // Receiver::Gate(g) => g.get_open_input_ids(ids),
            Receiver::None => {
                ids.push(self.id);
            }
            Receiver::Observer(o) => {}
            Receiver::Gate(g) => {
                g.get_open_input_ids(ids);
            }
        }
        ids.to_vec()
    }
}

// impl TryInto<LogicGate> for Value {
//     type Error = ();
//
//     fn try_into(self) -> Result<LogicGate, Self::Error> {
//         match {
//
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
pub enum GateType {
    NAnd,
    XOr,
}
