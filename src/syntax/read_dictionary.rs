

use crate::source_file::{
  SourceFile,
  SourceInfo,

};

use crate::token::{
  Token,
  TokenKind,
  advance,
  strip_spaces,
  read_string_of_others,
  get_token,
  get_identifier,
  get_string,
  get_others,
};

use crate::token::tokenize::{
  tokenize,
};

use super::dictionary::{
  Expression,
  Definition,
  Dictionary,

};


fn
to_literal(s: &str)-> Result<Expression,String>
{
       if s == "Identifier"{Ok(Expression::IdentifierLiteral)}
  else if s ==     "Number"{Ok(Expression::NumberLiteral    )}
  else if s ==  "Character"{Ok(Expression::CharacterLiteral )}
  else if s ==     "String"{Ok(Expression::StringLiteral    )}
  else
    {Err(format!("{} is unknown literal keyword",s))}
}


fn
read_operand_that_begins_others_token(toks: &Vec<Token>, pos: &mut usize, c: char)-> Result<Expression,String>
{
    match c
    {
  '('=>
    {
        match read_binary_string(toks,pos,")")
        {
      Ok(e)=>{Ok(Expression::Expression(Box::new(e)))}
      Err(s)=>{Err(s)}
        }
    }
  '['=>
    {
        match read_binary_string(toks,pos,"]")
        {
      Ok(e)=>{Ok(Expression::Option(Box::new(e)))}
      Err(s)=>{Err(s)}
        }
    }
  '{'=>
    {
        match read_binary_string(toks,pos,"}")
        {
      Ok(e)=>{Ok(Expression::Repetition(Box::new(e)))}
      Err(s)=>{Err(s)}
        }
    }
  '\''=>
    {
        if let Some(s) = get_identifier(toks,*pos)
        {
          advance(pos);

          Ok(Expression::Keyword(s.clone()))
        }

      else
        {Err(format!("keyword is missing"))}
    }
  '.'=>
    {
        if let Some(s) = get_identifier(toks,*pos)
        {
          advance(pos);

          to_literal(s.as_str())
        }

      else
        {Err(format!("literal keyword is missing"))}
    }
    _=>{Err(format!("unknown others element"))}
    }
}


fn
read_operand(toks: &Vec<Token>, pos: &mut usize)-> Result<Expression,String>
{
    if let Some(tok) = get_token(toks,*pos)
    {
        match tok.get_kind()
        {
      TokenKind::Identifier(s)=>
        {
          advance(pos);

          let  o = Expression::Identifier(s.clone());

          Ok(o)
        }
      TokenKind::String(s)=>
        {
          let  o = Expression::String(s.clone());

          advance(pos);

          Ok(o)
        }
      TokenKind::Others(c)=>
        {
          advance(pos);

            match read_operand_that_begins_others_token(toks,pos,*c)
            {
          Ok(o)=>{Ok(o)}
          Err(s)=>{Err(s)}
            }
        }
      _=>{Err(format!("unknown operand element"))}
        }
    }

  else
    {Err(format!("オペランドがない"))}
}


fn
read_operator(toks: &Vec<Token>, pos: &mut usize)-> Result<&'static str,String>
{
       if read_string_of_others(toks,pos, "&"){Ok("&")}
  else if read_string_of_others(toks,pos, "|"){Ok("|")}
  else if read_string_of_others(toks,pos,"->"){Ok("->")}
  else if read_string_of_others(toks,pos, ")"){Ok(")")}
  else if read_string_of_others(toks,pos, "]"){Ok("]")}
  else if read_string_of_others(toks,pos, "}"){Ok("}")}
  else if read_string_of_others(toks,pos, ";"){Ok(";")}
  else
    {Err(format!("不明な演算子"))}
}


pub fn
read_binary_string(toks: &Vec<Token>, pos: &mut usize, closer: &'static str)-> Result<Expression,String>
{
    match read_operand(toks,pos)
    {
  Ok(mut left_o)=>
    {
        loop
        {
            match read_operator(toks,pos)
            {
          Ok(op)=>
            {
                if op == closer
                {
                  return Ok(left_o);
                }

              else
                if (op == ")") || (op == "]") || (op == "}") || (op == ";")
                {
                  return Err(format!("wrong closer {}",closer));
                }

              else
                {
                    match read_operand(toks,pos)
                    {
                  Ok(right_o)=>
                    {
                      left_o = Expression::BinaryOperation(Box::new(left_o),Box::new(right_o),op.to_string());
                    }
                  Err(e)=>
                    {
                      return Err(format!("right operand is missing"));
                    }
                    }
                }
            }
          Err(s)=>{return Err(s);}
            }
        }
    }
  Err(e)=>{Err(format!("{}\nオペランドが一つもない",&e))}
    }
}


pub fn
read_definition(toks: &Vec<Token>, pos: &mut usize)-> Result<Option<Definition>,String>
{
    if let Some(first_tok) = get_token(toks,*pos)
    {
      advance(pos);

        if let TokenKind::Identifier(s) = first_tok.get_kind()
        {
          advance(pos);

            if let Some(c) = get_others(toks,*pos)
            {
                if c == ':'
                {
                  advance(pos);
                }
            }


            match read_binary_string(toks,pos,";")
            {
          Ok(expr)=>
            {
              let  def = Definition::new(s.clone(),expr);

              Ok(Some(def))
            }
          Err(err_s)=>{Err(format!("{}の定義中のエラー: {}",s,err_s))}
            }
        }

      else
        {Err(format!("定義の開始が不正"))}
    }

  else
    {Ok(None)}
}


pub fn
read_dictionary(src: &SourceFile)-> Result<Dictionary,String>
{
  let  mut dic = Dictionary::new();

    match tokenize(src)
    {
  Ok(toks)=> 
    {
      let  stripped = strip_spaces(toks);

      let  mut pos: usize = 0;

        loop
        {
            match read_definition(&stripped,&mut pos)
            {
          Ok(def_opt)=>
            {
                match def_opt
                {
              Some(def)=>{dic.add(def);}
              None=>{return Ok(dic);}
                }
            }
          Err(s)=>{return Err(s);}
            }
        }
    }
  Err(e)=>
    {
      let  s = e.to_string();

      Err(format!("辞書字句解析エラー: {}",&s))
    }
    }
}




