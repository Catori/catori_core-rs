use lexpr::{
    parse::Error as ParseError,
    Value as SExprValue,
    Value::{Cons, Nil, Null, Number, Symbol},
};
use log::info;

fn main() -> Result<(), ParseError> {
    let mut sexprs = vec![];
    sexprs.push("(true)");
    sexprs.push("(true true)");
    sexprs.push("(true (true true) true)");
    sexprs.push("(true (true true) true)");
    sexprs.push("(* true true)");
    sexprs.push("(* (true true) (true))");
    sexprs.push("(* (true true true) (true true))");
    sexprs.push("(true true true true (3 1))");

    // Access parts of the data by indexing with square brackets.
    //    println!("Please call {} at the number {}", v["name"], v["phones"][1]);

    for sexpr in sexprs {
        let lexpr: SExprValue = lexpr::from_str(sexpr).unwrap();
        println!("original form:       {}", sexpr);
        println!("parsed debug form:   {:?}", lexpr.clone());
        println!("parsed simple form:  {}", lexpr.clone());
        println!("catori Form:         {}", PathWrapper(&lexpr.clone()).simple());
        println!("catori condensed:    {}", PathWrapper(&lexpr.clone()).condense());
        println!("catori flattened:    {}", PathWrapper(&lexpr.clone()).flatten());
        println!("catori Size:         {}\n", PathWrapper(&lexpr).size());
        //        dbg!(lexpr);
    }
    Ok(())
}

enum Error {}

trait Path {
    fn simple(self) -> String;
}



struct PathWrapper<'a>(&'a SExprValue);

impl<'a> PathWrapper<'a> {
    fn simple(self) -> String {
        match self.0 {
            Null => "".to_string(),
            Number(num) => format!("{}", &num.to_string()),
            Symbol(sym) => format!("{}", &sym.to_string()),
            Number(num) => format!("{}", &num.to_string()),
            Cons(cons) => format!(
                "( {} {} )",
                &PathWrapper(cons.car()).simple(),
                &PathWrapper(cons.cdr()).simple()
            ),
            _ => unimplemented!(),
        }
    }

    fn condense(self) -> String {
        self._condense(false)
    }

    fn _condense(self, in_cons: bool) -> String {
        match self.0 {
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
                &PathWrapper(cons.car())._condense(false),
                &PathWrapper(cons.cdr())._condense(true),
                if !in_cons { (") ") } else { "" }
            ),
            Number(num) => format!("{} ", &num.to_string()),
            Symbol(sym) => format!("{} ", &sym),

            _ => unimplemented!(),
        }
    }

    pub fn flatten(self) -> String {
        self._flatten(false)
    }

    fn _flatten(self, in_cons: bool) -> String {
        match self.0 {
            Null => "".to_string(),
            Number(num) => num.to_string(),
            Symbol(sym) => {
                match sym.clone().into_string().as_str() {
                    "true" => "true".to_string(),
                    "*" => "*".to_string(),//panic!(),
                    str => panic!("invalid symbol {}", sym)
                }
            },
            Cons(cons) => format!(
                "{} {}{}{}",
                {
                    if !in_cons {
                        ("(")
                    } else {
                        ""
                    }
                },

                PathWrapper(cons.car())._flatten(true),
                PathWrapper(cons.cdr())._flatten(true),
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

    fn size(&self) -> usize {
        match &self.0 {
            Nil | Null => 0,
            Cons(sexpr) =>
                PathWrapper(sexpr.car()).size() +
                PathWrapper(sexpr.cdr()).size(),
            _ => 1,
        }
    }
}
