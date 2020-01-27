use std::fmt;

// Token Struct
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub value: String,
}
impl Token {
  pub fn to_xml(&self) -> String {
    let value = match self.value.as_ref() {
      ">" => "&gt;",
      "<" => "&lt;",
      "\"" => "&quot;",
      "&" => "&amp;",
      _ => &self.value,
    };
    format!("<{}> {} </{}>", self.token_type, value, self.token_type)
  }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
  Keyword,
  Symbol,
  Identifier,
  StringConstant,
  IntegerConstant,
}
impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      TokenType::Keyword => write!(f, "keyword"),
      TokenType::Symbol => write!(f, "symbol"),
      TokenType::Identifier => write!(f, "identifier"),
      TokenType::StringConstant => write!(f, "stringConstant"),
      TokenType::IntegerConstant => write!(f, "integerConstant"),
    }
  }
}

#[test]
fn token_fields_are_accessable() {
    let mock_token = Token { token_type: TokenType::Identifier, value: String::from("x") };
    assert_eq!(mock_token.token_type, TokenType::Identifier);
    assert_eq!(mock_token.value, String::from("x"));
}