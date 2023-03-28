

use super::{
  TokenInfo,
  TokenData,
  Token,
  print_char_string,
};


use super::read_number::{
  read_fraction_number,
  read_number,
};


use super::read_string::*;


use super::skip::*;


use crate::source_file::{
  SourceFile,
  Cursor,
  new_char_string,
};

use super::is::*;


pub type TokenString = Vec<Token>;

pub struct
Tokenizer<'a,'b>
{
  pub(crate) token_info: TokenInfo,

  pub(crate) input: &'a SourceFile,

  pub(crate) cursor: Cursor,

  pub(crate) output: &'b mut TokenString,

}




impl<'a,'b>
Tokenizer<'a,'b>
{


pub fn
new(src: &'a SourceFile, out: &'b mut TokenString)-> Tokenizer<'a,'b>
{
  let mut  tk = Tokenizer{
                  token_info: TokenInfo::new(src.get_path().as_str()),
                  input: src,
                  cursor: Cursor::new(),
                  output: out,
                };


  tk
}


pub fn
get_character(&self)-> Option<char>
{
  self.input.get_character(&self.cursor)
}


pub fn
read_identifier(src: &SourceFile, cur: &mut Cursor, s: &mut Vec<char>)
{
    while let Some(c) = src.get_character(cur)
    {
        if is_id_body(c)
        {
          s.push(c);

          cur.advance();
        }

      else
        {
          break;
        }
    }
}


pub fn
read_data_that_begins_from_id_head(&mut self, first_c: char)-> Result<(),()>
{
  let  mut s = new_char_string();

    if first_c == 'r'
    {
      let  mut sharps_n :usize = 0;

        if let Some(second_c) = self.get_character()
        {
            if second_c == '#'
            {
              sharps_n = read_sharps(&self.input,&mut self.cursor);
            }
        }


        if let Some(next_c) = self.get_character()
        {
            if next_c == '\"'
            {
              self.cursor.advance();

                if let Ok(s) = read_raw_string(&self.input,&mut self.cursor,sharps_n)
                {
                  self.push(TokenData::String(s));

                  return Ok(());
                }
            }
        }


         if sharps_n != 0
         {
           println!("不正な生文字列リテラルの開始");

           return Err(());
         }
    }


  s.push(first_c);

  Self::read_identifier(&self.input,&mut self.cursor,&mut s);

  self.push(TokenData::Identifier(s));

  Ok(())
}


pub fn
read_character_or_identifier_after_single_quote(&mut self, c: char)-> Result<(),()>
{
  let  mut s = new_char_string();

  s.push(c);

  self.cursor.advance();

  Self::read_identifier(&self.input,&mut self.cursor,&mut s);

    if let Some(last_c) = self.get_character()
    {
        if last_c == '\''
        {
            if s.len() == 1
            {
              self.cursor.advance();

              self.push(TokenData::Character(c));

              return Ok(());
            }


          println!("文字リテラルに一文字を超える内容{},{}",last_c,s.len());

          return Err(());
        }
    }


  self.push(TokenData::Others('\''));
  self.push(TokenData::Identifier(s));

  Ok(())
}


pub fn
read_escape_sequence(&mut self)-> Result<(),()>
{
    if let Ok(esc) = read_escape_sequence(&self.input,&mut self.cursor)
    {
        if let Some(last_c) = self.get_character()
        {
            if last_c == '\''
            {
              self.cursor.advance();

              self.push(TokenData::Character(esc));

              return Ok(());
            }
        }


      println!("文字列リテラルが不正な閉じ方");
    }

  else
    {
      println!("文字列リテラルで不正なシーケンス文字");
    }


  Err(())
}


pub fn
read_data_that_begins_from_single_quote(&mut self)-> Result<(),()>
{
    if let Some(c) = self.get_character()
    {
        if c == '\''
        {
          println!("empty character literal");

          return Err(());
        }


        if c == '\\'
        {
          self.cursor.advance();

          return self.read_escape_sequence();
        }


        if is_id_head(c)
        {
          return self.read_character_or_identifier_after_single_quote(c);
        }


        if let Ok(ch) = read_character(&self.input,&mut self.cursor)
        {
            if let Some(last_c) = self.get_character()
            {
                if last_c == '\''
                {
                  self.cursor.advance();

                  self.push(TokenData::Character(c));

                  return Ok(());
                }


              println!("文字リテラルに不明な内容{}",last_c);

              return Err(());
            }
        }
    }


  println!("不正なシングルクオート要素");

  Err(())
}


pub fn
read_data_that_begins_from_digit(&mut self, c: char)-> Result<(),()>
{
    if let Ok(n) = read_number(&self.input,&mut self.cursor,c)
    {
        if let Some(next_c) = self.get_character()
        {
            if next_c == '.'
            {
              self.cursor.advance();

                if let Ok(f) = read_fraction_number(&self.input,&mut self.cursor,n)
                {
                  self.push(TokenData::Floating(f));
                }

              else
                {
                  self.push(TokenData::Integer(n));
                  self.push(TokenData::Others('.'));
                }


              return Ok(());
            }
        }


      self.push(TokenData::Integer(n));

      return Ok(());
    }


  Err(())
}


pub fn
read_data_that_begins_from_slash(&mut self)-> Result<(),()>
{
    if let Some(c) = self.get_character()
    {
        if c == '*'
        {
          self.cursor.advance();

          let  old_y = self.cursor.get_y();

            if skip_until_appears_end_of_comment_block(&self.input,&mut self.cursor).is_err()
            {
              return Err(());
            }


            if old_y != self.cursor.get_y()
            {
              self.push(TokenData::Space);
            }

          else
            {
              self.push(TokenData::Newline);
            }


          return Ok(());
        }

      else
        if c == '/'
        {
          self.cursor.advance();

            if skip_until_appears_newline(&self.input,&mut self.cursor).is_err()
            {
              return Err(());
            }


          self.push(TokenData::Newline);

          return Ok(());
        }
    }


  self.push(TokenData::Others('/'));

  Ok(())
}


pub fn
update_token_info(&mut self)
{
  self.token_info.cursor = self.cursor;
}


fn
skip_spaces(&mut self)
{
  let  old_y = self.cursor.get_y();

  skip_spaces(&self.input,&mut self.cursor);

    if self.cursor.get_y() != old_y
    {
      self.push(TokenData::Newline);
    }

  else
    {
      self.push(TokenData::Space);
    }
}


fn
step(&mut self, c: char)-> Result<(),()>
{
  self.update_token_info();

    if is_space(c)
    {
      self.skip_spaces();

      return Ok(());
    }

  else
    if c == '/'
    {
      self.cursor.advance();

      return self.read_data_that_begins_from_slash();
    }

  else
    if is_id_head(c)
    {
      self.cursor.advance();

      return self.read_data_that_begins_from_id_head(c);
    }

  else
    if is_digit(c)
    {
      return self.read_data_that_begins_from_digit(c);
    }

  else
    if c == '\"'
    {
      self.cursor.advance();

        if let Ok(s) = read_string(&self.input,&mut self.cursor)
        {
          self.push(TokenData::String(s));

          return Ok(());
        }
    }

  else
    if c == '\''
    {
      self.cursor.advance();

      return self.read_data_that_begins_from_single_quote();
    }

  else
    {
      self.cursor.advance();

      self.push(TokenData::Others(c));

      return Ok(());
    }


  Err(())
}


pub fn
push(&mut self, tokdat: TokenData)
{
    if let Some(last) = self.output.last()
    {
        if let TokenData::Space = tokdat
        {
            if let TokenData::Space = last.get_data()
            {
              return;
            }
        }

      else
        if let TokenData::Newline = tokdat
        {
            if let TokenData::Newline = last.get_data()
            {
              return;
            }
        }
    }


  self.output.push(Token::new(tokdat,self.token_info.clone()));
}




}



pub fn
tokenize(src: &SourceFile)-> Result<TokenString,()>
{
  let  mut toks: TokenString = Vec::new();

  let  mut tk = Tokenizer::new(src,&mut toks);

    while let Some(c) = tk.get_character()
    {
        if tk.step(c).is_err()
        {
          print!("tokenize error: ");

          tk.token_info.print();

          return Err(());
        }
    }


  Ok(toks)
}




