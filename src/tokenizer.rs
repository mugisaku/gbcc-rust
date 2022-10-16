

use std::rc::Rc;

use crate::token::TokenInfo;
use crate::token::TokenData;
use crate::token::Token;
use crate::source_file::SourceFile;

mod is;
mod read_number;

use crate::tokenizer::is::*;

pub type TokenString = Vec<Token>;

struct
Tokenizer<'a,'b>
{
  token_info: TokenInfo,

  input: std::str::Chars<'a>,

  x_pos: u32,
  y_pos: u32,

  data: Option<char>,

  output: &'b mut TokenString,

}




impl<'a,'b>
Tokenizer<'a,'b>
{


fn
new(src: &'a SourceFile, out: &'b mut TokenString)-> Tokenizer<'a,'b>
{
  let mut  tk = Tokenizer{
                  token_info: TokenInfo::new(src.get_path()),
                  input: src.get_content().chars(),
                  data: None,
                  x_pos: 0,
                  y_pos: 0,
                  output: out,
                };


  tk.data = tk.input.next();

    if let Some(c) = tk.data
    {
       if c == '\n'
       {
         tk.y_pos = 1;
       }
    }


  tk
}


fn
advance(&mut  self)
{
    if let Some(old) = self.data
    {
      self.data = self.input.next();

        if let Some(_) = self.data
        {
            if old == '\n'
            {
              self.x_pos  = 0;
              self.y_pos += 1;
            }

          else
            {
              self.x_pos += 1;
            }
        }
    }
}


fn
get_data(&  self)-> Option<char>
{
  self.data
}


fn
is_finished(&  self)-> bool
{
  return self.data == None;
}


fn
is_space(&  self)-> bool
{
    if let Some(data) = self.data
    {
      return is_space(data)
    }


  false
}


fn
is_digit(&  self)-> bool
{
    if let Some(data) = self.data
    {
      return is_digit(data)
    }


  false
}


fn
is_id_head(&  self)-> bool
{
    if let Some(data) = self.data
    {
      return is_id_head(data)
    }


  false
}


fn
is_id_body(&  self)-> bool
{
    if let Some(data) = self.data
    {
      return is_id_body(data)
    }


  false
}


fn
is_some(&  self, c:  char)-> bool
{
    if let Some(data) = self.data
    {
      return data == c;
    }


  false
}


fn
skip_until_appears_newline(&mut  self)-> Result<(),String>
{
    while let Some(c) = self.data
    {
      self.advance();

        if c == '\n'
        {
          return Ok(());
        }
    }


  Err(String::from("コメントラインが正しく終了していない"))
}


fn
skip_until_appears_end_of_comment_block(&mut  self)-> Result<(),String>
{
    while let Some(first) = self.data
    {
      self.advance();

        if first == '*'
        {
            if let Some(second) = self.data
            {
             self.advance();

                if second == '/'
                {
                  return Ok(());
                }
            }
        }
    }


  Err(String::from("コメントブロックが正しく終了していない"))
}


fn
skip_spaces(&mut  self)
{
  let  is_newline = self.is_some('\n');

  let  old_y_pos = self.y_pos;

  self.advance();

    while self.is_space()
    {
      self.advance();
    }


    return if is_newline || (self.y_pos != old_y_pos)
    {
      self.push(TokenData::Newline);
    }

  else
    {
      self.push(TokenData::Space);
    }
}


fn
read_identifier(&mut  self)
{
  let mut  s = String::new();

    if let Some(c) = self.data
    {
      s.push(c);
    }


  self.advance();

    while self.is_id_body()
    {
        if let Some(c) = self.data
        {
          s.push(c);
        }


      self.advance();
    }


  self.push(TokenData::Identifier(Rc::new(s)));
}


fn
read_string(&mut  self)-> Result<(),String>
{
  let mut  s = String::new();

  self.advance();

    while let Some(c) = self.data
    {
        match c
        {
      '\0'=> {return Err(String::from("文字列中にNull文字が出現"));},
      '\r'=> {return Err(String::from("文字列中に復帰文字が出現"));},
      '\t'=> {return Err(String::from("文字列中にタブ文字が出現"));},
      '\n'=> {return Err(String::from("文字列中に改行文字が出現"));},
      '\\'=>
            {
              self.advance();

                if let Some(e) = self.data
                {
                  s.push(crate::token::to_escape_character(e));

                  self.advance();
                }
            },
      _=>
            {
              self.advance();

                if c == '\"'
                {
                  break;
                }


              s.push(c);
            },
        }
    }


  self.push(TokenData::String(Rc::new(s)));

  Ok(())
}


fn
read_data_that_begins_from_single_quote(&mut  self)-> Result<(),String>
{
  self.advance();

    let  escaped = if self.is_some('\\')
                   {
                     self.advance();

                     true
                   }

                 else
                   {
                     false
                   };


    if let Some(c) = self.data
    {
      self.advance();

      let  final_c = if escaped
                     {
                       crate::token::to_escape_character(c)
                     }

                   else
                     {
                       c
                     };


        if let Some(tail) = self.data
        {
            if tail == '\''
            {
              self.advance();

              self.push(TokenData::Character(final_c));

              return Ok(());
            }
        }


      self.push(TokenData::Letter(final_c));

      return Ok(());
    }


  Err(String::from("シングルクオートの後に何もない"))
}


fn
read_data_that_begins_from_slash(&mut  self)-> Result<(),String>
{
  self.advance();

    if self.is_some('*')
    {
      self.advance();

        if let Err(msg) = self.skip_until_appears_end_of_comment_block()
        {
          return Err(msg);
        }


      self.push(TokenData::Space);
    }

  else
    if self.is_some('/')
    {
      self.advance();

        if let Err(msg) = self.skip_until_appears_newline()
        {
          return Err(msg);
        }


      self.push(TokenData::Newline)
    }

  else
    {
      self.push(TokenData::Others('/'));
    }


  Ok(())
}


fn
update_token_info(&mut  self)
{
  self.token_info.set_pos(self.x_pos,self.y_pos);
}


fn
step(&mut  self)-> Result<(),String>
{
  self.update_token_info();

    if self.is_space()
    {
      self.skip_spaces();

      return Ok(());
    }

  else
    if self.is_some('/')
    {
      return self.read_data_that_begins_from_slash();
    }

  else
    if self.is_id_head()
    {
      self.read_identifier();

      return Ok(());
    }

  else
    if self.is_digit()
    {
      return self.read_number();
    }

  else
    if self.is_some('\"')
    {
      return self.read_string();
    }

  else
    if self.is_some('\'')
    {
      return self.read_data_that_begins_from_single_quote();
    }

  else
    if let Some(c) = self.data
    {
      self.advance();

      self.push(TokenData::Others(c));

      return Ok(());
    }


  Err(String::from("no valid data"))
}


fn
push(&mut  self, tokdat: TokenData)
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


fn
print(&self)
{
    for tok in self.output.iter()
    {
      print!("//{}\n",tok);
    }
}


fn
restore(&self)
{
    for tok in self.output.iter()
    {
        match tok.get_data()
        {
      TokenData::Space=>         {print!(" ");},
      TokenData::Newline=>       {print!("\n");},
      TokenData::Identifier(s)=> {print!("{}",s);},
      TokenData::String(s)=>
            {
              print!("\"");
              crate::token::print_string(&s);
              print!("\"");
            },
      TokenData::Character(c)=>
            {
              print!("\'");
              crate::token::print_character(*c);
              print!("\'");
            },
      TokenData::Letter(c)=>
            {
              print!("\'");
              crate::token::print_character(*c);
            },
      TokenData::Integer(i)=>    {print!("{}",i);},
      TokenData::Floating(f)=>   {print!("{:.9}",f);},
      TokenData::Others(c)=>     {print!("{}",c);},
        }
    }
}




}



pub fn
tokenize(src: &SourceFile)-> Result<TokenString,()>
{
  let mut  toks: TokenString = Vec::new();

  let mut  tk = Tokenizer::new(src,&mut toks);

    while let Some(_) = tk.data
    {
        if let Err(msg) = tk.step()
        {
          print!("tokenize error: {}",msg);

          print!("{}",tk.token_info);

          return Err(());
        }
    }


  Ok(toks)
}




