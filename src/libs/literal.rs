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
