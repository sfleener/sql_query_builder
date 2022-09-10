use std::fmt::Formatter;
use crate::{
  behavior::{concat_raw_before_after, Concat, ConcatMethods},
  fmt,
  structure::{Select, SelectClause},
};
use crate::fmt::Format;

impl<'a> ConcatMethods<'a, SelectClause> for Select<'_> {}

impl Concat for Select<'_> {
  fn concat(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    self.concat_raw(fmt, &fmts, &self._raw)?;
    #[cfg(feature = "postgresql")]
    {
      self.concat_with(
        &self._raw_before,
        &self._raw_after,
        fmt,
        &fmts,
        SelectClause::With,
        &self._with,
      )?;
    }
    self.concat_select(fmt, &fmts)?;
    self.concat_from(
      &self._raw_before,
      &self._raw_after,
      fmt,
      &fmts,
      SelectClause::From,
      &self._from,
    )?;
    self.concat_join(fmt, &fmts)?;
    self.concat_where(
      &self._raw_before,
      &self._raw_after,
      fmt,
      &fmts,
      SelectClause::Where,
      &self._where,
    )?;
    self.concat_group_by(fmt, &fmts)?;
    self.concat_having(fmt, &fmts)?;
    self.concat_order_by(fmt, &fmts)?;
    self.concat_limit(fmt, &fmts)?;
    self.concat_offset(fmt, &fmts)?;
    #[cfg(feature = "postgresql")]
    {
      use crate::structure::Combinator;
      self.concat_combinator(fmt, &fmts, Combinator::Except)?;
      self.concat_combinator(fmt, &fmts, Combinator::Intersect)?;
      self.concat_combinator(fmt, &fmts, Combinator::Union)?;
    }

    Ok(())
  }
}

impl Select<'_> {
  #[cfg(feature = "postgresql")]
  fn concat_combinator(
    &self,
    fmt: &mut std::fmt::Formatter<'_>,
    fmts: &fmt::Format,
    combinator: crate::structure::Combinator,
  ) -> std::fmt::Result {
    use crate::behavior::raw_queries;
    use crate::structure::Combinator;

    let fmt::Format { lb, .. } = fmts;
    let (clause, clause_name, clause_list) = match combinator {
      Combinator::Except => (SelectClause::Except, "EXCEPT", &self._except),
      Combinator::Intersect => (SelectClause::Intersect, "INTERSECT", &self._intersect),
      Combinator::Union => (SelectClause::Union, "UNION", &self._union),
    };

    let raw_before = raw_queries(&self._raw_before, &clause).join(" ");
    let raw_after = raw_queries(&self._raw_after, &clause).join(" ");

    let space_before = if raw_before.is_empty() {
      ""
    } else {
      " "
    };
    let space_after = if raw_after.is_empty() {
      ""
    } else {
      " "
    };

    if clause_list.is_empty() {
      return write!(fmt, "{raw_before}{space_before}{raw_after}{space_after}");
    }

    write!(fmt, "({raw_before}) ")?;

    clause_list.iter().try_for_each(|select| {
      write!(fmt, "{clause_name} ({lb}")?;
      select.concat(fmt, fmts)?;
      write!(fmt, ") {lb}")
    })?;

    write!(fmt, "{raw_after}{space_after}")
  }

  fn concat_group_by(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { comma, lb, .. } = fmts;
    let sql = if self._group_by.is_empty() == false {
      let columns = self._group_by.join(comma);
      format!("GROUP BY {columns} {lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::GroupBy,
      sql,
    )
  }

  fn concat_having(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { lb, .. } = fmts;
    let sql = if self._having.is_empty() == false {
      let conditions = self._having.join(" AND ");
      format!("HAVING {conditions} {lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Having,
      sql,
    )
  }

  fn concat_join(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { lb, .. } = fmts;
    let sql = if self._join.is_empty() == false {
      let joins = self._join.join(format!(" {lb}").as_str());
      format!("{joins} {lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Join,
      sql,
    )
  }

  fn concat_limit(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { lb, .. } = fmts;
    let sql = if self._limit.is_empty() == false {
      let count = self._limit;
      format!("LIMIT {count} {lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Limit,
      sql,
    )
  }

  fn concat_offset(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { lb, .. } = fmts;
    let sql = if self._offset.is_empty() == false {
      let start = self._offset;
      format!("OFFSET {start} {lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Offset,
      sql,
    )
  }

  fn concat_order_by(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { comma, lb, .. } = fmts;
    let sql = if self._order_by.is_empty() == false {
      let columns = self._order_by.join(comma);
      format!("ORDER BY {columns} {lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::OrderBy,
      sql,
    )
  }

  fn concat_select(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result {
    let fmt::Format { comma, lb, .. } = fmts;
    let sql = if self._select.is_empty() == false {
      let columns = self._select.join(comma);
      format!("SELECT {columns} {lb}")
    } else {
      "".to_owned()
    };

    concat_raw_before_after(
      &self._raw_before,
      &self._raw_after,
      query,
      fmts,
      SelectClause::Select,
      sql,
    )
  }
}
