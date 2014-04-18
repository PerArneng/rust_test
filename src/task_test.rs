



fn main() {
	println!("task test");

	let (tx, rx): (Sender<int>, Receiver<int>) = channel();


	spawn(proc() {
    	tx.send(10);
	});

	spawn(proc() {
    	tx.send(20);
	});

	let a = rx.recv();
	let b = rx.recv();

	println!("a={} b={}", a, b);
}