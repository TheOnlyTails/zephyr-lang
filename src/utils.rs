pub(crate) trait IntoOk<T, E> {
	fn ok(self) -> Result<T, E>;
}

impl<T, E> IntoOk<T, E> for T {
	fn ok(self) -> Result<T, E> {
		Ok(self)
	}
}

pub(crate) trait IntoErr<T, E> {
	fn err(self) -> Result<T, E>;
}

impl<T, E> IntoErr<T, E> for E {
	fn err(self) -> Result<T, E> {
		Err(self)
	}
}
