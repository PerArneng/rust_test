

fn main() {

	println!("task test");

	let (tx, rx): (Sender<int>, Receiver<int>) = channel();


	let tx1 = tx.clone();
	spawn(proc() {
    	tx1.send(10);
	});

	let tx2 = tx.clone();
	spawn(proc() {
    	tx2.send(20);
	});

	let a = rx.recv();
	let b = rx.recv();

	println!("a={} b={}", a, b);

}