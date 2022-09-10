use std::fmt::Formatter;
use crate::{
  behavior::{Concat, ConcatMethods},
  fmt,
  structure::{Values, ValuesClause},
};
use crate::fmt::Format;

impl<'a> ConcatMethods<'a, ValuesClause> for Values {}

impl Concat for Values {
  fn concat(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts, &self._raw);
    query = self.concat_values(
      &self._raw_before,
      &self._raw_after,
      query,
      &fmts,
      ValuesClause::Values,
      &self._values,
    );

    query.trim_end().to_owned()
  }
}
