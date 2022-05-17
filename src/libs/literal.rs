use crate::Dyn;
pub trait Literal {
	fn literal(&self) -> Option<&str>;
}

impl<T> Literal for T where T: Dyn {
	fn literal(&self) -> Option<&str> {
		match self.isstr() {
			true => Some(self.str()),
			false => None
		}
	}
}

pub(crate) fn literal_vec<'a, T>(vec: &'a Vec<T>) -> Vec<Option<&'a str>> where T: Literal + 'a {
	vec.into_iter().map(|x| x.literal()).collect()
}