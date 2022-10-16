



use std::rc::Rc;


pub struct
TokenInfo
{
  filepath: Rc<String>,

  x_pos: u32,
  y_pos: u32,

}


impl
TokenInfo
{


pub fn
new(filepath: &str)-> TokenInfo
{
  let  s = String::from(filepath);

  TokenInfo{ filepath: Rc::new(s), x_pos: 0, y_pos: 0}
}


pub fn
clone(&self)-> TokenInfo
{
  TokenInfo{ filepath: Rc::clone(&self.filepath), x_pos: self.x_pos, y_pos: self.y_pos}
}


pub fn  get_filepath(&self)-> &String{&*self.filepath}
pub fn  get_x_pos(&self)-> u32{self.x_pos}
pub fn  get_y_pos(&self)-> u32{self.y_pos}

pub fn
set_pos(&mut self, x: u32, y: u32)
{
  self.x_pos = x;
  self.y_pos = y;
}


}


impl
fmt::Display for TokenInfo
{


fn
fmt(&self, f: &mut fmt::Formatter)-> fmt::Result
{
  write!(f,"[X:{:05} Y:{:05}] ",1+self.x_pos,1+self.y_pos)
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
  Identifier(Rc<String>),
  String(Rc<String>),
  Character(char),
  Letter(char),
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
        print!("{}",&*s);
      },
  TokenData::String(s)=>
      {
        print!("\"");
        print_string(&*s);
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
  TokenData::Letter(c)=>
      {
        print!("\'{}",c);
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


use std::fmt;


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
get_identifier(&self)-> Option<&Rc<String>>
{
    if let TokenData::Identifier(s) = &self.data
    {
      return Some(s);
    }


  None
}


pub fn
get_string(&self)-> Option<&Rc<String>>
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
get_letter(&self)-> Option<char>
{
    if let TokenData::Letter(c) = self.data
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
  print!("{}",self);
}


}


impl
fmt::Display for Token
{


fn
fmt(&self, f: &mut fmt::Formatter)-> fmt::Result
{
  let  _ = write!(f,"[X:{:06} Y:{:06}] ",1+self.info.x_pos,1+self.info.y_pos);

    match &self.data
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
        print!("identifier: {}",&*s);
      },
  TokenData::String(s)=>
      {
        print!("\"");
        print_string(&*s);
        print!("\"");
      },
  TokenData::Character(c)=>
      {
          match c
          {
        '\0'=> {print!("Null Character");},
        '\n'=> {print!("Newline Character");},
        '\t'=> {print!("Tab Character");},
        '\r'=> {print!("return Character");},
        _   => {print!("\'{}\'",c);},
          }
      },
  TokenData::Letter(c)=>
      {
        print!("\'{}",c);
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
        print!("others: {}",c);
      },
    }


  write!(f,"")
}


}




pub type TokenString = Vec<Token>;


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
get_letter(&self)-> Option<char>
{
    if let Some(tok) = self.get()
    {
      return tok.get_letter();
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




