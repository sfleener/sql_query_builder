use std::fmt::Formatter;
use ansi_term::Color::Blue;
use crate::{
  behavior::{Concat, ConcatMethods},
  fmt,
  structure::{Delete, DeleteClause},
};
use crate::behavior::{concat_raw_after, concat_raw_before};
use crate::fmt::Format;

impl<'a> ConcatMethods<'a, DeleteClause> for Delete<'_> {}

impl Concat for Delete<'_> {
  fn concat(&self, fmt: &mut Formatter<'_>, fmts: &Format) -> std::fmt::Result {
    self.concat_raw(fmt, &fmts, &self._raw)?;
    #[cfg(feature = "postgresql")]
    {
      self.concat_with(
        &self._raw_before,
        &self._raw_after,
        fmt,
        &fmts,
        DeleteClause::With,
        &self._with,
      )?;
    }
    self.concat_delete_from(fmt, &fmts)?;
    self.concat_where(
      &self._raw_before,
      &self._raw_after,
      fmt,
      &fmts,
      DeleteClause::Where,
      &self._where,
    )?;
    #[cfg(feature = "postgresql")]
    {
      self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        fmt,
        &fmts,
        DeleteClause::Returning,
        &self._returning,
      )?;
    }
    Ok(())
  }
}

impl Delete<'_> {
  fn concat_delete_from(&self, fmt: &mut Formatter, fmts: &Format) -> std::fmt::Result {
    let fmt::Format { lb, .. } = fmts;
    let clause = DeleteClause::DeleteFrom;

    concat_raw_before(&self._raw_before, fmt, fmts, &clause)?;

    if self._delete_from.is_empty() == false {
      let table_name = self._delete_from;
      fmts.write_blue(fmt, "DELETE FROM")?;
      write!(fmt, " {table_name} {lb}")?;
    }

    concat_raw_after(&self._raw_after, fmt, fmts, &clause)
  }
}
