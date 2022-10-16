

use std::rc::Rc;
use crate::source_file::SourceFile;
use crate::token::Token;
use crate::token::TokenData;
use crate::token::Cursor;
use crate::tokenizer::tokenize;

pub type TokenString = Vec<Token>;


pub enum
Expression
{
  Empty,
  UnaryOperation( Box<UnaryOperation>),
  BinaryOperation(Box<BinaryOperation>),
  Primary(        Box<PrimaryExpression>),

}


impl
Expression
{


pub fn
print(&self)
{
    match self
    {
  Expression::Empty=>{print!("EMPTY");},
  Expression::UnaryOperation(o)=>
        {
            match o.operator
            {
          UnaryOperator::Nop=>
                {
                },
          UnaryOperator::Not=>
                {
                  print!("!");
                },
            }


          o.operand.print();
        },
  Expression::BinaryOperation(o)=>
        {
          o.left.print();

            match o.operator
            {
          BinaryOperator::And=>
                {
                  print!(" & ");
                },
          BinaryOperator::Or=>
                {
                  print!(" | ");
                },
            }


          o.right.print();
        },
  Expression::Primary(pr)=>
        {
            match &**pr
            {
          PrimaryExpression::None=>
                {
                  print!("NONE");
                },
          PrimaryExpression::One(e)=>
                {
                  print!("(");
                  e.print();
                  print!(")");
                },
          PrimaryExpression::Option(e)=>
                {
                  print!("[");
                  e.print();
                  print!("]");
                },
          PrimaryExpression::Repetition(e)=>
                {
                  print!("{}","{");
                  e.print();
                  print!("{}","}");
                },
          PrimaryExpression::Identifier(s)=>{print!("{}",s);},
          PrimaryExpression::String(s)=>{print!("\"{}\"",&*s);},
            }
        },
    }
}


}




pub enum
PrimaryExpression
{
  None,
  One(       Expression),
  Option(    Expression),
  Repetition(Expression),

  Identifier(Rc<String>),
  String(Rc<String>),

}


pub enum
UnaryOperator
{
  Nop,
  Not,

}


pub struct
UnaryOperation
{
  operator: UnaryOperator,
  operand: Expression,

}


impl
UnaryOperation
{


pub fn
get_operator(&self)-> &UnaryOperator
{
  &self.operator
}


pub fn
get_operand(&self)-> &Expression
{
  &self.operand
}


}


pub enum
BinaryOperator
{
  And,
  Or,

}


pub struct
BinaryOperation
{
  operator: BinaryOperator,

   left: Expression,
  right: Expression,

}


impl
BinaryOperation
{


pub fn
get_operator(&self)-> &BinaryOperator
{
  &self.operator
}


pub fn
get_left(&self)-> &Expression
{
  &self.left
}


pub fn
get_right(&self)-> &Expression
{
  &self.right
}


}


pub struct
Definition
{
  name: Rc<String>,

  expression: Expression,

}


impl
Definition
{


pub fn
new(name: &str)-> Definition
{
  Definition{ name: Rc::new(String::from(name)), expression: Expression::Empty}
}


pub fn
get_name(&self)-> &String
{
  &self.name
}


pub fn
clone_name(&self)-> Rc<String>
{
  self.name.clone()
}


pub fn
get_expression(&self)-> &Expression
{
  &self.expression
}


pub fn
set_expression(&mut self, expr: Expression)
{
  self.expression = expr;
}


pub fn
print(&self)
{
  print!("{}: ",self.name);

  self.expression.print();

  print!(";");
}


}




pub struct
Dictionary
{
  definition_list: Vec<Definition>,

}


