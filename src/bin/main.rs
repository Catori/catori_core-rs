use core::fmt::Display;
use lexpr::parse::NilSymbol;
use lexpr::{
    parse::Error as ParseError,
    Cons as ConsCell, Number, Value as SExprValue, Value,
    Value::{Bool, Bytes, Char, Cons, Keyword, Nil, Null, Number as ValNumber, String as SString, Symbol, Vector},
};
use log::info;
use std::convert::TryInto;
use std::fmt::Formatter;
use std::ops::Add;

pub trait Catori {
    fn car(&self) -> Path;
    fn cdr(&self) -> Path;
    fn sum(self, other: Path) -> Path;
    fn product(self, other: Path) -> Path;
    fn entangle(self, other: Path) -> Path;
}

fn main() -> Result<(), Error> {
    let mut sexprs = vec![];
    // sexprs.push("(true)");
    sexprs.push("(true true)");
    sexprs.push("(true (true true) true)");
    sexprs.push("(true (true _) true)");
    sexprs.push("(* true true)");
    sexprs.push("(* (true true) (true))");
    sexprs.push("(* (true true true) (true true))");
    ///There is no awareness of digits yet. this is just a syntactic example
    sexprs.push("(true true true true (3 1))");

    let two_truth = Path(lexpr::from_str("((true true))").unwrap());
    let one_hole = Path(lexpr::from_str("(_)").unwrap());
    let two_hole = Path(lexpr::from_str("(_ _)").unwrap());

    for path in sexprs.clone() {
        let path = Path(lexpr::from_str(path).unwrap());
        println!("original form:              {}", path);
        println!("lexpr debug form:           {}", path.expand());
        println!("lexpr simple form:          {}", path.to_string());
        println!("catori expanded form:       {}", path.clone().explicit());
        println!("catori condensed:           {}", path.clone().condense());
        println!("catori flattened:           {}", path.clone().flatten());
        println!("catori length:              {}", path.clone().length());
        println!("addition:                   {}", path.clone().sum(two_truth.clone()));
        println!(
            "multiplication:             {}\n",
            path.clone().product(two_truth.clone())
        );
    }

    for path in sexprs {
        let path = Path(lexpr::from_str(path.clone()).unwrap());

        print!("entangle:↑ {} {} => ", one_hole, path);
        println!("             {}", one_hole.clone().entangle(path.clone()));

        print!("entangle:↑ {} {} => ", two_hole, path);
        println!("             {}", two_hole.clone().entangle(path.clone()));
    }
    Ok(())
}

