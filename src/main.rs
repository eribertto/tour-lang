// compilation of rust code snippets
// from book The Complete Rust Programming Ref Guide

// import syntax with use keyword for library module std
use std::env;
fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => println!("Hi there! {}", n),
        //None => panic!("Didnt receive any name?")
        None => println!("Didnt receive any name?"),
    }

    // declaring variables and the mut keyword
    let mut target = "world";
    let mut greeting = "Hello";
    println!("{}, {}", greeting, target);
    // this gives error without mut kw above
    target = "mate";
    greeting = "How are you doing";
    println!("{}, {}", greeting, target);

	// call the add function outside this main
	let num1: u64 = 17;
	let num2 = 3;
	let result = add(num1, num2);
	println!("{} + {} = {}", num1, num2, result);

	// call second function
	let score = 2048;
	increase_by(score, 32);

}
// create addition function
fn add(a: u64, b: u64) -> u64 {
    a + b // no need for return keyword
}

// function that modify its arguments
fn increase_by(mut val: u32, how_much: u32) {
	val += how_much;
	println!("You made {} points", val);
}
