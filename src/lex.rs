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
pub enum Token {
  StringToken( String ),
  QuotedToken( String, char ),
  CommaToken,
  LeftParenToken,
  RightParenToken,
  SemiColonToken
}

impl Token {
  fn str_tok() -> Token {
    StringToken( String::new() )
  }

  fn quot_tok( char: char ) -> Token {
    QuotedToken( String::new(), char )
  }

  fn from_str( str: &str ) -> Token {
    StringToken( str.to_string() )
  }

  fn from_str_vec( str_vec: Vec<&str> ) -> Vec<Token> {
    let mut token_vec = Vec::new();
    for str in str_vec.iter() {
      token_vec.push( Token::from_str( *str ) );
    }
    token_vec
  }

  fn push( &mut self, char: char ) {
    match *self {
      StringToken( ref mut s ) => s.push_char( char ),
      QuotedToken( ref mut s, _ ) => s.push_char( char ),
      _ => {}
    }
  }

  fn len( &self ) -> uint {
    match *self {
      StringToken( ref s ) => s.len(),
      QuotedToken( ref s, _ ) => s.len(),
      _ => 1
    }
  }

  pub fn get_str<'a>( &'a self ) -> &'a str {
    match *self {
      StringToken( ref s ) => s.as_slice(),
      QuotedToken( ref s, _ ) => s.as_slice(),
      CommaToken => ",",
      LeftParenToken => "(",
      RightParenToken => ")",
      SemiColonToken => ";"
    }
  }
}

impl PartialEq for Token {
  fn eq( &self, other: &Token ) -> bool {
    match (self, other) {
      (&StringToken(ref s1), &StringToken(ref s2)) => s1.eq(s2),
      (&QuotedToken(ref s1, ref c1), &QuotedToken(ref s2, ref c2)) => {
        s1.eq(s2) && c1.eq(c2)
      },
      (&CommaToken, &CommaToken) => true,
      (&LeftParenToken, &LeftParenToken) => true,
      (&RightParenToken, &RightParenToken) => true,
      (&SemiColonToken, &SemiColonToken) => true,
      _ => false
    }
  }
}

impl fmt::Show for Token {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    match *self {
      StringToken( ref s ) =>
        write!( f, "{}", s ),
      QuotedToken( ref s, ref c ) =>
        write!( f, "{}{}{}", c, s, c ),
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
    match $token {
      StringToken( ref s ) if s.len() == 0 => {},
      QuotedToken( ref s, _ ) if s.len() == 0 => {},
      _ => {
        $tokens.push( $token );
        $token = Token::str_tok();
      }
    }
  )
)

pub fn lex_statement( input: &str ) -> Result<Vec<Token>, LexError> {
  let mut token = Token::str_tok();
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
            token = Token::quot_tok( char );
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
        token = match char {
          ',' => CommaToken,
          '(' => LeftParenToken,
          ')' => RightParenToken,
          ';' => SemiColonToken,
          _ => Token::str_tok(),
        };
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
