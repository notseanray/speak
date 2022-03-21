use std::collections::HashMap;
use crate::Literal;

pub(crate) struct Map<T> {
	keys: Vec<T>,
	values: Vec<T>
}

// In this case this function is public, because maybe an user would like to translate the HashMap, for some reason idk
pub(crate) trait ToMap<T> {
	fn to_map(self) -> Map<T>;
}

// General HashMap -> Map

impl<T> ToMap<String> for HashMap<T, T> where T: Clone + Literal<String> {
	fn to_map(self) -> Map<String> {
		return Map::<String> {
			keys: self.clone().into_keys().collect::<Vec<T>>().literal(),
			values: self.into_values().collect::<Vec<T>>().literal()
		}
	}
}