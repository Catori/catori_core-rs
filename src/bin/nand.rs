#![deny(unused_variables)]

use catori_core::{LogicGate, Receiver, Universe};
use lexpr::{
    parse::Error as ParseError,
    Cons as ConsCell, Number, Value as SExprValue, Value,
    Value::{
        Bool, Bytes, Char, Cons, Keyword, Nil, Null, Number as ValNumber, String as SString,
        Symbol, Vector,
    },
};
use std::borrow::Borrow;

//struct Input()

//type NAndGate = GenericLogicGate

//struct Universe

fn main() {
    let mut u = Universe::default();
    let (gate1, gate2) = (u.nand(), u.xor());
    let (gate4, gate5, gate6) = (u.nand(), u.nand(), u.nand());
    let gate4 = gate4.set_a(gate5.set_b(gate6));
    //let gate3 = gate3.set_B(gate4).set_A(gate1);
    let gate1 = gate1.set_a(gate2).set_b(gate4);

    //  dbg!("foo: {:?}", gate1);

    // qa

    let mut sexprs = vec![];
    // sexprs.push("(true)");

    // sexprs.push("(()())");
    // sexprs.push(" (()(()))");
    //    sexprs.push("(true true)");

    //The following syntax presupposes that there is a primitive NAND type that all other types are built from.
    //A lisp like let operation can alias multiple connected NAND gates.
    //A NAND gate has two inputs, and therefore must be called with two arguments. If the arguments are
    //previously defined structural aliases (using 'let'), then substitution is performed.
    //If any of the arguments are unknown, then those are implicitly defined variable names that can be used to assign inputs
    //later on.

    //A [NOT](https://en.wikipedia.org/wiki/NAND_logic#NOT) is a NAND where both inputs are constrained to the same variable
    sexprs.push("(let NOT (NAND a a)");

    //An [AND](https://en.wikipedia.org/wiki/NAND_logic#AND) is a NOT who's only input is a NAND
    sexprs.push("(let AND (NAND (NAND a b) (NAND a a) b)");
    sexprs.push("(let AND (NOT (NAND a b))");

    //An [OR](https://en.wikipedia.org/wiki/NAND_logic#OR) is a NAND who's inputs are two different NOT gates
    sexprs.push("(let OR (NAND ((NOT a) (NOT a))))");
    sexprs.push("(let OR (NAND ( NAND A  A ) ( NAND  B B )");

    //An [NOR](https://en.wikipedia.org/wiki/NAND_logic#NOR) is a NOT who's only input is a NAND gate, whose inputs, in turn are
    //two different NOT gates.
    sexprs.push("(let NOR (NOT (NAND (NOT a) (NOT b))))");
    sexprs.push(" (let NOR (NAND ( NAND ( NOT A )  ( NOT B ) ) ( NAND ( NOT A ) ( NOT B ) ) ))");
    //or "NOT-simplified"
    sexprs.push(" (let NOR (NOT ( NAND ( NOT A )  ( NOT B ))))");

    //An [XOR](https://en.wikipedia.org/wiki/NAND_logic#XOR)
    //TODO simplify this?
    sexprs.push(
        " 
    (let XOR (NAND ( NAND A ( NAND A B ) ) 
                   ( NAND B ( NAND A B ) )
              )
     )",
    );

    sexprs.push(
        " (let XNOR 
                    (NAND (NAND ( NOT A ) ( NOT B ) ) 
                          ( NAND A B ))    
                 )",
    );

    sexprs.push(
        "let MUX 
        OR   (AND A  
                  (NOT S ) )
             ( AND B S )
        )",
    );

    sexprs.push(
        "let DEMUX 
        OR   (AND A  
                  (NOT S ) )
             ( AND B S )
        )",
    );

    // sexprs.push("(nand (nand nand) nand)");
    // sexprs.push("(nand (nand nand nand))");
    // sexprs.push("(* nand nand)");
    // sexprs.push("(* (nand nand) (nand))");
    // sexprs.push("(* (nand nand nand) (nand nand))");
    // //There is no awareness of digits yet. this is just a syntactic example
    // sexprs.push("(nand nand nand nand (3 1))");

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
        println!("{}{}{:?}", prefix, "|- ", node.a);
        match &node.a {
            Receiver::Gate(a) => pprint_tree(a.borrow(), prefix.to_string()),
            Receiver::None => (),
            Receiver::Observer(_o) => unimplemented!(),
        }
        println!("{}{}{:?}", prefix, "- ", node.b);
        match &node.b {
            Receiver::Gate(b) => pprint_tree(b.borrow(), prefix.to_string()),
            Receiver::None => (),
            Receiver::Observer(_o) => unimplemented!(),
        }
    }

    //   pprint_tree(&node, "".to_string());
}
