use petgraph::prelude::NodeIndex;
use petgraph::{Directed, Direction, Graph};
use std::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    print!("\nNAND");
    print_input_descendants(&nand_circuit());
    print_output_ancestors(&nand_circuit());

    println!("\nNOT");
    print_input_descendants(&not());
    print_output_ancestors(&not());

    print!("\nAND");
    print_input_descendants(&and());
    print_output_ancestors(&and());

    print!("\nOR");
    print_input_descendants(&or());
    print_output_ancestors(&or());

    print!("\nXOR");
    print_input_descendants(&xor());
    print_output_ancestors(&xor());

    print!("\nAdder");
    print_input_descendants(&full_adder());
    print_output_ancestors(&full_adder());
}

fn print_output_ancestors(circuit: &Graph<Node, (), Directed, u32>) {
    for node in circuit.externals(Direction::Outgoing) {
        println!("\noutput {} wired from: ", index2symbol(&node));
        ascend(&circuit, node, 0);
    }
    println!();
}

fn print_input_descendants(circuit: &Graph<Node, (), Directed, u32>) {
    for node in circuit.externals(Direction::Incoming) {
        println!("\ninput {} wired to: ", index2symbol(&node));

        descend(&circuit, node, 0);
    }
    println!();
}

#[derive(Debug)]
enum Node {
    Input,
    Output,
    NAND(Gate),
}

#[derive(Debug)]
struct Gate(Option<bool>);

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Input => write!(f, "(Input)"),
            Node::Output => write!(f, "(Output)"),
            Node::NAND(g) => write!(f, "(NAND {:?})", g),
        }
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
        Node::Input
    }
}

impl Into<Label> for &str {
    fn into(self) -> Label {
        Label(self.to_string())
    }
}

fn not() -> Graph<Node, ()> {
    let mut dag = Graph::<Node, ()>::new();
    let switch1 = dag.add_node(Node::Input);
    let a = dag.add_node(nand());
    dag.extend_with_edges(&[(switch1, a), (switch1, a)]);
    dag
}

fn nand_circuit() -> Graph<Node, ()> {
    let mut dag = Graph::<Node, ()>::new();
    let switch1 = dag.add_node(Node::Input);
    let switch2 = dag.add_node(Node::Input);
    let a = dag.add_node(nand());
    dag.extend_with_edges(&[(switch1, a), (switch2, a)]);
    dag
}

fn xor() -> Graph<Node, ()> {
    let mut dag = Graph::<Node, ()>::new();
    let switch1 = dag.add_node(Node::Input);
    let switch2 = dag.add_node(Node::Input);
    let a = dag.add_node(nand());
    let b = dag.add_node(nand());
    let c = dag.add_node(nand());
    let d = dag.add_node(nand());

    dag.extend_with_edges(&[
        (switch1, a),
        (switch1, b),
        (switch2, a),
        (switch2, c),
        (a, b),
        (a, c),
        (b, d),
        (c, d),
    ]);
    dag
}

fn or() -> Graph<Node, ()> {
    let mut dag = Graph::<Node, ()>::new();

    let switch1 = dag.add_node(Node::Input);
    let switch2 = dag.add_node(Node::Input);
    let a = dag.add_node(nand());
    let b = dag.add_node(nand());
    let c = dag.add_node(nand());
    dag.extend_with_edges(&[(switch1, a), (switch1, a), (switch2, b), (switch2, b), (a, c), (b, c)]);
    dag
}

fn and() -> Graph<Node, ()> {
    let mut dag = Graph::<Node, ()>::new();
    let switch1 = dag.add_node(Node::Input);
    let switch2 = dag.add_node(Node::Input);
    let a = dag.add_node(nand());
    let b = dag.add_node(nand());
    dag.extend_with_edges(&[(switch1, a), (switch2, a), (a, b), (a, b)]);
    dag
}

fn nand() -> Node {
    Node::NAND(Gate(None))
}

fn full_adder() -> Graph<Node, ()> {
    let mut dag = Graph::<Node, ()>::new();
    let switch1 = dag.add_node(Node::Input);
    let switch2 = dag.add_node(Node::Input);
    let switch3 = dag.add_node(Node::Input);
    let a = dag.add_node(nand());
    let b = dag.add_node(nand());
    let c = dag.add_node(nand());
    let d = dag.add_node(nand());
    let e = dag.add_node(nand());
    let f = dag.add_node(nand());
    let g = dag.add_node(nand());
    let h = dag.add_node(nand());
    let i = dag.add_node(nand());
    let j = dag.add_node(nand());
    let k = dag.add_node(nand());
    let l = dag.add_node(nand());
    let m = dag.add_node(nand());
    dag.extend_with_edges(&[
        (switch1, a),
        (switch1, b),
        (switch1, c),
        (switch2, h),
        (switch2, i),
        (switch2, j),
        (switch3, a),
        (switch3, c),
        (switch3, d),
        (a, d),
        (a, b),
        (b, e),
        (c, l),
        (c, l),
        (d, e),
        (e, f),
        (e, h),
        (e, j),
        (f, g),
        (h, i),
        (h, f),
        (i, g),
        (j, k),
        (j, k),
        (k, m),
        (l, m),
    ]);
    dag
}

fn ascend(dag: &Graph<Node, ()>, node: NodeIndex, depth: usize) {
    for node in dag.neighbors_directed(node, Direction::Incoming) {
        print!("( ");
        print!("{} ", index2symbol(&node));
        ascend(dag, node, depth + 1);
        print!(")");
    }
}

fn descend(dag: &Graph<Node, ()>, node: NodeIndex, depth: usize) {
    for node in dag.neighbors_directed(node, Direction::Outgoing) {
        print!("( ");
        print!("{} ", index2symbol(&node));
        descend(dag, node, depth + 1);
        print!(")");
    }
}

fn index2symbol(idx: &NodeIndex) -> char {
    (idx.index() as u8 + 65).into()
}
