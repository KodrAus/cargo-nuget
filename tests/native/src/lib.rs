#[no_mangle]
pub extern fn run() -> bool {
    println!("Hello, World!");

    true
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		
	}
}