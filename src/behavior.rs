use crate::fmt;
use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter};
use crate::fmt::{Joiner};

pub fn push_unique<T: Eq>(list: &mut Vec<T>, value: T) {
  let prev_item = list.iter().find(|&item| *item == value);
  if prev_item.is_none() {
    list.push(value);
  }
}

pub fn raw_queries<'a, Clause: PartialEq>(raw_list: &'a Vec<(Clause, String)>, clause: &'a Clause) -> Vec<String> {
  raw_list
    .iter()
    .filter(|item| item.0 == *clause)
    .map(|item| item.1.clone())
    .collect::<Vec<_>>()
}

/// Represents all statements that can be used in the with method
pub trait WithQuery: Concat {}

pub trait Concat {
  fn concat(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format) -> std::fmt::Result;
}

pub fn concat_raw_before<Clause: PartialEq>(
  items_before: &Vec<(Clause, String)>,
  fmt: &mut std::fmt::Formatter<'_>,
  fmts: &fmt::Format,
  clause: &Clause,
) -> std::fmt::Result {
  let mut joiner = Joiner::new(fmt, |fmt| write!(fmt, " "));
  let raw_before = raw_queries(items_before, clause);
  raw_before.iter().try_for_each(|item| joiner.entry(item))?;
  joiner.finish();

  let space_before = if raw_before.is_empty() == false { " " } else { "" };
  write!(fmt, "{space_before}")
}

pub fn concat_raw_after<Clause: PartialEq>(
  items_after: &Vec<(Clause, String)>,
  fmt: &mut std::fmt::Formatter<'_>,
  fmts: &fmt::Format,
  clause: &Clause,
) -> std::fmt::Result {
  let mut joiner = Joiner::new(fmt, |fmt| write!(fmt, " "));
  let raw_after = raw_queries(items_after, clause);
  raw_after.iter().try_for_each(|item| joiner.entry(item))?;
  joiner.finish();

  let space_after = if raw_after.is_empty() == false { " " } else { "" };
  write!(fmt, "{space_after}")
}

pub fn concat_raw_before_after<Clause: PartialEq>(
  items_before: &Vec<(Clause, String)>,
  items_after: &Vec<(Clause, String)>,
  fmt: &mut std::fmt::Formatter<'_>,
  fmts: &fmt::Format,
  clause: Clause,
  sql: String,
) -> std::fmt::Result {
  concat_raw_before(items_before, fmt, fmts, &clause)?;
  write!(fmt, "{sql}")?;
  concat_raw_after(items_after, fmt, fmts, &clause)
}

pub trait ConcatMethods<'a, Clause: PartialEq> {
  fn concat_from(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    fmt: &mut std::fmt::Formatter<'_>,
    fmts: &fmt::Format,
    clause: Clause,
    items: &Vec<String>,
  ) -> std::fmt::Result {
    let fmt::Format { comma, lb, .. } = fmts;

    concat_raw_before(items_raw_before, fmt, fmts, &clause)?;

    if items.is_empty() == false {
      fmts.write_blue(fmt, "FROM ")?;

      let mut joiner = Joiner::new(fmt, |fmt| write!(fmt, "{}", comma));
      items.iter().try_for_each(|item| joiner.entry(item))?;
      joiner.finish();

      write!(fmt, " {lb}")?;
    };

    concat_raw_after(items_raw_after, fmt, fmts, &clause)
  }

  fn concat_raw(&self, fmt: &mut std::fmt::Formatter<'_>, fmts: &fmt::Format, items: &Vec<String>) -> std::fmt::Result {
    if items.is_empty() {
      return Ok(());
    }
    let fmt::Format { lb, .. } = fmts;

    let mut joiner = Joiner::new(fmt, |fmt| write!(fmt, " "));
    items.iter().try_for_each(|item| joiner.entry(item))?;

    write!(fmt, " {lb}")
  }

  #[cfg(feature = "postgresql")]
  fn concat_returning(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    fmt: &mut std::fmt::Formatter<'_>,
    fmts: &fmt::Format,
    clause: Clause,
    items: &Vec<String>,
  ) -> std::fmt::Result {
    let fmt::Format { lb, comma, .. } = fmts;

    concat_raw_before(items_raw_before, fmt, fmts, &clause)?;

    if items.is_empty() == false {
      fmts.write_blue(fmt, "RETURNING ")?;

      let mut joiner = Joiner::new(fmt, |fmt| write!(fmt, "{}", comma));
      items.iter().try_for_each(|item| joiner.entry(item))?;
      joiner.finish();

      write!(fmt, " {lb}")?;
    }

    concat_raw_after(items_raw_after, fmt, fmts, &clause)
  }

  fn concat_values(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    fmt: &mut std::fmt::Formatter<'_>,
    fmts: &fmt::Format,
    clause: Clause,
    items: &Vec<String>,
  ) -> std::fmt::Result {
    let fmt::Format { comma, lb, .. } = fmts;

    concat_raw_before(items_raw_before, fmt, fmts, &clause)?;

    if items.is_empty() == false {
      let sep = format!("{comma}{lb}");
      let values = items.join(&sep);
      fmts.write_blue(fmt, "VALUES")?;
      write!(fmt, " {lb}{values} {lb}")?;
    }

    concat_raw_after(items_raw_after, fmt, fmts, &clause)
  }

  fn concat_where(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    fmt: &mut std::fmt::Formatter<'_>,
    fmts: &fmt::Format,
    clause: Clause,
    items: &Vec<String>,
  ) -> std::fmt::Result {
    let fmt::Format { lb, indent, colorize, .. } = fmts;
    concat_raw_before(items_raw_before, fmt, fmts, &clause)?;

    if items.is_empty() == false {
      fmts.write_blue(fmt, "WHERE ")?;

      let mut joiner = Joiner::new(fmt, |fmt| {
        write!(fmt, " {lb}{indent}")?;
        fmts.write_blue(fmt, "AND ")
      });
      items.iter().try_for_each(|item| joiner.entry(item))?;
      joiner.finish();

      write!(fmt, " {lb}")?;
    }

    concat_raw_after(items_raw_after, fmt, fmts, &clause)
  }

  #[cfg(feature = "postgresql")]
  fn concat_with(
    &self,
    items_raw_before: &Vec<(Clause, String)>,
    items_raw_after: &Vec<(Clause, String)>,
    fmt: &mut std::fmt::Formatter<'_>,
    fmts: &fmt::Format,
    clause: Clause,
    items: &Vec<(&'a str, std::sync::Arc<dyn WithQuery>)>,
  ) -> std::fmt::Result {
    let fmt::Format {
      comma,
      lb,
      indent,
      ..
    } = fmts;

    concat_raw_before(items_raw_before, fmt, fmts, &clause)?;

    if !items.is_empty() {
      fmts.write_blue(fmt, "WITH")?;
      write!(fmt, " {lb}")?;

      let with = items.iter().try_for_each(|(name, query)| {
        let inner_lb = format!("{lb}{indent}");
        let inner_fmts = fmt::Format {
          comma,
          lb: inner_lb.as_str(),
          indent,
          ..*fmts
        };
        write!(fmt, "{name} ")?;
        fmts.write_blue(fmt, "AS")?;
        write!(fmt, " ({lb}{indent}")?;
        query.concat(fmt, &inner_fmts)?;
        write!(fmt, "{lb}){comma}{lb}")
      })?;

      write!(fmt, " {lb}")?;
    }

    concat_raw_after(items_raw_after, fmt, fmts, &clause)
  }
}
