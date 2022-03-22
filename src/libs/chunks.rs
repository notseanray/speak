pub(crate) struct Chunks<T> {
	base: Vec<T>
}

impl<T> Chunks<T> {
	fn new() -> Chunks<T> {
		return Chunks::<T> {
			base: Vec::new()
		};
	}
}

pub(crate) trait Chunkable<T> {
	fn into_chunks(&self, memory: usize) -> Chunks<&[T]>;
}

// Sometimes when I see Rust I remember that I will have to debug this in 2 days later.
impl<T> Chunkable<T> for Vec<T> {
	fn into_chunks(&self, memory: usize) -> Chunks<&[T]> {
		let mut chunks: Chunks<&[T]> = Chunks::new();
		for i in (memory .. self.len() + 1).step_by(memory) {
			chunks.base.push(&self[memory - i .. i]);
		}
		return chunks;
	}
}
