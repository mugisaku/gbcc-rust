

use crate::token::{
  Token,
  TokenInfo,
  TokenData,
  ParseTokenError,
  get_token,
  get_token_info,
  get_number,
  get_character,
  get_string,
  get_identifier,
  get_others,
  read_string_of_others,
  strip_spaces,
  tokenize::tokenize,
  tokenize::tokenize_from_string,
};


use super::dictionary::{
  Definition,
  Dictionary,
  Expression,

};

use crate::node::{
  Node,
  Value,
  Cursor,

};


use super::ParseSyntaxError as Error;

enum
ParseSyntaxResult
{
  Some(Vec<Value>),
  None,
  Err(Error),

}


use ParseSyntaxResult::Some;
use ParseSyntaxResult::None;
use ParseSyntaxResult::Err;

use std::result::Result       as StdResult;
use StdResult::Ok             as     StdOk;
use StdResult::Err            as    StdErr;

use std::option::Option::Some as StdSome;
use std::option::Option::None as StdNone;


struct
Status<'a,'b>
{
  dictionary: &'a Dictionary,

  token_string: &'b Vec<Token>,

  position: usize,
     depth: usize,

}


impl<'a,'b>
Status<'a,'b>
{


fn
advance(&mut self)
{
  self.position += 1;
}


pub fn
print_indent(&self)
{
    for _ in 0..self.depth
    {
      print!("--|");
    }


  print!(">> ");
}


fn
get_token_info(&self)-> TokenInfo
{
  self.token_string[self.position].get_info().clone()
}


fn
read_repetition(&mut self, e: &Expression)-> ParseSyntaxResult
{
    match self.read_by_expression(e)
    {
  Some(mut first_vals)=>
    {
        loop
        {
            match self.read_by_expression(e)
            {
          Some(mut vals)=>{first_vals.append(&mut vals);}
          None=>{break;}
          Err(e)=>{return Err(e);}
            }
        }


      Some(first_vals)
    }
  None=>{None}
  Err(e)=>{Err(e)}
    }
}


fn
read_keyword(&mut self, s: &str)-> ParseSyntaxResult
{
    match get_identifier(&self.token_string,self.position)
    {
  StdSome(kw)=>
    {
        if kw == s
        {
          let  v = Value::Keyword(kw.clone());

          self.position += 1;

          Some(vec![v])
        }

      else
        {None}
    }
  StdNone=>{None}
    }
}


fn
read_number_literal(&mut self)-> ParseSyntaxResult
{
    match get_number(&self.token_string,self.position)
    {
  StdSome(pn)=>
    {
      let  inf = get_token_info(&self.token_string,self.position).unwrap();

      self.advance();

        if pn.is_float()
        {
            match pn.try_to_f64()
            {
          StdOk(f)=>
            {
              let  v = Value::Float(f);

              Some(vec![])
            }
          StdErr(_)=>{Err(Error::new_with_token_info(inf,format!("整数が不正")))}
            }
        }

      else
        {
            match pn.try_to_u64()
            {
          StdOk(u)=>
            {
              let  v = Value::Uint(u);

              Some(vec![v])
            }
          StdErr(_)=>{Err(Error::new_with_token_info(inf,format!("浮動小数点数が不正")))}
            }
        }
    }
  StdNone=>{None}
    }
}


fn
read_by_string(&mut self, s: &str)-> ParseSyntaxResult
{
  let  old_pos = self.position;

    if read_string_of_others(&self.token_string,&mut self.position,s)
    {
      let  v = Value::SemiString(s.to_string());

      Some(vec![v])
    }

  else
    {
      self.position = old_pos;

      None
    }
}


fn
read_by_identifier(&mut self, s: &str)-> ParseSyntaxResult
{
    if let StdSome(def) = self.dictionary.find(s)
    {
        if self.depth >= 800
        {
          return Err(Error::new(format!("read_by_identifier: depth limit is over")));
        }


      self.read_by_definition(def)
    }

  else
    {Err(Error::new(format!("read_by_identifier: {}という定義はない",s)))}
}


fn
read_and(&mut self, l: &Expression, r: &Expression)-> ParseSyntaxResult
{
  let  old_pos = self.position;

    match self.read_by_expression(l)
    {
  Some(mut l_vals)=>
    {
        match self.read_by_expression(r)
        {
      Some(mut r_vals)=>
        {
          l_vals.append(&mut r_vals);

          Some(l_vals)
        }
      None=>
        {
          self.position = old_pos;

          None
        }
      Err(e)=>{Err(e)}
        }
    }
  None=>{None}
  Err(e)=>{Err(e)}
    }
}


fn
read_or(&mut self, l: &Expression, r: &Expression)-> ParseSyntaxResult
{
  let  old_pos = self.position;

    match self.read_by_expression(l)
    {
  Some(l_vals)=>{Some(l_vals)}
  None=>
    {
      self.position = old_pos;

        match self.read_by_expression(r)
        {
      Some(r_vals)=>{Some(r_vals)}
      None=>
        {
          self.position = old_pos;

          None
        }
      Err(e)=>{Err(e)}
        }
    }
  Err(e)=>{Err(e)}
    }
}


fn
read_arrow(&mut self, l: &Expression, r: &Expression)-> ParseSyntaxResult
{
  let  old_pos = self.position;

    match self.read_by_expression(l)
    {
  Some(mut l_vals)=>
    {
        match self.read_by_expression(r)
        {
      Some(mut r_vals)=>
        {
          l_vals.append(&mut r_vals);

          Some(l_vals)
        }
      None=>{Err(Error::new(format!("解析失敗を確定")))}
      Err(e)=>{Err(e)}
        }
    }
  None=>{None}
  Err(e)=>{Err(e)}
    }
}


fn
read_by_binary_operation(&mut self, l: &Expression, r: &Expression, op: &str)-> ParseSyntaxResult
{
    match op
    {
  (s) if s == "&" =>{self.read_and(  l,r)}
  (s) if s == "|" =>{self.read_or(   l,r)}
  (s) if s == "->"=>{self.read_arrow(l,r)}
  _=>{Err(Error::new(format!("不明な演算子 {}",op)))}
    }
}


pub fn
read_by_expression(&mut self, e: &Expression)-> ParseSyntaxResult
{
    match e
    {
  Expression::Expression(e_e)=>{self.read_by_expression(e_e)}
  Expression::Option(op_e)=>
    {
        match self.read_by_expression(op_e)
        {
      Some(vals)=>{Some(vals)}
      None      =>{Some(vec![])}
      Err(e)    =>{Err(e)}
        }
    }
  Expression::Repetition(rep_e)=>{self.read_repetition(rep_e)}
  Expression::Identifier(s)=>{self.read_by_identifier(s)}
  Expression::String(s)    =>{self.read_by_string(s)}
  Expression::Keyword(s)   =>{self.read_keyword(s)}
  Expression::IdentifierLiteral=>
    {
        match get_identifier(&self.token_string,self.position)
        {
      StdSome(s)=>
        {
          let  v = Value::Identifier(s.clone());

          self.advance();

          Some(vec![v])
        }
      StdNone=>{None}
        }
    }
  Expression::NumberLiteral=>{self.read_number_literal()}
  Expression::CharacterLiteral=>
    {
        match get_character(&self.token_string,self.position)
        {
      StdSome(c)=>
        {
          let  v = Value::Char(c);

          self.advance();

          Some(vec![v])
        }
      StdNone=>{None}
        }
    }
  Expression::StringLiteral=>
    {
        match get_string(&self.token_string,self.position)
        {
      StdSome(s)=>
        {
          let  v = Value::String(s.clone());

          self.advance();

          Some(vec![v])
        }
      StdNone=>{None}
        }
    }
  Expression::BinaryOperation(l,r,op)=>{self.read_by_binary_operation(&*l,&*r,op)}
    }
}


fn
read_by_definition(&mut self, def: &Definition)-> ParseSyntaxResult
{
    if let StdSome(tok) = get_token(&self.token_string,self.position)
    {
      let  old_pos = self.position;

      self.depth += 1;

        match self.read_by_expression(def.get_expression())
        {
      Some(vals)=>
        {
          self.depth -= 1;

          let  mut nd = Node::new(&def.get_name());

          nd.add_value_list(vals);

          let  val = Value::Node(Box::new(nd));

          Some(vec![val])
        }
      None=>
        {
          self.depth -= 1;

          self.position = old_pos;

          None
        }
      Err(e)=>{Err(e)}
        }
    }

  else
    {None}
}


}




