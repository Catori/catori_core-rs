use log::info;
use std::sync::atomic::AtomicBool;

#[derive(Clone, Debug)]
enum Bool {
    F,
    T,
}

impl Bool {
    fn to_bool(self) -> bool {
        match self {
            Bool::F => false,
            Bool::T => true,
        }
    }
}

impl Into<bool> for Bool {
    fn into(self) -> bool {
        self.to_bool()
    }
}

#[derive(Default)]
struct BoolIterator(Option<Bool>);

impl Iterator for BoolIterator {
    type Item = Bool;

    fn next(&mut self) -> Option<Self::Item> {
        use Bool::*;
        match self.0 {
            None => self.0 = Some(F),
            Some(F) => self.0 = Some(T),
            Some(T) => self.0 = None,
        }
        self.0.clone()
    }
}

impl Cat for Bool {
    type T = bool;

    fn min() -> bool {
        false
    }

    fn max() -> bool {
        true
    }
}

trait Cat {
    type T;
    fn min() -> Self::T;
    fn max() -> Self::T;
}

trait TypeIterator<T> {}

impl TypeIterator<Bool> for Bool {}

// fn fib(nand_gates: Vec<Option<Bool>>) -> Option<Bool> {
//     // A function declared inside another function does not pollute the outer namespace.
//     fn actual_fib(n: Bool) -> Bool {
//         if n < 2 {
//             n
//         } else {
//             actual_fib(n - 1) + actual_fib(n - 2)
//         }
//     }
//
//     for gate in &nand_gates {
//         match gate {
//             Some(gate) => {return Some(actual_fib(gate.clone()))},
//             None => {return None}
//         }
//         // if gate < 0 {
//         //     return None
//         // } else {
//         //     return Some(actual_fib(n))
//         // }
//     }
//     None
// }
//
// fn recursive_nand(mut nand_gates: Vec<Option<Bool>>) -> Option<Bool> {
//     for gate in &nand_gates {
//         match nand_gates.is_empty() {
//             true => todo!(),
//             false => todo!(),
//         }
//     }
//     None
//     // fn nand(mut nand_gates) -> Bool {
//     //     match nand_gates.is_empty()     {
//     //     true => gate
//     // }
// }

fn main() {
    let gates: Vec<Option<Bool>> = vec![None, None, None];

    for a in BoolIterator::default() {
        //print!("{:?} ", &a);
        for b in BoolIterator::default() {
            //  println!("{:?}", &b);
            for c in BoolIterator::default() {
                print!(
                    "foo: {} {} {} ",
                    a.clone().to_bool(),
                    b.clone().to_bool(),
                    c.clone().to_bool()
                );
                use std::sync::atomic::Ordering;
                let a1 = AtomicBool::new(a.clone().into());
                a1.fetch_nand(b.clone().into(), Ordering::SeqCst);
                a1.fetch_nand(c.clone().into(), Ordering::SeqCst);
                println!("{:?}", a1);
            }
        }
        //
        // struct CatIterator<T>(Option<T>);
        //
        // impl Iterator for CatIterator<bool> {
        //     type Item = bool;
        //
        //     fn next(&mut self) -> Option<Self::Item> {
        //         match self.0 {
        //             None => self.0 = Some(false),
        //             Some(false) => self.0 = Some(true),
        //             Some(true) => self.0 = None,
        //         }
        //         self.0
        //     }
    }

    let inputs = vec![BoolIterator, BoolIterator, BoolIterator];

    // println!("True : {:?}", Bool::T);
    // println!("False: {:?}", Bool::F);
}
