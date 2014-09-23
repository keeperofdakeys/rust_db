use lex::{ Token, StringToken, QuotedToken, CommaToken, LeftParenToken, RightParenToken, SemiColonToken };
use lex::{ LexError, UnmatchedQuote, UnmatchedEscape, lex_statement };
use std::fmt;

fn parse_statement( tokens: Vec<Token> ) -> Result<(), ()> {
  let mut parser = Parser::new();
  for token in tokens.iter() {
    match parser.handle_token( token ) {
      Err( p ) => return Err( () ),
      _ => {}
    }
  }
  Ok( () )
}

enum SqlStmt<'a> {
  StmtNone,
  StmtSelect(SelectStruct<'a>),
  StmtInsert(InsertStruct<'a>),
  StmtUpdate(UpdateStruct<'a>),
  StmtDelete(DeleteStruct<'a>)
}

struct SelectStruct<'a> {
  state: SelectState,
  columns: Vec<ColumnVal<'a>>,
  tables: Vec<TableVal<'a>>,
  join_on: Vec<OnVal<'a>>,
  wheres: Vec<WhereVal<'a>>
}

impl<'a> SelectStruct<'a> {
  fn new<'a>() -> SelectStruct<'a> {
    SelectStruct {
      state: SelectColumnsValue,
      columns: Vec::new(),
      tables: Vec::new(),
      join_on: Vec::new(),
      wheres: Vec::new()
    }
  }

  fn handle_token( &mut self, token: &'a Token ) -> Result<(), ()> {
    match *token {
      StringToken( _ ) => self.handle_string( token ),
      QuotedToken( _, c ) => self.handle_quoted( token, c ),
      CommaToken => self.handle_comma(),
      LeftParenToken => self.handle_left_paren(),
      RightParenToken => self.handle_right_paren(),
      SemiColonToken => self.handle_semicolon()
    }
  }

  fn handle_string( &mut self, token: &'a Token ) -> Result<(), ()> {
    match self.state {
      SelectColumnsValue => {
        self.handle_column_token( token )
      },
      SelectColumnsNext => {
        match token.get_str() {
          "from" => {
            self.state = SelectTablesValue;
            Ok( () )
          },
          _ => Err( () )
        }
      },
      SelectColumnsFuncParam => {
        self.handle_column_func_param( token );
        self.state = SelectColumnsFuncNext;
        Ok( () )
      },
      SelectColumnsFuncNext => Err( () ),
      SelectTablesValue => {
        self.handle_table_token( token )
      },
      SelectTablesNext => {
        match token.get_str() {
          "using" => {
            self.state = SelectJoinUsingValue;
            Ok( () )
          },
          "left" => {
            self.handle_left_join();
            self.state = SelectTablesJoin;
            Ok( () )
          },
          "right" => {
            self.handle_right_join();
            self.state = SelectTablesJoin;
            Ok( () )
          },
          "inner" => {
            self.handle_inner_join();
            self.state = SelectTablesJoin;
            Ok( () )
          },
          "natural" => {
            self.handle_natural_join();
            self.state = SelectTablesJoin;
            Ok( () )
          },
          _ => Err( () )
        }
      },
      SelectTablesJoin => {
        match token.get_str() {
          "join" => {
            self.handle_table_token( token );
            self.state = SelectTablesValue;
            Ok( () )
          },
          _ => Err( () )
        }
      },
      SelectJoinUsingValue => {
        self.handle_using_token( token );
        Ok( () )
      },
      SelectJoinUsingNext => {
        Ok( () )
      },
      SelectEnd => Err( () )
    }
  }

  fn handle_quoted( &mut self, token: &'a Token, char: char ) -> Result<(), ()> {
    match self.state {
      SelectColumnsValue => {
        self.handle_column_token( token )
      },
      SelectColumnsFuncParam => {
        self.handle_column_token( token )
      },
      _ => Err( () )
    }
  }

  fn handle_comma( &mut self ) -> Result<(), ()> {
    match self.state {
      SelectColumnsNext => {
        self.state = SelectColumnsValue;
        Ok( () )
      },
      SelectColumnsFuncNext => {
        self.state = SelectColumnsFuncParam;
        Ok( () )
      },
      SelectTablesNext => {
        self.handle_cross_join();
        self.state = SelectTablesValue;
        Ok( () )
      },
      _ => Err( () )
    }
  }

  fn handle_left_paren( &mut self ) -> Result<(), ()> {
    match self.state {
      SelectColumnsNext => {
        self.handle_column_func();
        self.state = SelectColumnsFuncParam;
        Ok( () )
      }
      // TODO: Handle subquery. New state, recuse like Parser.
      SelectTablesValue => Err( () ),
      _ => Err( () )
    }
  }

  fn handle_right_paren( &mut self ) -> Result<(), ()> {
    match self.state {
      SelectColumnsFuncNext => {
        self.state = SelectColumnsNext;
        Ok( () )
      },
      // TODO: Handle right paren ending subquery. Go to TablesNext
      _ => Err( () )
    }
  }

  fn handle_semicolon( &mut self ) -> Result<(), ()> {
    match self.state {
      SelectColumnsNext | SelectTablesNext => {
        self.state = SelectEnd;
        Ok( () )
      },
      _ => Err( () )
    }
  }

  fn handle_column_token( &mut self, token: &'a Token ) -> Result<(), ()> {
    match *token {
      StringToken( ref s ) => {
        self.columns.push( ColumnString( s.as_slice() ) );
        Ok( () )
      },
      QuotedToken( ref s, c ) => {
        self.columns.push( ColumnQuoted( s.as_slice(), c ) );
        Ok( () )
      },
      _ => Err( () )
    }
  }

  fn handle_column_func( &mut self ) -> Result<(), ()> {
    let col_val = match self.columns.pop() {
      Some( t ) => t,
      None => return Err( () )
    };
    match col_val {
      ColumnString( s ) => {
        self.columns.push(
          ColumnFunc( s, Vec::new() )
        );
        Ok( () )
      },
      _ => Err( () )
    }
  }

  fn handle_column_func_param( &mut self, token: &'a Token ) -> Result<(), ()> {
    let tok_str = match *token {
      StringToken( ref s ) => s.as_slice(),
      _ => return Err( () )
    };
    match self.columns.last_mut() {
      Some( &ColumnFunc( s, ref mut v ) ) => {
        v.push( ColumnString( tok_str ) );
        Ok( () )
      },
      Some( _ ) => Err( () ),
      None => Err( () )
    }
  }


  fn handle_table_token( &mut self, token: &'a Token ) -> Result<(), ()> {
    match *token {
      StringToken( ref s ) => {
        self.tables.push( TableString( s.as_slice() ) );
        Ok( () )
      },
      _ => Err( () )
    }
  }

  fn handle_left_join( &mut self ) -> Result<(), ()> {
    self.tables.push( TableLeftJoin );
    Ok( () )
  }

  fn handle_right_join( &mut self ) -> Result<(), ()> {
    self.tables.push( TableLeftJoin );
    Ok( () )
  }

  fn handle_inner_join( &mut self ) -> Result<(), ()> {
    self.tables.push( TableLeftJoin );
    Ok( () )
  }

  fn handle_natural_join( &mut self ) -> Result<(), ()> {
    self.tables.push( TableLeftJoin );
    Ok( () )
  }

  fn handle_cross_join( &mut self ) -> Result<(), ()> {
    self.tables.push( TableLeftJoin );
    Ok( () )
  }

  fn handle_using_token( &mut self, token: &'a Token ) -> Result<(), ()> {
    let tok_str = match *token {
      StringToken( ref s ) => s.as_slice(),
      _ => return Err( () )
    };

    match self.join_on.pop() {
      Some( OnColumn( s1 ) ) => {
        self.join_on.push( OnColumns( s1, tok_str ) );
        Ok( () )
      },
      Some( c ) => {
        self.join_on.push( c );
        Ok( () )
      },
      None => {
        self.join_on.push( OnColumn( tok_str ) );
        Ok( () )
      }
    }
  }
}

enum SelectState {
  SelectColumnsValue,
  SelectColumnsNext,
  SelectColumnsFuncParam,
  SelectColumnsFuncNext,
  SelectTablesValue,
  SelectTablesNext,
  SelectTablesJoin,
  SelectJoinUsingValue,
  SelectJoinUsingNext,
  SelectEnd
}
  
enum ColumnVal<'a> {
  ColumnString(&'a str),
  ColumnQuoted(&'a str, char),
  ColumnFunc(&'a str, Vec<ColumnVal<'a>>)
}

enum TableVal<'a> {
  TableString(&'a str),
  TableNestedQuery(&'a str),
  TableLeftJoin,
  TableRightJoin,
  TableInnerJoin,
  TableNaturalJoin,
  TableCrossJoin
}

enum OnVal<'a> {
  OnColumn(&'a str),
  OnColumns(&'a str, &'a str)
}

enum WhereVal<'a> {
  WhereString
}

struct InsertStruct<'a>;

impl<'a> InsertStruct<'a> {
  fn handle_token( &self, token: &'a Token ) -> Result<(), ()> {
    Err( () )
  }
}

struct UpdateStruct<'a>;

impl<'a> UpdateStruct<'a> {
  fn handle_token( &self, token: &'a Token ) -> Result<(), ()> {
    Err( () )
  }
}

struct DeleteStruct<'a>;

impl<'a> DeleteStruct<'a> {
  fn handle_token( &self, token: &'a Token ) -> Result<(), ()> {
    Err( () )
  }
}


enum ParsingState {
  ParseCmd,
  ParseStmt,
}

struct Parser<'a> {
  stmt: SqlStmt<'a>,
  state: ParsingState,
}

impl<'a> Parser<'a> {
  fn new<'a>() -> Parser<'a> {
    Parser {
      stmt: StmtNone,
      state: ParseCmd
    }
  }

  /*
  fn push_val( &mut self, token: &'a Token ) {
    let pos = self.values.len() - 1;
    let mut back_vec = self.values.as_mut_slice().get_mut(pos).unwrap();
    back_vec.push( token );
  }
  */
  
  fn handle_token( &mut self, token: &'a Token ) -> Result<(), ()> {
    match self.state {
      ParseCmd => self.handle_command( token ),
      ParseStmt => match self.stmt {
        StmtSelect( ref mut stmt ) => stmt.handle_token( token ),
        StmtInsert( ref mut stmt ) => stmt.handle_token( token ),
        StmtUpdate( ref mut stmt ) => stmt.handle_token( token ),
        StmtDelete( ref mut stmt ) => stmt.handle_token( token ),
        _ => Err( () )
      }
    }
  }

  fn handle_command( &mut self, token: &Token ) -> Result<(), ()> {
    match token.get_str() {
      "select" => {
        self.stmt = StmtSelect( SelectStruct::new() );
        self.state = ParseStmt;
        Ok( () )
      },
      "insert" => Err( () ),
      "update" => Err( () ),
      "delete" => Err( () ),
      _ => Err( () )
    }
  }
}
