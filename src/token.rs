

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




pub fn
get_token(toks: &Vec<Token>, pos: usize)-> Option<&Token>
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
skip_spaces(toks: &Vec<Token>, pos: &mut usize)
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
read_string_of_others(toks: &Vec<Token>, pos: &mut usize, s: &str)-> bool
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
get_integer(toks: &Vec<Token>, pos: usize)-> Option<u64>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_integer();
    }


  None
}


pub fn
get_floating(toks: &Vec<Token>, pos: usize)-> Option<f64>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_floating();
    }


  None
}


pub fn
get_identifier(toks: &Vec<Token>, pos: usize)-> Option<&String>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_identifier();
    }


  None
}


pub fn
get_string(toks: &Vec<Token>, pos: usize)-> Option<&String>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_string();
    }


  None
}


pub fn
get_character(toks: &Vec<Token>, pos: usize)-> Option<char>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_character();
    }


  None
}


pub fn
get_others(toks: &Vec<Token>, pos: usize)-> Option<char>
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.get_others();
    }


  None
}


pub fn
is_space(toks: &Vec<Token>, pos: usize)-> bool
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.is_space();
    }


  false
}


pub fn
is_newline(toks: &Vec<Token>, pos: usize)-> bool
{
    if let Some(tok) = get_token(toks,pos)
    {
      return tok.is_newline();
    }


  false
}




