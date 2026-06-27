

use crate::token::{
  Token,
  TokenKind,
  get_token,
  get_source_info,
  get_number,
  get_character,
  get_string,
  get_identifier,
  get_others,
  read_string_of_others,
  strip_spaces,

};


use super::dictionary::{
  Definition,
  Dictionary,
  Expression,

};

use crate::node::{
  Node,
  Value,
  ValueKind,
  Cursor,

};

use crate::source_file::{
  SourceFile,
  SourceInfo,
  SourceReader,
  Error,

};


enum
ParseResult
{
  Some(Vec<Value>),
  None,
  Err(Error),

}


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
read_repetition(&mut self, e: &Expression)-> ParseResult
{
    match self.read_by_expression(e)
    {
  ParseResult::Some(mut first_vals)=>
    {
        loop
        {
            match self.read_by_expression(e)
            {
          ParseResult::Some(mut vals)=>{first_vals.append(&mut vals);}
          ParseResult::None=>{break;}
          ParseResult::Err(e)=>{return ParseResult::Err(e);}
            }
        }


      ParseResult::Some(first_vals)
    }
  ParseResult::None=>{ParseResult::None}
  ParseResult::Err(e)=>{ParseResult::Err(e)}
    }
}


fn
read_keyword(&mut self, kw_s: &str)-> ParseResult
{
    match get_token(&self.token_string,self.position)
    {
  Some(tok)=>
    {
        match tok.get_kind()
        {
      TokenKind::Identifier(s)=>
        {
            if kw_s == s
            {
              let  info = tok.get_source_info().clone();
              let  kind = ValueKind::Keyword(s.clone());

              self.position += 1;

              ParseResult::Some(vec![Value::new(info,kind)])
            }

          else
            {ParseResult::None}
        }
      _=>{ParseResult::None}
        }
    }
  None=>{ParseResult::None}
    }
}


fn
read_number_literal(&mut self)-> ParseResult
{
    match get_number(&self.token_string,self.position)
    {
  Some(pn)=>
    {
      let  info = get_source_info(&self.token_string,self.position).unwrap().clone();

      self.advance();

        if pn.is_float()
        {
            match pn.try_to_f64()
            {
          Ok(f)=>
            {
              let  kind = ValueKind::Float(f);

              ParseResult::Some(vec![Value::new(info,kind)])
            }
          Err(_)=>{ParseResult::Err(info.to_error(format!("整数が不正")))}
            }
        }

      else
        {
            match pn.try_to_u64()
            {
          Ok(u)=>
            {
              let  kind = ValueKind::Uint(u);

              ParseResult::Some(vec![Value::new(info,kind)])
            }
          Err(_)=>{ParseResult::Err(info.to_error(format!("浮動小数点数が不正")))}
            }
        }
    }
  None=>{ParseResult::None}
    }
}


fn
read_by_string(&mut self, s: &str)-> ParseResult
{
  let  old_pos = self.position;

  let  info_opt = get_source_info(&self.token_string,self.position);

    if read_string_of_others(&self.token_string,&mut self.position,s)
    {
      let  kind = ValueKind::SemiString(s.to_string());

      ParseResult::Some(vec![Value::new(info_opt.unwrap(),kind)])
    }

  else
    {
      self.position = old_pos;

      ParseResult::None
    }
}


fn
read_by_identifier(&mut self, s: &str)-> ParseResult
{
    if let Some(def) = self.dictionary.find(s)
    {
        if self.depth >= 800
        {
          return ParseResult::Err(Error::new(format!("read_by_identifier: depth limit is over")));
        }


      self.read_by_definition(def)
    }

  else
    {ParseResult::Err(Error::new(format!("read_by_identifier: {}という定義はない",s)))}
}


fn
read_and(&mut self, l: &Expression, r: &Expression)-> ParseResult
{
  let  old_pos = self.position;

    match self.read_by_expression(l)
    {
  ParseResult::Some(mut l_vals)=>
    {
        match self.read_by_expression(r)
        {
      ParseResult::Some(mut r_vals)=>
        {
          l_vals.append(&mut r_vals);

          ParseResult::Some(l_vals)
        }
      ParseResult::None=>
        {
          self.position = old_pos;

          ParseResult::None
        }
      ParseResult::Err(e)=>{ParseResult::Err(e)}
        }
    }
  ParseResult::None=>{ParseResult::None}
  ParseResult::Err(e)=>{ParseResult::Err(e)}
    }
}


fn
read_or(&mut self, l: &Expression, r: &Expression)-> ParseResult
{
  let  old_pos = self.position;

    match self.read_by_expression(l)
    {
  ParseResult::Some(l_vals)=>{ParseResult::Some(l_vals)}
  ParseResult::None=>
    {
      self.position = old_pos;

        match self.read_by_expression(r)
        {
      ParseResult::Some(r_vals)=>{ParseResult::Some(r_vals)}
      ParseResult::None=>
        {
          self.position = old_pos;

          ParseResult::None
        }
      ParseResult::Err(e)=>{ParseResult::Err(e)}
        }
    }
  ParseResult::Err(e)=>{ParseResult::Err(e)}
    }
}


