use lexpr::to_string;
use lexpr::from_str;
use lexpr::parse::NilSymbol;
use lexpr::{
    parse::Error as ParseError,
    Cons as ConsCell, Number, Value as SExprValue, Value,
    Value::{
        Bool, Bytes, Char, Cons, Keyword, Nil, Null, Number as ValNumber, String as SString,
        Symbol, Vector,
    },
};
use petgraph::prelude::NodeIndex;
use petgraph::{Directed, Direction, Graph};
use serde::export::fmt::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    let u = Universe::default();

    let gates = vec![
        ("NAND", u.nand()),
        ("NOT", u.not()),
        ("AND", u.and()),
        ("OR", u.or()),
        ("NOR", u.nor()),
        ("XOR", u.xor()),
        ("XNOR", u.xnor()),
//        ("MUX", u.mux()),
        ("Adder", u.full_nand_adder()),
     //   ("SimplifiedAdder", u.full_adder()),
    ];

    for (key, val) in gates {
        let sexpr = defun(&val, &key);
        println!("{}", sexpr);
        let lexpr = from_str(&sexpr).unwrap();
        println!("{:#}", lexpr);
    }
}

#[derive(Default)]
struct Universe {}

impl Universe {
    fn not(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();
        let switcha = dag.add_node(Node::Input(Input("A".to_string())));
        let b = dag.add_node(self.NAND_gate());
        dag.extend_with_edges(&[(switcha, b), (switcha, b)]);
        dag
    }

    fn nand(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();
        let switcha = dag.add_node(Node::Input(Input("A".to_string())));
        let switchb = dag.add_node(Node::Input(Input("B".to_string())));
        let c = dag.add_node(self.NAND_gate());
        dag.extend_with_edges(&[(switchb, c), (switcha, c)]);
        dag
    }

    // fn simple_not(&self) -> Graph<Node, ()> {
    //     let mut dag = Graph::<Node, ()>::new();
    //     let switcha = dag.add_node(Node::Input(Input("A".to_string())));
    //     let b = dag.add_node(self.NOT_gate());
    //     dag.extend_with_edges(&[(switcha, b)]);
    //     dag
    // }

    fn xor(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();
        let switcha = dag.add_node(Node::Input(Input("A".to_string())));
        let switchb = dag.add_node(Node::Input(Input("B".to_string())));
        let c = dag.add_node(self.NAND_gate());
        let d = dag.add_node(self.NAND_gate());
        let e = dag.add_node(self.NAND_gate());
        let f = dag.add_node(self.NAND_gate());

        dag.extend_with_edges(&[
            (switchb, c),
            (switchb, d),
            (switcha, c),
            (switcha, e),
            (c, d),
            (c, e),
            (d, f),
            (e, f),
        ]);
        dag
    }

    // fn mux(&self) -> Graph<Node, ()> {
    //     let mut dag = Graph::<Node, ()>::new();
    //     let in1 = dag.add_node(Node::Input(Input("IN1".to_string())));
    //     let in2 = dag.add_node(Node::Input(Input("IN2".to_string())));
    //     let sel = dag.add_node(Node::Input(Input("SEL".to_string())));
    //     let d = dag.add_node(self.not());
    //     let e = dag.add_node(self.and());
    //     let f = dag.add_node(self.and());
    //     let g = dag.add_node(self.and());
    //
    //     dag.extend_with_edges(&[
    //         (sel, d),
    //         (sel, f),
    //         (in1, e),
    //         (in2, f),
    //         (d, e),
    //         (e, g),
    //         (f, g),
    //     ]);
    //     dag
    // }

    fn or(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();

        let switcha = dag.add_node(Node::Input(Input("A".to_string())));
        let switchb = dag.add_node(Node::Input(Input("B".to_string())));
        let c = dag.add_node(self.NAND_gate());
        let d = dag.add_node(self.NAND_gate());
        let e = dag.add_node(self.NAND_gate());
        dag.extend_with_edges(&[
            (switchb, c),
            (switchb, c),
            (switcha, d),
            (switcha, d),
            (c, e),
            (d, e),
        ]);
        dag
    }

