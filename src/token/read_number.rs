

use crate::source_file::{
  SourceFile,
  Cursor,
};


use super::is::*;


const CODE_OF_ZERO: u64 = 48;


fn
get_binary_digit(src: &SourceFile, cur: &Cursor)-> Option<u64>
{
    if let Some(c) = src.get_character(cur)
    {
        match c
        {
      '0'=>{return Some(0);},
      '1'=>{return Some(1);},
      _=>{}
        }
    }


  None
}


pub fn
read_binary_number(src: &SourceFile, cur: &mut Cursor)-> Result<u64,()>
{
    if let Some(first_d) = get_binary_digit(src,cur)
    {
      let  mut n = first_d;

      cur.advance();

        while let Some(d) = get_binary_digit(src,cur)
        {
          cur.advance();

          n <<= 1;
          n  |= d;
        }


      return Ok(n);
    }


  Err(())
}


fn
get_octal_digit(src: &SourceFile, cur: &Cursor)-> Option<u64>
{
    if let Some(c) = src.get_character(cur)
    {
        if is_octal(c)
        {
          return Some(c as u64-CODE_OF_ZERO);
        }
    }


  None
}


pub fn
read_octal_number(src: &SourceFile, cur: &mut Cursor)-> Result<u64,()>
{
    if let Some(first_d) = get_octal_digit(src,cur)
    {
      let  mut n = first_d;

      cur.advance();

        if let Some(d) = get_octal_digit(src,cur)
        {
          cur.advance();

          n <<= 3;
          n  |= d;
        }


      return Ok(n);
    }


  Err(())
}


pub fn
read_fraction_number(src: &SourceFile, cur: &mut Cursor, i: u64)-> Result<f64,()>
{
    if let Some(first_d) = get_decimal_digit(src,cur)
    {
      let  mut f: f64 = first_d as f64;
      let  mut w: u64 =              1;

      cur.advance();

        while let Some(d) = get_decimal_digit(src,cur)
        {
          cur.advance();

          f *= 10.0;
          f += d as f64;
          w *= 10;
        }


      return Ok((i as f64)+(f/w as f64));
    }


  Err(())
}


pub fn
get_decimal_digit(src: &SourceFile, cur: &Cursor)-> Option<u64>
{
    if let Some(c) = src.get_character(cur)
    {
        if is_digit(c)
        {
          return Some(c as u64-CODE_OF_ZERO);
        }
    }


  None
}


pub fn
read_decimal_number(src: &SourceFile, cur: &mut Cursor)-> Result<u64,()>
{
    if let Some(first_d) = get_decimal_digit(src,cur)
    {
      let  mut n = first_d;

      cur.advance();

        while let Some(d) = get_decimal_digit(src,cur)
        {
          cur.advance();

          n *= 10;
          n +=  d;
        }


      return Ok(n);
    }


  Err(())
}


fn
get_hexadecimal_digit(src: &SourceFile, cur: &Cursor)-> Option<u64>
{
    if let Some(c) = src.get_character(cur)
    {
      const CODE_OF_A_UPPER: u64 = 65;
      const CODE_OF_A_LOWER: u64 = 97;

        match c
        {
      '0'..='9'=> {return Some(c as u64-CODE_OF_ZERO);},
      'a'..='f'=> {return Some(c as u64-(CODE_OF_A_LOWER+10));},
      'A'..='F'=> {return Some(c as u64-(CODE_OF_A_UPPER+10));},
      _=>{},
        }
    }


  None
}


pub fn
read_hexadecimal_number(src: &SourceFile, cur: &mut Cursor)-> Result<u64,()>
{
    if let Some(first_d) = get_hexadecimal_digit(src,cur)
    {
      let  mut n = first_d;

      cur.advance();

        while let Some(d) = get_hexadecimal_digit(src,cur)
        {
          cur.advance();

          n <<= 4;
          n  |= d;
        }


      return Ok(n);
    }


  Err(())
}


pub fn
read_number_that_begins_from_zero(src: &SourceFile, cur: &mut Cursor)-> Result<u64,()>
{
    match src.get_character(cur)
    {
  Some('b')=>
        {
          cur.advance();

          return read_binary_number(src,cur);
        },
  Some('o')=>
        {
          cur.advance();

          return read_octal_number(src,cur);
        },
  Some('x')=>
        {
          cur.advance();

          return read_hexadecimal_number(src,cur);
        },
  _=>{return Ok(0);},
    }
}


pub fn
read_number(src: &SourceFile, cur: &mut Cursor, first_c: char)-> Result<u64,()>
{
    if first_c == '0'
    {
      cur.advance();

      return read_number_that_begins_from_zero(src,cur);
    }


  read_decimal_number(src,cur)
}




