pub(crate) struct Chunks<'a, T> {
    pub(crate) base: Vec<&'a [T]>,
	pub(crate) buf_size: usize
}

impl<'a, T> Chunks<'a, T> {
	pub(crate) fn new(buf: usize) -> Chunks::<'a, T> {
		return Chunks::<T> {
			base: Vec::new(),
			buf_size: buf
		};
	}

	pub(crate) fn iterate(self) -> Vec<&'a[T]> {
		return self.base; 
	}
}

pub(crate) trait Chunkable<'a, T> {
    fn into_chunks(&'a self, memory: usize) -> Chunks<'a, T>;
}

impl<'a, T> Chunkable<'a, T> for Vec<T> {
    fn into_chunks(&'a self, memory: usize) -> Chunks<T> {
        let mut chunks: Vec<&'a [T]> = Vec::new();
        for i in (memory..self.len() + 1).step_by(memory) {
            chunks.push(&self[memory - i..i]);
        }
        return Chunks { base: chunks, buf_size: memory };
    }
}
