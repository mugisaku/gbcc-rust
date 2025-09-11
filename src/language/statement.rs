

use super::expression::{
  Expression,
  AssignOperator,

};


use super::element::{
  Symbol,

};




pub enum
Statement
{
  Empty,
  Let(Symbol),
  Const(Symbol),
  Static(Symbol),
  Expression(Expression,Option<(AssignOperator,Expression)>),
  If(IfBranch),
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
  Statement::Static(v)=>
        {
          print!("static  ");

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
  Statement::If(br)=>
       {
         print!("if ");

         br.expression.print();

         print!(" ");

         br.on_true.print();

           if let Statement::Empty = &*br.on_false
           {
           }

         else
           {
             print!("else");

             br.on_false.print();
           }
        }
  Statement::For(fo)=>
        {
          print!("for {} in ",&fo.var_name);

          fo.expression.print();

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
  pub(crate) name: String,

  pub(crate) statement_list: Vec<Statement>,

}


impl
Block
{


pub fn
new(name: String, statement_list: Vec<Statement>)-> Self
{
  Self{
    name,
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
IfBranch
{
  pub(crate) expression: Expression,

  pub(crate)   on_true: Box<Statement>,
  pub(crate)  on_false: Box<Statement>,

}


pub struct
For
{
  pub(crate) var_name: String,
  pub(crate) expression: Expression,

  pub(crate) block: Block,

}


impl
For
{


pub fn
new(var_name: String, expression: Expression, block: Block)-> Self
{
  Self{
    var_name,
    expression,
    block,
  }
}


}




