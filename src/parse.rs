use lex::{ TokenType, StringToken, QuotedToken, CommaToken, LeftParenToken, RightParenToken };
use lex::{ Token, LexError, UnmatchedQuote, UnmatchedEscape, lex_statement };

enum ParseError {
  ErrNoCommand,
}

fn parse_statement( tokens: Vec<Token> ) -> Result<(), ParseError> {
  let mut paren_nest = 0u;
  let command = match parse_command( tokens.as_slice() ) {
    Some( c ) => c,
    None => return Err( ErrNoCommand )
  };
  Ok( () )
}

enum Command {
  CmdSelect,
  CmdInsert,
  CmdUpdate
}

fn parse_command( tokens: &[Token] ) -> Option<Command> {
  if tokens.len() == 0 {
    return None;
  }
  match tokens[0].get_token() {
    "select" => parse_select( tokens.slice_from( 1 ) ),
    "insert" => parse_insert( tokens.slice_from( 1 ) ),
    "update" => parse_update( tokens.slice_from( 1 ) ),
    _ => None
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
    

fn parse_select( tokens: &[Token] ) -> Option<Command> {
  let mut values = Vec::new();
  let mut iter = tokens.iter().enumerate();
  let (index, val): (uint, &Token);
  loop {
    advance_iter!( iter, index, val );
  }
  None
}

fn parse_insert( tokens: &[Token] ) -> Option<Command> {
  None
}

fn parse_update( tokens: &[Token] ) -> Option<Command> {
  None
}