fn print_forms(path: Path) -> Result<(), Error> {
    // println!("default observation:        {}\n", &lexpr.observe("()".to_string()));
    //  println!("split at two:        {}\n", &lexpr.observe(Some(sub_two_lens))?);
    println!();
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Path(SExprValue);

impl Into<Path> for SExprValue {
    fn into(self) -> Path {
        Path(self)
    }
}

impl Catori for Path {
    fn car(&self) -> Self {
        Path(match self.0.clone() {
            Cons(cons) => cons.car().clone(),
            other => other,
        })
    }

    fn cdr(&self) -> Path {
        Path(match self.0.clone() {
            Cons(cons) => cons.cdr().clone(),
            _ => SExprValue::Null,
        })
    }
    //MINIMAL OPERATORS
    //observation is eager, sum and product are lazy

    ///Concatenation is the same as addition in one dimensional catori space
    /// Only two *like* things can be concatenated, but "like" is in the eye of the beholder.
    /// From the perspective of abstract paths, all paths are "alike"
    /// This is also an AND gate, as the two inputs both have to be not null in order
    /// for this to be a meaningful operation. Nil is not occupyable, therefore can't be concatenated
    /// with anything
    /// It's also the plus '+" operator
    /// Summation neither creates nor destroys information
    /// This operation can be visualized as taking two paths that are already pointed in the same direction
    /// and abutting them end to end
    fn sum(self, other: Path) -> Path {
        Path(Cons(ConsCell::new::<Value, ConsCell>(
            Value::symbol("+"),
            ConsCell::new(self.0, other.0.clone()),
        )))
    }
    ///This is multiplication and is the only way that things can combined to be more than the sum of their parts
    ///This can be considered shorthand for the impossibly laborious construction of the entire problem
    /// space using ANDs and specifying every single truth value manually.
    ///
    ///This can be visualized as taking 2 paths that exist perpendicular to each other, and creating a 2 dimensional
    /// field/space/array out of them
    /// Like summation, this is a lazy operation
    fn product(self, other: Path) -> Path {
        Path(Cons(ConsCell::new::<Value, ConsCell>(
            Value::symbol("*"),
            ConsCell::new(self.0, other.0.clone()),
        )))
    }

    ///two paths can be entangled at one or more points. an underscore is used to represent a hole where entanglement can happen
    /// when two paths are entangled, they are recursively comparedd.
    /// Normally a path can only contain truth. If, however, we project a path onto a space where it can contain either
    /// some amount of truth or a hole (represented by an underscore "_" symbol), then we can map two like structures onto each
    /// other and observe the differences.
    /// entanglement follows the logic of a primitive logic gate, and depending on the desired runtime properties of a program,
    ///different gate building blocks can be chosen.
    /// For purposes of logical operations, consider a hole (_) to be false and and any other path to be true.
    /// starting at the root of each tree, nodes will be recursively compared as follows:
    ///
    ///
    /// On of the most obvious is a [OR](https://en.wikipedia.org/wiki/OR_gate) gate as follows:
    ///
    ///
    /// (+ (_) (_) ) => ( _ )    //Two holes are not filled and are left open to be entangled with another structure
    /// (+ (_) (r) ) => ( r )    //Any value on the right hand side will be output and the hole on the left will be discarded
    /// (+ (l) (_) ) => (l)      //Any value on the left hand side will be output and the hole on the right will be discarded
    /// (+ (l) (r) ) => (+ l r ) //When both values are occupied and there are no holes, concatenate the two paths into a new one
    ///
    /// that last one is very odd semantics for many programs, so you might prefer to use a
    ///
    /// [XOR](https://en.wikipedia.org/wiki/XOR_gate) gates where the first three rows above are the same, but the last row is:
    ///
    /// (+ (l) (r) ) => ( _ ) //When both values are occupied discard both and create a new hole at this location
    ///                       //Note that with this approach can be used to inject new entanglement points by matching the
    ///                       //pattern that should be replaced with a (_) hole.
    ///
    ///
    fn entangle(self, other: Self) -> Self {
        self.or(other).0.into()
    }
}

trait Entangle {
    fn or(self, rhs: Self) -> Path;

    fn is_hole(&self) -> bool {
        false
    }
}

impl Entangle for bool {
    fn or(self, rhs: Self) -> Path {
        match (self, rhs) {
            (false, false) => Path(SExprValue::Bool(false)),
            (false, true) => Path(SExprValue::Bool(true)),
            (true, false) => Path(SExprValue::Bool(true)),
            (true, true) => Path(SExprValue::Bool(true)),
        }
    }
}

impl Entangle for Number {
    fn or(self, rhs: Self) -> Path {
        match (self, rhs) {
            (lhs, rhs) => Path(SExprValue::Cons(ConsCell::new(lhs, rhs))),
        }
    }
}

impl Entangle for char {
    fn or(self, rhs: Self) -> Path {
        match (self, rhs) {
            (lhs, rhs) => Path(SExprValue::Cons(ConsCell::new(lhs, rhs))),
        }
    }
}

impl Entangle for String {
    fn or(self, rhs: Self) -> Path {
        match (self, rhs) {
            (lhs, rhs) => Path(SExprValue::Cons(ConsCell::new(lhs, rhs))),
        }
    }
}

impl Entangle for Value {
    fn or(self, rhs: Self) -> Path {
        match (self, rhs) {
            (Symbol(sym), _) => todo!(),
            (_, Symbol(sym)) => todo!(),
            (Cons(lhs), Cons(rhs)) => SExprValue::Cons(ConsCell::new(lhs, rhs)).into(),
            (Nil, Nil) => Nil.into(),
            (Null, Null) => Null.into(),
            (Bool(lhs), Bool(rhs)) => lhs.or(rhs),
            (ValNumber(lhs), ValNumber(rhs)) => lhs.or(rhs),
            (Char(lhs), Char(rhs)) => lhs.or(rhs),
            (SString(lhs), SString(rhs)) => lhs.into_string().or(rhs.into_string()),
            (Keyword(lhs), Keyword(rhs)) => todo!(),
            (Bytes(lhs), Bytes(rhs)) => todo!(),
            (Vector(lhs), Vector(rhs)) => todo!(),
            (Nil, Null) | (Null, Nil) => todo!(),
            (Nil, val) | (Null, val) | (val, Nil) | (val, Null) => val.into(),
            (lhs, rhs) => Path(SExprValue::Cons(ConsCell::new(lhs, rhs))),
        }
    }

    fn is_hole(&self) -> bool {
        match &self {
            Symbol(sym) => match sym.clone().into_string().as_str() {
                "_" => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl Entangle for Path {
    fn or(self, rhs: Self) -> Path {
        self.0.or(rhs.0)
        // self.cdr().or(rhs.cdr())
    }

    fn is_hole(&self) -> bool {
        self.0.is_hole()
    }
}

impl Path {
    ///observing a path through another path (using it as a lens) causes the observed path
    ///to collapse into the shape (lens) that the observer is expecting.
    /// In practice this can be achieve by iterating in parallel through the two paths, and
    /// and at every juncture, the observed subpath is collapse into its equivalent within the lens
    /// This is effectively applying a recursive [XOR gate](https://en.wikipedia.org/wiki/XOR_gate) to the two
    /// inputs
    /// see also  https://en.wikipedia.org/wiki/Functional_completeness
    /// This is the question mark '?' operator
    /// observation can't create any new information and but does destroy it
    fn xor(&self, other: Self) -> Self {
        match self.length() == other.length() {
            true => {}
            false => {}
        }
        todo!(
            "recursively XOR both sides. only output something when there is a 
        hole on one side and a path on the other"
        );
    }

    //todo add alias function?

    //VISUALIZATION UTILITIES
    //TODO demonstrate that these can all be implemented in terms of the primitives above

    ///Converts any path into an Sexpression and generates the fully expanded form
    fn expand(&self) -> String {
        format!("{:?}", &self)
    }

    ///Displays any path in its explicit catori form that elides Cons and uses + instead,
    ///but still includes full nesting
    fn explicit(&self) -> String {
        match self.0.clone() {
            Null => "()".to_string(),
            Symbol(sym) => format!("{}", &sym.to_string()),
            ValNumber(num) => format!("{}", &num.to_string()),
            Cons(cons) => format!(
                "( {} {} )",
                Path(cons.car().clone()).explicit(),
                Path(cons.cdr().clone()).explicit()
            ),
            SString(string) => format!("{}", string.to_string()),
            _ => unimplemented!(),
        }
    }

    ///Condensing is just a convenient human readable form that should be identical
    /// to the simple form generated by sexpr library
    fn condense(&self) -> String {
        fn _condense(value: &Path, in_cons: bool) -> String {
            match &value.0 {
                Null => "".to_string(),
                Cons(cons) => format!(
                    "{}{}{}{}",
                    {
                        if !in_cons {
                            ("( ")
                        } else {
                            ""
                        }
                    },
                    _condense(&Path(cons.car().clone()), false),
                    _condense(&Path(cons.cdr().clone()), true),
                    if !in_cons { ") " } else { "" }
                ),
                ValNumber(num) => format!("{} ", &num.to_string()),
                Symbol(sym) => format!("{} ", &sym),
                SString(string) => format!("{} ", string.to_string()),

                wtf => unimplemented!("{}", wtf),
            }
        }
        _condense(self, false)
    }

    ///Iterates through a nested path and flattens all structures
    fn flatten(&self) -> String {
        fn _flatten(value: &Path, in_cons: bool) -> String {
            match &value.0 {
                Null => "".to_string(),
                ValNumber(num) => num.to_string(),
                Symbol(sym) => match sym.clone().into_string().as_str() {
                    "true" => "true".to_string(),
                    "*" => "*".to_string(),
                    "_" => "_".to_string(),
                    str => panic!("invalid symbol {}", sym),
                },
                Cons(cons) => format!(
                    "{} {}{}{}",
                    {
                        if !in_cons {
                            "("
                        } else {
                            ""
                        }
                    },
                    _flatten(&Path(cons.car().clone()), true),
                    _flatten(&Path(cons.cdr().clone()), true),
                    {
                        if !in_cons {
                            " )"
                        } else {
                            ""
                        }
                    }
                ),
                _ => unimplemented!(),
            }
        }

        _flatten(&self, false)
    }

    ///Calculates the full size of all nested paths recursively
    fn size(&self) -> u64 {
        match &self.0 {
            Nil | Null => 0,
            Cons(sexpr) => Path(sexpr.car().clone()).size() + Path(sexpr.cdr().clone()).size(),
            _ => 1,
        }
    }

    ///Iterates through top level paths without recursing into nested ones
    ///and generates the linear length of the path
    fn length(&self) -> u64 {
        match &self.0 {
            Nil | Null => 0,
            Cons(sexpr) => 1 + Path(sexpr.cdr().clone()).length(),
            _ => 1,
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0);
        Ok(())
    }
}

#[derive(Debug)]
enum Error {}
