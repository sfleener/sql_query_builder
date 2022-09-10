use crate::{
  behavior::{Concat, ConcatMethods},
  fmt,
  structure::{Insert, InsertClause},
};
use crate::behavior::{concat_raw_after, concat_raw_before};

impl<'a> ConcatMethods<'a, InsertClause> for Insert<'_> {}

impl Concat for Insert<'_> {
  fn concat(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    self.concat_raw(fmt, &fmts, &self._raw)?;
    #[cfg(feature = "postgresql")]
    {
      self.concat_with(
        &self._raw_before,
        &self._raw_after,
        fmt,
        &fmts,
        InsertClause::With,
        &self._with,
      )?;
    }
    self.concat_insert_into(fmt, &fmts)?;
    self.concat_overriding(fmt, &fmts)?;
    self.concat_values(
      &self._raw_before,
      &self._raw_after,
      fmt,
      &fmts,
      InsertClause::Values,
      &self._values,
    )?;
    self.concat_select(fmt, &fmts)?;
    self.concat_on_conflict(fmt, &fmts)?;

    #[cfg(feature = "postgresql")]
    {
      self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        fmt,
        &fmts,
        InsertClause::Returning,
        &self._returning,
      )?;
    }

    Ok(())
  }
}

impl Insert<'_> {
  fn concat_insert_into(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { lb, .. } = fmts;

    let clause = InsertClause::InsertInto;
    concat_raw_before(&self._raw_before, fmt, fmts, &clause)?;

    if self._insert_into.is_empty() == false {
      let insert_into = self._insert_into;
      fmts.write_blue(fmt, "INSERT INTO")?;
      write!(fmt, " {insert_into} {lb}")?;
    }

    concat_raw_after(&self._raw_after, fmt, fmts, &clause)
  }

  fn concat_overriding(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { lb, .. } = fmts;

    let clause = InsertClause::Overriding;
    concat_raw_before(&self._raw_before, fmt, fmts, &clause)?;

    if self._overriding.is_empty() == false {
      let overriding = self._overriding;
      fmts.write_blue(fmt, "OVERRIDING")?;
      write!(fmt, " {overriding} {lb}")?;
    }

    concat_raw_after(&self._raw_after, fmt, fmts, &clause)
  }

  fn concat_on_conflict(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { lb, .. } = fmts;

    let clause = InsertClause::OnConflict;
    concat_raw_before(&self._raw_before, fmt, fmts, &clause)?;

    if self._on_conflict.is_empty() == false {
      let overriding = self._on_conflict;
      fmts.write_blue(fmt, "ON CONFLICT")?;
      write!(fmt, " {overriding} {lb}")?;
    }

    concat_raw_after(&self._raw_after, fmt, fmts, &clause)
  }

  fn concat_select(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { lb, .. } = fmts;

    let clause = InsertClause::Select;
    concat_raw_before(&self._raw_before, fmt, fmts, &clause)?;

    if let Some(select) = &self._select {
      select.concat(fmt, fmts)?;
      write!(fmt, " {lb}")?;
    }

    concat_raw_after(&self._raw_after, fmt, fmts, &clause)
  }
}
