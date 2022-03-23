pub(crate) struct Chunks<T> {
    pub(crate) base: Vec<T>,
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
