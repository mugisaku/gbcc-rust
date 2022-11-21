

use std::rc::Rc;
use super::expression::Expression;
use super::space::VariableDeclaration;
use crate::syntax::parser::ObjectData;
use crate::syntax::parser::Directory;
use crate::syntax::parser::Cursor;

pub enum
Statement
{
  Empty,
  Block(Block),
  If(Vec<Block>),
  While(Block),
  Break,
  Continue,
  Return(Option<Expression>),
  Expression(Expression),
  VariableDeclaration(VariableDeclaration),

}


impl
Statement
{


pub fn
read_return(dir: &Directory)-> Statement
{
  let mut  cur = Cursor::from(dir);

    if let Some(d) = cur.seek_directory("expression")
    {
      let  e = Expression::from(d);

      return Statement::Return(Some(e));
    }


  Statement::Return(None)
}


pub fn
read_if(dir: &Directory)-> Statement
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

    if let Some(d) = cur.get_directory_with_name("block_statement")
    {
    }


  Statement::Return(None)
}


pub fn
read_while(dir: &Directory)-> Statement
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

    if let Some(expr_d) = cur.get_directory_with_name("expression")
    {
      let  e = Expression::from(expr_d);

      cur.advance(1);

        if let Some(blk_d) = cur.get_directory_with_name("block_statement")
        {
          let mut  blk = Block::from(blk_d);

          blk.set_condition(e);

          return Statement::While(blk);
        }
    }


  Statement::Empty
}


pub fn
from(dir: &Directory)-> Statement
{
  let mut  cur = Cursor::from(dir);

    if let Some(d) = cur.get_directory()
    {
      let  d_name = d.get_name();

        if d.get_name() == "if_statement"
        {
          return Self::read_if(d);
        }

      else
        if d.get_name() == "block_statement"
        {
          return Statement::Block(Block::from(d));
        }

      else
        if d.get_name() == "while_statement"
        {
          return Self::read_while(d);
        }

      else
        if d.get_name() == "break_statement"
        {
          return Statement::Break;
        }

      else
        if d.get_name() == "continue_statement"
        {
          return Statement::Continue;
        }

      else
        if d.get_name() == "return_statement"
        {
          return Self::read_return(d);
        }


      cur.advance(1);
    }


  Statement::Empty
}


pub fn
print(&self)
{
    match self
    {
  Statement::Empty=>{print!("__EMPTY_STATEMENT__");},
  Statement::Block(blk)=>{blk.print();},
  Statement::If(blks)=>
        {
          print!("if ");

          let mut  it = blks.iter();

            if let Some(first_blk) = it.next()
            {
              first_blk.print();

                while let Some(blk) = it.next()
                {
                  print!("else");

                    if let Some(cond) = &blk.condition
                    {
                      print!("if ");
                    }


                  blk.print();
                }
            }
        },
  Statement::While(blk)=>
        {
          print!("while ");

          blk.print();
        },
  Statement::Break=>{print!("break");},
  Statement::Continue=>{print!("continue");},
  Statement::Return(op_e)=>
        {
          print!("return ");

            if let Some(e) = op_e
            {
              e.print();
            }
        },
  Statement::Expression(e)=>{e.print();},
  Statement::VariableDeclaration(vd)=>{vd.print();},
    }
}


}




pub struct
Block
{
  condition: Option<Expression>,

  statements: Vec<Statement>,

}


impl
Block
{


pub fn
new()-> Block
{
  Block{ condition: None, statements: Vec::new()}
}


pub fn
from(dir: &Directory)-> Block
{
  let mut  cur = Cursor::from(dir);

  let mut  stmts: Vec<Statement> = Vec::new();

    while let Some(o) = cur.get()
    {
        if let ObjectData::Directory(d) = o.get_data()
        {
          let  d_name = d.get_name();

            if d_name == "statement"
            {
              stmts.push(Statement::from(d));
            }

          else
            if d_name == "variable_declartion"
            {
              let  vd = VariableDeclaration::from(d);

              stmts.push(Statement::VariableDeclaration(vd));
            }

          else
            if d_name == "expression"
            {
              let  e = Expression::from(d);

              stmts.push(Statement::Expression(e));
            }
        }

      cur.advance(1);
    }


  Block{ condition: None, statements: stmts}
}


pub fn
set_condition(&mut self, e: Expression)
{
  self.condition = Some(e);
}


pub fn
get_condition(&self)-> &Option<Expression>
{
  &self.condition
}


pub fn
get_statements(&self)-> &Vec<Statement>
{
  &self.statements
}


pub fn
print(&self)
{
    if let Some(cond) = &self.condition
    {
      cond.print();
    }


  print!("{}\n","{");

    for stmt in &self.statements
    {
      stmt.print();

      print!("\n");
    }


  print!("{}\n","}");
}


}





