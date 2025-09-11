

use super::{
  ParsedNumber,

};


use crate::source_file::{
  SourceFile,
  Cursor,
};


use super::is::*;


fn
get_binary_integer(src: &SourceFile, cur: Cursor)-> Option<u64>
{
    if let Some(c) = src.get_character(cur)
    {
        match c
        {
      '0'=>{return Some(0);}
      '1'=>{return Some(1);}
      _=>{}
        }
    }


  None
}


pub fn
read_binary_integer(src: &SourceFile, mut cur: Cursor)-> (u64,Cursor)
{
  let  mut a = 0u64;

    while let Some(i) = get_binary_integer(src,cur)
    {
      a <<= 1;
      a  |= i;

      cur.advance();
    }


  (a,cur)
}


fn
get_octal_integer(src: &SourceFile, cur: Cursor)-> Option<u64>
{
    if let Some(c) = src.get_character(cur)
    {
       match c
       {
     '0'..='7'=>{return Some((c as u64)-('0' as u64));}
      _=>{}
       }
    }


  None
}


pub fn
read_octal_integer(src: &SourceFile, mut cur: Cursor)-> (u64,Cursor)
{
  let  mut a = 0u64;

    while let Some(i) = get_octal_integer(src,cur)
    {
      a <<= 3;
      a  |= i;

      cur.advance();
    }


  (a,cur)
}


pub fn
get_decimal_integer(src: &SourceFile, cur: Cursor)-> Option<u64>
{
    if let Some(c) = src.get_character(cur)
    {
       match c
       {
     '0'..='9'=>{return Some((c as u64)-('0' as u64));}
      _=>{}
       }
    }


  None
}


pub fn
read_decimal_integer(src: &SourceFile, mut cur: Cursor)-> (u64,Cursor)
{
  let  mut a = 0u64;

    while let Some(i) = get_decimal_integer(src,cur)
    {
      a *= 10;
      a +=  i;

      cur.advance();
    }


  (a,cur)
}


pub fn
read_decimal_fraction(src: &SourceFile, mut cur: Cursor)-> (f64,Cursor)
{
  let  mut f =  0.0f64;
  let  mut d = 10.0f64;

    while let Some(i) = get_decimal_integer(src,cur)
    {
      f += (i as f64)/d;
      d *= 10.0;

      cur.advance();
    }


  (f,cur)
}


fn
get_hexadecimal_integer(src: &SourceFile, cur: Cursor)-> Option<u64>
{
    if let Some(c) = src.get_character(cur)
    {
       match c
       {
     '0'..='9'=>{return Some(   (c as u64)-('0' as u64));}
     'A'..='F'=>{return Some(10+(c as u64)-('A' as u64));}
      _=>{}
       }
    }


  None
}


pub fn
read_hexadecimal_integer(src: &SourceFile, mut cur: Cursor)-> (u64,Cursor)
{
  let  mut a = 0u64;

    while let Some(i) = get_hexadecimal_integer(src,cur)
    {
      a <<= 4;
      a  |= i;

      cur.advance();
    }


  (a,cur)
}


pub fn
read_integer_that_begins_from_zero(src: &SourceFile, mut cur: Cursor)-> (u64,Cursor)
{
    match src.get_character(cur)
    {
  Some('b')=>
        {
          cur.advance();

          read_binary_integer(src,cur)
        },
  Some('o')=>
        {
          cur.advance();

          read_octal_integer(src,cur)
        },
  Some('x')=>
        {
          cur.advance();

          read_hexadecimal_integer(src,cur)
        },
  _=>{(0,cur)},
    }
}


pub fn
read_integer(src: &SourceFile, mut cur: Cursor)-> (u64,Cursor)
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
          read_decimal_integer(src,cur)
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
  let  mut pn = ParsedNumber::new();

  let  (i,mut new_cur) = read_integer(src,cur);

  pn.i_part = i;

    if let Some(c) = src.get_character(new_cur)
    {
        if c == '.'
        {
          new_cur.advance();

          let  (f,final_cur) = read_decimal_fraction(src,new_cur);

          new_cur = final_cur;

          pn.f_part_opt = Some(f);
        }
    }


  (pn,new_cur)
}




