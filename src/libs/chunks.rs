pub(crate) struct Chunks<T> {
    pub(crate) base: Vec<T>,
}

impl<T> Chunks<T> {
	pub(crate) fn new() -> Chunks::<T> {
		return Chunks::<T> {
			base: Vec::new()
		};
	}

	pub(crate) fn iterate(self) -> Vec<T> {
		return self.base; 
	}
}

pub(crate) trait Chunkable<'a, T> {
    fn into_chunks(&'a self, memory: usize) -> Chunks<&'a [T]>;
}

impl<'a, T> Chunkable<'a, T> for Vec<T> {
    fn into_chunks(&'a self, memory: usize) -> Chunks<&'a [T]> {
        let mut chunks: Vec<&'a [T]> = Vec::new();
        for i in (memory..self.len() + 1).step_by(memory) {
            chunks.push(&self[memory - i..i]);
        }
        return Chunks { base: chunks };
    }
}
