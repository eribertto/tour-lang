// this functions will be called in main
// e.g imported

// create addition function
pub fn add(a: u64, b: u64) -> u64 {
    a + b // no need for return keyword
}



// function that modify its arguments
pub fn increase_by(mut val: u32, how_much: u32) {
	val += how_much;
	println!("You made {} points", val);
}

// conditionals if-else
pub fn myconditionals() {
	let rust_is_awesome = true;
	if rust_is_awesome {
		println!("Indeed, Rust is awesome!");
	} else {
		println!("Well, you should try Rust!");
	}

	// assigning conditional result to a variable
	let result = if 1 == 2 {
		"Wait, what???!!!"
	} else {
		"Rust is smart to figure out equality"
	};		// note this semicolon is needed for the let statement above
	println!("You know what? {}.", result);
}
