use core::fmt::Display;
use lexpr::{
    parse::Error as ParseError,
    Cons as ConsCell, Value as SExprValue, Value,
    Value::{Bool, Bytes, Char, Cons, Keyword, Nil, Null, Number, String as SString, Symbol, Vector},
};
use log::info;
use std::convert::TryInto;
use std::fmt::Formatter;
use std::ops::Add;

fn main() -> Result<(), Error> {
    let mut sexprs = vec![];
    // sexprs.push("(true)");
    sexprs.push("(true true)");
    sexprs.push("(true (true true) true)");
    sexprs.push("(true (true true) true)");
    sexprs.push("(* true true)");
    sexprs.push("(* (true true) (true))");
    sexprs.push("(* (true true true) (true true))");
    ///There is no awareness of digits yet. this is just a syntactic example
    sexprs.push("(true true true true (3 1))");

    for sexpr in sexprs {
        println!("original form:              {}", sexpr);
        print_forms(Path::Expr(lexpr::from_str(sexpr).unwrap()))?
    }
    Ok(())
}

fn print_forms(path: Path) -> Result<(), Error> {
    let two_truth = Path::Expr(lexpr::from_str("((true true))").unwrap());
    println!("lexpr debug form:           {}", &path.expand());
    println!("lexpr simple form:          {}", path.to_string());
    println!("catori Form:                {}", &path.clone().explicit());
    println!("catori condensed:           {}", &path.clone().condense());
    println!("catori flattened:           {}", &path.clone().flatten());
    println!("catori length:              {}", &path.length());
    println!("addition:                   {}", &path.sum(two_truth.clone()));
    println!("multiplication:             {}", &path.product(two_truth));
    // println!("default observation:        {}\n", &lexpr.observe("()".to_string()));
    //  println!("split at two:        {}\n", &lexpr.observe(Some(sub_two_lens))?);
    println!();
    Ok(())
}

#[derive(Debug, Clone)]
pub enum Path {
    Expr(SExprValue),
    String(String),
}

impl Path {
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
    fn sum(&self, other: Self) -> Self {
        let path: Value = match self {
            Path::String(str) => lexpr::from_str(str).unwrap(),
            Path::Expr(expr) => expr.clone(),
        };

        let other: Value = match other {
            Path::String(str) => lexpr::from_str(&str).unwrap(),
            Path::Expr(expr) => expr.clone(),
        };
        Path::Expr(Cons(ConsCell::new(path, other)))
    }
    ///This is multiplication and is the only way that things can combine to be more than the sum of their parts
    ///This can be considered shorthand for the impossibly laborious construction of the entire problem
    /// space using ANDs and specifying every single truth value manually
    ///
    ///This can be visualized as taking 2 paths that exist perpendicular to each other, and creating a 2 dimensional
    /// field/space/array out of them
    /// Like summation, this is a lazy operation
    fn product(&self, other: Self) -> Self {
        let path: Value = match self {
            Path::String(str) => lexpr::from_str(str).unwrap(),
            Path::Expr(expr) => expr.clone(),
        };

        let other: Value = match other {
            Path::String(str) => lexpr::from_str(&str).unwrap(),
            Path::Expr(expr) => expr.clone(),
        };
        let cons = Cons(ConsCell::new(path, other));
        Path::Expr(Cons(ConsCell::new("*", cons)))
    }

    ///observing a path through another path (using it as a lens) causes the observed path
    ///to collapse into the shape (lens) that the observer is expecting.
    /// In practice this can be achieve by iterating in parallel through the two paths, and
    /// and at every juncture, the observed subpath is collapse into its equivalent within the lens
    /// This is effectively applying a recursive [XOR gate](https://en.wikipedia.org/wiki/XOR_gate) to the two
    /// inputs
    /// see also  https://en.wikipedia.org/wiki/Functional_completeness
    /// This is the question mark '?' operator
    /// observation can't create any new information and but does destroy it
    ///Experimentally caling this "filter" because it only allows the things through where there are holes
    /// might also be called interfere or destruct
    fn filter(&self, other: Self) -> Self {
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
        let path: Value = match self {
            Path::String(str) => lexpr::from_str(str).unwrap(),
            Path::Expr(expr) => expr.clone(),
        };
        match path {
            Null => "()".to_string(),
            Number(num) => format!("{}", &num.to_string()),
            Symbol(sym) => format!("{}", &sym.to_string()),
            Number(num) => format!("{}", &num.to_string()),
            Cons(cons) => format!(
                "( {} {} )",
                Path::Expr(cons.car().clone()).explicit(),
                Path::Expr(cons.cdr().clone()).explicit()
            ),
            SString(string) => format!("{}", string.to_string()),
            _ => unimplemented!(),
        }
    }

    ///Condensing is just a convenient human readable form that should be identical
    /// to the simple form generated by sexpr library
    fn condense(&self) -> String {
        fn _condense(value: &Path, in_cons: bool) -> String {
            let value: Value = match value {
                Path::String(str) => lexpr::from_str(str).unwrap(),
                Path::Expr(expr) => expr.clone(),
            };
            match value {
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
                    _condense(&Path::Expr(cons.car().clone()), false),
                    _condense(&Path::Expr(cons.cdr().clone()), true),
                    if !in_cons { ") " } else { "" }
                ),
                Number(num) => format!("{} ", &num.to_string()),
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
            let path: Value = match value {
                Path::String(str) => lexpr::from_str(str).unwrap(),
                Path::Expr(expr) => expr.clone(),
            };
            match path {
                Null => "".to_string(),
                Number(num) => num.to_string(),
                Symbol(sym) => {
                    match sym.clone().into_string().as_str() {
                        "true" => "true".to_string(),
                        "*" => "*".to_string(), //panic!(),
                        str => panic!("invalid symbol {}", sym),
                    }
                }
                Cons(cons) => format!(
                    "{} {}{}{}",
                    {
                        if !in_cons {
                            "("
                        } else {
                            ""
                        }
                    },
                    _flatten(&Path::Expr(cons.car().clone()), true),
                    _flatten(&Path::Expr(cons.cdr().clone()), true),
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
        let path = match self {
            Path::String(str) => lexpr::from_str(str).unwrap(),
            Path::Expr(expr) => expr.clone(),
        };
        match path {
            Nil | Null => 0,
            Cons(sexpr) => Path::Expr(sexpr.car().clone()).size() + Path::Expr(sexpr.cdr().clone()).size(),
            _ => 1,
        }
    }

    ///Iterates through top level paths without recursing into nested ones
    ///and generates the linear length of the path
    fn length(&self) -> u64 {
        let path = match self {
            Path::String(str) => lexpr::from_str(str).unwrap(),
            Path::Expr(expr) => expr.clone(),
        };
        match path {
            Nil | Null => 0,
            Cons(sexpr) => 1 + Path::Expr(sexpr.cdr().clone()).length(),
            _ => 1,
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Path::String(string) => {
                write!(f, "{}", string);
                Ok(())
            }
            Path::Expr(expr) => {
                write!(f, "{}", expr);
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
enum Error {}
