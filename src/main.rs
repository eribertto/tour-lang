// compilation of rust code snippets
// from book The Complete Rust Programming Ref Guide

// import syntax with use keyword for library module std
use std::env;

use crate::functions::myconditionals;
mod functions;		// import from functions.rs

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
	let result = functions::add(num1, num2);
	println!("{} + {} = {}", num1, num2, result);

	// call second function
	let score = 2048;
	functions::increase_by(score, 32);

	// introducing closure e.g. like a function but has more info
	// of the environment or scope in which it is declared
	// unlike functions, closures are defined without a func name
	let doubler = |x| x * 2;
	let value = 5;
	let twice = doubler(value);
	println!("{} doubled is {}", value, twice);

	// storing a closure in a variable
	let big_closure = |b: i32, c| {
		let z = b + c;
		z * twice
	};		// note the closing semicolon

	let some_number = big_closure(10, 20);
	println!("Result from big closure: {}", some_number);

	// all about strings, in Rust there are 2 types: the &str
	// and the String type
	// note: String types are allocated on the heap while & are
	// usually pointers to an existing string, which can either be
	// on the stack, the heap or a string in the data segment of compiled code.
	let question = "How are you?";	// & is an operator used to create a pointer to any type
	let person: String = "Bob".to_string();
	let namaste = String::from("नमते");	// unicodes!
	println!("{}! {} {}", namaste, question, person);


	// call conditionals in functions.rs
	myconditionals();

	// about match expression
	fn req_status() -> u32 {
		// return 200
		200
	}
	let status = req_status();
	match status {
		200 => println!("Success, match status found 200!"),
		404 => println!("NOt found!"),
		other => {
			println!("Request failed with code: {}", other);
			// get response from cache
		}
	}

	// all about loops
	let mut x = 1024;
	loop {
		if x < 0 {
			break;
		}
		println!("{} more runs to go", x);
		x -= 1; 	//decrement by one each loop
	}
}