pub fn
parse<'a>(toks: &Vec<Token>, dic: &'a Dictionary, main_def_name: &str)-> StdResult<Node,Error>
{
  let  mut nd = Node::new("");

  let  mut st = Status{
                  dictionary: dic,
                  token_string: toks,
                  position: 0,
                  depth: 0,
                };


  let  mut prev_pos: usize = 0;

    loop
    {
        match st.read_by_identifier(main_def_name)
        {
      Some(vals)=>
        {
            if st.position == prev_pos
            {
              break;
            }


          nd.add_value_list(vals);
        }
      None=>{break;}
      Err(e)=>{return StdErr(e);}
        }


      prev_pos = st.position;
    }


    if st.position >= toks.len()
    {
      StdOk(nd)
    }

  else
    {
      let  mut buf = String::new();

      let  tok = &toks[st.position];

      buf.push_str("解析途中で停止");

      StdErr(Error::new_with_token_info(tok.get_info().clone(),buf))
    }
}


pub fn
parse_from_string<'a>(s: &str, dic: &'a Dictionary, main_def_name: &str)-> StdResult<Node,Error>
{
    match tokenize_from_string(s)
    {
  StdOk(toks)=>
    {
//crate::token::print_token_string(&toks);
      let  stripped = strip_spaces(toks);

      parse(&stripped,dic,main_def_name)
    }
  StdErr(e)=>
    {
      let  message = format!("字句解析エラー: {}",&e.to_string());

      StdErr(Error::new(message))
    }
    }
}




