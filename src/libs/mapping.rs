use std::collections::HashMap;

pub(crate) struct Map<T> {
	pub(crate) keys: Vec<T>,
	pub(crate) values: Vec<T>
}

trait ToMap<T> {
	fn to_map(self) -> Map<T>;
}

impl<T> ToMap<T> for HashMap<T, T> where T: Clone {
	fn to_map(self) -> Map<T> {
		Map::<T> {
			keys: self.clone().into_keys().collect(),
			values: self.into_values().collect()
		}
	}
}
