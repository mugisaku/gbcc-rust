

use crate::token::{
  Token,
  TokenInfo,
  TokenData,
  get_token,
  get_integer,
  get_floating,
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
  BinaryOperation,
  BinaryOperator,
  Operand,
};

use super::{
  Directory,
  Object,
  ObjectData,
};


struct
Status<'a,'b>
{
  dictionary_list : Vec<&'a Dictionary>,
  dictionary_stack: Vec<&'a Dictionary>,

  token_string: &'b Vec<Token>,

  position: usize,

  depth: usize,

  interruption: bool,

}


impl<'a,'b>
Status<'a,'b>
{


fn
open_dictionary(&mut self, name: &str)-> Result<(),()>
{
    for dic in &self.dictionary_list
    {
        if dic.name == name
        {
          self.dictionary_stack.push(dic);

          return Ok(());
        }
    }


  println!("Status::open_directory error: dictionary {} is not found",name);

  Err(())
}


fn
close_dictionary(&mut self, name: &str)-> Result<(),()>
{
    if let Some(dic) = self.dictionary_stack.pop()
    {
        if dic.name == name
        {
          return Ok(());
        }
    }


  println!("Status::close_directory error: opened dictionary {} is none",name);

  Err(())
}


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
read_by_string(&mut self, s: &str)-> Option<Vec<Object>>
{
  let  old_pos = self.position;

    if read_string_of_others(&self.token_string,&mut self.position,s)
    {
      let  o = Object{token_info: Some(self.token_string[old_pos].get_info().clone()), data: ObjectData::OthersString(String::from(s))};

      return Some(vec![o]);
    }


  self.position = old_pos;

  None
}


fn
read_by_identifier(&mut self, id: &str, d_name_opt: &Option<String>)-> Option<Vec<Object>>
{
   if let Some(d_name) = d_name_opt
   {
       if self.open_dictionary(d_name).is_err()
       {
         return None;
       }
   }


  let  mut objs_opt: Option<Vec<Object>> = None;

   if let Some(objs) = self.read_by_name(id)
   {
     objs_opt = Some(objs);
   }


   if let Some(d_name) = d_name_opt
   {
       if self.close_dictionary(d_name).is_err()
       {
         return None;
       }
   }


  objs_opt
}


fn
read_by_operand(&mut self, o: &Operand)-> Option<Vec<Object>>
{
  let  old_pos = self.position;

    match o
    {
  Operand::One(e)=>
        {
            if let Some(objs) = self.read_by_expression(e)
            {
              return Some(objs);
            }
        },
  Operand::Option(e)=>
        {
            if let Some(objs) = self.read_by_expression(e)
            {
              return Some(objs);
            }


            if !self.interruption
            {
              return Some(vec![]);
            }
        },
  Operand::Repetition(e)=>
        {
            if let Some(mut first_objs) = self.read_by_expression(e)
            {
                while let Some(mut objs) = self.read_by_expression(e)
                {
                  first_objs.append(&mut objs);
                }


                if !self.interruption
                {
                  return Some(first_objs);
                }
            }
        },
  Operand::Identifier(s,d_name_opt)=>
        {
          return self.read_by_identifier(s,d_name_opt);
        },
  Operand::String(s)=>
        {
            if let Some(objs) = self.read_by_string(s)
            {
              return Some(objs);
            }
        },
  Operand::Keyword(kw)=>
        {
            if let Some(id) = get_identifier(&self.token_string,self.position)
            {
                if kw == id
                {
                  let  o = Object{token_info: Some(self.get_token_info()), data: ObjectData::Keyword(kw.clone())};

                  self.position += 1;

                  return Some(vec![o]);
                }
            }
        },
  Operand::IdentifierLiteral=>
        {
            if let Some(s) = get_identifier(&self.token_string,self.position)
            {
              let  o = Object{token_info: Some(self.get_token_info()), data: ObjectData::Identifier(s.clone())};

              self.advance();

              return Some(vec![o]);
            }
        },
  Operand::IntegerLiteral=>
        {
            if let Some(i) = get_integer(&self.token_string,self.position)
            {
              let  o = Object{token_info: Some(self.get_token_info()), data: ObjectData::Integer(i)};

              self.advance();

              return Some(vec![o]);
            }
        },
  Operand::FloatingLiteral=>
        {
            if let Some(f) = get_floating(&self.token_string,self.position)
            {
              let  o = Object{token_info: Some(self.get_token_info()), data: ObjectData::Floating(f)};

              self.advance();

              return Some(vec![o]);
            }
        },
  Operand::CharacterLiteral=>
        {
            if let Some(c) = get_character(&self.token_string,self.position)
            {
              let  o = Object{token_info: Some(self.get_token_info()), data: ObjectData::Character(c)};

              self.advance();

              return Some(vec![o]);
            }
        },
  Operand::StringLiteral=>
        {
            if let Some(s) = get_string(&self.token_string,self.position)
            {
              let  o = Object{token_info: Some(self.get_token_info()), data: ObjectData::String(s.clone())};

              self.advance();

              return Some(vec![o]);
            }
        },
    }


  self.position = old_pos;

  None
}


fn
read_by_binary_operation(&mut self, op: &BinaryOperation)-> Option<Vec<Object>>
{
  let  old_pos = self.position;

    match &op.operator
    {
  BinaryOperator::And=>
        {
            if let Some(mut l_objs) = self.read_by_operand(op.get_left())
            {
                if let Some(mut r_objs) = self.read_by_operand(op.get_right())
                {
                  l_objs.append(&mut r_objs);

                  return Some(l_objs);
                }
            }
        },
  BinaryOperator::Or=>
        {
            if let Some(pac) = self.read_by_operand(op.get_left())
            {
              return Some(pac);
            }


            if !self.interruption
            {
              self.position = old_pos;

                if let Some(pac) = self.read_by_operand(op.get_right())
                {
                  return Some(pac);
                }
            }
        },
  BinaryOperator::Arrow=>
        {
            if let Some(mut l_objs) = self.read_by_operand(op.get_left())
            {
                if let Some(mut r_objs) = self.read_by_operand(op.get_right())
                {
                  l_objs.append(&mut r_objs);

                  return Some(l_objs);
                }


              println!("確定構文の解析が失敗した");

              self.interruption = true;
            }
        },
    }


  self.position = old_pos;

  None
}


pub fn
read_by_expression(&mut self, e: &Expression)-> Option<Vec<Object>>
{
  let  old_pos = self.position;

    match e
    {
  Expression::Empty=>{},
  Expression::UnaryOperation(op)=>
        {
            if let Some(objs) = self.read_by_operand(&op.operand)
            {
              return Some(objs);
            }
        },
  Expression::BinaryOperation(op)=>
        {
            if let Some(objs) = self.read_by_binary_operation(op)
            {
              return Some(objs);
            }
        },
  Expression::Operand(o)=>
        {
            if let Some(objs) = self.read_by_operand(o)
            {
              return Some(objs);
            }
        },
    }


  self.position = old_pos;

  None
}


fn
read_by_definition(&mut self, def: &Definition)-> Option<Vec<Object>>
{
    if let Some(tok) = get_token(&self.token_string,self.position)
    {
      let  old_pos = self.position;

//      self.print_indent();

//      println!("{}としての解析を開始({})",&def.name,self.dictionary_stack.len());

      self.depth += 1;

        if let Some(objs) = self.read_by_expression(def.get_expression())
        {
          let  dir = Directory{ name: def.name.clone(), object_list: objs};

          let  obj = Object{ token_info: None, data: ObjectData::Directory(dir)};

          self.depth -= 1;

//          self.print_indent();

//          println!("{}としての解析に成功",&def.name);

          return Some(vec![obj]);
        }


      self.depth -= 1;

//      self.print_indent();

//      println!("{}としての解析に失敗",&def.name);

      self.position = old_pos;
    }


  None
}


fn
read_by_name(&mut self, name: &str)-> Option<Vec<Object>>
{
    if let Some(dic) = self.dictionary_stack.last()
    {
        if let Some(def) = dic.find(name)
        {
            if self.depth >= 800
            {
              println!("read_by_name: depth limit is over");

              return None;
            }


          return self.read_by_definition(def);
        }
    }


  None
}


}




