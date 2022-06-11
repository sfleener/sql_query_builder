use crate::{
  behavior::raw_queries,
  fmt,
  structure::{Combinator, SelectBuilder, SelectClause},
};

impl<'a> SelectBuilder<'a> {
  /// The same as `where_clause` method, useful to write more idiomatic SQL query
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .where_clause("login = foo")
  ///   .and("active = true");
  /// ```
  pub fn and(mut self, clause: &'a str) -> Self {
    self = self.where_clause(clause);
    self
  }

  /// Gets the current state of the SelectBuilder returns it as string
  pub fn as_string(&self) -> String {
    let fmts = fmt::Formatter::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the SelectBuilder into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .select("*")
  ///   .from("users")
  ///   .where_clause("login = foo")
  ///   .and("active = true")
  ///   .debug();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users
  /// WHERE login = foo AND active = true
  /// ```
  ///
  /// You can debug different parts of the select putting it in another position
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .select("*")
  ///   .from("users")
  ///   .debug()
  ///   .where_clause("login = foo")
  ///   .and("active = true");
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::Formatter::multi_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// The except clause
  pub fn except(mut self, select: Self) -> Self {
    self._except.push(select);
    self
  }

  /// The from clause
  pub fn from(mut self, tables: &'a str) -> Self {
    self._from.push(tables.to_owned());
    self
  }

  /// The group by clause
  pub fn group_by(mut self, column: &'a str) -> Self {
    self._group_by.push(column.to_owned());
    self
  }

  /// The having clause
  pub fn having(mut self, condition: &'a str) -> Self {
    self._having.push(condition.to_owned());
    self
  }

  /// The cross join clause
  pub fn cross_join(mut self, table: &'a str) -> Self {
    self._join.push(format!("CROSS JOIN {table}"));
    self
  }

  /// The inner join clause
  pub fn inner_join(mut self, condition: &'a str) -> Self {
    self._join.push(format!("INNER JOIN {condition}"));
    self
  }

  /// The left join clause
  pub fn left_join(mut self, condition: &'a str) -> Self {
    self._join.push(format!("LEFT JOIN {condition}"));
    self
  }

  /// The right join clause
  pub fn right_join(mut self, condition: &'a str) -> Self {
    self._join.push(format!("RIGHT JOIN {condition}"));
    self
  }

  /// The intersect clause
  pub fn intersect(mut self, select: Self) -> Self {
    self._intersect.push(select);
    self
  }

  /// The limit clause. This method overrides the previous value
  ///
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .limit("123");
  ///
  /// let select = SelectBuilder::new()
  ///   .limit("1000")
  ///   .limit("123");
  /// ```
  pub fn limit(mut self, num: &'a str) -> Self {
    self._limit = num;
    self
  }

  /// Create SelectBuilder's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// The offset clause. This method overrides the previous value
  ///
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let select = SelectBuilder::new()
  ///   .offset("1500");
  ///
  /// let select = SelectBuilder::new()
  ///   .offset("1000")
  ///   .offset("1500");
  /// ```
  pub fn offset(mut self, num: &'a str) -> Self {
    self._offset = num;
    self
  }

  /// The order by clause
  pub fn order_by(mut self, column: &'a str) -> Self {
    self._order_by.push(column.to_owned());
    self
  }

  /// Prints the current state of the SelectBuilder into console output similar to debug method,
  /// the diference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::Formatter::one_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// ```
  /// use sql_query_builder::SelectBuilder;
  ///
  /// let raw_query = "select * from users u inner join address addr on u.login = addr.owner_login";
  /// let select = SelectBuilder::new()
  ///   .raw(raw_query)
  ///   .where_clause("u.login = foo");
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// select * from users u inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw(mut self, raw_sql: &'a str) -> Self {
    self._raw.push(raw_sql.to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{SelectClause, SelectBuilder};
  ///
  /// let raw_join = "inner join address addr on u.login = addr.owner_login";
  /// let select = SelectBuilder::new()
  ///   .select("*")
  ///   .from("users u")
  ///   .raw_after(SelectClause::From, raw_join)
  ///   .where_clause("u.login = foo");
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// FROM users u
  /// inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw_after(mut self, clause: SelectClause, raw_sql: &'a str) -> Self {
    self._raw_after.push((clause, raw_sql.to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{SelectClause, SelectBuilder};
  ///
  /// let raw_query = "from users u inner join address addr on u.login = addr.owner_login";
  /// let select = SelectBuilder::new()
  ///   .select("*")
  ///   .raw_before(SelectClause::Where, raw_query)
  ///   .where_clause("u.login = foo");
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// SELECT *
  /// from users u inner join address addr on u.login = addr.owner_login
  /// WHERE u.login = foo
  /// ```
  pub fn raw_before(mut self, clause: SelectClause, raw_sql: &'a str) -> Self {
    self._raw_before.push((clause, raw_sql.to_owned()));
    self
  }

  /// The select by clause
  pub fn select(mut self, column: &'a str) -> Self {
    self._select.push(column.to_owned());
    self
  }

  /// The union by clause
  pub fn union(mut self, select: Self) -> Self {
    self._union.push(select);
    self
  }

  /// The where by clause
  pub fn where_clause(mut self, condition: &'a str) -> Self {
    self._where.push(condition.to_owned());
    self
  }

  /// The with by clause
  pub fn with(mut self, name: &'a str, select: Self) -> Self {
    self._with.push((name, select));
    self
  }
}

impl SelectBuilder<'_> {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts);
    query = self.concat_with(query, &fmts);
    query = self.concat_select(query, &fmts);
    query = self.concat_from(query, &fmts);
    query = self.concat_join(query, &fmts);
    query = self.concat_where(query, &fmts);
    query = self.concat_group_by(query, &fmts);
    query = self.concat_having(query, &fmts);
    query = self.concat_order_by(query, &fmts);
    query = self.concat_limit(query, &fmts);
    query = self.concat_offset(query, &fmts);
    query = self.concat_combinator(query, &fmts, Combinator::Except);
    query = self.concat_combinator(query, &fmts, Combinator::Intersect);
    query = self.concat_combinator(query, &fmts, Combinator::Union);

    query.trim_end().to_owned()
  }

  fn concat_raw_before_after(&self, query: String, fmts: &fmt::Formatter, clause: SelectClause, sql: String) -> String {
    let fmt::Formatter { space, .. } = fmts;
    let raw_before = raw_queries(&self._raw_before, &clause).join(space);
    let raw_after = raw_queries(&self._raw_after, &clause).join(space);
    let space_after = if raw_after.is_empty() == false { space } else { "" };
    let space_before = if raw_before.is_empty() == false { space } else { "" };

    format!("{query}{raw_before}{space_before}{sql}{raw_after}{space_after}")
  }

  fn concat_combinator(&self, query: String, fmts: &fmt::Formatter, combinator: Combinator) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let (clause, clause_name, clause_list) = match combinator {
      Combinator::Except => (SelectClause::Except, "EXCEPT", &self._except),
      Combinator::Intersect => (SelectClause::Intersect, "INTERSECT", &self._intersect),
      Combinator::Union => (SelectClause::Union, "UNION", &self._union),
    };

    let raw_before = raw_queries(&self._raw_before, &clause).join(space);
    let raw_after = raw_queries(&self._raw_after, &clause).join(space);

    let space_before = if raw_before.is_empty() {
      "".to_owned()
    } else {
      space.to_string()
    };
    let space_after = if raw_after.is_empty() {
      "".to_owned()
    } else {
      space.to_string()
    };

    if clause_list.is_empty() {
      let sql = "".to_owned();
      return format!("{query}{raw_before}{space_before}{sql}{raw_after}{space_after}");
    }

    let right_stmt = clause_list.iter().fold("".to_owned(), |acc, select| {
      let query = select.concat(&fmts);
      format!("{acc}{clause_name}{space}({lb}{query}){space}{lb}")
    });

    let query = query.trim_end();
    let space_before = space;
    let left_stmt = format!("({query}{raw_before}){space_before}");

    format!("{left_stmt}{right_stmt}{raw_after}{space_after}")
  }

  fn concat_from(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._from.is_empty() == false {
      let tables = self._from.join(comma);
      format!("FROM {tables}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::From, sql)
  }

  fn concat_group_by(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._group_by.is_empty() == false {
      let columns = self._group_by.join(comma);
      format!("GROUP BY {columns}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::GroupBy, sql)
  }

  fn concat_having(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._having.is_empty() == false {
      let conditions = self._having.join(" AND ");
      format!("HAVING {conditions}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::Having, sql)
  }

  fn concat_join(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._join.is_empty() == false {
      let joins = self._join.join(format!("{space}{lb}").as_str());
      format!("{joins}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::Join, sql)
  }

  fn concat_limit(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._limit.is_empty() == false {
      let count = self._limit;
      format!("LIMIT {count}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::Limit, sql)
  }

  fn concat_offset(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._offset.is_empty() == false {
      let start = self._offset;
      format!("OFFSET {start}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::Offset, sql)
  }

  fn concat_order_by(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._order_by.is_empty() == false {
      let columns = self._order_by.join(comma);
      format!("ORDER BY {columns}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::OrderBy, sql)
  }

  fn concat_raw(&self, query: String, fmts: &fmt::Formatter) -> String {
    if self._raw.is_empty() {
      return query;
    }
    let fmt::Formatter { lb, space, .. } = fmts;
    let raw_sql = self._raw.join(space);

    format!("{query}{raw_sql}{space}{lb}")
  }

  fn concat_select(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { comma, lb, space, .. } = fmts;
    let sql = if self._select.is_empty() == false {
      let columns = self._select.join(comma);
      format!("SELECT {columns}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::Select, sql)
  }

  fn concat_where(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._where.is_empty() == false {
      let conditions = self._where.join(" AND ");
      format!("WHERE {conditions}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::Where, sql)
  }

  fn concat_with(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter {
      comma,
      lb,
      indent,
      space,
    } = fmts;
    let sql = if self._with.is_empty() == false {
      let with = self._with.iter().fold("".to_owned(), |acc, item| {
        let (name, select) = item;
        let inner_lb = format!("{lb}{indent}");
        let inner_fmts = fmt::Formatter {
          comma,
          lb: inner_lb.as_str(),
          indent,
          space,
        };
        let select_string = select.concat(&inner_fmts);

        format!("{acc}{name} AS ({lb}{indent}{select_string}{lb}){comma}")
      });
      let with = &with[..with.len() - comma.len()];

      format!("WITH {with}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(query, fmts, SelectClause::With, sql)
  }
}

impl<'a> std::fmt::Display for SelectBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl<'a> std::fmt::Debug for SelectBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::Formatter::multi_line();
    write!(f, "{}", fmt::colorize(self.concat(&fmts)))
  }
}
