use std::fmt;
use std::iter::Peekable;

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

// Wrapper to make the peekable-iterator less verbose
// type MyIter<'a> = std::slice::Iter<'a, Token>;
// type TokenStream<'a> = Peekable<MyIter<'a>>;
pub type TokenStream<'a> = Peekable<std::slice::Iter<'a, Token>>;

// TODO (if time, implement PeekableTakeWhile)
// (like crate https://docs.rs/peeking_take_while/0.1.2/peeking_take_while/)
// trait PeekableTakeWhile {
//   fn peek_take_while<P>(&mut self, predicat: P);
// }

// impl<'a> PeekableTakeWhile for TokenStream<'a> {
//     // peeking_take_while_helper
//   fn peek_take_while<P>(&mut self, predicat: P) {
//     let result_tokens = Vec::new();
//     loop {
//         let next_token = token_tail.peek().unwrap();
//     }
//   }
// }

// TESTS
#[test]
fn token_stream_can_be_taken_by_function() {
    let token_vec = vec![Token { token_type: TokenType::Symbol, value: String::from("+") }];
    fn takes_token_stream<'a>(token_stream: &mut TokenStream<'a>) -> String {
        token_stream.peek().unwrap().value.to_string()
    }
    assert_eq!(
        takes_token_stream(&mut token_vec.iter().peekable()), String::from("+"));
}

#[test]
fn token_fields_are_accessable() {
    let mock_token = Token { token_type: TokenType::Identifier, value: String::from("x") };
    assert_eq!(mock_token.token_type, TokenType::Identifier);
    assert_eq!(mock_token.value, String::from("x"));
}