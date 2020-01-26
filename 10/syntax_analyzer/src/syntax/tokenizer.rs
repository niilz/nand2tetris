extern crate regex;

use std::fmt;
use regex::Regex;
// static values to categorize and validate tokens
static VALID_SYMBOLS: &'static [char] = &[
  '{', '}', '(', ')', '[', ']', '.', ',', ';', '*',
  '+', '-', '/', '&', '|', '<', '>', '=', '~'
  ];
static BREAK_CHARACTERS: &'static [char] = &[' ', '\n'];
static VALID_KEYWORDS: &'static [&str] = &[
    "class", "constructor", "function", "method", "field", "static", "var",
    "int", "char", "boolean", "void", "true", "false", "null", "this", "let",
    "do", "if", "else", "while", "return"
    ];
static UPPER_INTEGER_BOUND: u32 = 32767;
lazy_static! {
  static ref INVALID_CHARACTERS: Regex = Regex::new("[\"\n]+").unwrap();
  static ref VALID_IDENTIFIER_PATTERN: Regex = Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
  static ref LINE_COMMENT_IDENTIFIER: Regex = Regex::new(r"//.*").unwrap();
  // End of Block comment ( */ ) is not considered. So block-comments in between
  // code will not work, because everything after /** is ignored:
  static ref BLOCK_COMMENT_IDENTIFIER: Regex = Regex::new(r"/\*\*.*").unwrap();
}

// Only public method, which is used in the main
// program to translate given Jack code into an XML-representation
pub fn get_tokens_in_xml(tokens: &str) -> String {
  let tokens_as_xml = tokenize(tokens)
                        .iter()
                        .map(|token| token.to_xml())
                        .collect::<Vec<String>>().join("\n");
  format!("<tokens>\n{}\n</tokens>", tokens_as_xml)
}

