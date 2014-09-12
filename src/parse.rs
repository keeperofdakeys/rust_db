use lex::{ TokenType, StringToken, QuotedToken, CommaToken, LeftParenToken, RightParenToken };
use lex::{ Token, LexError, UnmatchedQuote, UnmatchedEscape, lex_statement };
use std::fmt;

fn parse_statement( tokens: Vec<Token> ) -> Result<(), ParseError> {
  let mut parser = Parser::new();
  for token in tokens.iter() {
    match parser.handle_token( token ) {
      Err( p ) => return Err( ParseError{ found: token.clone(), expected: p } ),
      _ => {}
    }
  }
  Ok( () )
}

enum SqlCommand {
  CmdSelect,
  CmdInsert,
  CmdUpdate
}

enum ParsingState {
  ParseInit,
  ParseCmd,
  ParseColumns,
  ParseColumnsComma,
  ParseTables,
  ParseTablesComma,
  ParseWhere,
  ParseOrderBy
}

struct Parser<'a> {
  state: ParsingState,
  command: Option<SqlCommand>,
  values: Vec<Vec<&'a Token>>
}

impl<'a> Parser<'a> {
  fn new<'a>() -> Parser<'a> {
    let mut p = Parser {
      state: ParseInit,
      command: None,
      values: Vec::new()
    };
    p.values.push( Vec::new() );
    p
  }

  fn push_val( &mut self, token: &'a Token ) {
    let pos = self.values.len() - 1;
    let mut back_vec = self.values.as_mut_slice().get_mut(pos).unwrap();
    back_vec.push( token );
  }
  
  fn handle_token( &mut self, token: &Token ) -> Result<(), Token> {
    let contents = token.get_token();

    match token.get_type() {
      StringToken => self.handle_string( contents ),
      QuotedToken( c ) => self.handle_quoted( contents, c ),
      CommaToken => self.handle_comma(),
      LeftParenToken => self.handle_left_paren(),
      RightParenToken => self.handle_right_paren()
    }
  }

  fn handle_string( &mut self, token: &str ) -> Result<(), Token> {
    match self.state {
      ParseInit => {},
      ParseCmd => {},
      ParseColumns => {},
      ParseColumnsComma => {
        //self.push_val( token );
        self.state = ParseColumnsComma
      },
      ParseTables => {},
      ParseTablesComma => {},
      ParseWhere => {},
      ParseOrderBy => {},
    }
    Ok( () )
  }

  fn handle_quoted( &self, token: &str, quote: char ) -> Result<(), Token> {
    Ok( () )
  }

  fn handle_comma( &mut self ) -> Result<(), Token> {
    match self.state {
      ParseInit => {
        return Err( self.comma_token() )
      },
      ParseCmd => {},
      ParseColumns => {
        self.state = ParseColumnsComma
      },
      ParseColumnsComma => {
        return Err( self.comma_token() )
      },
      ParseTables => {
        self.state = ParseTables
      },
      ParseTablesComma => {
        return Err( self.comma_token() )
      },
      ParseWhere => {},
      ParseOrderBy => {},
    }
    Ok( () )
  }

  fn handle_left_paren( &self ) -> Result<(), Token> {
    Ok( () )
  }

  fn handle_right_paren( &self ) -> Result<(), Token> {
    Ok( () )
  }

  fn comma_token( &self ) -> Token {
    Token::new_tok( CommaToken, "," )
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
