// FROM HERE
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d85ed5c8255c2d903a9d3df972ae2f75

use std::borrow::Borrow;

struct ReferenceHoldingStruct<S: Borrow<str>> {
    pub prop: S,
}

impl<S: Borrow<str>> ReferenceHoldingStruct<S> {
    fn do_something(&self) {
        println!("{}", self.prop.borrow());
    }
}

fn generate_these_1<'a>(input: &'a str) -> ReferenceHoldingStruct<&'a str> {
    ReferenceHoldingStruct { prop: input }
}

fn generate_these_2(input: String) -> ReferenceHoldingStruct<String> {
    ReferenceHoldingStruct { prop: input }
}

fn main() {
    let r = generate_these_1("Reference");
    let o = generate_these_2("Owned".to_string());

    r.do_something();
    o.do_something();
}


// cargo build --example referenceHoldingStruct
// cargo run --example referenceHoldingStruct