

use crate::source_file::{
  SourceFile,
  Cursor,
};


use super::is::*;


const CODE_OF_ZERO: u64 = 48;


pub enum
Number
{
   Integer(u64),
  Floating(f64),
}


pub fn
read_binary_number(src: &SourceFile, cur: &mut Cursor)-> Result<u64,()>
{
  let mut  n: u64 = 0;

    while let Some(c) = src.get_character(cur)
    {
        match c
        {
      '0'=>
            {
              n <<= 1;
            },
      '1'=>
            {
              n <<= 1;
              n  |= 1;
            },
      '2'..='9'=>
            {
              println!("２進数に０か１以外の数を使おうとした");

              return Err(());
            },
      _=>
            {
              break;
            },
        }


      cur.advance();
    }


  Ok(n)
}


pub fn
read_octal_number(src: &SourceFile, cur: &mut Cursor)-> Result<u64,()>
{
  let mut  n: u64 = 0;

    while let Some(c) = src.get_character(cur)
    {
        if is_octal(c)
        {
          n *= 8;

          n |= c as u64-CODE_OF_ZERO;

          cur.advance();
        }

      else
        if (c == '8') || (c =='9')
        {
          println!("８進数に８か９の数を使おうとした");

          return Err(());
        }

      else
        {
          break;
        }
    }


  Ok(n)
}


pub fn
read_fraction_number(src: &SourceFile, cur: &mut Cursor, i: u64)-> Result<f64,()>
{
  let mut  f: f64 = 0.0;
  let mut  w: u64 =   1;
  let _: f64 = 0.1234567;

    while let Some(c) = src.get_character(cur)
    {
        if is_digit(c)
        {
          f *= 10.0;

          f += (c as u64-CODE_OF_ZERO)as f64;

          w *= 10;

          cur.advance();
        }

      else
        {
          break;
        }
    }


  let  final_value = (i as f64)+(f/w as f64);

  Ok(final_value)
}


pub fn
read_decimal_number(src: &SourceFile, cur: &mut Cursor)-> Result<u64,()>
{
  let mut  n: u64 = 0;

    while let Some(c) = src.get_character(cur)
    {
        if is_digit(c)
        {
          n *= 10;

          n += c as u64-CODE_OF_ZERO;

          cur.advance();
        }

      else
        {
          break;
        }
    }


  Ok(n)
}


pub fn
read_hexadecimal_number(src: &SourceFile, cur: &mut Cursor)-> Result<u64,()>
{
  let mut  n: u64 = 0;

    while let Some(c) = src.get_character(cur)
    {
      const CODE_OF_A_UPPER: u64 = 65;
      const CODE_OF_A_LOWER: u64 = 97;

        match c
        {
      '0'..='9' =>
            {
              n *= 16;

              n |= c as u64-CODE_OF_ZERO;
            },
      'a'..='f' =>
            {
              n *= 16;

              n |= c as u64-(CODE_OF_A_LOWER+10);
            },
      'A'..='F' =>
            {
              n *= 16;

              n |= c as u64-(CODE_OF_A_UPPER+10);
            },
      _=>
            {
              break;
            },
        }


      cur.advance();
    }


  Ok(n)
}


pub fn
read_number_that_begins_from_zero(src: &SourceFile, cur: &mut Cursor)-> Result<Number,()>
{
    match src.get_character(cur)
    {
  Some('b')=>
        {
          cur.advance();

            if let Ok(i) = read_binary_number(src,cur)
            {
              return Ok(Number::Integer(i));
            }
        },
  Some('o')=>
        {
          cur.advance();

            if let Ok(i) =  read_octal_number(src,cur)
            {
              return Ok(Number::Integer(i));
            }
        },
  Some('x')=>
        {
          cur.advance();

            if let Ok(i) = read_hexadecimal_number(src,cur)
            {
              return Ok(Number::Integer(i));
            }
        },
  Some('.')=>
        {
          cur.advance();

            if let Ok(f) = read_fraction_number(src,cur,0)
            {
              return Ok(Number::Floating(f));
            }
        },
  _=>{return Ok(Number::Integer(0));},
    }


  Err(())
}


pub fn
read_number(src: &SourceFile, cur: &mut Cursor, first_c: char)-> Result<Number,()>
{
    if first_c == '0'
    {
      cur.advance();

      return read_number_that_begins_from_zero(src,cur);
    }


    if let Ok(i) = read_decimal_number(src,cur)
    {
        if let Some(c) = src.get_character(cur)
        {
            if c == '.'
            {
              cur.advance();

                if let Ok(f) = read_fraction_number(src,cur,i)
                {
                  return Ok(Number::Floating(f));
                }


              println!("小数を読めなかった");

              return Err(());
            }
        }


      return Ok(Number::Integer(i));
    }


  println!("整数を読めなかった");

  Err(())
}




