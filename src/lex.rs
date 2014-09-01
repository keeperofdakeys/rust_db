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

enum TokenType {
  StringToken,
  QuotedToken( char ),
  CommaToken,
  LeftParenToken,
  RightParenToken
}

struct Token {
  token: String,
  token_type: TokenType,
}

impl Token {
  fn new() -> Token {
    Token{
      token_type: StringToken,
      token: String::new()
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
}

impl PartialEq for Token {
  fn eq( &self, other: &Token ) -> bool {
    self.token.eq( &other.token )
  }
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
      ',' | '(' | ')' => {
        tokens_append!( token, tokens );
        token.push( char );
        token.set_type( match char {
          ',' => CommaToken,
          '(' => LeftParenToken,
          ')' => RightParenToken,
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
