use speak::{DynMap, Dyn, IsString, DE};
fn main() {
	let mut map		=	speak::map![("hi", "hello"), ("bye", 0usize)];
	let learnt	=	speak::learn(&map, None);
	let ran		=	speak::run("bye", &map, &learnt, None, None, None, None);

	map.push_dyn((&DE::String("hi"), &DE::Number(0usize)));

	println!("{:?}", ran);
}