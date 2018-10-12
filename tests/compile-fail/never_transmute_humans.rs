// This should fail even without validation
// compile-flags: -Zmir-emit-validate=0 -Zmiri-disable-validation

#![feature(never_type)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

struct Human;

fn main() {
    let x: ! = unsafe {
        std::mem::transmute::<Human, !>(Human) //~ ERROR constant evaluation error
        //^~ NOTE entered unreachable code
    };
    f(x)
}

fn f(x: !) -> ! { x }
