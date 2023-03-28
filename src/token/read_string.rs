

use super::{
  to_escape_character,
};


use crate::source_file::{
  SourceFile,
  Cursor,
  new_char_string,
};




pub fn
read_raw_string(src: &SourceFile, cur: &mut Cursor, n: usize)-> Result<Vec<char>,()>
{
  let  mut s = new_char_string();

    while let Some(c) = src.get_character(cur)
    {
      cur.advance();

        if c == '\"'
        {
          let  end_n = read_sharps(src,cur);

            if n == end_n
            {
              break;
            }


          s.push('\"');

            for _ in 0..end_n
            {
              s.push('#');
            }
        }

      else
        {
          s.push(c);

            if c == '\n'
            {
              cur.newline();
            }
        }
    }


  Ok(s)
}


pub fn
read_sharps(src: &SourceFile, cur: &mut Cursor)-> usize
{
  let  mut n: usize = 0;

    while let Some(c) = src.get_character(cur)
    {
        if c == '#'
        {
          n += 1;
        }

      else
        {
          break;
        }


      cur.advance();
    }


  n
}




pub fn
to_hexadecimal(c: char)-> Result<u32,()>
{
    match c
    {
  '0'=>{Ok(0)}
  '1'=>{Ok(1)}
  '2'=>{Ok(2)}
  '3'=>{Ok(3)}
  '4'=>{Ok(4)}
  '5'=>{Ok(5)}
  '6'=>{Ok(6)}
  '7'=>{Ok(7)}
  '8'=>{Ok(8)}
  '9'=>{Ok(9)}
  'a'=>{Ok(10)}
  'b'=>{Ok(11)}
  'c'=>{Ok(12)}
  'd'=>{Ok(13)}
  'e'=>{Ok(14)}
  'f'=>{Ok(15)}
  'A'=>{Ok(10)}
  'B'=>{Ok(11)}
  'C'=>{Ok(12)}
  'D'=>{Ok(13)}
  'E'=>{Ok(14)}
  'F'=>{Ok(15)}
  _=>{Err(())}
    }
}


pub fn
to_unicode(a: char, b: char, c: char, d: char)-> Result<char,()>
{
    if let Ok(aa) = to_hexadecimal(a){
    if let Ok(bb) = to_hexadecimal(b){
    if let Ok(cc) = to_hexadecimal(c){
    if let Ok(dd) = to_hexadecimal(d){
      let  u = (aa<<12)
              |(bb<< 8)
              |(cc<< 4)
              |(dd    )
              ;

        if let Some(c) = char::from_u32(u)
        {
          return Ok(c);
        }
    }}}}


  Err(())
}


pub fn
read_unicode_escape_sequence(src: &SourceFile, cur: &mut Cursor)-> Result<char,()>
{
    if let Some(a) = src.get_character(cur)
    {
      cur.advance();

        if let Some(b) = src.get_character(cur)
        {
          cur.advance();

            if let Some(c) = src.get_character(cur)
            {
              cur.advance();

                if let Some(d) = src.get_character(cur)
                {
                  cur.advance();

                  return to_unicode(a,b,c,d);
                }
            }
        }
    }


  Err(())
}


pub fn
read_escape_sequence(src: &SourceFile, cur: &mut Cursor)-> Result<char,()>
{
    if let Some(c) = src.get_character(cur)
    {
        match c
        {
      '0'=> {  cur.advance();  return Ok('\0');},
      'r'=> {  cur.advance();  return Ok('\r');},
      't'=> {  cur.advance();  return Ok('\t');},
      'n'=> {  cur.advance();  return Ok('\n');},
      '\\'=> {  cur.advance();  return Ok('\\');},
      '\''=> {  cur.advance();  return Ok('\'');},
      '\"'=> {  cur.advance();  return Ok('\"');},
      'u'=>
            {
              cur.advance();

              return read_unicode_escape_sequence(src,cur);
            },
      _=>
            {
              cur.advance();

              return Ok(c);
            },
        }
    }


  Err(())
}


pub fn
read_character(src: &SourceFile, cur: &mut Cursor)-> Result<char,()>
{
    if let Some(c) = src.get_character(cur)
    {
        match c
        {
      '\0'=> {  println!("Null文字が出現");  return Err(());},
      '\r'=> {  println!("復帰文字が出現");  return Err(());},
      '\t'=> {  println!("タブ文字が出現");  return Err(());},
      '\n'=> {  println!("改行文字が出現");  return Err(());},
      '\\'=>
            {
              cur.advance();

              return read_escape_sequence(src,cur);
            },
      _=>
            {
              cur.advance();

              return Ok(c);
            },
        }
    }


  Err(())
}


pub fn
read_string(src: &SourceFile, cur: &mut Cursor)-> Result<Vec<char>,()>
{
  let  mut s = new_char_string();

    while let Some(c) = src.get_character(cur)
    {
        if c == '\"'
        {
          cur.advance();

          break;
        }

      else
        if let Ok(cc) = read_character(src,cur)
        {
          s.push(cc);
        }

      else
        {
          return Err(());
        }
    }


  Ok(s)
}




