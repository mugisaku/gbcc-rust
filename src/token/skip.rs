

use crate::source_file::{
  SourceFile,
  Cursor,
};

use super::is::*;


pub fn
skip_until_appears_newline(src: &SourceFile, cur: &mut Cursor)-> Result<(),String>
{
    while let Some(c) = src.get_character(*cur)
    {
      cur.advance();

        if c == '\n'
        {
          cur.newline();

          return Ok(());
        }
    }


  Err(format!("コメントラインが正しく終了していない"))
}


pub fn
skip_until_appears_end_of_comment_block(src: &SourceFile, cur: &mut Cursor)-> Result<(),String>
{
    while let Some(first) = src.get_character(*cur)
    {
      cur.advance();

        if first == '\n'
        {
          cur.newline();
        }

      else
        if first == '*'
        {
            if let Some(second) = src.get_character(*cur)
            {
              cur.advance();

                if second == '/'
                {
                  return Ok(());
                }
            }
        }
    }


  Err(format!("コメントブロックが正しく終了していない"))
}


pub fn
skip_spaces(src: &SourceFile, cur: &mut Cursor)
{
    while let Some(c) = src.get_character(*cur)
    {
        if is_space(c)
        {
            if c == '\n'
            {
              cur.newline();
            }

          else
            {
              cur.advance();
            }
        }

      else
        {
          break;
        }
    }
}




