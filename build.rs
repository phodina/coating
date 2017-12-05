extern crate cheddar;

fn main() {

	println!("Generating header for coating ...");
    cheddar::Cheddar::new().expect("could not read manifest")
        .module("msg").expect("malformed module path")
        .run_build("include/coating.h");
}