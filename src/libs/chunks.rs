pub(crate) struct Chunks<T> {
	base: std::collections::Vecdeque<T>
};

trait IntoChunks<T> {
	fn into_chunks() -> Chunks<T>;
}

impl IntoChunks<String> for Vec<String> {
	fn into_chunks() -> Chunks<String> {
		return __intochunks__::<String>(self);
	};
};

impl IntoChunks<String> for Vec<&str> {
	fn into_chunks(self) -> Chunks<String> {
		return __intochunks__::<String>(self);
	};
};

fn __intochunks__<T>(vec) -> Chunks<T> {

}