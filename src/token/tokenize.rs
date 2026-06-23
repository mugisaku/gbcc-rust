

use super::{
  TokenKind,
  Token,
  ParseTokenError,
  print_string,
  read_identifier,

};


use super::read_number::{
  ParsedNumber,
  read_number,

};


use super::read_string::*;


use super::skip::*;


use crate::source_file::{
  SourceFile,
  SourceInfo,
  Cursor,

};

use super::is::*;


pub type TokenString = Vec<Token>;

struct
Tokenizer<'a,'b>
{
  source_info: SourceInfo,

  input: &'a SourceFile,

  cursor: Cursor,

  output: &'b mut TokenString,

}




impl<'a,'b>
Tokenizer<'a,'b>
{


pub fn
new(src: &'a SourceFile, out: &'b mut TokenString)-> Tokenizer<'a,'b>
{
  let  mut source_info = SourceInfo::new();

  source_info.set_filepath(src.get_path().as_str());

  let mut  tk = Tokenizer{
                  source_info,
                  input: src,
                  cursor: Cursor::new(),
                  output: out,
                };


  tk
}


fn
get_character(&self)-> Option<char>
{
  self.input.get_character(self.cursor)
}


fn
read_data_that_begins_from_id_head(src: &SourceFile, cur: Cursor, first_c: char)-> (TokenKind,Cursor)
{
    if first_c == 'r'
    {
      let  mut tmp_cur = cur;

      tmp_cur.advance();

        if let Some(second_c) = src.get_character(tmp_cur)
        {
            if second_c == '#'
            {
              let  mut st = (String::new(),tmp_cur);

              read_raw_string(src,&mut st);

              return (TokenKind::String(st.0),st.1);
            }
        }
    }


  let  (s,new_cur) = read_identifier(src,cur);

  (TokenKind::Identifier(s),new_cur)
}


fn
read_character_or_identifier_after_single_quote(&mut self, c: char)-> Result<(),String>
{
  let  (s,new_cur) = read_identifier(&self.input,self.cursor);

  self.cursor = new_cur;

    if let Some(last_c) = self.get_character()
    {
        if last_c == '\''
        {
            if s.len() == 1
            {
              self.cursor.advance();

              self.push(TokenKind::Character(c));

              return Ok(());
            }


          return Err(format!("文字リテラルに一文字を超える内容{},{}",last_c,s.len()));
        }
    }


  self.push(TokenKind::Others('\''));
  self.push(TokenKind::Identifier(s));

  Ok(())
}


fn
read_escape_sequence(&mut self)-> Result<(),String>
{
    if let Ok(esc) = read_escape_sequence(&self.input,&mut self.cursor)
    {
        if let Some(last_c) = self.get_character()
        {
            if last_c == '\''
            {
              self.cursor.advance();

              self.push(TokenKind::Character(esc));

              return Ok(());
            }
        }


      Err(format!("文字列リテラルが不正な閉じ方"))
    }

  else
    {
      Err(format!("文字列リテラルで不正なシーケンス文字"))
    }
}


fn
read_data_that_begins_from_single_quote(&mut self)-> Result<(),String>
{
    if let Some(c) = self.get_character()
    {
        if c == '\''
        {
          return Err(format!("empty character literal"));
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

                  self.push(TokenKind::Character(c));

                  return Ok(());
                }


              return Err(format!("文字リテラルに不明な内容{}",last_c));
            }
        }
    }


  Err(format!("不正なシングルクオート要素"))
}


fn
read_data_that_begins_from_slash(&mut self)-> Result<(),String>
{
    if let Some(c) = self.get_character()
    {
        if c == '*'
        {
          self.cursor.advance();

          let  old_y = self.cursor.get_y();

            if let Err(s) = skip_until_appears_end_of_comment_block(&self.input,&mut self.cursor)
            {
              return Err(s);
            }


            if old_y != self.cursor.get_y()
            {
              self.push(TokenKind::Space);
            }

          else
            {
              self.push(TokenKind::Newline);
            }


          return Ok(());
        }

      else
        if c == '/'
        {
          self.cursor.advance();

            if let Err(s) = skip_until_appears_newline(&self.input,&mut self.cursor)
            {
              return Err(s);
            }


          self.push(TokenKind::Newline);

          return Ok(());
        }
    }


  self.push(TokenKind::Others('/'));

  Ok(())
}


fn
update_source_info(&mut self)
{
  self.source_info.set_cursor(&self.cursor);
}


fn
skip_spaces(&mut self)
{
  let  old_y = self.cursor.get_y();

  skip_spaces(&self.input,&mut self.cursor);

    if self.cursor.get_y() != old_y
    {
      self.push(TokenKind::Newline);
    }

  else
    {
      self.push(TokenKind::Space);
    }
}


fn
step(&mut self, c: char)-> Result<(),String>
{
  self.update_source_info();

    if is_space(c)
    {
      self.skip_spaces();

      Ok(())
    }

  else
    if c == '/'
    {
      self.cursor.advance();

      self.read_data_that_begins_from_slash()
    }

  else
    if is_id_head(c)
    {
      let  (k,new_cur) = Self::read_data_that_begins_from_id_head(&self.input,self.cursor,c);

      self.push(k);

      self.cursor = new_cur;

      Ok(())
    }

  else
    if is_digit(c)
    {
      let  (pn,cur) = read_number(self.input,self.cursor);

      self.push(TokenKind::Number(pn));

      self.cursor = cur;

      Ok(())
    }

  else
    if c == '\"'
    {
      self.cursor.advance();

        match read_string(&self.input,&mut self.cursor)
        {
      Ok(s)=>
        {
          self.push(TokenKind::String(s));

          Ok(())
        }
       Err(s)=>{Err(s)}
        }
    }

  else
    if c == '\''
    {
      self.cursor.advance();

      self.read_data_that_begins_from_single_quote()
    }

  else
    {
      self.cursor.advance();

      self.push(TokenKind::Others(c));

      Ok(())
    }
}


fn
push(&mut self, k: TokenKind)
{
    if let Some(last) = self.output.last()
    {
        if let TokenKind::Space = k
        {
            if let TokenKind::Space = last.get_kind()
            {
              return;
            }
        }

      else
        if let TokenKind::Newline = k
        {
            if let TokenKind::Newline = last.get_kind()
            {
              return;
            }
        }
    }


  self.output.push(Token::new(k,self.source_info.clone()));
}




}



pub fn
tokenize(src: &SourceFile)-> Result<TokenString,ParseTokenError>
{
  let  mut toks: TokenString = Vec::new();

  let  mut tk = Tokenizer::new(src,&mut toks);

    while let Some(c) = tk.get_character()
    {
        if let Err(s) = tk.step(c)
        {
          return Err(ParseTokenError{source_info: tk.source_info.clone(), message: s});
        }
    }


  Ok(toks)
}


pub fn
tokenize_from_string(s: &str)-> Result<TokenString,ParseTokenError>
{
  let  src = SourceFile::from_string(s);

  return tokenize(&src);
}




