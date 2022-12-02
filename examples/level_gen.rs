use tankgen::level::Level;

fn main() {
	let mut level = Level::default();
	level.set_cell_size((16, 6));
	level.gen(0..5, 0..5);
	println!("{}", String::from(level));
}
