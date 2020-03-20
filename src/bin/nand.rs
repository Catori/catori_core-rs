#![deny(unused_variables)]

use catori_core::{LogicGate, Receiver, Universe};
use lexpr::{
    parse::Error as ParseError,
    Cons as ConsCell, Number, Value as SExprValue, Value,
    Value::{Bool, Bytes, Char, Cons, Keyword, Nil, Null, Number as ValNumber, String as SString, Symbol, Vector},
};
use std::borrow::Borrow;

//struct Input()

//type NAndGate = GenericLogicGate

fn main() {
    let mut u = Universe::default();
    let (gate1, gate2) = (u.nand(), u.xor());
    let (gate4, gate5, gate6) = (u.nand(), u.nand(), u.nand());
    let gate4 = gate4.set_A(gate5.set_B(gate6));
    //let gate3 = gate3.set_B(gate4).set_A(gate1);
    let gate1 = gate1.set_A(gate2).set_B(gate4);

    //  dbg!("foo: {:?}", gate1);

    // qa

    let mut sexprs = vec![];
    // sexprs.push("(true)");
    sexprs.push("(true true)");
    sexprs.push("(true (true true) true)");
    sexprs.push("(true (true _) true)");
    sexprs.push("(* true true)");
    sexprs.push("(* (true true) (true))");
    sexprs.push("(* (true true true) (true true))");
    //There is no awareness of digits yet. this is just a syntactic example
    sexprs.push("(true true true true (3 1))");

    let path = "(nand xor (xor xor) nand)";

    let _path = Path(lexpr::from_str(path).unwrap());
    println!("{}", gate1);

    use serde_lexpr::{from_str, to_string};

    let v1: Vec<u32> = from_str("(1 2 3)").unwrap();
    assert_eq!(v1, vec![1, 2, 3]);
    to_string(&gate1).unwrap();

    let mut ids = vec![];

    dbg!(gate1.get_open_input_ids(&mut ids));
    // let v2: Vec<u32> = from_str("#(1 2 3)").unwrap();
    // assert_eq!(v1, v2);
    // assert_eq!(to_string(&v2).unwrap(), "(1 2 3)".to_string());
}

#[derive(Debug, Clone)]
pub struct Path(SExprValue);

impl Into<Path> for SExprValue {
    fn into(self) -> Path {
        Path(self)
    }
}

fn pprint_tree(_node: LogicGate) {
    fn pprint_tree(node: &LogicGate, prefix: String) {
        //        let prefix_current = if last { "`- " } else { "|- " };
        println!("{}{}{:?}", prefix, "|- ", node.A);
        match &node.A {
            Receiver::Gate(a) => pprint_tree(a.borrow(), prefix.to_string()),
            Receiver::None => (),
            Receiver::Observer(_o) => unimplemented!(),
        }
        println!("{}{}{:?}", prefix, "- ", node.B);
        match &node.B {
            Receiver::Gate(b) => pprint_tree(b.borrow(), prefix.to_string()),
            Receiver::None => (),
            Receiver::Observer(_o) => unimplemented!(),
        }
    }

    //   pprint_tree(&node, "".to_string());
}
