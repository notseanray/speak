use crate::mapping::{Dyn, DE};
pub trait Literal {
	fn literal(&self) -> Option<&str>;
}

impl<T> Literal for T where T: Dyn {
	fn literal(&self) -> Option<&str> {
		match self.to_enum() {
			DE::String(_) => Some(self.to_str()),
			DE::Number(_) => None
		}
	}
}
