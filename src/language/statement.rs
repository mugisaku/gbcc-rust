

pub mod read_statement;
pub mod dictionary;

use super::expression::Expression;
use super::typesystem::{
  TypeNote,
  r#struct::Struct,
  r#union::Union,
  r#enum::Enum,
  function_signature::FunctionSignature,

};


pub struct
Var
{
  pub(crate) type_note: TypeNote,

  pub(crate) expression: Option<Expression>,

}


pub struct
Fn
{
  pub(crate) signature: FunctionSignature,

  pub(crate) block: Block,

}


pub enum
Definition
{
  Fn(Fn),
  Var(Var),
  Struct(Struct),
  Enum(Enum),
  Union(Union),

}




pub struct
Declaration
{
  pub(crate) name: String,

  pub(crate) definition: Definition,

}


impl
Declaration
{


pub fn
new(name: &str, def: Definition)-> Declaration
{
  Declaration{name: String::from(name), definition: def}
}


pub fn
print(&self)
{
    match &self.definition
    {
  Definition::Fn(f)=>
        {
          print!("fn\n{}",&self.name);
          f.signature.print();
          print!("\n");
          f.block.print();
        },
  Definition::Var(v)=>
        {
          print!("var\n{}: ",&self.name);
          v.type_note.print();
        },
  Definition::Struct(st)=>
        {
          print!("struct\n{}",&self.name);

          st.print();
        },
  Definition::Union(un)=>
        {
          print!("union\n{}",&self.name);

          un.print();
        },
  Definition::Enum(en)=>
        {
          print!("enum\n{}",&self.name);

          en.print();
        },
    }
}


}




pub enum
Statement
{
  Empty,
  Declaration(Declaration),
  Block(Block),
  If(Vec<Block>),
  For(Block),
  While(Block),
  Break,
  Continue,
  Return(Option<Expression>),
  Expression(Expression),

}


impl
Statement
{


pub fn
make_from_string(s: &str)-> Result<Statement,()>
{
  use crate::syntax::dictionary::Dictionary;

  let       dic = self::dictionary::get_dictionary();
  let  expr_dic = super::expression::dictionary::get_dictionary();
  let    ty_dic = super::typesystem::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![dic,expr_dic,ty_dic];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"statement",Some(dics))
    {
      let  cur = crate::syntax::Cursor::new(&dir);

        if let Some(e_dir) = cur.get_directory()
        {
//                  e_dir.print(0);

          return self::read_statement::read_statement(&e_dir);
        }
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
execute(&self)
{
    match self
    {
  Statement::Empty=>{},
  Statement::Declaration(decl)=>{},
  Statement::Block(blk)=>{},
  Statement::If(blks)=>
        {
        },
  Statement::For(blks)=>{},
  Statement::While(blk)=>
        {
        },
  Statement::Break=>{},
  Statement::Continue=>{},
  Statement::Return(op_e)=>
        {
        },
  Statement::Expression(e)=>{},
    }
}


pub fn
print(&self)
{
    match self
    {
  Statement::Empty=>{print!(";");},
  Statement::Declaration(decl)=>{decl.print();},
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
  Statement::For(blks)=>{},
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
    }
}


}




pub struct
Block
{
  condition: Option<Expression>,

  statement_list: Vec<Statement>,

}


impl
Block
{


pub fn
new()-> Block
{
  Block{ condition: None, statement_list: Vec::new()}
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
get_statement_list(&self)-> &Vec<Statement>
{
  &self.statement_list
}


pub fn
execute(&self)
{
    for st in &self.statement_list
    {
      st.execute();
    }
}


pub fn
print(&self)
{
    if let Some(cond) = &self.condition
    {
      cond.print();
    }


  print!("{}\n","{");

    for stmt in &self.statement_list
    {
      stmt.print();

      print!("\n");
    }


  print!("{}\n","}");
}


}




pub struct
Program
{
  pub(crate) statement_list: Vec<Statement>,

}


impl
Program
{


pub fn
make_from_string(s: &str)-> Result<Program,()>
{
  use crate::syntax::dictionary::Dictionary;

  let       dic = self::dictionary::get_dictionary();
  let  expr_dic = super::expression::dictionary::get_dictionary();
  let    ty_dic = super::typesystem::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![dic,expr_dic,ty_dic];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"primary_statement",Some(dics))
    {
      return crate::language::statement::read_statement::read_program(&dir)
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
print(&self)
{
  print!("program\n\n");

    for st in &self.statement_list
    {
      st.print();
    }
}


}




