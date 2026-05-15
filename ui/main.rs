#![allow(unused)]

struct A;
impl A {
    fn do_nothing() {}

    fn fo(&self) {}

    pub fn foo(&self) {}
    //~^ funcorder_rs

    pub fn my_constructor() -> Self {
        //~^ funcorder_rs
        A
    }
}

struct B;
impl B {
    pub fn my_constructor() -> Self {
        B
    }

    pub fn my_constructor2() -> Self {
        B
    }
}

struct C;
impl C {
    pub fn foo(&self) {}

    pub fn maybe_constructor() -> Option<Self> {
        //~^ funcorder_rs
        Option::Some(C)
    }
}

fn main() {
    let a = A::my_constructor();
    a.foo();
}
