

pub mod tokenize;
pub mod read_number;
pub mod read_string;
pub mod skip;
pub mod is;


use std::rc::Rc;
use crate::source_file::Cursor;


#[derive(Clone)]
pub struct
TokenInfo
{
  pub(crate) filepath: Rc<String>,

  pub(crate) cursor: Cursor,

}


impl
TokenInfo
{


pub fn
new(filepath: &str)-> TokenInfo
{
  TokenInfo{ filepath: Rc::new(String::from(filepath)), cursor: Cursor::new()}
}


pub fn  get_filepath(&self)-> &String{&*self.filepath}
pub fn  get_x(&self)-> usize{self.cursor.get_x()}
pub fn  get_y(&self)-> usize{self.cursor.get_y()}


pub fn
print(&self)
{
  print!("[X:{:05} Y:{:05}] ",1+self.get_x(),1+self.get_y())
}


}






pub fn
to_escape_character(c: char)-> char
{
    match c
    {
  '0'=>{'\0'},
  'n'=>{'\n'},
  't'=>{'\t'},
  'r'=>{'\r'},
  _=>{c},
    }
}


pub fn
print_character(c: char)
{
    match c
    {
  '\0'=>{print!("\\0")},
  '\n'=>{print!("\\n")},
  '\t'=>{print!("\\t")},
  '\r'=>{print!("\\r")},
  '\\'=>{print!("\\\\")},
  '\''=>{print!("\\\'")},
  '\"'=>{print!("\\\"")},
  _=>{print!("{}",c)},
    }
}


pub fn
print_char_string(s: &Vec<char>)
{
    for c in s
    {
      print_character(*c);
    }
}




pub enum
TokenData
{
  Space,
  Newline,
  Identifier(Vec<char>),
  String(Vec<char>),
  Character(char),
  Integer(u64),
  Floating(f64),
  Others(char),

}


impl
TokenData
{


pub fn
print(&self)
{
    match self
    {
  TokenData::Space=>
      {
        print!("SPACE");
      },
  TokenData::Newline=>
      {
        print!("NEWLINE");
      },
  TokenData::Identifier(s)=>
      {
        print_char_string(&*s);
      },
  TokenData::String(s)=>
      {
        print!("\"");
        print_char_string(&*s);
        print!("\"");
      },
  TokenData::Character(c)=>
      {
          match c
          {
        '\0'=> {print!("\\0");},
        '\n'=> {print!("\\n");},
        '\t'=> {print!("\\t");},
        '\r'=> {print!("\\r");},
        _   => {print!("\'{}\'",c);},
          }
      },
  TokenData::Integer(i)=>
      {
        print!("{}",i);
      },
  TokenData::Floating(f)=>
      {
        print!("{}",f);
      },
  TokenData::Others(c)=>
      {
        print!("{}",c);
      },
    }
}


}




pub struct
Token
{
  data: TokenData,
  info: TokenInfo,

}


impl
Token
{


pub fn
new(data: TokenData, info: TokenInfo)-> Token
{
  Token{data,info}
}


pub fn
get_info(&self)-> &TokenInfo
{
  &self.info
}


pub fn
get_data(&self)-> &TokenData
{
  &self.data
}


pub fn
is_space(&self)-> bool
{
    if let TokenData::Space = &self.data
    {
      return true;
    }


  false
}


pub fn
is_newline(&self)-> bool
{
    if let TokenData::Newline = &self.data
    {
      return true;
    }


  false
}


pub fn
get_identifier(&self)-> Option<&Vec<char>>
{
    if let TokenData::Identifier(s) = &self.data
    {
      return Some(s);
    }


  None
}


pub fn
get_string(&self)-> Option<&Vec<char>>
{
    if let TokenData::String(s) = &self.data
    {
      return Some(s);
    }


  None
}


pub fn
get_integer(&self)-> Option<u64>
{
    if let TokenData::Integer(i) = self.data
    {
      return Some(i);
    }


  None
}


pub fn
get_floating(&self)-> Option<f64>
{
    if let TokenData::Floating(f) = self.data
    {
      return Some(f);
    }


  None
}


pub fn
get_character(&self)-> Option<char>
{
    if let TokenData::Character(c) = self.data
    {
      return Some(c);
    }


  None
}


pub fn
get_others(&self)-> Option<char>
{
    if let TokenData::Others(c) = self.data
    {
      return Some(c);
    }


  None
}


pub fn
print(&self)
{
  print!("[X:{:06} Y:{:06}] ",1+self.info.get_x(),1+self.info.get_y());

    match &self.data
    {
  TokenData::Space=>
      {
        print!("Space");
      },
  TokenData::Newline=>
      {
        print!("Newline");
      },
  TokenData::Identifier(s)=>
      {
        print!("Identifier: ");

          for c in s
          {
            print!("{}",c);
          }
      },
  TokenData::String(s)=>
      {
        print!("String: \"");
        print_char_string(&*s);
        print!("\"");
      },
  TokenData::Character(c)=>
      {
          match c
          {
        '\0'=> {print!("Null Character");},
        '\n'=> {print!("Newline Character");},
        '\t'=> {print!("Tab Character");},
        '\r'=> {print!("Return Character");},
        _   => {print!("Character: \'{}\'",c);},
          }
      },
  TokenData::Integer(i)=>
      {
        print!("Integer: {}",i);
      },
  TokenData::Floating(f)=>
      {
        print!("Floating: {}",f);
      },
  TokenData::Others(c)=>
      {
        print!("Others: {}",c);
      },
    }
}


}




