

use super::expression::{
  OpId,
  Expression,

};


use super::ty::{
  Type,

};


use super::declaration::{
  Declaration,

};




pub enum
Statement
{
  Empty,
  Declaration(Declaration),
  Expression(Expression),
  Assign(OpId,Expression,Expression),
  If(Branch),
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




/*
pub fn
read(s: &str)-> Self
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"statement",Some(dics))
    {
      let  mut cur = crate::syntax::Cursor::new(&dir);

        if let Some(d_dir) = cur.get_directory_with_name("statement")
        {
          return super::read::read_statement(d_dir);
        }
    }


  panic!();
}
*/


pub fn
print(&self)
{
    match self
    {
  Statement::Empty=>{print!(";");}
  Statement::Declaration(decl)=>
    {
      decl.print();
    }
  Statement::Expression(e)=>
    {
      e.print();
    }
  Statement::Assign(o,l,r)=>
    {
      l.print();
      o.print();
      r.print();
    }
  Statement::If(br)=>
    {
      br.print();
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
Branch
{
  pub(crate) expression_opt: Option<Expression>,

  pub(crate) block: Block,

  pub(crate) sub_branch_opt: Option<Box<Branch>>,

}


impl
Branch
{


pub fn
print(&self)
{
    if let Some(e) = &self.expression_opt
    {
      print!("if ");

      e.print();

      self.block.print();

        if let Some(br) = &self.sub_branch_opt
        {
          print!("else");

          br.print();
        }
    }
}


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




