// compilation of rust code snippets
// from book The Complete Rust Programming Ref Guide

// import syntax with use keyword for library module std
use std::env;
fn main() {
    let name = env::args().skip(1).next();
	match name {
		Some(n) => println!("Hi there! {}", n),
		//None => panic!("Didnt receive any name?")
		None => println!("Didnt receive any name?")
	}
	let mut target = "world";
	let mut greeting = "Hello";
	println!("{}, {}", greeting, target);
	// this gives error without mut kw above
	target = "mate";
	greeting = "How are you doing";
	println!("{}, {}", greeting, target);

}
