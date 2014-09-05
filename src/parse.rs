use lex::{ TokenType, StringToken, QuotedToken, CommaToken, LeftParenToken, RightParenToken };
use lex::{ Token, LexError, UnmatchedQuote, UnmatchedEscape, lex_statement };
use std::fmt;

fn parse_statement( tokens: Vec<Token> ) -> Result<(), ParseError> {
  let mut parser = Parser::new();
  for token in tokens.iter() {
    match parser.handle_token( token ) {
      Err( p ) => return Err( p ),
      _ => {}
    }
  }
  Ok( () )
}

enum Command {
  CmdSelect,
  CmdInsert,
  CmdUpdate
}

enum ParsingState {
  ParseInit,
  ParseCmd,
  ParseColumns,
  ParseTables,
  ParseWhere,
  ParseOrderBy
}

struct Parser {
  state: ParsingState,
}

impl Parser {
  fn new() -> Parser {
    Parser {
      state: ParseInit
    }
  }
  
  fn handle_token( &self, token: &Token ) -> Result<(), ParseError> {
    let contents = token.get_token();

    match token.get_type() {
      StringToken => self.handle_string( contents ),
      QuotedToken( c ) => self.handle_quoted( contents, c ),
      CommaToken => self.handle_comma(),
      LeftParenToken => self.handle_left_paren(),
      RightParenToken => self.handle_right_paren()
    }
  }

  fn handle_string( &self, token: &str ) -> Result<(), ParseError> {
    Ok( () )
  }

  fn handle_quoted( &self, token: &str, quote: char ) -> Result<(), ParseError> {
    Ok( () )
  }

  fn handle_comma( &self ) -> Result<(), ParseError> {
    Ok( () )
  }

  fn handle_left_paren( &self ) -> Result<(), ParseError> {
    Ok( () )
  }

  fn handle_right_paren( &self ) -> Result<(), ParseError> {
    Ok( () )
  }

}

struct ParseError {
  found: Token,
  expected: Token
}

impl fmt::Show for ParseError {
  fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
    write!( f, "Error: found {}, expected {}", self.found, self.expected )
  }
}

macro_rules! advance_iter(
  ($iter: ident, $index: ident, $val: ident) => (
    match $iter.next() {
      Some( (i, v) ) => {
        $index = i;
        $val = v;
      }
      None => break
    }
  )
)
