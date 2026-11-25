#![allow(unused)]

struct A;

// private first, then public, then constructors
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

// Two constructors, both should be considered constructors
impl B {
    pub fn my_constructor() -> Self {
        B
    }

    pub fn my_constructor2() -> Self {
        B
    }
}

struct C;

// Option<Self> should be considered a constructor
impl C {
    pub fn foo(&self) {}

    pub fn maybe_constructor() -> Option<Self> {
        //~^ funcorder_rs
        Option::Some(C)
    }
}

struct D;

// Not using Self, but using the struct type directly, should still be considered a constructor
impl D {
    pub fn foo(&self) {}

    pub fn my_constructor() -> D {
        //~^ funcorder_rs
        D {}
    }
}

struct E;

// Using Result<E>, should still be considered a constructor
impl E {
    pub fn foo(&self) {}

    pub fn my_constructor() -> Result<E, ()> {
        //~^ funcorder_rs
        Ok(E {})
    }
}

struct GeneratedImpl;

macro_rules! generated_impl {
    () => {
        impl GeneratedImpl {
            fn private(&self) {}
            pub fn public(&self) {}
        }
    };
}

generated_impl!();

struct GeneratedMethod;

macro_rules! generated_public_method {
    () => {
        pub fn public(&self) {}
    };
}

impl GeneratedMethod {
    fn private(&self) {}
    generated_public_method!();
}

fn main() {
    let a = A::my_constructor();
    a.foo();
}