// Token Struct
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Token {
  token_type: TokenType,
  value: String,
}
impl Token {
  fn to_xml(&self) -> String {
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
enum TokenType {
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

// Turns a given string (expects valid jack code) into a Vec of Tokens
fn tokenize(token_stream: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  for line in token_stream.split("\n") {
    let cleaned_line = clean_line(line);
    if !cleaned_line.is_empty() {
      tokens.append(&mut tokenize_line(&cleaned_line));
    };
  }
  tokens
}

// cleanes a line from comments
fn clean_line(line: &str) -> String {
  let line_without_line_comments = LINE_COMMENT_IDENTIFIER.replace(line, "");
  let line_without_block_comments = BLOCK_COMMENT_IDENTIFIER.replace(&line_without_line_comments, "");
  if line_without_block_comments.starts_with(" *") {
    return String::from("");
  }
  line_without_block_comments.trim().to_string()
}

// Workhorse of the Tokenizer-module.
// Cotegorizes character(s) in a line into Tokens
// with their associated type and value
fn tokenize_line(token_line: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut is_string_sequence = false;
  let mut token_string = String::new();

  for (idx, character) in token_line.chars().enumerate() {
    // start/end of string-sequence
    if character == '"' {
      // if sequence ends:
      if is_string_sequence {
        tokens.push(Token {
          token_type: TokenType::StringConstant,
          value: token_string,
        });
        token_string = String::new();
        is_string_sequence = !is_string_sequence;
        continue;
      }
      // if sequence starts, flag it and keep going
      is_string_sequence = !is_string_sequence;
      continue;
    }
    // for all other (non-string-tokens):
    if !is_string_sequence {
      // If it's a symbol add it to the result tokens.
      // Before push token in token_string, if there is one.
      if is_symbol(character) {
        if !token_string.is_empty() {
          tokens.push(resolve_token(&token_string));
          token_string = String::new();
        }
        tokens.push(Token { token_type: TokenType::Symbol, value: character.to_string() });
        // If current char isnt marking a break (new-line, blank, end-of-buffer)
        // add it the token_string
      } else if !char_at_idx_is_break(idx, &token_line) {
        token_string.push(character);
      }
      // If next char is marking a break (new-line, blank, end-of-buffer)
      // add token to result Vec
      if char_at_idx_is_break(idx+1, &token_line) && !token_string.is_empty() {
        tokens.push(resolve_token(&token_string));
        token_string = String::new();
      }
    // we're in a string-sequence, so add the character to the token_string
    } else {
      token_string.push(character);
    }
  }
  tokens
}


// Check if next char is a bound between Tokens
fn char_at_idx_is_break(idx: usize, char_str: &str) -> bool {
  let next_char = char_str.chars().nth(idx);
  match next_char {
    Some(c) => BREAK_CHARACTERS.contains(&c),
    None => true,
  }
}

// Token-Resolver
fn resolve_token(maybe_token: &str) -> Token {
  if is_keyword(maybe_token) {
    return Token {
      token_type: TokenType::Keyword,
      value: maybe_token.to_string(),
    };
  }
  if is_valid_identifier(maybe_token) {
    return Token {
      token_type: TokenType::Identifier,
      value: maybe_token.to_string(),
    };
  }
  if is_integer_constant(maybe_token) {
    return Token {
      token_type: TokenType::IntegerConstant,
      value: maybe_token.to_string(),
    };
  }
  panic!("no valid token has been passed: {}<-Invalid", maybe_token);
}

// Matcher functions
fn is_symbol(maybe_symbol: char) -> bool {
  VALID_SYMBOLS.contains(&maybe_symbol)
}

fn is_keyword(maybe_keyword: &str) -> bool {
  VALID_KEYWORDS.contains(&maybe_keyword)
}

fn is_integer_constant(maybe_integer_constant: &str) -> bool {
  match maybe_integer_constant.parse::<u32>() {
    Ok(val) => val <= UPPER_INTEGER_BOUND,
    Err(_) => false,
  }
}

fn is_valid_identifier(maybe_identifier: &str) -> bool {
  let optional_match = VALID_IDENTIFIER_PATTERN.find(maybe_identifier);
  match optional_match {
    Some(m) => m.as_str() == maybe_identifier,
    None => false,
  }
}


// ### Integration - Tests ###
#[test]
fn turns_string_into_token_xml() {
    assert_eq!(
      get_tokens_in_xml("let foo = x * y;"),
      "<tokens>\n<keyword> let </keyword>\n<identifier> foo </identifier>\n<symbol> = </symbol>\n<identifier> x </identifier>\n<symbol> * </symbol>\n<identifier> y </identifier>\n<symbol> ; </symbol>\n</tokens>"
    );
}

// ### Unit - TESTS ###
//
#[test]
fn multilines_with_comments_can_be_tokenized() {
  let mock_tokens = vec![
    Token { token_type: TokenType::Keyword, value: String::from("let") },
    Token { token_type: TokenType::Identifier, value: String::from("x") },
    Token { token_type: TokenType::Symbol, value: String::from("=") },
    Token { token_type: TokenType::Identifier, value: String::from("y") },
    Token { token_type: TokenType::Symbol, value: String::from("+") },
    Token { token_type: TokenType::IntegerConstant, value: String::from("2") },
    Token { token_type: TokenType::Symbol, value: String::from(";") },
    Token { token_type: TokenType::Keyword, value: String::from("let") },
    Token { token_type: TokenType::Identifier, value: String::from("s") },
    Token { token_type: TokenType::Symbol, value: String::from("=") },
    Token { token_type: TokenType::StringConstant, value: String::from("Hello World") },
    Token { token_type: TokenType::Symbol, value: String::from(";") },
    ];
  let statements = r#"// Comments and should be ignored, so shoul empty lines (line 2)

  /** block comments should be ignored */
  let x = y + 2;
  let s = "Hello World"; // nasty in line comments should be ignored..
  "#;
  assert_eq!(tokenize(statements), mock_tokens);
}

// Tokens-in-line-parser-Tests
#[test]
fn division_is_token_not_comment() {
    let code = "let j = j / (-2);";
    let mock_tokens = vec! [
      Token { token_type: TokenType::Keyword, value: String::from("let") },
      Token { token_type: TokenType::Identifier, value: String::from("j") },
      Token { token_type: TokenType::Symbol, value: String::from("=") },
      Token { token_type: TokenType::Identifier, value: String::from("j") },
      Token { token_type: TokenType::Symbol, value: String::from("/") },
      Token { token_type: TokenType::Symbol, value: String::from("(") },
      Token { token_type: TokenType::Symbol, value: String::from("-") },
      Token { token_type: TokenType::IntegerConstant, value: String::from("2") },
      Token { token_type: TokenType::Symbol, value: String::from(")") },
      Token { token_type: TokenType::Symbol, value: String::from(";") },
    ];
    assert_eq!(tokenize_line(code), mock_tokens);
}
#[test]
fn simple_tokens_are_categorized() {
  let mock_tokens = vec![
    Token { token_type: TokenType::Identifier, value: String::from("x") },
    Token { token_type: TokenType::Symbol, value: String::from("+") },
    Token { token_type: TokenType::IntegerConstant, value: String::from("2") },
  ];
  assert_eq!(tokenize_line("x + 2"), mock_tokens);
}

#[test]
fn all_statement_tokens_are_categorized() {
    let mock_tokens = vec![
      Token { token_type: TokenType::Keyword, value: String::from("let") },
      Token { token_type: TokenType::Identifier, value: String::from("x") },
      Token { token_type: TokenType::Symbol, value: String::from("=") },
      Token { token_type: TokenType::Identifier, value: String::from("y") },
      Token { token_type: TokenType::Symbol, value: String::from("+") },
      Token { token_type: TokenType::IntegerConstant, value: String::from("2") },
    ];
    assert_eq!(tokenize_line("let x = y + 2"), mock_tokens);
}

// Break-character-Tests
#[test]
fn blank_is_break_character() {
    assert_eq!(char_at_idx_is_break(3, "let x"), true);
}
#[test]
fn char_is_not_break_character() {
    assert_eq!(char_at_idx_is_break(1, "let x"), false);
}
#[test]
fn end_of_sequence_is_break() {
    assert_eq!(char_at_idx_is_break(5, "let x"), true);
}

// VALIDATORS
// keywords recognition
#[test]
fn a_keyword_is_recognized_correctly() {
  assert_eq!(is_keyword("if"), true);
}
#[test]
fn a_non_keyword_is_not_recognized() {
  assert_eq!(is_keyword("rust"), false);
}

// symbols recognition
#[test]
fn square_bracket_is_valid_symbol() {
  assert_eq!(is_symbol('{'), true);
}
#[test]
fn whitespace_is_no_symbol() {
  assert_eq!(is_symbol(' '), false);
}

// integerConstant regognition
#[test]
fn integer_is_valid_integer_constant() {
    assert_eq!(is_integer_constant("1"), true);
}
#[test]
fn not_a_number_is_no_integer_constant() {
    assert_eq!(is_integer_constant("a"), false);
}
#[test]
fn number_below_one_is_no_integer_constant() {
    assert_eq!(is_integer_constant("-1"), false);
}
#[test]
fn last_number_of_range_limit_is_integer_constant() {
    assert_eq!(is_integer_constant("32767"), true);
}
#[test]
fn number_above_range_limit_is_no_integer_constant() {
    assert_eq!(is_integer_constant("32768"), false);
}

// identifier regognition
#[test]
fn single_letter_is_identifier() {
    assert_eq!(is_valid_identifier("x"), true);
}
#[test]
fn just_letters_is_valid_identifier() {
    assert_eq!(is_valid_identifier("value"), true);
}
#[test]
fn just_underscore_is_valid_identifier() {
    assert_eq!(is_valid_identifier("_"), true);
}
#[test]
fn letters_and_numbers_is_valid_identifier() {
    assert_eq!(is_valid_identifier("a123"), true);
}
#[test]
fn number_first_is_no_valid_identifier() {
    assert_eq!(is_valid_identifier("1abc"), false);
}
#[test]
fn non_alphanumeric_letters_means_no_valid_identifier() {
    assert_eq!(is_valid_identifier("Rust2.0"), false);
}