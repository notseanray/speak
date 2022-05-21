use speak::*;
fn main() {
	let map		=	speak::map![("hi", "hello"), ("bye", 0usize)];
	let learnt	=	speak::learn(&map, None);
	let ran		=	speak::run("bye", &map, &learnt, None, None, None, None);

	println!("{:?}", ran);
}