pub fn
restore_token_string(toks: &Vec<Token>)
{
    for tok in toks
    {
        match tok.get_data()
        {
      TokenData::Space=>         {print!(" ");},
      TokenData::Newline=>       {print!("\n");},
      TokenData::Identifier(s)=> {print_char_string(&s);},
      TokenData::String(s)=>
            {
              print!("\"");
              print_char_string(&s);
              print!("\"");
            },
      TokenData::Character(c)=>
            {
              print!("\'");
              crate::token::print_character(*c);
              print!("\'");
            },
      TokenData::Integer(i)=>    {print!("{}",i);},
      TokenData::Floating(f)=>   {print!("{:.9}",f);},
      TokenData::Others(c)=>     {print!("{}",c);},
        }
    }
}


pub fn
print_token_string(toks: &Vec<Token>)
{
    for tok in toks
    {
      tok.print();

      print!("\n");
    }
}


/*
pub struct
Cursor<'a>
{
  data: &'a TokenString,

  base_index: usize,

}


impl<'a>
Cursor<'a>
{


pub fn
from(toks: &'a Vec<Token>)-> Self
{
  Self{data: toks, base_index: 0}
}


pub fn
clone(&self)-> Self
{
  Self{data: self.data, base_index: self.base_index}
}


pub fn
advance(&mut self)
{
  self.base_index += 1;
}


pub fn
skip_spaces(&mut self)
{
    while let Some(tok) = self.get()
    {
        if tok.is_space() || tok.is_newline()
        {
          self.advance();
        }

      else
        {
          break;
        }
    }
}


pub fn
is_finished(&self)-> bool
{
  self.base_index >= self.data.len()
}


pub fn
is_not_finished(&self)-> bool
{
  self.base_index < self.data.len()
}


pub fn
get(&self)-> Option<&Token>
{
    if self.is_not_finished()
    {
      return Some(&self.data[self.base_index]);
    }

  else
    {
      return None;
    }
}


pub fn
get_integer(&self)-> Option<u64>
{
    if let Some(tok) = self.get()
    {
      return tok.get_integer();
    }


  None
}


pub fn
get_floating(&self)-> Option<f64>
{
    if let Some(tok) = self.get()
    {
      return tok.get_floating();
    }


  None
}


pub fn
get_identifier(&self)-> Option<&Rc<String>>
{
    if let Some(tok) = self.get()
    {
      return tok.get_identifier();
    }


  None
}


pub fn
get_string(&self)-> Option<&Rc<String>>
{
    if let Some(tok) = self.get()
    {
      return tok.get_string();
    }


  None
}


pub fn
get_character(&self)-> Option<char>
{
    if let Some(tok) = self.get()
    {
      return tok.get_character();
    }


  None
}


pub fn
get_others(&self)-> Option<char>
{
    if let Some(tok) = self.get()
    {
      return tok.get_others();
    }


  None
}


pub fn
is_space(&self)-> bool
{
    if let Some(tok) = self.get()
    {
      return tok.is_space();
    }


  false
}


pub fn
is_newline(&self)-> bool
{
    if let Some(tok) = self.get()
    {
      return tok.is_newline();
    }


  false
}


}
*/



