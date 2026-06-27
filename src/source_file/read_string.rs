

use crate::token::{
  to_escape_character,

};


use crate::source_file::{
  SourceFile,
  SourceInfo,
  SourceReader,
  Error,

};




impl
SourceReader
{


pub fn
read_raw_string(&mut self)-> Result<String,Error>
{
  let  begin_n = self.read_sharps();

  let  mut s = String::new();

    if let Some(c) = self.get_character()
    {
        if c == '\"'
        {
          self.advance();

            while let Some(next_c) = self.get_character()
            {
              self.advance();

                if next_c == '\"'
                {
                  let  end_n = self.read_sharps();

                    if end_n == begin_n
                    {
                      return Ok(s);
                    }


                  s.push('\"');

                    for _ in 0..end_n
                    {
                      s.push('#');
                    }
                }

              else
                {
                  s.push(next_c);

                    if next_c == '\n'
                    {
                      self.newline();
                    }
                }
            }


          return Err(self.info.to_error(format!("生文字列が閉じられていない")));
        }
    }


  Err(self.info.to_error(format!("生文字列の始まりが不正")))
}


fn
read_sharps(&mut self)-> usize
{
  let  mut n: usize = 0;

    while let Some(c) = self.get_character()
    {
        if c == '#'
        {
          n += 1;
        }

      else
        {
          break;
        }


      self.advance();
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


fn
to_unicode(a: char, b: char, c: char, d: char)-> Result<char,String>
{
    if let Ok(aa) = Self::to_hexadecimal(a){
    if let Ok(bb) = Self::to_hexadecimal(b){
    if let Ok(cc) = Self::to_hexadecimal(c){
    if let Ok(dd) = Self::to_hexadecimal(d){
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


  Err(format!("１６進数ではない文字"))
}


fn
read_unicode_escape_sequence(&mut self)-> Result<char,Error>
{
    if let Some(a) = self.get_character()
    {
      self.advance();

        if let Some(b) = self.get_character()
        {
          self.advance();

            if let Some(c) = self.get_character()
            {
              self.advance();

                if let Some(d) = self.get_character()
                {
                  self.advance();

                    match Self::to_unicode(a,b,c,d)
                    {
                  Ok(c)=>{return Ok(c);}
                  Err(s)=>{return Err(self.info.to_error(s));}
                    }
                }
            }
        }
    }


  Err(self.info.to_error(format!("処理すべき文字が足りなし")))
}


pub fn
read_escape_sequence(&mut self)-> Result<char,Error>
{
    if let Some(c) = self.get_character()
    {
        match c
        {
      '0'=> {  self.advance();  return Ok('\0');},
      'r'=> {  self.advance();  return Ok('\r');},
      't'=> {  self.advance();  return Ok('\t');},
      'n'=> {  self.advance();  return Ok('\n');},
      '\\'=>{  self.advance();  return Ok('\\');},
      '\''=>{  self.advance();  return Ok('\'');},
      '\"'=>{  self.advance();  return Ok('\"');},
      'u'=>
        {
          self.advance();

          return self.read_unicode_escape_sequence();
        },
      _=>
        {
          self.advance();

          return Ok(c);
        },
        }
    }


  Err(self.info.to_error(format!("処理すべき文字がない")))
}


pub fn
read_character(&mut self)-> Result<char,Error>
{
    if let Some(c) = self.get_character()
    {
        match c
        {
      '\0'=> {return Err(self.info.to_error(format!("Null文字が出現")));},
      '\r'=> {return Err(self.info.to_error(format!("復帰文字が出現")));},
      '\t'=> {return Err(self.info.to_error(format!("タブ文字が出現")));},
      '\n'=> {return Err(self.info.to_error(format!("改行文字が出現")));},
      '\\'=>
        {
          self.advance();

          self.read_escape_sequence()
        }
      _=>
        {
          self.advance();

          Ok(c)
        }
        }
    }

  else
    {
      Err(self.to_error(format!("処理すべき文字がない")))
    }
}


pub fn
read_string(&mut self)-> Result<String,Error>
{
  let  mut s = String::new();

    while let Some(c) = self.get_character()
    {
        if c == '\"'
        {
          self.advance();

          break;
        }

      else
        {
            match self.read_character()
            {
          Ok(cc)=>{s.push(cc);}
          Err(e)=>{return Err(e);}
            }
        }
    }


  Ok(s)
}


}




