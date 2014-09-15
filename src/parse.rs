use lex::{ TokenType, StringToken, QuotedToken, CommaToken, LeftParenToken, RightParenToken, SemiColonToken };
use lex::{ Token, LexError, UnmatchedQuote, UnmatchedEscape, lex_statement };
use std::fmt;

fn parse_statement( tokens: Vec<Token> ) -> Result<(), ()> {
  let mut parser = Parser::new();
  for token in tokens.iter() {
    match parser.handle_token( token ) {
      //Err( p ) => return Err( ParseError{ found: token.clone(), expected: p } ),
      Err( p ) => return Err( () ),
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
  ParseCmd,
  ParseColumnsValue,
  ParseColumnsNext,
  ParseTablesValue,
  ParseTablesNext,
  ParseEnd
}

enum ParsingVec<'a> {
  Columns(Vec<&'a Token>),
  Tables(Vec<&'a Token>),
  Wheres(Vec<&'a Token>),
  Values(Vec<&'a Token>)
}


struct Parser<'a> {
  state: ParsingState,
  command: Option<SqlCommand>,
  columns: Vec<&'a Token>,
  tables: Vec<&'a Token>,
  wheres: Vec<&'a Token>,
  values: Vec<&'a Token>
}

impl<'a> Parser<'a> {
  fn new<'a>() -> Parser<'a> {
    let mut p = Parser {
      state: ParseCmd,
      command: None,
      columns: Vec::new(),
      tables: Vec::new(),
      wheres: Vec::new(),
      values: Vec::new(),
    };
    p
  }

  /*
  fn push_val( &mut self, token: &'a Token ) {
    let pos = self.values.len() - 1;
    let mut back_vec = self.values.as_mut_slice().get_mut(pos).unwrap();
    back_vec.push( token );
  }
  */
  
  fn handle_token( &mut self, token: &Token ) -> Result<(), ()> {
    match token.get_type() {
      StringToken => self.handle_string( token ),
      QuotedToken( c ) => self.handle_quoted( token, c ),
      CommaToken => self.handle_comma(),
      LeftParenToken => self.handle_left_paren(),
      RightParenToken => self.handle_right_paren(),
      SemiColonToken => self.handle_semicolon()
    }
  }

  fn handle_string( &mut self, token: &Token ) -> Result<(), ()> {
    match self.state {
      ParseCmd => self.handle_command( token ),
      ParseColumnsValue => {
        self.handle_column_token( token )
      },
      ParseColumnsNext => {
        match token.get_token() {
          "from" => {
            self.state = ParseTablesValue;
            Ok( () )
          },
          _ => Err( () )
        }
      },
      ParseTablesValue => {
        self.handle_table_token( token )
      },
      ParseTablesNext => {
        Err( () )
      },
      ParseEnd => Err( () )
    }
  }

  fn handle_quoted( &self, token: &Token, quote: char ) -> Result<(), ()> {
    match self.state {
      ParseColumnsValue => {
        self.handle_column_token( token )
      },
      _ => Err( () )
    }
  }

  fn handle_comma( &mut self ) -> Result<(), ()> {
    match self.state {
      ParseCmd => Err( () ),
      ParseColumnsValue => Err( () ),
      ParseColumnsNext => {
        self.state = ParseColumnsValue;
        Ok( () )
      },
      ParseTablesValue => Err( () ),
      ParseTablesNext => Err( () ),
      ParseEnd => Err( () )
    }
  }

  fn handle_left_paren( &self ) -> Result<(), ()> {
    Err( () )
  }

  fn handle_right_paren( &self ) -> Result<(), ()> {
    Err( () )
  }

  fn handle_semicolon( &mut self ) -> Result<(), ()> {
    match self.state {
      ParseColumnsNext | ParseTablesNext => {
        self.state = ParseEnd;
        Ok( () )
      },
      _ => Err( () )
    }
  }

  fn handle_command( &mut self, token: &Token ) -> Result<(), ()> {
    self.command = match token.get_token() {
      "select" => Some(CmdSelect),
      "insert" => Some(CmdInsert),
      "update" => Some(CmdUpdate),
      _ => None
    };
    Ok( () )
  }

  fn handle_column_token( &self, token: &Token ) -> Result<(), ()> {
    Ok( () )
  }

  fn handle_table_token( &self, token: &Token ) -> Result<(), ()> {
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
