

use crate::source_file::{
  SourceFile,
  SourceInfo,

};




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






pub struct
ParsedNumber
{
  radix: u32,

   integer: String,
  fraction: String,

}


impl
ParsedNumber
{


pub fn
new(radix: u32)-> Self
{
  Self{
    radix,
     integer: String::new(),
    fraction: String::new(),
  }
}


pub fn
new_zero()-> Self
{
  Self{
    radix: 10,
     integer: "0".to_string(),
    fraction: String::new(),
  }
}


pub fn
is_float(&self)-> bool
{
  self.fraction.len() != 0
}


pub fn
push_to_integer(&mut self, c: char)
{
  self.integer.push(c);
}


pub fn
push_to_fraction(&mut self, c: char)
{
  self.fraction.push(c);
}


pub fn
try_to_u64(&self)-> Result<u64,std::num::ParseIntError>
{
    if self.integer.len() == 0{Ok(0)}
  else{u64::from_str_radix(&self.integer,self.radix)}
}


pub fn
try_to_f64(&self)-> Result<f64,std::num::ParseFloatError>
{
  use std::str::FromStr;

  let  s = format!("{}.{}",&self.integer,&self.fraction);

  f64::from_str(&s)
}


pub fn
print(&self)
{
    match self.radix
    {
   2=>{print!("0b");}
   8=>{print!("0o");}
  10=>{}
  16=>{print!("0x");}
  _=>{panic!();}
    }


  print!("{}",&self.integer);

  if self.fraction.len() != 0{print!(".{}",&self.fraction);}
}


}




pub enum
TokenKind
{
  Null,
  Space,
  Newline,
  Identifier(String),
  WithApostrophe(String),
  String(String),
  Character(char),
  Number(ParsedNumber),
  Others(char),

}


impl
TokenKind
{


pub fn
print(&self)
{
    match self
    {
  TokenKind::Null=>
      {
        print!("NULL");
      },
  TokenKind::Space=>
      {
        print!("SPACE");
      },
  TokenKind::Newline=>
      {
        print!("NEWLINE");
      },
  TokenKind::Identifier(s)=>
      {
        print!("{}",s);
      },
  TokenKind::WithApostrophe(s)=>
      {
        print!("\'{}",s);
      },
  TokenKind::String(s)=>
      {
        print!("\"");
        print_string(s);
        print!("\"");
      },
  TokenKind::Character(c)=>
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
  TokenKind::Number(pn)=>
      {
        pn.print();
      },
  TokenKind::Others(c)=>
      {
        print!("{}",c);
      },
    }
}


}




pub struct
Token
{
  source_info: SourceInfo,

  kind: TokenKind,

}


impl
Token
{


pub fn
new(source_info: SourceInfo, kind: TokenKind)-> Self
{
  Self{source_info,kind}
}


pub fn
get_source_info(&self)-> &SourceInfo
{
  &self.source_info
}


pub fn
get_kind(&self)-> &TokenKind
{
  &self.kind
}


pub fn
is_space(&self)-> bool
{
    if let TokenKind::Space = &self.kind
    {
      return true;
    }


  false
}


pub fn
is_newline(&self)-> bool
{
    if let TokenKind::Newline = &self.kind
    {
      return true;
    }


  false
}


pub fn
is_identifier(&self, s: &str)-> bool
{
    if let TokenKind::Identifier(target_s) = &self.kind
    {
      return s == target_s;
    }


  false
}


pub fn
get_identifier(&self)-> Option<&String>
{
    if let TokenKind::Identifier(s) = &self.kind
    {
      return Some(s);
    }


  None
}


pub fn
get_string(&self)-> Option<&String>
{
    if let TokenKind::String(s) = &self.kind
    {
      return Some(s);
    }


  None
}


pub fn
get_number(&self)-> Option<&ParsedNumber>
{
    if let TokenKind::Number(pn) = &self.kind
    {
      return Some(pn);
    }


  None
}


pub fn
get_character(&self)-> Option<char>
{
    if let TokenKind::Character(c) = self.kind
    {
      return Some(c);
    }


  None
}


pub fn
get_others(&self)-> Option<char>
{
    if let TokenKind::Others(c) = self.kind
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
  self.source_info.print();

  self.kind.print();
}


}




pub fn
restore_token_string(toks: &[Token])
{
    for tok in toks
    {
        match &tok.kind
        {
      TokenKind::Null=>             {print!("");},
      TokenKind::Space=>            {print!(" ");},
      TokenKind::Newline=>          {print!("\n");},
      TokenKind::Identifier(s)=>    {print!("{}",s);},
      TokenKind::WithApostrophe(s)=>{print!("\'{}",s);},
      TokenKind::String(s)=>
            {
              print!("\"");
              print_string(s);
              print!("\"");
            },
      TokenKind::Character(c)=>
            {
              print!("\'");
              crate::token::print_character(*c);
              print!("\'");
            },
      TokenKind::Number(pn)=>
            {
              pn.print();
            },
      TokenKind::Others(c)=>{print!("{}",c);},
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
get_source_info(toks: &[Token], pos: usize)-> Option<SourceInfo>
{
    if pos < toks.len()
    {
      return Some(toks[pos].source_info.clone());
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




