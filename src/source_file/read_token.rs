

use std::rc::Rc;

use crate::token::{
  TokenKind,
  Token,
  ParsedNumber,
  print_string,

};


use crate::source_file::{
  SourceFile,
  SourceInfo,
  SourceReader,
  Error,
  read_number,
  read_string,

};


pub type TokenString = Vec<Token>;




impl
SourceReader
{


const fn
is_upper(c: char)-> bool
{
  (c >= 'A') && (c <= 'Z')
}


const fn
is_lower(c: char)-> bool
{
  (c >= 'a') && (c <= 'z')
}


const fn
is_alphabet(c: char)-> bool
{
  Self::is_upper(c) || Self::is_lower(c)
}


const fn
is_id_head(c: char)-> bool
{
  Self::is_alphabet(c) || (c == '_')
}


const fn
is_digit(c: char)-> bool
{
  (c >= '0') && (c <= '9')
}


const fn
is_id_body(c: char)-> bool
{
  Self::is_id_head(c) || Self::is_digit(c)
}


fn
read_data_that_begins_from_id_head(&mut self, first_c: char)-> Result<TokenKind,Error>
{
  let  mut s = String::new();

  s.push(first_c);

    if first_c == 'r'
    {
        if let Some(second_c) = self.get_character()
        {
          self.advance();

            if second_c == '#'
            {
                match self.read_raw_string()
                {
              Ok(s)=>{return Ok(TokenKind::String(s));}
              Err(e)=>{return Err(e);}
                }
            }

          else
            if Self::is_id_body(second_c)
            {
              s.push(second_c);
            }
        }
    }


    while let Some(c) = self.get_character()
    {
        if Self::is_id_body(c)
        {
          s.push(c);

          self.advance();
        }

      else
        {
          break;
        }
    }


  Ok(TokenKind::Identifier(s))
}


fn
read_data_that_begins_from_single_quote(&mut self)-> Result<TokenKind,Error>
{
    if let Some(mut first_c) = self.get_character()
    {
        if first_c == '\''
        {
          return Err(self.to_error(format!("空の文字リテラル")));
        }


      self.advance();

        if first_c == '\\'
        {
            match self.read_escape_sequence()
            {
          Ok(c)=>{first_c = c;}
          Err(e)=>{return Err(e);}
            }
        }


        if let Some(second_c) = self.get_character()
        {
            if second_c == '\''
            {
              self.advance();

              Ok(TokenKind::Character(first_c))
            }

          else
            if Self::is_id_head(first_c)
            {
              let  mut s = String::new();

              s.push(first_c);

               if Self::is_id_body(second_c)
               {
                 s.push(second_c);

                 self.advance();

                   while let Some(next_c) = self.get_character()
                   {
                       if Self::is_id_body(next_c)
                       {
                         s.push(next_c);

                         self.advance();
                       }

                     else
                       {break;}
                   }
                }


              Ok(TokenKind::WithApostrophe(s))
            }

          else
            {
              Err(self.to_error(format!("不正な、アポトロフィー付き識別子: {} {}",first_c,second_c)))
            }
        }

      else
        if Self::is_id_head(first_c)
        {
          let  mut s = String::new();

          s.push(first_c);

          Ok(TokenKind::WithApostrophe(s))
        }

      else
        {
          Err(self.to_error(format!("不正な、アポトロフィー付き識別子: {}",first_c)))
        }
    }

  else
    {
      Err(self.to_error(format!("シングルクオート後に何もない")))
    }
}


fn
read_data_that_begins_from_slash(&mut self)-> Result<TokenKind,Error>
{
    if let Some(c) = self.get_character()
    {
        if c == '*'
        {
          self.advance();

          let  old_y = self.get_y();

            if let Err(s) = self.skip_until_appears_end_of_comment_block()
            {
              return Err(self.to_error(s));
            }


          return if old_y != self.get_y()
                 {
                   Ok(TokenKind::Space)
                 }

               else
                 {
                   Ok(TokenKind::Newline)
                 };
        }

      else
        if c == '/'
        {
          self.advance();

            if let Err(s) = self.skip_until_appears_newline()
            {
              return Err(self.to_error(s));
            }


          return Ok(TokenKind::Newline);
        }
    }


  Ok(TokenKind::Others('/'))
}


fn
read_token_kind(&mut self)-> Result<TokenKind,Error>
{
    if let Some(c) = self.get_character()
    {
        if SourceReader::is_space(c)
        {
          let  old_y = self.get_y();

          self.skip_spaces();

            if self.get_y() != old_y
            {
              Ok(TokenKind::Newline)
            }

          else
            {
              Ok(TokenKind::Space)
            }
        }

      else
        if c == '/'
        {
          self.advance();

            match self.read_data_that_begins_from_slash()
            {
          Ok(k)=>{Ok(k)}
          Err(e)=>{Err(e)}
            }
        }

      else
        if Self::is_id_head(c)
        {
          self.advance();

            match self.read_data_that_begins_from_id_head(c)
            {
          Ok(k)=>{Ok(k)}
          Err(e)=>{Err(e)}
            }
        }

      else
        if Self::is_digit(c)
        {
          let  pn = self.read_number();

          Ok(TokenKind::Number(pn))
        }

      else
        if c == '\"'
        {
          self.advance();

            match self.read_string()
            {
          Ok(s)=>{Ok(TokenKind::String(s))}
          Err(e)=>{Err(e)}
            }
        }

      else
        if c == '\''
        {
          self.advance();

            match self.read_data_that_begins_from_single_quote()
            {
          Ok(k)=>{Ok(k)}
          Err(e)=>{Err(e)}
            }
        }

      else
        if (c >= '!') && (c <= '~')
        {
          self.advance();

          Ok(TokenKind::Others(c))
        }

      else
        {
          Err(self.to_error(format!("処理不能な文字 {}",c as u8)))
        }
    }

  else
    {
      Ok(TokenKind::Null)
    }
}


pub fn
read_token(&mut self)-> Result<Token,Error>
{
  let  info = self.info.clone();

    match self.read_token_kind()
    {
  Ok(k)=>{Ok(Token::new(info,k))}
  Err(e)=>{Err(e)}
    }
}


pub fn
read_token_string(&mut self)-> Result<TokenString,Error>
{
  let  mut buf = TokenString::new();

    loop
    {
        match self.read_token()
        {
     Ok(tok)=>
        {
            if let TokenKind::Null = tok.get_kind()
            {
              break;
            }

          else
            {
              buf.push(tok);
            }
        }
     Err(e)=>{return Err(e);}
        }
    }


  Ok(buf)
}


}




