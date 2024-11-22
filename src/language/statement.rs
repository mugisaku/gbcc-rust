

use super::expression::{
  Expression,
  AssignOperator,

};


use super::space::{
  VariableDecl,

};


use super::memory::{
  Memory,

};


use super::evaluator::{
  Instruction,

};


use super::symbol::{
  Symbol,
  SymbolDirectory,

};


use super::type_info::{
  Align,
  Parameter,
  TypeKind,
  TypeInfo,
  StorageInfo,

};




pub enum
Statement
{
  Empty,
  Let(VariableDecl),
  Const(VariableDecl),
  Expression(Expression,Option<(AssignOperator,Expression)>),
  If(Vec<(Expression,Block)>,Option<Block>),
  While(Expression,Block),
  For(For),
  Loop(Block),
  Block(Block),
  Return(Option<Expression>),
  Break,
  Continue,
  PrintS(String),
  PrintV(String),

}


impl
Statement
{


pub fn
print(&self)
{
    match self
    {
  Statement::Empty=>{print!(";");}
  Statement::Let(v)=>
        {
          print!("let  ");
          v.print();
        }
  Statement::Const(v)=>
        {
          print!("const  ");

          v.print();
        }
  Statement::Expression(e,ass_opt)=>
        {
          e.print();

            if let Some((ass_op,r)) = ass_opt
            {
              ass_op.print();

              r.print();
            }
        }
  Statement::If(ls,blk_opt)=>
       {
            if let Some((first_e,first_blk)) = ls.first()
            {
              print!("if ");

              first_e.print();

              first_blk.print();

              print!("\n");

                for i in 1..ls.len()
                {
                  let  (e,blk) = &ls[i];

                  print!("else if ");

                  e.print();

                  blk.print();

                  print!("\n");
                }
            }


            if let Some(blk) = blk_opt
            {
              print!("else");

              blk.print();
            }
        }
  Statement::For(fo)=>
        {
          print!("for {} in ",&fo.current_var.name);

          fo.get_end_expression().print();

          fo.block.print();
        }
  Statement::While(e,blk)=>
        {
          print!("while ");

          e.print();

          blk.print();
        }
  Statement::Loop(blk)=>
        {
          print!("loop");

          blk.print();
        }
  Statement::Block(blk)=>
        {
          print!("//plain block");

          blk.print();
        }
  Statement::Return(e_opt)=>
        {
          print!("return ");

            if let Some(e) = e_opt
            {
              e.print();
            }
        }
  Statement::Break=>{print!("break");}
  Statement::Continue=>{print!("continue");}
  Statement::PrintS(s)=>{print!("print \"{}\"",s);}
  Statement::PrintV(s)=>{print!("print {}",s);}
    }
}


}




pub struct
Block
{
  pub(crate) statement_list: Vec<Statement>,

}


impl
Block
{


pub fn
new(statement_list: Vec<Statement>)-> Self
{
  Self{
    statement_list,
  }
}


pub fn
print(&self)
{
  print!("{{\n");

    for stmt in &self.statement_list
    {
      stmt.print();

      print!("\n");
    }


  print!("}}\n");
}


}




pub struct
For
{
  pub(crate) current_var: VariableDecl,
  pub(crate)     end_var: VariableDecl,

  pub(crate) block: Block,

}


impl
For
{


pub fn
new(name: String, expression: Expression, block: Block)-> Self
{
  Self{
    current_var: VariableDecl{name, type_kind: TypeKind::Unknown, expression: Expression::Void},
        end_var: VariableDecl{name: "**FOR_END_VAR**".to_string(), type_kind: TypeKind::Unknown, expression},
    block,
  }
}


pub fn
get_end_expression(&self)-> &Expression
{
  &self.end_var.expression
}


}




