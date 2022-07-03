use crate::{
  behavior::{push_unique, BuilderInner},
  fmt,
  structure::{DeleteBuilder, DeleteClause},
};

impl<'a> DeleteBuilder<'a> {
  /// The same as `where_clause` method, useful to write more idiomatic SQL query
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let delete = DeleteBuilder::new()
  ///   .delete_from("users")
  ///   .where_clause("created_at < $1")
  ///   .and("active = false");
  /// ```
  pub fn and(mut self, condition: &'a str) -> Self {
    self = self.where_clause(condition);
    self
  }

  /// Gets the current state of the DeleteBuilder and returns it as string
  pub fn as_string(&self) -> String {
    let fmts = fmt::Formatter::one_line();
    self.concat(&fmts)
  }

  /// Prints the current state of the DeleteBuilder into console output in a more ease to read version.
  /// This method is useful to debug complex queries or just to print the generated SQL while you type
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let delete_query = DeleteBuilder::new()
  ///   .delete_from("users")
  ///   .where_clause("login = 'foo'")
  ///   .debug()
  ///   .where_clause("name = 'Foo'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM users
  /// WHERE login = 'foo'
  /// ```
  pub fn debug(self) -> Self {
    let fmts = fmt::Formatter::multi_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// The delete clause. This method overrides the previous value
  ///
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let delete = DeleteBuilder::new()
  ///   .delete_from("orders");
  ///
  /// let delete = DeleteBuilder::new()
  ///   .delete_from("address")
  ///   .delete_from("orders");
  /// ```
  pub fn delete_from(mut self, table_name: &'a str) -> Self {
    self._delete_from = table_name.trim();
    self
  }

  /// Create DeleteBuilder's instance
  pub fn new() -> Self {
    Self::default()
  }

  /// Prints the current state of the DeleteBuilder into console output similar to debug method,
  /// the difference is that this method prints in one line.
  pub fn print(self) -> Self {
    let fmts = fmt::Formatter::one_line();
    println!("{}", fmt::colorize(self.concat(&fmts)));
    self
  }

  /// Adds at the beginning a raw SQL query.
  ///
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let raw_query = "delete from users";
  /// let delete_query = DeleteBuilder::new()
  ///   .raw(raw_query)
  ///   .where_clause("login = 'foo'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// delete from users
  /// WHERE login = 'foo'
  /// ```
  pub fn raw(mut self, raw_sql: &'a str) -> Self {
    push_unique(&mut self._raw, raw_sql.trim().to_owned());
    self
  }

  /// Adds a raw SQL query after a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{DeleteClause, DeleteBuilder};
  ///
  /// let raw = "where name = 'Foo'";
  /// let delete_query = DeleteBuilder::new()
  ///   .delete_from("users")
  ///   .raw_after(DeleteClause::DeleteFrom, raw)
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// DELETE FROM users
  /// where name = 'Foo'
  /// ```
  pub fn raw_after(mut self, clause: DeleteClause, raw_sql: &'a str) -> Self {
    self._raw_after.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// Adds a raw SQL query before a specified clause.
  ///
  /// ```
  /// use sql_query_builder::{DeleteClause, DeleteBuilder};
  ///
  /// let raw = "delete from users";
  /// let delete_query = DeleteBuilder::new()
  ///   .raw_before(DeleteClause::Where, raw)
  ///   .where_clause("name = 'Bar'")
  ///   .as_string();
  /// ```
  ///
  /// Output
  ///
  /// ```sql
  /// delete from users
  /// WHERE name = 'Bar'
  /// ```
  pub fn raw_before(mut self, clause: DeleteClause, raw_sql: &'a str) -> Self {
    self._raw_before.push((clause, raw_sql.trim().to_owned()));
    self
  }

  /// The where clause
  /// ```
  /// use sql_query_builder::DeleteBuilder;
  ///
  /// let delete = DeleteBuilder::new()
  ///   .delete_from("users")
  ///   .where_clause("login = 'foo'");
  /// ```
  pub fn where_clause(mut self, condition: &'a str) -> Self {
    push_unique(&mut self._where, condition.trim().to_owned());
    self
  }
}

impl BuilderInner<'_, DeleteClause> for DeleteBuilder<'_> {
  fn concat(&self, fmts: &fmt::Formatter) -> String {
    let mut query = "".to_owned();

    query = self.concat_raw(query, &fmts);
    query = self.concat_delete_from(query, &fmts);
    query = self.concat_where(query, &fmts);

    query.trim_end().to_owned()
  }

  fn _raw(&self) -> &Vec<String> {
    &self._raw
  }

  fn _raw_after(&self) -> &Vec<(DeleteClause, String)> {
    &self._raw_after
  }

  fn _raw_before(&self) -> &Vec<(DeleteClause, String)> {
    &self._raw_before
  }
}

impl DeleteBuilder<'_> {
  fn concat_delete_from(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._delete_from.is_empty() == false {
      let table_name = self._delete_from;
      format!("DELETE FROM{space}{table_name}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(DeleteClause::DeleteFrom, query, fmts, sql)
  }

  fn concat_where(&self, query: String, fmts: &fmt::Formatter) -> String {
    let fmt::Formatter { lb, space, .. } = fmts;
    let sql = if self._where.is_empty() == false {
      let conditions = self._where.join(" AND ");
      format!("WHERE {conditions}{space}{lb}")
    } else {
      "".to_owned()
    };

    self.concat_raw_before_after(DeleteClause::Where, query, fmts, sql)
  }
}

impl<'a> std::fmt::Display for DeleteBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_string())
  }
}

impl<'a> std::fmt::Debug for DeleteBuilder<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let fmts = fmt::Formatter::multi_line();
    write!(f, "{}", fmt::colorize(self.concat(&fmts)))
  }
}
