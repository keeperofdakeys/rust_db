use std::char::is_whitespace;
use std::fmt;

#[deriving(PartialEq)]
enum QuoteState {
  NoQuote,
  Quote( char )
}

#[deriving(PartialEq)]
enum EscapeState {
  NoEscape,
  Escaping
}

#[deriving(Clone)]
pub enum TokenType {
  StringToken,
  QuotedToken( char ),
  CommaToken,
  LeftParenToken,
  RightParenToken,
  SemiColonToken
}

#[deriving(Clone)]
pub struct Token {
  token: String,
  token_type: TokenType,
}

impl Token {
  pub fn new() -> Token {
    Token{
      token_type: StringToken,
      token: String::new()
    }
  }

  pub fn new_tok( token_type: TokenType, string: &str ) -> Token {
    Token{
      token_type: token_type,
      token: string.to_string()
    }
  }

  fn from_str( str: &str ) -> Token {
    Token{
      token_type: StringToken,
      token: String::from_str( str )
    }
  }

  fn from_str_vec( str_vec: Vec<&str> ) -> Vec<Token> {
    let mut token_vec = Vec::new();
    for str in str_vec.iter() {
      token_vec.push( Token::from_str( *str ) );
    }
    token_vec
  }

  fn push( &mut self, char: char ) {
    self.token.push_char( char );
  }

  fn len( &self ) -> uint {
    self.token.len()
  }

  fn set_type( &mut self, token_type: TokenType ) {
    self.token_type = token_type;
  }

  pub fn get_type( &self ) -> TokenType {
    self.token_type
  }

  pub fn get_token<'a>( &'a self ) -> &'a str {
    self.token.as_slice()
  }
}

impl PartialEq for Token {
  fn eq( &self, other: &Token ) -> bool {
    self.token.eq( &other.token )
  }
}

impl fmt::Show for Token {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    match self.token_type {
      StringToken =>
        write!( f, "{}", self.token ),
      QuotedToken( char ) =>
        write!( f, "\"{}\"", self.token ),
      CommaToken =>
        write!( f, "," ),
      LeftParenToken =>
        write!( f, "(" ),
      RightParenToken =>
        write!( f, ")" ),
      SemiColonToken =>
        write!( f, ";" )
    }
  }
}

#[deriving(PartialEq)]
pub enum LexError {
  UnmatchedQuote,
  UnmatchedEscape
}

macro_rules! tokens_append(
  ($token: ident, $tokens: ident) => (
    match $token.len() {
      0 => {},
      _ => {
        $tokens.push( $token );
        $token = Token::new();
      }
    }
  )
)

pub fn lex_statement( input: &str ) -> Result<Vec<Token>, LexError> {
  let mut token = Token::new();
  let mut tokens = Vec::new();
  let mut quote_state = NoQuote;
  let mut escape_state = NoEscape;

  for char in input.chars() {
    if escape_state == Escaping {
      escape_state = NoEscape;
      token.push( char );
      continue;
    } else if char == '\\' {
      escape_state = Escaping;
      continue;
    }

    match quote_state {
      NoQuote => {
        if is_whitespace( char ) {
          tokens_append!( token, tokens );
          continue;
        }
        match char {
          '\'' | '"' => {
            tokens_append!( token, tokens );
            token.set_type( QuotedToken( char ) );
            quote_state = Quote( char );
            continue;
          },
          _ => {}
        }
      },
      Quote( quote_char ) => {
        if char == quote_char {
          quote_state = NoQuote;
          tokens_append!( token, tokens );
          continue;
        } else {
          token.push( char );
          continue;
        }
      }
    }
    match char {
      ',' | '(' | ')' | ';' => {
        tokens_append!( token, tokens );
        token.push( char );
        token.set_type( match char {
          ',' => CommaToken,
          '(' => LeftParenToken,
          ')' => RightParenToken,
          ';' => SemiColonToken,
          _ => StringToken
        } );
        tokens_append!( token, tokens );
      },
      _ => {
        token.push( char );
      }
    }
  }

  if escape_state == Escaping {
    return Err( UnmatchedEscape );
  }
  match quote_state {
    Quote(_) => return Err( UnmatchedQuote ),
    _ => {}
  }
  if token.len() > 0 {
    tokens.push( token );
  }

  Ok( tokens )
}

macro_rules! test_lex_statement {
  ($str: expr, $str_vec: expr) => (
    assert!( lex_statement( $str ) == Ok( Token::from_str_vec( $str_vec ) ) );
  )
}

#[test]
fn test_one_token() {
  test_lex_statement!( "token",
    vec![ "token" ]
  );
}

#[test]
fn test_two_tokens() {
  test_lex_statement!( "token1 token2",
    vec!["token1", "token2"]
  );
}

#[test]
fn test_newline() {
  test_lex_statement!( "token1\ntoken2",
    vec![ "token1", "token2" ]
  );
}

#[test]
fn test_quoted_tokens_one() {
  test_lex_statement!( "token1\"token2 \"",
    vec![ "token1", "token2 " ]
  );
}

#[test]
fn test_quoted_tokens_two() {
  test_lex_statement!( "token1\"token2 tok\"en3",
    vec![ "token1", "token2 tok", "en3" ]
  );
}

#[test]
fn test_escaping_one() {
  test_lex_statement!( "token1\\\\ token2",
    vec![ "token1\\", "token2" ]
  );
}

#[test]
fn test_quote_escape() {
  test_lex_statement!( "\"token1 \\\" token2\"",
    vec![ "token1 \" token2" ]
  );
}

#[test]
fn test_comma() {
  test_lex_statement!( "token1,token2, token3 ,token4",
    vec![ "token1", ",", "token2", ",", "token3", ",", "token4" ]
  );
}

#[test]
fn test_parens() {
  test_lex_statement!( "token1( token2 )token3",
    vec![ "token1", "(", "token2", ")", "token3" ]
  );
}

#[test]
fn test_parens_with_commas_quotes_escapes() {
  test_lex_statement!( "\\(token1 (token2, token3, to\"ken4, \"token5\\, token6\\))",
    vec![ "(token1", "(", "token2", ",", "token3", ",", "to", "ken4, ", "token5,", "token6)", ")" ]
  );
}

#[test]
fn test_unbalanced_quotes_one() {
  let str = "token1 \"token2";
  assert!( lex_statement( str ) == Err( UnmatchedQuote ) );
}

#[test]
fn test_unbalanced_quotes_two() {
  let str = "token1\"token2\\\"";
  assert!( lex_statement( str ) == Err( UnmatchedQuote ) );
}

#[test]
fn test_unbalanced_escape() {
  let str = "token1\\";
  assert!( lex_statement( str ) == Err( UnmatchedEscape ) );
}
