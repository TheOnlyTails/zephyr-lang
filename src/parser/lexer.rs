use logos::{Logos, SpannedIter};

use crate::utils::IntoOk;

pub(crate) type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Clone)]
pub(crate) struct Lexer<'input, T: Logos<'input>> where T::Extras: Clone {
	token_stream: SpannedIter<'input, T>,
}

impl<'input, T: Logos<'input>> Lexer<'input, T>
where
	T::Extras: Default + Clone,
{
	pub(crate) fn new(input: &'input T::Source) -> Lexer<'input, T> {
		Self {
			token_stream: T::lexer(input).spanned(),
		}
	}
}

impl<'input, T: Logos<'input>> Iterator for Lexer<'input, T> where T::Extras: Clone {
	type Item = Spanned<T, usize, T::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		self
			.token_stream
			.next()
			.map(|(token, span)| (span.start, token?, span.end).ok())
	}
}
