

fn main() {
	let mut sum: u64 = 0;
	let start = time::now();
	for i in 0..1000000000 {
		sum += i
	}
	let end = time::now();
	println!("{} {}", end - start, sum);
}
