

use crate::source_file::{
  SourceFile,
  to_string,
};

use crate::token::{
  Token,
  TokenData,
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
  Operand,
  UnaryOperator,
  UnaryOperation,
  BinaryOperator,
  BinaryOperation,
  Expression,
  Definition,
  Dictionary,
};


pub fn
to_literal_operand(s: &str)-> Result<Operand,()>
{
       if s == "Identifier"{return Ok(Operand::IdentifierLiteral);}
  else if s ==    "Integer"{return Ok(Operand::IntegerLiteral   );}
  else if s ==   "Floating"{return Ok(Operand::FloatingLiteral  );}
  else if s ==  "Character"{return Ok(Operand::CharacterLiteral );}
  else if s ==     "String"{return Ok(Operand::StringLiteral    );}


  println!("{} is unknown literal keyword",s);

  Err(())
}


pub fn
read_operand_that_begins_others_token(toks: &Vec<Token>, pos: &mut usize, c: char)-> Result<Operand,()>
{
    match c
    {
  '('=>
        {
            if let Ok(e) = read_expression(toks,pos,')')
            {
              return Ok(Operand::One(Box::new(e)));
            }
        },
  '['=>
        {
            if let Ok(e) = read_expression(toks,pos,']')
            {
              return Ok(Operand::Option(Box::new(e)));
            }
        },
  '{'=>
        {
            if let Ok(e) = read_expression(toks,pos,'}')
            {
              return Ok(Operand::Repetition(Box::new(e)));
            }
        },
  '\''=>
        {
            if let Some(s) = get_identifier(toks,*pos)
            {
              advance(pos);

              return Ok(Operand::Keyword(s.clone()));
            }


          println!("keyword is missing");
        },
  '.'=>
        {
            if let Some(s) = get_identifier(toks,*pos)
            {
              advance(pos);

              return to_literal_operand(s.as_str());
            }


          println!("literal keyword is missing");
        },
    _=>{println!("unknown others element");},
    }


  Err(())
}


pub fn
read_operand(toks: &Vec<Token>, pos: &mut usize)-> Option<Operand>
{
    if let Some(tok) = get_token(toks,*pos)
    {
        match tok.get_data()
        {
      TokenData::Identifier(s)=>
            {
              let  mut name = String::new();

              let  mut d_name_opt: Option<String> = None;

              advance(pos);

                if read_string_of_others(toks,pos,"::")
                {
                    if let Some(last_name) = get_identifier(toks,*pos)
                    {
                      advance(pos);

                      name = last_name.clone();

                      d_name_opt = Some(s.clone());
                    }

                  else
                    {
                      println!("辞書名の後ろの識別子がない");

                      return None;
                    }
                }

              else
                {
                  name = s.clone();
                }


              let  o = Operand::Identifier(name,d_name_opt);

              return Some(o);
            },
      TokenData::String(s)=>
            {
              let  o = Operand::String(s.clone());

              advance(pos);

              return Some(o);
            },
      TokenData::Others(c)=>
            {
              advance(pos);

                if let Ok(o) = read_operand_that_begins_others_token(toks,pos,*c)
                {
                  return Some(o);
                }
            },
      _=>{println!("unknown operand element");},
        }
    }


  None
}


pub fn
read_unary_operation(toks: &Vec<Token>, pos: &mut usize)-> Option<UnaryOperation>
{
    if let Some(o) = read_operand(toks,pos)
    {
      let  unop = UnaryOperation{
                    operator: UnaryOperator::Nop,
                     operand: o
                  };

      return Some(unop);
    }


  None
}


pub fn
read_binary_operator(toks: &Vec<Token>, pos: &mut usize)-> Option<BinaryOperator>
{
    if read_string_of_others(toks,pos,"&")
    {
      return Some(BinaryOperator::And);
    }

  else
    if read_string_of_others(toks,pos,"|")
    {
      return Some(BinaryOperator::Or);
    }

  else
    if read_string_of_others(toks,pos,"->")
    {
      return Some(BinaryOperator::Arrow);
    }


  None
}


pub fn
read_closing(toks: &Vec<Token>, pos: &mut usize, closer: char)-> bool
{
    if let Some(c) = get_others(toks,*pos)
    {
        if c == closer
        {
          advance(pos);

          return true;
        }
    }


  false
}


pub fn
read_expression(toks: &Vec<Token>, pos: &mut usize, closer: char)-> Result<Expression,()>
{
    if read_closing(toks,pos,closer)
    {
      return Ok(Expression::Empty);
    }


    if let Some(first_o) = read_operand(toks,pos)
    {
        if read_closing(toks,pos,closer)
        {
          return Ok(Expression::Operand(first_o));
        }


      let  mut left_o = first_o;

        while let Some(bin_op) = read_binary_operator(toks,pos)
        {
            if let Some(right_o) = read_operand(toks,pos)
            {
              let  b = BinaryOperation{operator: bin_op, left: left_o, right: right_o };

              let  e = Expression::BinaryOperation(b);

                if read_closing(toks,pos,closer)
                {
                  return Ok(e);
                }


              left_o = Operand::One(Box::new(e));
            }

          else
            {
              println!("right operand is missing");

              break;
            }
        }
    }


  Err(())
}


pub fn
read_definition(toks: &Vec<Token>, pos: &mut usize)-> Option<Definition>
{
    if let Some(s) = get_identifier(toks,*pos)
    {
      let  mut def = Definition::new(s.as_str());

      advance(pos);

        if let Some(c) = get_others(toks,*pos)
        {
            if c == ':'
            {
              advance(pos);

                if let Ok(e) = read_expression(toks,pos,';')
                {
                  def.set_expression(e);

                  return Some(def);
                }
            }
        }
    }


  None
}


pub fn
read_dictionary(src: &SourceFile)-> Result<Dictionary,()>
{
  let  mut dic = Dictionary::new("");

    if let Ok(toks) = tokenize(src)
    {
      let  stripped = strip_spaces(toks);

      let  mut pos: usize = 0;

        if read_string_of_others(&stripped,&mut pos,"#")
        {
            if let Some(s) = get_identifier(&stripped,pos)
            {
              dic.name = s.clone();

              advance(&mut pos);
            }

          else
            {
              println!("読むべき辞書名がない");

              return Err(());
            }
        }


        while let Some(def) = read_definition(&stripped,&mut pos)
        {
          dic.add(def);
        }


      return Ok(dic);
    }


  Err(())
}




