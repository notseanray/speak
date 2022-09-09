// TODOS
// - .specific_chunk(...): Returns an specific chunk instead of the whole list
//   of chunks (Improve performance multiple times)

#[must_use]
#[doc(hidden)]
pub struct Chunks<'a, T> {
	pub(crate) base: Vec<&'a [T]>,
}

#[doc(hidden)]
pub trait Chunkable<'a, T> {
	#[must_use]
	fn into_chunks(&'a self, memory: usize) -> Chunks<'a, T>;
	// fn specific_chunk(&'a self, memory: usize, index: usize) -> &'a [T];
}

impl<'a, T> Chunkable<'a, T> for Vec<T> {
	#[must_use]
	fn into_chunks(&'a self, memory: usize) -> Chunks<T> {
		if memory >= self.len() {
			return Chunks {
				base: vec![&self[..]],
			};
		}

		let mut chunks: Vec<&'a [T]> = Vec::new();
		for i in (memory..self.len() + 1).step_by(memory) {
			chunks.push(&self[i - memory..i]);
		}

		if self.len() % memory != 0 {
			chunks.push(&self[self.len() - memory..]);
		};

		Chunks::<T> { base: chunks }
	}

	// fn specific_chunk(&'a self, memory: usize, index: usize) -> &'a [T] {
	// 	if (self.len() % memory != 0) && (index > self.len() - memory) {
	// 			return &self[self.len() - memory ..];
	// 	};
	// 	return &self[index * memory .. (index + 1) * memory];
	// }
}
