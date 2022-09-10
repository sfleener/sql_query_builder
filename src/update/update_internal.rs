use std::fmt::Formatter;
use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatMethods},
  fmt,
  structure::{Update, UpdateClause},
};
use crate::fmt::Format;

impl<'a> ConcatMethods<'a, UpdateClause> for Update<'_> {}

impl Concat for Update<'_> {
  fn concat(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts, &self._raw);
    #[cfg(feature = "postgresql")]
    {
      query = self.concat_with(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::With,
        &self._with,
      );
    }
    query = self.concat_update(query, &fmts);
    query = self.concat_set(query, &fmts);
    #[cfg(feature = "postgresql")]
    {
      query = self.concat_from(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::From,
        &self._from,
      );
    }
    query = self.concat_where(
      &self._raw_before,
      &self._raw_after,
      query,
      &fmts,
      UpdateClause::Where,
      &self._where,
    );

    #[cfg(feature = "postgresql")]
    {
      query = self.concat_returning(
        &self._raw_before,
        &self._raw_after,
        query,
        &fmts,
        UpdateClause::Returning,
        &self._returning,
      );
    }

    query.trim_end().to_owned()
  }
}

impl Update<'_> {
  fn concat_set(&self, query: String, fmts: &fmt::Format) -> String {
    let fmt::Format { comma, lb, .. } = fmts;
    let sql = if self._set.is_empty() == false {
      let values = self._set.join(comma);
      format!("SET {values} {lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(&self._raw_before, &self._raw_after, query, fmts, UpdateClause::Set, sql)
  }

  fn concat_update(&self, query: String, fmts: &fmt::Format) -> String {
    let fmt::Format { lb, .. } = fmts;
    let sql = if self._update.is_empty() == false {
      let table_name = self._update;
      format!("UPDATE {table_name} {lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      UpdateClause::Update,
      sql,
    )
  }
}
