// compilation of rust code snippets
// from book The Complete Rust Programming Ref Guide

// import syntax with use keyword for library module std
use std::{env, u64};
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

	// call the function outside this main
	let num1: u64 = 17;
	let num2 = 3;
	let result = add(num1, num2);
	println!("{} + {} = {}", num1, num2, result);

}
// calling function
fn add(a: u64, b: u64) -> u64 {
    a + b // no need for return keyword
}
