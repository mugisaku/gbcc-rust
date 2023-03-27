



pub fn
is_space(data: char)-> bool
{
  return (data == ' ')  ||
         (data == '\n') ||
         (data == '\t') ||
         (data == '\r')
        ;
}


pub fn
is_octal(data: char)-> bool
{
  return (data >= '0') && (data <= '7');
}


pub fn
is_digit(data: char)-> bool
{
  return (data >= '0') && (data <= '9');
}


pub fn
is_upper(data: char)-> bool
{
  return (data >= 'A') && (data <= 'Z');
}


pub fn
is_lower(data: char)-> bool
{
  return (data >= 'a') && (data <= 'z');
}


pub fn
is_alphabet(data: char)-> bool
{
  return is_upper(data) || is_lower(data);
}


pub fn
is_id_head(data: char)-> bool
{
  return is_alphabet(data) || (data == '_');
}


pub fn
is_id_body(data: char)-> bool
{
  return is_id_head(data) || is_digit(data);
}



