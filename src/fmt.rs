use std::fmt::Formatter;
use ansi_term::Color::Blue;

pub struct Format<'a> {
  pub comma: &'a str,
  pub hr: &'a str, // horizontal rule
  pub indent: &'a str,
  pub lb: &'a str, // line break
  colorize: bool
}

pub fn one_line<'a>() -> Format<'a> {
  Format {
    comma: ", ",
    hr: "",
    indent: "",
    lb: "",
    colorize: false,
  }
}

pub fn multiline<'a>() -> Format<'a> {
  Format {
    comma: ", ",
    hr: "-- ------------------------------------------------------------------------------\x1b[0m",
    indent: "  ",
    lb: "\n",
    colorize: false,
  }
}

pub fn multiline_color<'a>() -> Format<'a> {
  let mut f = multiline();
  f.colorize = true;
  f
}

impl<'a> Format<'a> {
  pub fn write_blue(&self, fmt: &mut Formatter<'_>, text: &str) -> std::fmt::Result {
    if self.colorize {
      write!(fmt, "{}", Blue.paint(text))
    } else {
      write!(fmt, "{text}")
    }
  }
}

pub fn colorize(query: String) -> String {
  let sql_syntax: [(fn(&str) -> String, &str, &str); 46] = [
    (blue, "AND ", "and "),
    (blue, "CROSS ", "cross "),
    (blue, "DELETE ", "delete "),
    (blue, "EXCEPT ", "except "),
    (blue, "FROM ", "from "),
    (blue, "FULL ", "full "),
    (blue, "GROUP ", "group "),
    (blue, "HAVING ", "having "),
    (blue, "INNER ", "inner "),
    (blue, "INSERT ", "insert "),
    (blue, "INTERSECT ", "intersect "),
    (blue, "INTO ", "into "),
    (blue, "JOIN ", "join "),
    (blue, "LEFT ", "left "),
    (blue, "LIMIT ", "limit "),
    (blue, "OFFSET ", "offset "),
    (blue, "ORDER ", "order "),
    (blue, "OVERRIDING ", "overriding "),
    (blue, "RETURNING ", "returning "),
    (blue, "RIGHT ", "right "),
    (blue, "SELECT ", "select "),
    (blue, "SET ", "set "),
    (blue, "UNION ", "union "),
    (blue, "UPDATE ", "update "),
    (blue, "VALUES ", "values "),
    (blue, "WHERE ", "where "),
    (blue, "WITH ", "with "),
    (blue, " ALL", " all"),
    (blue, " ASC", " asc"),
    (blue, " AS", " as"),
    (blue, " BY", " by"),
    (blue, " CONFLICT", " CONFLICT"),
    (blue, " DESC", " desc"),
    (blue, " DO", " do"),
    (blue, " DISTINCT", " distinct"),
    (blue, " FIRST", " first"),
    (blue, " IN", " in"),
    (blue, " LAST", " last"),
    (blue, " NOTHING", " nothing"),
    (blue, " ON", " on"),
    (blue, " OR", " or"),
    (blue, " OUTER", " OUTER"),
    (blue, " USING", " using"),
    (comment_start, "--", "--"),
    (comment_start, "/*", "/*"),
    (comment_end, "*/", "*/"),
  ];

  let mut query = sql_syntax.iter().fold(query, |acc, item| {
    let (color_fn, text_upper, text_lower) = item;
    acc
      .replace(text_upper, &color_fn(text_upper))
      .replace(text_lower, &color_fn(text_lower))
  });

  for index in 1..=10 {
    let arg_number = format!("${index}");
    query = query.replace(&arg_number, &bold(&arg_number))
  }

  query
}

pub fn format(query: String, fmts: &Format) -> String {
  let template = format!("{0}{1}{0}{query}{0}{1}{0}", fmts.lb, fmts.hr);
  let template = colorize(template);
  template
}

fn blue(text: &str) -> String {
  format!("\x1b[34;1m{text}\x1b[0m")
}

fn bold(text: &str) -> String {
  format!("\x1b[0;1m{text}\x1b[0m")
}

fn comment_start(text: &str) -> String {
  format!("\x1b[32;2m{text}")
}

fn comment_end(text: &str) -> String {
  format!("\x1b[32;2m{text}\x1b[0m")
}

pub(crate) struct Joiner<'a, 'b, F> {
  fmt: &'a mut Formatter<'b>,
  join: F,
  has_first: bool,
}

impl<'a, 'b, F: FnMut(&'a mut Formatter<'b>) -> std::fmt::Result> Joiner<'a, 'b, F> {
  pub fn new(fmt: &'a mut Formatter<'b>, join: F) -> Self {
    Joiner { fmt, join, has_first: false }
  }

  pub fn entry(&mut self, entry: &str) -> std::fmt::Result {
    if self.has_first {
      (self.join)(self.fmt)?;
    } else {
      self.has_first = true;
    }

    write!(self.fmt, "{entry}")
  }

  pub fn finish(self) {}
}