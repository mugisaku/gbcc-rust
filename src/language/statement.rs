

use crate::syntax::print_indent;

use super::get_aligned_size;
use super::declaration::{
  Declaration,

};

use super::expression::{
  Expression,
  ExpressionKeeper,

};


#[derive(Clone)]
pub enum
AssignOperator
{
  Nop,
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Shl,
  Shr,
  And,
  Or,
  Xor,

}


impl
AssignOperator
{


pub fn
print(&self)
{
    match self
    {
  AssignOperator::Nop=>{print!("=");},
  AssignOperator::Add=>{print!("+=");},
  AssignOperator::Sub=>{print!("-=");},
  AssignOperator::Mul=>{print!("*=");},
  AssignOperator::Div=>{print!("/=");},
  AssignOperator::Rem=>{print!("%=");},
  AssignOperator::Shl=>{print!("<<=");},
  AssignOperator::Shr=>{print!(">>=");},
  AssignOperator::And=>{print!("&=");},
  AssignOperator::Or=>{print!("|=");},
  AssignOperator::Xor=>{print!("^=");},
    }
}


}




pub enum
Statement
{
  Empty,
  Declaration(Declaration),
  Block(Block),
  Break,
  Continue,
  Return(Option<ExpressionKeeper>),
  Expression(ExpressionKeeper,Option<(AssignOperator,ExpressionKeeper)>),

}


impl
Statement
{


pub fn
print(&self, indent: usize)
{
  print_indent(indent);

    match self
    {
  Statement::Empty=>{},
  Statement::Declaration(d)=>{d.print();},
  Statement::Block(b)=>{b.print(indent);},
  Statement::Break=>{print!("break");},
  Statement::Continue=>{print!("continue");},
  Statement::Return(ek_opt)=>
        {
          print!("return ");

            if let Some(ek) = ek_opt
            {
              ek.expression.print();
            }
        },
  Statement::Expression(ek,ass_opt)=>
        {
          ek.expression.print();

            if let Some((o,r)) = ass_opt
            {
              o.print();

              r.expression.print();
            }
        },
    }
}


}




pub fn
print_statement_list(ls: &Vec<Statement>, indent: usize)
{
    for stmt in ls
    {
      stmt.print(indent);

      print!(";\n");
    }
}




pub enum
Block
{
  Plain(Vec<Statement>),
  IfList(Vec<Block>),
  If(ExpressionKeeper,Vec<Statement>),
  While(ExpressionKeeper,Vec<Statement>),
  Loop(Vec<Statement>),
  For,

}


impl
Block
{


pub fn
print(&self, indent: usize)
{
    match self
    {
  Block::Plain(ls)=>
        {
          print!("\n");

          print_indent(indent);

          print!("{{\n");

          print_statement_list(ls,indent+1);
        },
  Block::IfList(ls)=>
        {
            if let Some(top) = ls.first()
            {
              top.print(indent);

                for i in 1..ls.len()
                {
                  print!(" else ");

                  ls[i].print(indent);
                }
            }


          return;
        },
  Block::If(cond,ls)=>
        {
          print!("if ");

          cond.expression.print();

          print!("\n");

          print_indent(indent);

          print!("{{\n");

          print_statement_list(ls,indent+1);
        },
  Block::For=>{},
  Block::While(cond,ls)=>
        {
          print!("while ");

          cond.expression.print();

          print!("\n");

          print_indent(indent);

          print!("{{\n");

          print_statement_list(&ls,indent+1);
        },
  Block::Loop(ls)=>
        {
          print!("loop\n");

          print_indent(indent);

          print!("{{\n");

          print_statement_list(&ls,indent+1);
        },
    }


  print_indent(indent);

  print!("}}\n");
}


}