    fn nor(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();

        let switcha = dag.add_node(Node::Input(Input("A".to_string())));
        let switchb = dag.add_node(Node::Input(Input("B".to_string())));
        let c = dag.add_node(self.NAND_gate());
        let d = dag.add_node(self.NAND_gate());
        let e = dag.add_node(self.NAND_gate());
        let f = dag.add_node(self.NAND_gate());
        dag.extend_with_edges(&[
            (switchb, c),
            (switchb, c),
            (switcha, d),
            (switcha, d),
            (c, e),
            (d, e),
            (e, f),
            (e, f),
        ]);
        dag
    }


    fn xnor(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();

        let switcha = dag.add_node(Node::Input(Input("A".to_string())));
        let switchb = dag.add_node(Node::Input(Input("B".to_string())));
        let c = dag.add_node(self.NAND_gate());
        let d = dag.add_node(self.NAND_gate());
        let e = dag.add_node(self.NAND_gate());
        let f = dag.add_node(self.NAND_gate());
        let g = dag.add_node(self.NAND_gate());
        dag.extend_with_edges(&[
            (switchb, d),
            (switchb, d),
            (switchb, e),
            (switcha, c),
            (switcha, c),
            (switcha, e),
            (c, f),
            (d, f),
            (e, g),
            (f, g),
        ]);
        dag
    }

    fn and(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();
        let switcha = dag.add_node((Node::Input(Input("A".to_string()))));
        let switchb = dag.add_node((Node::Input(Input("B".to_string()))));
        let c = dag.add_node(self.NAND_gate());
        let d = dag.add_node(self.NAND_gate());
        dag.extend_with_edges(&[(switchb, c), (switcha, c), (c, d), (c, d)]);
        dag
    }

    fn NAND_gate(&self) -> Node {
        Node::Gate(Gate {
            gate_type: GateType::NAnd,
            value: None,
        })
    }

    // fn XOR_gate(&self) -> Node {
    //     Node::Gate(Gate {
    //         gate_type: GateType::XOr,
    //         value: None,
    //     })
    // }
    //
    // fn OR_gate(&self) -> Node {
    //     Node::Gate(Gate {
    //         gate_type: GateType::Or,
    //         value: None,
    //     })
    // }
    //
    // fn AND_gate(&self) -> Node {
    //     Node::Gate(Gate {
    //         gate_type: GateType::And,
    //         value: None,
    //     })
    // }
    //
    // fn NOT_gate(&self) -> Node {
    //     Node::Gate(Gate {
    //         gate_type: GateType::Not,
    //         value: None,
    //     })
    // }


    // fn full_adder(&self) -> Graph<Node, ()> {
    //     let mut dag = Graph::<Node, ()>::new();
    //     let switcha = dag.add_node((Node::Input(Input("A".to_string()))));
    //     let switchb = dag.add_node((Node::Input(Input("B".to_string()))));
    //     let switchc = dag.add_node((Node::Input(Input("C".to_string()))));
    //     let d = dag.add_node(self.XOR_gate());
    //     let e = dag.add_node(self.AND_gate());
    //     let f = dag.add_node(self.XOR_gate());
    //     let g = dag.add_node(self.AND_gate());
    //     let h = dag.add_node(self.OR_gate());
    //     dag.extend_with_edges(&[
    //         (switchc, f),
    //         (switchc, g),
    //         (switchb, d),
    //         (switchb, e),
    //         (switcha, d),
    //         (switcha, e),
    //         (d, g),
    //         (d, f),
    //         (e, h),
    //         (g, h),
    //     ]);
    //     dag
    // }

