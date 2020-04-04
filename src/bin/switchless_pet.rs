use lexpr::to_string;
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
        ("XOR", u.xor()),
        ("XNOR", u.xnor()),
        ("Adder", u.full_adder()),
    ];

    for (key, val) in gates {
        println!("\n{}", key);
        println!("inputs -> {} \n", descendants(&val));
        //println!("inputs -> {} \n", descendants(&val).replace("NAND ", ""));
        println!("output <- {}", ancestors(&val));
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
        dag.extend_with_edges(&[(switcha, c), (switchb, c)]);
        dag
    }

    fn xor(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();
        let switcha = dag.add_node(Node::Input(Input("A".to_string())));
        let switchb = dag.add_node(Node::Input(Input("B".to_string())));
        let c = dag.add_node(self.NAND_gate());
        let d = dag.add_node(self.NAND_gate());
        let e = dag.add_node(self.NAND_gate());
        let f = dag.add_node(self.NAND_gate());

        dag.extend_with_edges(&[
            (switcha, c),
            (switcha, d),
            (switchb, c),
            (switchb, e),
            (c, d),
            (c, e),
            (d, f),
            (e, f),
        ]);
        dag
    }

    fn or(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();

        let switcha = dag.add_node(Node::Input(Input("A".to_string())));
        let switchb = dag.add_node(Node::Input(Input("B".to_string())));
        let c = dag.add_node(self.NAND_gate());
        let d = dag.add_node(self.NAND_gate());
        let e = dag.add_node(self.NAND_gate());
        dag.extend_with_edges(&[
            (switcha, c),
            (switcha, c),
            (switchb, d),
            (switchb, d),
            (c, e),
            (d, e),
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
            (switcha, d),
            (switcha, d),
            (switcha, e),
            (switchb, c),
            (switchb, c),
            (switchb, e),
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
        dag.extend_with_edges(&[(switcha, c), (switchb, c), (c, d), (c, d)]);
        dag
    }

    fn NAND_gate(&self) -> Node {
        Node::Gate(Gate {
            gate_type: GateType::NAnd,
            value: None,
        })
    }

    fn XOR_gate(&self) -> Node {
        Node::Gate(Gate {
            gate_type: GateType::XOr,
            value: None,
        })
    }

    fn full_adder(&self) -> Graph<Node, ()> {
        let mut dag = Graph::<Node, ()>::new();
        let switcha = dag.add_node((Node::Input(Input("A".to_string()))));
        let switchb = dag.add_node((Node::Input(Input("B".to_string()))));
        let switchc = dag.add_node((Node::Input(Input("B".to_string()))));
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
            (switcha, d),
            (switcha, e),
            (switcha, f),
            (switchb, k),
            (switchb, l),
            (switchb, m),
            (switchc, d),
            (switchc, e),
            (switchc, f),
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
pub enum GateType {
    NAnd,
    XOr,
}

impl Display for GateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            GateType::NAnd => "NAND",
            GateType::XOr => "XOR",
        };
        write!(f, "{}", name)
    }
}

fn ancestors(circuit: &Graph<Node, (), Directed, u32>) -> String {
    let mut output = String::new();
    for node in circuit.externals(Direction::Outgoing) {
        &output.push_str("(NAND ");
        ascend(&circuit, node, &mut output, 0);
        &output.push_str(" )");
    }
    output
}

fn descendants(circuit: &Graph<Node, (), Directed, u32>) -> String {
    let mut input = String::new();
    for node in circuit.externals(Direction::Incoming) {
        input.push_str(&format!("\n{} ", index2symbol(&node)));
        descend(&circuit, node, &mut input, 0);
    }
    input
}

#[derive(Debug)]
enum Node {
    Input(Input),
    Output(Output),
    Gate(Gate),
}

#[derive(Debug)]
struct Input(String);

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Output(String);

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Gate {
    gate_type: GateType,
    value: Option<bool>,
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

#[derive(Debug)]
struct Label(String);

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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

fn index2symbol(idx: &NodeIndex) -> char {
    (idx.index() as u8 + 65).into()
}
