

use std::rc::Rc;

use crate::source_file::{
  SourceFile,
  SourceInfo,
  SourceReader,
  Message,

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

use super::dictionary::{
  Expression,
  Definition,
  Dictionary,

};


fn
to_literal(s: &str)-> Result<Expression,Message>
{
       if s == "Identifier"{Ok(Expression::IdentifierLiteral)}
  else if s ==     "Number"{Ok(Expression::NumberLiteral    )}
  else if s ==  "Character"{Ok(Expression::CharacterLiteral )}
  else if s ==     "String"{Ok(Expression::StringLiteral    )}
  else
    {Err(Message::new(format!("{} is unknown literal keyword",s)))}
}


fn
read_operand_that_begins_others_token(toks: &Vec<Token>, pos: &mut usize, c: char)-> Result<Expression,Message>
{
    match c
    {
  '('=>
    {
      let  e = read_binary_string(toks,pos,")")?;

      Ok(Expression::Expression(Box::new(e)))
    }
  '['=>
    {
      let  e = read_binary_string(toks,pos,"]")?;

      Ok(Expression::Option(Box::new(e)))
    }
  '{'=>
    {
      let  e = read_binary_string(toks,pos,"}")?;

      Ok(Expression::Repetition(Box::new(e)))
    }
  '.'=>
    {
        if let Some(s) = get_identifier(toks,*pos)
        {
          advance(pos);

          to_literal(s.as_str())
        }

      else
        {Err(Message::from("literal keyword is missing"))}
    }
  _=>{Err(Message::from("unknown others element"))}
    }
}


fn
read_operand(toks: &Vec<Token>, pos: &mut usize)-> Result<Expression,Message>
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
      TokenKind::WithApostrophe(s)=>
        {
          advance(pos);

          let  o = Expression::Keyword(s.clone());

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

          Ok(read_operand_that_begins_others_token(toks,pos,*c)?)
        }
      _=>{Err(Message::from("unknown operand element"))}
        }
    }

  else
    {Err(Message::from("オペランドがない"))}
}


fn
read_operator(toks: &Vec<Token>, pos: &mut usize)-> Result<&'static str,Message>
{
       if read_string_of_others(toks,pos, "&"){Ok("&")}
  else if read_string_of_others(toks,pos, "|"){Ok("|")}
  else if read_string_of_others(toks,pos,"->"){Ok("->")}
  else if read_string_of_others(toks,pos, ")"){Ok(")")}
  else if read_string_of_others(toks,pos, "]"){Ok("]")}
  else if read_string_of_others(toks,pos, "}"){Ok("}")}
  else if read_string_of_others(toks,pos, ";"){Ok(";")}
  else
    {Err(Message::from("不明な演算子"))}
}


pub fn
read_binary_string(toks: &Vec<Token>, pos: &mut usize, closer: &'static str)-> Result<Expression,Message>
{
    match read_operand(toks,pos)
    {
  Ok(mut left_o)=>
    {
        loop
        {
          let  op = read_operator(toks,pos)?;

            if op == closer
            {
              return Ok(left_o);
            }

          else
            if (op == ")") || (op == "]") || (op == "}") || (op == ";")
            {
              return Err(Message::new(format!("wrong closer {}",closer)));
            }

          else
            {
                match read_operand(toks,pos)
                {
              Ok(right_o)=>
                {
                  left_o = Expression::BinaryOperation(Box::new(left_o),Box::new(right_o),op.to_string());
                }
              Err(msg)=>
                {
                  return Err(msg+"right operand is missing");
                }
                }
            }
        }
    }
  Err(msg)=>{Err(msg+"オペランドが一つもない")}
    }
}


pub fn
read_definition(toks: &Vec<Token>, pos: &mut usize)-> Result<Option<Definition>,Message>
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
          Err(msg)=>{Err(msg+format!("{}の定義中のエラー",s))}
            }
        }

      else
        {Err(Message::from("定義の開始が不正"))}
    }

  else
    {Ok(None)}
}


pub fn
read_dictionary(file: &Rc<SourceFile>)-> Result<Dictionary,Message>
{
  let  mut dic = Dictionary::new();

  let  mut r = SourceReader::new(file);

  let  toks = r.read_token_string()?;

  let  stripped = strip_spaces(toks);

  let  mut pos: usize = 0;

    loop
    {
      let  def_opt = read_definition(&stripped,&mut pos)?;

        match def_opt
        {
      Some(def)=>{dic.add(def);}
      None=>{return Ok(dic);}
        }
    }
}