fn
read_arrow(&mut self, l: &Expression, r: &Expression)-> ParseResult
{
  let  old_pos = self.position;

    match self.read_by_expression(l)
    {
  ParseResult::Some(mut l_vals)=>
    {
        match self.read_by_expression(r)
        {
      ParseResult::Some(mut r_vals)=>
        {
          l_vals.append(&mut r_vals);

          ParseResult::Some(l_vals)
        }
      ParseResult::None=>{ParseResult::Err(Error::new(format!("解析失敗を確定")))}
      ParseResult::Err(e)=>{ParseResult::Err(e)}
        }
    }
  ParseResult::None=>{ParseResult::None}
  ParseResult::Err(e)=>{ParseResult::Err(e)}
    }
}


fn
read_by_binary_operation(&mut self, l: &Expression, r: &Expression, op: &str)-> ParseResult
{
    match op
    {
  (s) if s == "&" =>{self.read_and(  l,r)}
  (s) if s == "|" =>{self.read_or(   l,r)}
  (s) if s == "->"=>{self.read_arrow(l,r)}
  _=>{ParseResult::Err(Error::new(format!("不明な演算子 {}",op)))}
    }
}


pub fn
read_by_expression(&mut self, e: &Expression)-> ParseResult
{
    match e
    {
  Expression::Expression(e_e)=>{self.read_by_expression(e_e)}
  Expression::Option(op_e)=>
    {
        match self.read_by_expression(op_e)
        {
      ParseResult::Some(vals)=>{ParseResult::Some(vals)}
      ParseResult::None      =>{ParseResult::Some(vec![])}
      ParseResult::Err(e)    =>{ParseResult::Err(e)}
        }
    }
  Expression::Repetition(rep_e)=>{self.read_repetition(rep_e)}
  Expression::Identifier(s)=>{self.read_by_identifier(s)}
  Expression::String(s)    =>{self.read_by_string(s)}
  Expression::Keyword(s)   =>{self.read_keyword(s)}
  Expression::IdentifierLiteral=>
    {
        match get_token(&self.token_string,self.position)
        {
      Some(tok)=>
        {
            if let TokenKind::Identifier(s) = tok.get_kind()
            {
              let  info = tok.get_source_info().clone();
              let  kind = ValueKind::Identifier(s.clone());

              self.advance();

              ParseResult::Some(vec![Value::new(info,kind)])
            }

          else
            {ParseResult::None}
        }
      None=>{ParseResult::None}
        }
    }
  Expression::NumberLiteral=>{self.read_number_literal()}
  Expression::CharacterLiteral=>
    {
        match get_token(&self.token_string,self.position)
        {
      Some(tok)=>
        {
            if let TokenKind::Character(c) = tok.get_kind()
            {
              let  info = tok.get_source_info().clone();
              let  kind = ValueKind::Char(*c);

              self.advance();

              ParseResult::Some(vec![Value::new(info,kind)])
            }

          else
            {ParseResult::None}
        }
      None=>{ParseResult::None}
        }
    }
  Expression::StringLiteral=>
    {
        match get_token(&self.token_string,self.position)
        {
      Some(tok)=>
        {
            if let TokenKind::String(s) = tok.get_kind()
            {
              let  info = tok.get_source_info().clone();
              let  kind = ValueKind::String(s.clone());

              self.advance();

              ParseResult::Some(vec![Value::new(info,kind)])
            }

          else
            {ParseResult::None}
        }
      None=>{ParseResult::None}
        }
    }
  Expression::BinaryOperation(l,r,op)=>{self.read_by_binary_operation(&*l,&*r,op)}
    }
}


fn
read_by_definition(&mut self, def: &Definition)-> ParseResult
{
    if let Some(tok) = get_token(&self.token_string,self.position)
    {
      let  old_pos = self.position;

      self.depth += 1;

        match self.read_by_expression(def.get_expression())
        {
      ParseResult::Some(vals)=>
        {
          self.depth -= 1;

          let  info = tok.get_source_info().clone();

          let  mut nd = Node::new(info.clone(),&def.get_name());

          nd.add_value_list(vals);

          let  kind = ValueKind::Node(Box::new(nd));

          ParseResult::Some(vec![Value::new(info,kind)])
        }
      ParseResult::None=>
        {
          self.depth -= 1;

          self.position = old_pos;

          ParseResult::None
        }
      ParseResult::Err(e)=>{ParseResult::Err(e)}
        }
    }

  else
    {ParseResult::None}
}


}




pub fn
parse<'a>(toks: &Vec<Token>, dic: &'a Dictionary, main_def_name: &str)-> Result<Node,Error>
{
  let  mut nd = Node::new(SourceInfo::new(),"");

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
      ParseResult::Some(vals)=>
        {
            if st.position == prev_pos
            {
              break;
            }


          nd.add_value_list(vals);
        }
      ParseResult::None=>{break;}
      ParseResult::Err(e)=>{return Err(e);}
        }


      prev_pos = st.position;
    }


    if st.position >= toks.len()
    {
      Ok(nd)
    }

  else
    {
      let  mut buf = String::new();

      let  tok = &toks[st.position];

      buf.push_str("解析途中で停止");

      Err(tok.get_source_info().to_error(buf))
    }
}


pub fn
parse_from_string<'a>(s: &str, dic: &'a Dictionary, main_def_name: &str)-> Result<Node,Error>
{
  let  file = std::rc::Rc::new(SourceFile::from_string(s));

  let  mut r = SourceReader::new(&file);

    match r.read_token_string()
    {
  Ok(toks)=>
    {
//crate::token::print_token_string(&toks);
      let  stripped = strip_spaces(toks);

      parse(&stripped,dic,main_def_name)
    }
  Err(e)=>
    {
      let  message = format!("字句解析エラー: {}",&e.to_string());

      Err(Error::new(message))
    }
    }
}