pub fn
parse<'a>(toks: &Vec<Token>, dic: &'a Dictionary, main_def_name: &str, dics_opt: Option<Vec<&'a Dictionary>>)-> Result<Directory,()>
{
  let  mut dir = Directory::new("/");

    if let Some(main_def) = dic.find(main_def_name) 
    {
      let  mut st = Status{
                      dictionary_list : Vec::new(),
                      dictionary_stack: Vec::new(),
                      token_string: toks,
                      position: 0,
                      depth: 0,
                      interruption: false,
                    };


      st.dictionary_list.push(&dic);

        if let Some(mut dics) = dics_opt
        {
          st.dictionary_list.append(&mut dics);
        }


      st.dictionary_stack.push(dic);

      let  mut prev_pos: usize = 0;

        while let Some(mut objs) = st.read_by_definition(main_def)
        {
          dir.object_list.append(&mut objs);

            if st.position == prev_pos
            {
              println!("parse is stopped");

              break;
            }


          prev_pos = st.position;

          println!("\n--\n");
        }

      println!("parse is end");


        if st.interruption
        {
          println!("parse is interrupted");

          return Err(());
        }


        if st.position < toks.len()
        {
          println!("there are remained some unparsed tokens.");

           for i in st.position..toks.len()
           {
             print!("{}: ",i);

             toks[i].print();

             println!();
           }


          println!("{} parsed",dir.object_list.len());

          return Err(());
        }
    }

  else
    {
      println!("{} as main definition is nout found.",main_def_name);

      return Err(());
    }


  Ok(dir)
}


pub fn
parse_from_string<'a>(s: &str, dic: &'a Dictionary, main_def_name: &str, dics_opt: Option<Vec<&'a Dictionary>>)-> Result<Directory,()>
{
    if let Ok(toks) = tokenize_from_string(s)
    {
//crate::token::print_token_string(&toks);
      let  stripped = strip_spaces(toks);

      return parse(&stripped,dic,main_def_name,dics_opt);
    }


  Err(())
}




