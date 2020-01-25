extern crate regex;

use regex::Regex;
// static values to categorize and validate tokens
// static VALID_SYMBOLS: &'static [char] = &[
//   '{', '}', '(', ')', '[', ']', '.', ',', ';', '*',
//   '+', '-', '/', '&', '|', '<', '>', '=', '~'
//   ];
static BREAK_CHARACTERS: &'static [char] = &[' ', '\n'];
static VALID_SYMBOLS: &'static [&str] = &[
  "{", "}", "(", ")", "[", "]", ".", ",", ";", "*",
  "+", "-", "/", "&", "|", "<", ">", "=", "~"
  ];
  static VALID_KEYWORDS: &'static [&str] = &[
    "class", "constructor", "function", "method", "field", "static", "var",
    "int", "char", "boolean", "void", "true", "false", "null", "this", "let",
    "do", "if", "else", "while", "return"
    ];
static UPPER_INTEGER_BOUND: u32 = 32767;
lazy_static! {
  static ref INVALID_CHARACTERS: Regex = Regex::new("[\"\n]+").unwrap();
  static ref VALID_IDENTIFIER_PATTERN: Regex = Regex::new("^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
  Keyword(String),
  Symbol(String),
  Identifier(String),
  StringConstant(String),
  IntegerConstant(String),
} 

pub fn tokenize(token_buffer: &str) -> Vec<Token> {
  // Result-vec of tokens
  let mut tokens = Vec::new();
  let mut is_string_sequence = false;
  let mut token_string = String::new();

  for (idx, character) in token_buffer.chars().enumerate() {
    // start/end of string-sequence
    if character == '"' {
      // if sequence ends:
      if is_string_sequence {
        tokens.push(Token::StringConstant(token_string));
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
      if is_symbol(&character.to_string()) {
        if !token_string.is_empty() {
          tokens.push(resolve_token(&token_string));
          token_string = String::new();
        }
        tokens.push(Token::Symbol(character.to_string()));
        // If current char isnt marking a break (new-line, blank, end-of-buffer)
        // add it the token_string
      } else if !char_at_idx_is_break(idx, &token_buffer) {
        token_string.push(character);
      }
      // If next char is marking a break (new-line, blank, end-of-buffer)
      // add token to result Vec
      if char_at_idx_is_break(idx+1, &token_buffer) && !token_string.is_empty() {
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
    return Token::Keyword(maybe_token.to_string());
  }
  if is_valid_identifier(maybe_token) {
    return Token::Identifier(maybe_token.to_string());
  }
  if is_integer_constant(maybe_token) {
    return Token::IntegerConstant(maybe_token.to_string());
  }
  panic!("no valid token has been passed: {}<-Invalid", maybe_token);
}

// Matcher functions
fn is_symbol(maybe_symbol: &str) -> bool {
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

fn is_string_constant(maybe_string_constant: &str) -> bool {
  !INVALID_CHARACTERS.is_match(maybe_string_constant)
}

fn is_valid_identifier(maybe_identifier: &str) -> bool {
  let optional_match = VALID_IDENTIFIER_PATTERN.find(maybe_identifier);
  match optional_match {
    Some(m) => m.as_str() == maybe_identifier,
    None => false,
  }
}




// ### Unit - TESTS ###
//
// Token-parser-Tests
#[test]
fn simple_tokens_are_categorized() {
    let mock_tokens = vec![
      Token::Identifier(String::from("x")),
      Token::Symbol(String::from("+")),
      Token::IntegerConstant(String::from("2"))
    ];
    assert_eq!(tokenize("x + 2"), mock_tokens);
}
#[test]
fn all_statement_tokens_are_categorized() {
    let mock_tokens = vec![
      Token::Keyword(String::from("let")),
      Token::Identifier(String::from("x")),
      Token::Symbol(String::from("=")),
      Token::Identifier(String::from("y")),
      Token::Symbol(String::from("+")),
      Token::IntegerConstant(String::from("2")),
    ];
    assert_eq!(tokenize("let x = y + 2"), mock_tokens);
}
#[test]
fn several_statements_can_be_tokenized() {
    let mock_tokens = vec![
      Token::Keyword(String::from("let")),
      Token::Identifier(String::from("x")),
      Token::Symbol(String::from("=")),
      Token::Identifier(String::from("y")),
      Token::Symbol(String::from("+")),
      Token::IntegerConstant(String::from("2")),
      Token::Symbol(String::from(";")),
      Token::Keyword(String::from("let")),
      Token::Identifier(String::from("s")),
      Token::Symbol(String::from("=")),
      Token::StringConstant(String::from("Hello World")),
      Token::Symbol(String::from(";")),
    ];
    let statements = r#"let x = y + 2;
                        let s = "Hello World";
                    "#;
    assert_eq!(tokenize(statements), mock_tokens);
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
  assert_eq!(is_symbol("{"), true);
}
#[test]
fn whitespace_is_no_symbol() {
  assert_eq!(is_symbol(" "), false);
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

// StringConstant regognition
#[test]
fn string_is_valid_string_constant() {
    assert_eq!(is_string_constant("Hello"), true);
}
#[test]
fn string_with_quotes_is_not_valid_string_constant() {
    assert_eq!(is_string_constant("He\"llo"), false);
}
#[test]
fn string_with_newline_is_not_valid_string_constant() {
    assert_eq!(is_string_constant("Hello\nWorld"), false);
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