    fn full_nand_adder(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();
        let switcha = dag.add_node((Node::Input(Input("InputA".to_string()))));
        let switchb = dag.add_node((Node::Input(Input("InputB".to_string()))));
        let switchc = dag.add_node((Node::Input(Input("CarryIN".to_string()))));
        let d = dag.add_node(self.NAND_gate());
        let e = dag.add_node(self.NAND_gate());
        let f = dag.add_node(self.NAND_gate());
        let g = dag.add_node(self.NAND_gate());
        let h = dag.add_node(self.NAND_gate());
        let i = dag.add_node(self.NAND_gate());
        let j = dag.add_node(self.NAND_gate());
        let k = dag.add_node(self.NAND_gate());
        let l = dag.add_node(self.NAND_gate());
        let m = dag.add_node(self.NAND_gate());
        let n = dag.add_node(self.NAND_gate());
        let o = dag.add_node(self.NAND_gate());
        let p = dag.add_node(self.NAND_gate());
        dag.extend_with_edges(&[
            (switchc, d),
            (switchc, e),
            (switchc, f),
            (switchb, k),
            (switchb, l),
            (switchb, m),
            (switcha, d),
            (switcha, e),
            (switcha, f),
            (d, g),
            (d, e),
            (e, h),
            (f, o),
            (f, o),
            (g, h),
            (h, i),
            (h, k),
            (h, m),
            (i, j),
            (k, l),
            (k, i),
            (l, i),
            (m, n),
            (m, n),
            (n, p),
            (o, p),
        ]);
        dag
    }
}



#[derive(Debug)]
struct Label(String);


#[derive(Debug)]
pub enum GateType {
    NAnd,
    // XOr,
    // Or,
    // And,
    // Not,
}

#[derive(Debug)]
enum Node {
    Input(Input),
    Output(Output),
    Gate(Gate),
}

#[derive(Debug)]
struct Input(String);

#[derive(Debug)]
struct Output(String);


#[derive(Debug)]
struct Gate {
    gate_type: GateType,
    value: Option<bool>,
}



//Graph recursion and string generation

fn defun(circuit: &Graph<Node, (), Directed, u32>, name: &str) -> String {
    let mut output = String::new();
    output.push_str((&format!("")));
    let mut inputs = String::new();
    for input in circuit.externals((Direction::Incoming)) {
        inputs.push_str(&format!("{} ",circuit.node_weight(input).unwrap()));
    }
    &output.push_str(&format!("\n(defun ({} ({})) (",name,inputs));
    for node in circuit.externals(Direction::Outgoing) {
        &output.push_str(&format!(""));
        ascend(&circuit, node, &mut output, 0);
    }

    &output.push_str(") )");
    output
}

fn descendants(circuit: &Graph<Node, (), Directed, u32>) -> String {
    let mut input = String::new();

    for node in circuit.externals(Direction::Incoming) {
        input.push_str((&format!("( ")));
        input.push_str(&format!("{} ", index2symbol(&node)));
        descend(&circuit, node, &mut input, 0);
        input.push_str((&format!("")));

    }
    input
}

fn ascend(dag: &Graph<Node, ()>, node: NodeIndex, output: &mut String, depth: usize) -> String {
    for node in dag.neighbors_directed(node, Direction::Incoming) {
        output.push_str("( ");
        output.push_str(&format!("{} ", &dag.node_weight(node).unwrap()));
        // print!("{} ", index2symbol(&node));
        ascend(dag, node, output, depth + 1);
        output.push_str(")");
    }
    output.clone()
}

fn descend(dag: &Graph<Node, ()>, node: NodeIndex, input: &mut String, depth: usize) {
    for node in dag.neighbors_directed(node, Direction::Outgoing) {
        input.push_str("( ");

        input.push_str(&format!("{} ", dag.node_weight(node).unwrap()));
        descend(dag, node, input, depth + 1);
        input.push_str(")");
    }
}



//Formatting and simple conversion

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


fn index2symbol(idx: &NodeIndex) -> char {
    (idx.index() as u8 + 65).into()
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Input(input) => write!(f, "{}", input),
            Node::Output(output) => write!(f, "{}", output),
            Node::Gate(g) => write!(f, "{}", g),
        }
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.gate_type)
    }
}

impl Default for Node {
    fn default() -> Self {
        Node::Input(Input("".to_string()))
    }
}

impl Into<Label> for &str {
    fn into(self) -> Label {
        Label(self.to_string())
    }
}

impl Display for GateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            GateType::NAnd => "",
            // GateType::And => "AND",
            // GateType::XOr => "XOR",
            // GateType::Or => "OR",
            // GateType::Not => "NOT",
        };
        write!(f, "{}", name)
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}