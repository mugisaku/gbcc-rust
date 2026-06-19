

use crate::source_file::{
  SourceFile,
  Cursor,
};


use super::is::*;


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
to_u64(&self)-> Result<u64,std::num::ParseIntError>
{
    if self.integer.len() == 0{Ok(0)}
  else{u64::from_str_radix(&self.integer,self.radix)}
}


pub fn
to_f64(&self)-> Result<f64,std::num::ParseFloatError>
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




fn
get_binary_integer(src: &SourceFile, cur: Cursor)-> Option<char>
{
    if let Some(c) = src.get_character(cur)
    {
        if (c == '0') || (c == '1')
        {
          return Some(c);
        }
    }


  None
}


fn
get_octal_integer(src: &SourceFile, cur: Cursor)-> Option<char>
{
    if let Some(c) = src.get_character(cur)
    {
       if (c >= '0') && (c <= '7')
       {
         return Some(c);
       }
    }


  None
}


pub fn
get_decimal_integer(src: &SourceFile, cur: Cursor)-> Option<char>
{
    if let Some(c) = src.get_character(cur)
    {
       if (c >= '0') && (c <= '9')
       {
         return Some(c);
       }
    }


  None
}


fn
get_hexadecimal_integer(src: &SourceFile, cur: Cursor)-> Option<char>
{
    if let Some(c) = src.get_character(cur)
    {
       if ((c >= '0') && (c <= '9'))
       || ((c >= 'A') && (c <= 'F'))
       {
         return Some(c);
       }
    }


  None
}


fn
read_integer_that_begins_from_zero(src: &SourceFile, mut cur: Cursor)-> (ParsedNumber,Cursor)
{
    match src.get_character(cur)
    {
  Some('b')=>
        {
          let  mut pn = ParsedNumber::new(2);

          cur.advance();

            while let Some(c) = get_binary_integer(src,cur)
            {
              pn.integer.push(c);

              cur.advance();
            }


          (pn,cur)
        },
  Some('o')=>
        {
          let  mut pn = ParsedNumber::new(8);

          cur.advance();

            while let Some(c) = get_octal_integer(src,cur)
            {
              pn.integer.push(c);

              cur.advance();
            }


          (pn,cur)
        },
  Some('x')=>
        {
          let  mut pn = ParsedNumber::new(16);

          cur.advance();

            while let Some(c) = get_hexadecimal_integer(src,cur)
            {
              pn.integer.push(c);

              cur.advance();
            }


          (pn,cur)
        },
  _=>{(ParsedNumber::new_zero(),cur)},
    }
}


fn
read_integer(src: &SourceFile, mut cur: Cursor)-> (ParsedNumber,Cursor)
{
    if let Some(first_c) = src.get_character(cur)
    {
        if first_c == '0'
        {
          cur.advance();

          read_integer_that_begins_from_zero(src,cur)
        }

      else
        {
          let  mut pn = ParsedNumber::new(10);

            while let Some(c) = get_decimal_integer(src,cur)
            {
              pn.integer.push(c);

              cur.advance();
            }


          (pn,cur)
        }
    }

  else
    {
      panic!();
    }
}


pub fn
read_number(src: &SourceFile, cur: Cursor)-> (ParsedNumber,Cursor)
{
  let  (mut pn,mut new_cur) = read_integer(src,cur);

    if let Some(c) = src.get_character(new_cur)
    {
        if c == '.'
        {
          new_cur.advance();

            while let Some(c) = get_decimal_integer(src,new_cur)
            {
              pn.fraction.push(c);

              new_cur.advance();
            }
        }
    }


  (pn,new_cur)
}




