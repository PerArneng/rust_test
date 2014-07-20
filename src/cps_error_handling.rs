
use std::io::{File, Open, ReadWrite, IoError};

fn open_file(path : &str, err_handler: |IoError| -> (IoError)) -> (File) {
	let p = Path::new(path);

	let file = match File::open_mode(&p, Open, ReadWrite) {
	    Ok(f) => f,
	    Err(e) => err_handler(e),
	};

	return file
}


fn main() {

	let err_handler = | err: IoError | { println!("err: {}", err.desc); return err; };

	let mut test_file = open_file("hello_world1.txt", err_handler);
	match test_file.write_line("Hello, World!") {
	    Ok(_) => test_file,
	    Err(e) => fail!("could not write to file: {}", e),
	};


	let p = Path::new("hello_world.txt");

	let mut file = match File::open_mode(&p, Open, ReadWrite) {
	    Ok(f) => f,
	    Err(e) => fail!("could not open file: {}", e),
	};	

	match file.write_line("Hello, World!") {
	    Ok(_) => file,
	    Err(e) => fail!("could not write to file: {}", e),
	};

}
