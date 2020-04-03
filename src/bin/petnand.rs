// use daggy::WouldCycle;
use std::iter::once;
//use daggy::Dag;
use petgraph::prelude::{GraphMap, NodeIndex};
use petgraph::visit::{GraphProp, NodeIndexable};
use petgraph::{Directed, Direction, Graph};
//use petgraph_graphml::GraphMl;
use std::fmt;
use std::fmt::{Display, Error, Formatter};

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

fn main() {
    let circuit = xor();

    for node in circuit.externals(Direction::Incoming) {
        println!("\nnode {:#?} has children: ", node.index());

        descend(&circuit, node, 0);
    }
    println!();

    for node in circuit.externals(Direction::Outgoing) {
        println!("\nnode {:#?} has parents: ", node.index());
        ascend(&circuit, node, 0);
    }
    println!();
}

fn not() -> Graph<Node, ()> {
    let mut dag = Graph::<Node, ()>::new();
    let switch1 = dag.add_node(Node::Input);
    let a = dag.add_node(nand());
    let q = dag.add_node(Node::Output);
    dag.extend_with_edges(&[(switch1, a), (switch1, a)]);
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

    let q = dag.add_node(Node::Output);
    dag.extend_with_edges(&[
        (switch1, a),
        (switch1, b),
        (switch2, a),
        (switch2, c),
        (a, b),
        (a, c),
        (b, d),
        (c, d),
        (d, q),
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
    let output1 = dag.add_node(Node::Output);
    dag.extend_with_edges(&[
        (switch1, a),
        (switch1, a),
        (switch2, b),
        (switch2, b),
        (a, c),
        (b, c),
        (c, output1),
    ]);

    dag
}
fn nand() -> Node {
    Node::NAND(Gate(None))
}

fn full_adder(dag: &mut Graph<Node, (), Directed, u32>) {
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
    let output1 = dag.add_node(Node::Output);
    let output2 = dag.add_node(Node::Output);
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
        (g, output1),
        (h, i),
        (h, f),
        (i, g),
        (j, k),
        (j, k),
        (k, m),
        (l, m),
        (m, output2),
    ]);
}

fn ascend(dag: &Graph<Node, ()>, node: NodeIndex, depth: usize) {
    for node in dag.neighbors_directed((node), Direction::Incoming) {
        print!("( ");
        print!("{:?} ", node.index());
        ascend(dag, node, depth + 1);
        print!(")");
        if depth == 0 {
            println!()
        }
    }
}

fn descend(dag: &Graph<Node, ()>, node: NodeIndex, depth: usize) {
    for node in dag.neighbors_directed((node), Direction::Outgoing) {
        print!("( ");
        print!("{:?} ", node.index());
        descend(dag, node, depth + 1);
        print!(")");
        if depth == 0 {
            println!()
        }
    }
}
