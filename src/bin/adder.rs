use catori_core::{Catori, Nil, Path};
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

#[derive(Default)]
struct Universe {
    gates: Vec<Rc<Gate>>,
    inputs: HashSet<Rc<Symbol>>,
    observations: HashSet<Rc<Symbol>>,
}

#[derive(Debug)]
pub enum GateType {
    NAnd,
    XOr,
}

#[derive(Debug)]
struct Gate {
    g: GateType,
    a: Input,
    b: Input,
    q: Weak<Output>,
}

#[derive(Debug)]
enum Input {
    Gate(Box<Gate>),
    Hole(Symbol),
}

#[derive(Debug)]
enum Output {
    NAND(Box<Gate>),
    Observable(Symbol),
}

impl Default for Output {
    fn default() -> Self {
        Output::Observable(Symbol::Observable("".to_string()))
    }
}

impl Default for Gate {
    fn default() -> Self {
        Gate {
            g: GateType::NAnd,
            a: Default::default(),
            b: Default::default(),
            q: Default::default(),
        }
    }
}

impl Universe {
    fn nand(&mut self) -> Weak<Gate> {
        let gate = Gate {
            g: GateType::NAnd,
            a: Default::default(),
            b: Default::default(),
            q: Rc::downgrade(&Rc::new(Output::Observable(Symbol::Observable(
                "".to_string(),
            )))),
        };
        let gate = Rc::new(gate);
        let weak_gate = Rc::downgrade(&gate);
        self.gates.push(gate);
        weak_gate
    }
}

impl Default for Input {
    fn default() -> Self {
        Input::Hole(Symbol::Hole("".to_string()))
    }
}

#[derive(Debug)]
enum Symbol {
    Observable(String),
    Hole(String),
}

//
//
// impl Default for GateType {
//     fn default() -> Self {
//         GateType::Hole("".into())
//     }
// }
//
//
//
//
// impl Gate {
//     fn nand() -> Self {
//         Gate {
//             g: GateType::NAnd,
//             a: Default::default(),
//             b: Default::default(),
//             q: Default::default(),
//         }
//     }
//
//     fn xor() -> Self {
//         Gate {
//             g: GateType::XOr,
//             a: Default::default(),
//             b: Default::default(),
//             q: Default::default(),
//         }
//     }
// }
//

// impl Gate {
//     //     pub fn entangle_a(&mut self, other: Gate) {
//     //         match self.a {
//     //             Input::Single(gate) => {self.a = Input::M},
//     //             Input::Multiple(_) => {},
//     //             Input::Hole => {},
//     //         }
//     //     }
// }
//
//

//

//

//
// #[derive(Default)]
// struct SymbolTable {
//     last_generated: String,
//     table: Vec<String>,
// }
//
// impl Into<Input> for Gate {
//     fn into(self) -> Input {
//         Input::Gate(Box::new(self))
//     }
// }
//
// impl Into<Input> for &str {
//     fn into(self) -> Input {
//         Input::Hole(Symbol(self.to_string()))
//     }
// }

fn main() {
    let mut u = Universe::default();
    let nand1 = u.nand();
    dbg!(nand1.upgrade());

    // let mut nand2 = Gate::nand();
    // let mut nand3 = Gate::nand();
    // let mut xor1 = Gate::xor();
    // let mut xor2 = Gate::xor();
    //
    // let mut switcha = "A".into();
    // let mut switchb = "B".into();
    // let mut switchc = "C".into();

    //     nand1.a = switchb;
    //     xor1.a = switchc;
    //     nand1.b = switchc;
    //     nand2.a = nand1.into();
    //     nand3.a = switcha;
    //     nand3.b = xor1.into();
    //     nand2.b = nand3.into();
}

/*

package gates;

public class NANDGate {
private int input1;
private int input2;

public int getInput1() {
return input1;
}

public void setInput1(int input1) {
this.input1 = input1;
}

public int getInput2() {
return input2;
}

public void setInput2(int input2) {
this.input2 = input2;
}

public NANDGate() {
input1 = 0;
input2 = 0;
}

public NANDGate(int input1, int input2) {
this.input1 = input1;
this.input2 = input2;
}

private int getOutput(int input1,int input2) {
setInput1(input1);
setInput2(input2);
return (this.input1 == 0 || this.input2 == 0) ? 1 : 0;
}

private void printOutput() {
for (int i = 0; i < 2; i++) {
for (int j = 0; j < 2; j++) {
System.out.println(i + " NAND " + j + " results " + getOutput(i,j));
}
}
}

public void printTruthTable() {
System.out.println("----Truth Table of NAND Gate----");
printOutput();
System.out.println();
}
}
*/
