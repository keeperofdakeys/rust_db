use std::char::is_whitespace;

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

#[deriving(PartialEq, Show)]
struct Token {
  token: String,
  quoted: Option<char>
}

impl Token {
  fn new() -> Token {
    Token{
      token: String::new(),
      quoted: None
    }
  }

  fn from_str( str: &str ) -> Token {
    Token{
      token: String::from_str( str ),
      quoted: None
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
}

macro_rules! tokens_append(
  ($t: ident, $ts: ident) => (
    match $t.len() {
      0 => {},
      _ => {
        $ts.push( $t );
        $t = Token::new();
      }
    }
  )
)

pub fn lex_statement( input: &str ) -> Option<Vec<Token>> {
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
        }
      }
    }
    token.push( char );
  }

  if escape_state == Escaping {
    return None;
  }
  match quote_state {
    Quote(_) => return None,
    _ => {}
  }
  if token.len() > 0 {
    tokens.push( token );
  }

  Some( tokens )
}

macro_rules! test_lex_statement {
  ($str: expr, $str_vec: expr) => (
    assert!( lex_statement( $str ) == Some( Token::from_str_vec( $str_vec ) ) );
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
