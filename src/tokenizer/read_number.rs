

use crate::tokenizer::Tokenizer;
use crate::token::TokenData;

use crate::tokenizer::is::*;


const CODE_OF_ZERO: u64 = 48;


impl<'a,'b>
Tokenizer<'a,'b>
{


pub(self) fn
read_binary_number(&mut  self)-> Result<(),String>
{
  let mut  n: u64 = 0;

    while let Some(c) = self.data
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
              return Err(String::from("２進数に０か１以外の数を使おうとした"));
            },
      _=>
            {
              break;
            },
        }


      self.advance();
    }


  self.push(TokenData::Integer(n));

  Ok(())
}


pub(self)fn
read_octal_number(&mut  self)-> Result<(),String>
{
  let mut  n: u64 = 0;

    while let Some(c) = self.data
    {
        if is_octal(c)
        {
          n *= 8;

          n |= c as u64-CODE_OF_ZERO;

          self.advance();
        }

      else
        if (c == '8') || (c =='9')
        {
          return Err(String::from("８進数に８か９の数を使おうとした"));
        }

      else
        {
          break;
        }
    }


  self.push(TokenData::Integer(n));

  Ok(())
}


pub(self)fn
read_fraction_number(&mut  self, i: u64)-> Result<(),String>
{
  let mut  f: f64 = 0.0;
  let mut  w: u64 =   1;
  let _: f64 = 0.1234567;

    while let Some(c) = self.data
    {
        if is_digit(c)
        {
          f *= 10.0;

          f += (c as u64-CODE_OF_ZERO)as f64;

          w *= 10;

          self.advance();
        }

      else
        {
          break;
        }
    }


  let  final_value = (i as f64)+(f/w as f64);

  self.push(TokenData::Floating(final_value));

  Ok(())
}


pub(self)fn
read_decimal_number(&mut  self)-> Result<u64,String>
{
  let mut  n: u64 = 0;

    while let Some(c) = self.data
    {
        if is_digit(c)
        {
          n *= 10;

          n += c as u64-CODE_OF_ZERO;

          self.advance();
        }

      else
        {
          break;
        }
    }


  Ok(n)
}


pub(self)fn
read_hexadecimal_number(&mut  self)-> Result<(),String>
{
  let mut  n: u64 = 0;

    while let Some(c) = self.data
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


      self.advance();
    }


  self.push(TokenData::Integer(n));

  Ok(())
}


pub(self)fn
read_number_that_begins_from_zero(&mut  self)-> Result<(),String>
{
    match self.data
    {
  Some('b')=>
        {
          self.advance();

          return self.read_binary_number();
        },
  Some('o')=>
        {
          self.advance();

          return self.read_octal_number();
        },
  Some('x')=>
        {
          self.advance();

          return self.read_hexadecimal_number();
        },
  Some('.')=>
        {
          self.advance();

          return self.read_fraction_number(0);
        },
  _=> {},
    }


  self.push(TokenData::Integer(0));

  return Ok(());
}


pub fn
read_number(&mut  self)-> Result<(),String>
{
    if self.is_some('0')
    {
      self.advance();

      return self.read_number_that_begins_from_zero();
    }


  let  d = self.read_decimal_number();

    if let Ok(i) = d
    {
        if self.is_some('.')
        {
          self.advance();

            if let Some(c) = self.data
            {
                if is_digit(c)
                {
                  self.update_token_info();

                  return self.read_fraction_number(i);
                }


              self.push(TokenData::Integer(i));
              self.push(TokenData::Others('.'));

              return Ok(());
            }
        }


      self.push(TokenData::Integer(i));

      return Ok(());
    }


  Err(String::from("整数を読めなかった"))
}


}



