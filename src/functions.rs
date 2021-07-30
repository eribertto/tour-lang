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