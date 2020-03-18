use core::fmt::Display;
use lexpr::{
    parse::Error as ParseError,
    Cons as ConsCell, Value as SExprValue, Value,
    Value::{Bool, Bytes, Char, Cons, Keyword, Nil, Null, Number, String as SString, Symbol, Vector},
};
use log::info;
use std::convert::TryInto;
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
        print_forms(lexpr::from_str(sexpr).unwrap())?
    }
    Ok(())
}

fn print_forms(sexpr: impl Path) -> Result<(), Error> {
    let truth = lexpr::from_str("true").unwrap();
    println!("lexpr debug form:           {}", &sexpr.expand());
    println!("lexpr simple form:          {}", sexpr.to_string());
    println!("catori Form:                {}", &sexpr.clone().explicit());
    println!("catori condensed:           {}", &sexpr.clone().condense());
    println!("catori flattened:           {}", &sexpr.clone().flatten());
    println!("catori length:              {}", &sexpr.length());
    println!("addition:                   {}", &sexpr.sum(truth));
    // println!("default observation:        {}\n", &lexpr.observe("()".to_string()));
    //  println!("split at two:        {}\n", &lexpr.observe(Some(sub_two_lens))?);
    println!();
    Ok(())
}

pub trait Path: Sized + Display + Clone {
    //MINIMAL OPERATORS
    //observation is eager, sum and product are lazy

    ///observing a path through another path (using it as a lens) causes the observed path
    ///to collapse into the shape (lens) that the observer is expecting.
    /// In practice this can be achieve by iterating in parallel through the two paths, and
    /// and at every juncture, the observed subpath is collapse into its equivalent within the lens
    /// This is effectively applying a recursive [XOR gate](https://en.wikipedia.org/wiki/XOR_gate) to the two
    /// inputs
    /// see also  https://en.wikipedia.org/wiki/Functional_completeness
    /// This is the question mark '?' operator
    /// observation can't create any new information and but does destroy it
    fn observe(&self, other: Self) -> Self;

    ///Concatenation is the same as addition in one dimensional catori space
    /// Only two *like* things can be concatenated, but "like" is in the eye of the beholder.
    /// From the perspective of abstract paths, all paths are "alike"
    /// This is an AND gate, as the two inputs both have to be not null in order
    /// for this to be a meaningful operation. Nil is not occupyable, therefore can't be concatenated
    /// with anything
    /// this is the plus '+" operator
    /// Summation neither creates nor destroys information
    /// This can be visualized as taking two paths that are already pointed in the same direction
    /// and putting them end to end
    fn sum(&self, other: Self) -> Self;

    ///This is multiplication and is the only way that things can combine to be more than the sum of their parts
    ///This can be considered shorthand for the impossibly laborious construction of the entire problem
    /// space using ANDs and specifying every single (true) manually
    ///
    ///This can be visualized as taking 2 paths that exist perpendicular to each, and creating a 2 dimensional
    /// field/space/array out of them
    /// Like summation,
    fn product(&self, other: Self) -> Result<Self, Error>;

    //VISUALIZATION UTILITIES
    //TODO demonstrate that these can all be implemented in terms of the primitives above

    ///Converts any path into an Sexpression and generates the fully expanded form
    fn expand(&self) -> String;
    ///Displays any path in its explicit catori form that elides Cons and uses + instead,
    ///but still includes full nesting
    fn explicit(&self) -> String;
    ///Generates the condensed human readable form of a catori path
    fn condense(&self) -> String;
    ///Iterates through a nested path and flattens all structures
    fn flatten(&self) -> String;
    ///Calculates the full size of all nested paths recursively
    fn size(&self) -> u64;
    ///Iterates through top level paths without recursing into nested ones
    ///and generates the linear length of the path
    fn length(&self) -> u64;
}

// impl Path for String {
//     fn expand(&self) -> String {
//         lexpr::from_str(&self).unwrap().expand()
//     }
//
//     fn explicit(&self) -> String {
//         lexpr::from_str(&self).unwrap().explicit()
//     }
//
//     fn condense(&self) -> String {
//         lexpr::from_str(&self).unwrap().condense()
//     }
//
//     fn flatten(&self) -> String {
//         lexpr::from_str(&self).unwrap().flatten()
//     }
//
//     fn size(&self) -> u64 {
//         lexpr::from_str(&self).unwrap().size()
//     }
//
//     fn length(&self) -> u64 {
//         lexpr::from_str(&self).unwrap().length()
//     }
//
//     fn observe(&self, other: Self) -> Self {
//         lexpr::from_str(&self)
//             .unwrap()
//             .observe(lexpr::from_str(&other).unwrap())
//             .to_string()
//     }
//
//     fn sum(&self, other: Self) -> Self {
//         lexpr::from_str(&self)
//             .unwrap()
//             .sum(lexpr::from_str(&other).unwrap())
//             .to_string()
//     }
//
//     fn product(&self, other: Self) -> Result<Self, Error> {
//         unimplemented!()
//     }
// }

#[derive(Debug)]
pub enum Error {}

impl Path for SExprValue {
    fn observe(&self, other: Self) -> Self {
        todo!(
            "recursively XOR both sides. only output something when there is a 
        hole on one side and a path on the other"
        );
    }

    fn sum(&self, other: Self) -> Self {
        Value::Cons(ConsCell::new(self.clone(), other))
    }

    fn product(&self, other: Self) -> Result<Self, Error> {
        unimplemented!()
    }

    fn expand(&self) -> String {
        format!("{:?}", &self)
    }

    fn explicit(&self) -> String {
        match self {
            Null => "".to_string(),
            Number(num) => format!("{}", &num.to_string()),
            Symbol(sym) => format!("{}", &sym.to_string()),
            Number(num) => format!("{}", &num.to_string()),
            Cons(cons) => format!("( {} {} )", cons.car().explicit(), cons.cdr().explicit()),
            _ => unimplemented!(),
        }
    }

    ///Condensing is just a convenient human readable form that should be identical
    /// to the simple form generated by sexpr library
    fn condense(&self) -> String {
        fn _condense(value: &SExprValue, in_cons: bool) -> String {
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
                    _condense(cons.car(), false),
                    _condense(cons.cdr(), true),
                    if !in_cons { (") ") } else { "" }
                ),
                Number(num) => format!("{} ", &num.to_string()),
                Symbol(sym) => format!("{} ", &sym),

                _ => unimplemented!(),
            }
        }
        _condense(self, false)
    }

    fn flatten(&self) -> String {
        fn _flatten(value: &SExprValue, in_cons: bool) -> String {
            match value {
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
                    _flatten(cons.car(), true),
                    _flatten(cons.cdr(), true),
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

        _flatten(self, false)
    }

    fn size(&self) -> u64 {
        match &self {
            Nil | Null => 0,
            Cons(sexpr) => sexpr.car().size() + sexpr.cdr().size(),
            _ => 1,
        }
    }

    fn length(&self) -> u64 {
        match &self {
            Nil | Null => 0,
            Cons(sexpr) => 1 + sexpr.cdr().length(),
            _ => 1,
        }
    }
}
