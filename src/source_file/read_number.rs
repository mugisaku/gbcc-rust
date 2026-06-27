

use crate::source_file::{
  SourceFile,
  SourceInfo,
  SourceReader,
  Error,

};


use crate::token::ParsedNumber;


impl
SourceReader
{


fn
get_binary_integer(&self)-> Option<char>
{
    if let Some(c) = self.get_character()
    {
        if (c == '0') || (c == '1')
        {
          return Some(c);
        }
    }


  None
}


fn
get_octal_integer(&self)-> Option<char>
{
    if let Some(c) = self.get_character()
    {
       if (c >= '0') && (c <= '7')
       {
         return Some(c);
       }
    }


  None
}


fn
get_decimal_integer(&self)-> Option<char>
{
    if let Some(c) = self.get_character()
    {
       if (c >= '0') && (c <= '9')
       {
         return Some(c);
       }
    }


  None
}


fn
get_hexadecimal_integer(&self)-> Option<char>
{
    if let Some(c) = self.get_character()
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
read_integer_that_begins_from_zero(&mut self)-> ParsedNumber
{
    match self.get_character()
    {
  Some('b')=>
        {
          let  mut pn = ParsedNumber::new(2);

          self.advance();

            while let Some(c) = self.get_binary_integer()
            {
              pn.push_to_integer(c);

              self.advance();
            }


          pn
        },
  Some('o')=>
        {
          let  mut pn = ParsedNumber::new(8);

          self.advance();

            while let Some(c) = self.get_octal_integer()
            {
              pn.push_to_integer(c);

              self.advance();
            }


          pn
        },
  Some('x')=>
        {
          let  mut pn = ParsedNumber::new(16);

          self.advance();

            while let Some(c) = self.get_hexadecimal_integer()
            {
              pn.push_to_integer(c);

              self.advance();
            }


          pn
        },
  _=>{ParsedNumber::new_zero()},
    }
}


fn
read_integer(&mut self)-> ParsedNumber
{
    if let Some(first_c) = self.get_character()
    {
        if first_c == '0'
        {
          self.advance();

          self.read_integer_that_begins_from_zero()
        }

      else
        {
          let  mut pn = ParsedNumber::new(10);

            while let Some(c) = self.get_decimal_integer()
            {
              pn.push_to_integer(c);

              self.advance();
            }


          pn
        }
    }

  else
    {
      panic!();
    }
}


pub fn
read_number(&mut self)-> ParsedNumber
{
  let  mut pn = self.read_integer();

    if let Some(c) = self.get_character()
    {
        if c == '.'
        {
          self.advance();

            while let Some(c) = self.get_decimal_integer()
            {
              pn.push_to_fraction(c);

              self.advance();
            }
        }
    }


  pn
}


}