impl
Dictionary
{


pub fn
open(filepath: &str)-> Dictionary
{
  let  src = SourceFile::open(filepath);

  Self::from(&src)
}


pub fn
from(src: &SourceFile)-> Dictionary
{
  let mut  dic = Dictionary{definition_list: Vec::new()};

  dic.read_source_file(src);

  dic
}




pub fn
get_first(&self)-> Option<&Definition>
{
  self.definition_list.first()
}


pub fn
find(&self, name: &str)-> Option<&Definition>
{
    for def in &self.definition_list
    {
        if def.get_name() == name
        {
          return Some(def);
        }
    }


  None
}


pub fn
test_expression(&self, e: &Expression)
{
    match e
    {
  Expression::Empty=>{},
  Expression::UnaryOperation(op)=>{self.test_expression(&op.operand);},
  Expression::BinaryOperation(op)=>
        {
          self.test_expression(&op.left );
          self.test_expression(&op.right);
        },
  Expression::Primary(pr)=>
        {
            match &**pr
            {
          PrimaryExpression::One(ee)=>      {self.test_expression(ee);},
          PrimaryExpression::Option(ee)=>   {self.test_expression(ee);},
          PrimaryExpression::Repetition(ee)=>{self.test_expression(ee);},
          PrimaryExpression::Identifier(s)=>
                {
                    if (**s != "IDENTIFIER") &&
                       (**s !=   "INTEGER_LITERAL") &&
                       (**s !=  "FLOATING_LITERAL") &&
                       (**s !=    "LETTER_LITERAL") &&
                       (**s != "CHARACTER_LITERAL") &&
                       (**s !=    "STRING_LITERAL")
                    {
                        if let None = self.find(&*s)
                        {
                          print!("definition <{}> not found.\n",s);
                        }
                    }
                },
          PrimaryExpression::String(_)=>{},
          PrimaryExpression::None=>{},
            }
        }
    }
}


pub fn
test(&self)
{
    for def in &self.definition_list
    {
      self.test_expression(def.get_expression());
    }
}




pub fn
read_packed_expression(cur: &mut Cursor, opener: char)-> Option<PrimaryExpression>
{
    match opener
    {
  '('=>
        {
            if let Some(e) = Self::read_expression(cur,')')
            {
              return Some(PrimaryExpression::One(e));
            }
        },
  '['=>
        {
            if let Some(e) = Self::read_expression(cur,']')
            {
              return Some(PrimaryExpression::Option(e));
            }
        },
  '{'=>
        {
            if let Some(e) = Self::read_expression(cur,'}')
            {
              return Some(PrimaryExpression::Repetition(e));
            }
        },
    _=>{},
    }


  None
}


pub fn
read_primary_expression(cur: &mut Cursor)-> Option<PrimaryExpression>
{
  cur.skip_spaces();

    if let Some(tok) = cur.get()
    {
        match tok.get_data()
        {
      TokenData::Identifier(s)=>
            {
              let  o = PrimaryExpression::Identifier(s.clone());

              cur.advance();

              return Some(o);
            },
      TokenData::String(s)=>
            {
              let  o = PrimaryExpression::String(s.clone());

              cur.advance();

              return Some(o);
            },
      TokenData::Others(c)=>
            {
              let  opener = *c;

              cur.advance();

              return Self::read_packed_expression(cur,opener);
            },
      _=>{},
        }
    }


  None
}


pub fn
read_unary_operation(cur: &mut Cursor)-> Option<UnaryOperation>
{
    if let Some(p) = Self::read_primary_expression(cur)
    {
      let  unop = UnaryOperation{
                    operator: UnaryOperator::Nop,
                     operand: Expression::Primary(Box::new(p))
                  };

      return Some(unop);
    }


  None
}


pub fn
read_binary_operator(cur: &mut Cursor)-> Option<BinaryOperator>
{
  cur.skip_spaces();

    if let Some(c) = cur.get_others()
    {
        match c
        {
      '&'=>
            {
              cur.advance();

              return Some(BinaryOperator::And);
            },
      '|'=>
            {
              cur.advance();

              return Some(BinaryOperator::Or);
            },
      _=>{},
        }
    }


  None
}


pub fn
read_binary_operation_element(cur: &mut Cursor)-> Option<(BinaryOperator,Expression)>
{
  cur.skip_spaces();

    if let Some(operator) = Self::read_binary_operator(cur)
    {
        if let Some(operation) = Self::read_unary_operation(cur)
        {
          let  boxed = Box::new(operation);

          return Some((operator,Expression::UnaryOperation(boxed)));
        }
    }


  None
}


pub fn
test_closing(cur: &Cursor, closer: char)-> bool
{
    if let Some(tok) = cur.get()
    {
        if let Some(c) = tok.get_others()
        {
          return c == closer;
        }
    }


  false
}


pub fn
test_illegal_closing(cur: &Cursor)-> bool
{
    if let Some(tok) = cur.get()
    {
        if let Some(c) = tok.get_others()
        {
            if (c == ')') ||
               (c == ']') ||
               (c == '}')
            {
              return true;
            }
        }
    }


  false
}


pub fn
read_expression(cur: &mut Cursor, closer: char)-> Option<Expression>
{
  cur.skip_spaces();

    if Self::test_closing(cur,closer)
    {
      cur.advance();

      return Some(Expression::Empty);
    }


    if Self::test_illegal_closing(cur)
    {
      return None;
    }


    if let Some(first) = Self::read_unary_operation(cur)
    {
      let mut  e = Expression::UnaryOperation(Box::new(first));

      cur.skip_spaces();

        if Self::test_closing(cur,closer)
        {
          cur.advance();

          return Some(e);
        }


        if Self::test_illegal_closing(cur)
        {
          return None;
        }


        while let Some((operator,operation)) = Self::read_binary_operation_element(cur)
        {
          let  boxed = Box::new(BinaryOperation{ operator, left: e, right: operation});

          e = Expression::BinaryOperation(boxed);

          cur.skip_spaces();

            if Self::test_closing(cur,closer)
            {
              cur.advance();

              return Some(e);
            }
        }
    }


  None
}


pub fn
read_definition(cur: &mut Cursor)-> Option<Definition>
{
  cur.skip_spaces();

    if let Some(s) = cur.get_identifier()
    {
      let mut  def = Definition::new(s);

      cur.advance();
      cur.skip_spaces();

        if let Some(c) = cur.get_others()
        {
            if c == '='
            {
              cur.advance();

                if let Some(e) = Self::read_expression(cur,';')
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
read_source_file(&mut self, src: &SourceFile)
{
    if let Ok(toks) = tokenize(src)
    {
      let mut  cur = Cursor::from(&toks);

        while let Some(def) = Self::read_definition(&mut cur)
        {
          self.definition_list.push(def);
        }
    }
}




pub fn
print(&self)
{
  self.test();

    for def in &self.definition_list
    {
      def.print();

      print!("\n");
    }
}




}




