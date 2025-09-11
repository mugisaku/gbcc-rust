

pub mod tokenize;
pub mod read_number;
pub mod read_string;
pub mod skip;
pub mod is;


use std::rc::Rc;
use crate::source_file::{
  SourceFile,
  Cursor,
};


use crate::token::is::{
  is_id_body,

};




#[derive(Clone)]
pub struct
ParsedNumber
{
  pub(crate) i_part: u64,

  pub(crate) f_part_opt: Option<f64>,

}


impl
ParsedNumber
{


pub fn
new()-> Self
{
  Self{
    i_part: 0,
    f_part_opt: None,
  }
}


pub fn
is_float(&self)-> bool
{
    if let Some(_) = self.f_part_opt
    {
      return true;
    }


  false
}


pub fn
get_float(&self)-> Option<f64>
{
    if let Some(f) = self.f_part_opt
    {
      return Some((self.i_part as f64)+f);
    }


  None
}


pub fn
print(&self)
{
    if let Some(f) = self.f_part_opt
    {
      print!("{}",(self.i_part as f64)+f);
    }

  else
    {
      print!("{}",self.i_part);
    }
}


}




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
print_string(s: &str)
{
    for c in s.chars()
    {
      print_character(c);
    }
}




pub enum
TokenData
{
  Space,
  Newline,
  Identifier(String),
  String(String),
  Character(char),
  Number(ParsedNumber),
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
        print_string(s);
      },
  TokenData::String(s)=>
      {
        print!("\"");
        print_string(s);
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
  TokenData::Number(pn)=>
      {
        pn.print();
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
is_identifier(&self, s: &str)-> bool
{
    if let TokenData::Identifier(target_s) = &self.data
    {
      return s == target_s;
    }


  false
}


pub fn
get_identifier(&self)-> Option<&String>
{
    if let TokenData::Identifier(s) = &self.data
    {
      return Some(s);
    }


  None
}


pub fn
get_string(&self)-> Option<&String>
{
    if let TokenData::String(s) = &self.data
    {
      return Some(s);
    }


  None
}


pub fn
get_number(&self)-> Option<&ParsedNumber>
{
    if let TokenData::Number(pn) = &self.data
    {
      return Some(pn);
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
is_others(&self, c: char)-> bool
{
    if let Some(target_c) = self.get_others()
    {
      return c == target_c;
    }


  false
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
        print!("Identifier: {}",s);
      },
  TokenData::String(s)=>
      {
        print!("String: \"");
        print_string(s);
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
  TokenData::Number(pn)=>
      {
        print!("Number: ");

        pn.print();
      },
  TokenData::Others(c)=>
      {
        print!("Others: {}",c);
      },
    }
}


}




pub fn
restore_token_string(toks: &[Token])
{
    for tok in toks
    {
        match tok.get_data()
        {
      TokenData::Space=>         {print!(" ");},
      TokenData::Newline=>       {print!("\n");},
      TokenData::Identifier(s)=> {print_string(s);},
      TokenData::String(s)=>
            {
              print!("\"");
              print_string(s);
              print!("\"");
            },
      TokenData::Character(c)=>
            {
              print!("\'");
              crate::token::print_character(*c);
              print!("\'");
            },
      TokenData::Number(pn)=>
            {
              pn.print();
            },
      TokenData::Others(c)=>{print!("{}",c);},
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




pub fn
get_token(toks: &[Token], pos: usize)-> Option<&Token>
{
    if pos < toks.len()
    {
      return Some(&toks[pos]);
    }


  None
}


pub fn
advance(pos: &mut usize)
{
  *pos += 1;
}


pub fn
skip_spaces(toks: &[Token], pos: &mut usize)
{
    while let Some(tok) = get_token(toks,*pos)
    {
        if tok.is_space() || tok.is_newline()
        {
          advance(pos);
        }

      else
        {
          break;
        }
    }
}


pub fn
strip_spaces(toks: Vec<Token>)-> Vec<Token>
{
  let  mut buf: Vec<Token> = Vec::new();

    for tok in toks
    {
        if  !tok.is_space()
         && !tok.is_newline()
        {
          buf.push(tok);
        }
    }


  buf
}


pub fn
read_string_of_others(toks: &[Token], pos: &mut usize, s: &str)-> bool
{
  let  tmp = *pos;

    for sc in s.chars()
    {
        if let Some(oc) = get_others(toks,*pos)
        {
            if sc == oc
            {
              advance(pos);

              continue;
            }
        }


      *pos = tmp;

      return false;
    }


  true
}


pub fn
get_number(toks: &[Token], pos: usize)-> Option<&ParsedNumber>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_number();
    }


  None
}


pub fn
get_identifier(toks: &[Token], pos: usize)-> Option<&String>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_identifier();
    }


  None
}


pub fn
get_string(toks: &[Token], pos: usize)-> Option<&String>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_string();
    }


  None
}


pub fn
get_character(toks: &[Token], pos: usize)-> Option<char>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_character();
    }


  None
}


pub fn
get_others(toks: &[Token], pos: usize)-> Option<char>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_others();
    }


  None
}


pub fn
is_space(toks: &[Token], pos: usize)-> bool
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.is_space();
    }


  false
}


pub fn
is_newline(toks: &[Token], pos: usize)-> bool
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.is_newline();
    }


  false
}




pub fn
read_identifier(src: &SourceFile, mut cur: Cursor)-> (String,Cursor)
{
  let  mut s = String::new();

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


  (s,cur)
}




#[derive(Clone)]
pub struct
Iterator<'a>
{
  reference: &'a [Token],

  position: usize,

}


impl<'a>
Iterator<'a>
{


pub fn
current(&self)-> Option<&Token>
{
    if self.position < self.reference.len()
    {
      return Some(&self.reference[self.position]);
    }


  None
}


pub fn
advance(&mut self)
{
  self.position += 1;
}


pub fn
skip_spaces(&mut self)
{
    while let Some(tok) = self.current()
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
get_number(&self)-> Option<&ParsedNumber>
{
    if let Some(tok) = self.current()
    {
      return tok.get_number();
    }


  None
}


pub fn
get_identifier(&self)-> Option<&String>
{
    if let Some(tok) = self.current()
    {
      return tok.get_identifier();
    }


  None
}


pub fn
get_string(&self)-> Option<&String>
{
    if let Some(tok) = self.current()
    {
      return tok.get_string();
    }


  None
}


pub fn
get_character(&self)-> Option<char>
{
    if let Some(tok) = self.current()
    {
      return tok.get_character();
    }


  None
}


pub fn
get_others(&self)-> Option<char>
{
    if let Some(tok) = self.current()
    {
      return tok.get_others();
    }


  None
}


pub fn
is_others(&self, c: char)-> bool
{
    if let Some(target_c) = self.get_others()
    {
      return c == target_c;
    }


  false
}


pub fn
is_space(&self)-> bool
{
    if let Some(tok) = self.current()
    {
      return tok.is_space();
    }


  false
}


pub fn
is_newline(&self)-> bool
{
    if let Some(tok) = self.current()
    {
      return tok.is_newline();
    }


  false
}


}


impl<'a>
std::iter::Iterator for Iterator<'a>
{


type Item = &'a Token;


fn
next(&mut self)-> Option<Self::Item>
{
    if self.position < self.reference.len()
    {
      let  r = &self.reference[self.position];

      self.position += 1;

      return Some(r);
    }


  None
}


}


impl<'a>
std::convert::From<&'a Vec<Token>> for Iterator<'a>
{


fn
from(value: &'a Vec<Token>)-> Self
{
  Iterator{reference: value, position: 0}
}


}




