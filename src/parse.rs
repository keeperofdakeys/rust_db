use lex::{ TokenType, StringToken, QuotedToken, CommaToken, LeftParenToken, RightParenToken };
use lex::{ Token, LexError, UnmatchedQuote, UnmatchedEscape, lex_statement };

enum ParseError {
  ErrNoCommand,
}

fn parse_statement( tokens: Vec<Token> ) -> Result<(), ParseError> {
  let mut paren_nest = 0u;
  let command = match parse_command( &tokens ) {
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

fn parse_command( tokens: &Vec<Token> ) -> Option<Command> {
  if tokens.len() == 0 {
    return None;
  }
  Some( match tokens[0].get_token() {
    "select" => CmdSelect,
    "insert" => CmdInsert,
    "update" => CmdUpdate,
    _ => return None
  } )
}
