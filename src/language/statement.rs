

pub mod read_statement;
pub mod read_declaration;
pub mod dictionary;

use super::library::{
  ExpressionIndex,
  StringIndex,
  Library
};

use std::cell::Cell;
use super::get_aligned_size;
use super::expression::Expression;
use super::value::Value;
use super::typesystem::{
  Type,
  r#struct::Struct,
  r#union::Union,
  r#enum::Enum,
  function_signature::FunctionSignature,

};


pub struct
Var
{
  pub(crate) r#type: Type,

  pub(crate) expression_index_opt: Option<ExpressionIndex>,

  pub(crate) value_optcel: Cell<Option<Value>>,

  pub(crate) address_optcel: Cell<Option<i64>>,

}




pub struct
Fn
{
  pub(crate) signature: FunctionSignature,

  pub(crate) parameter_name_list: Vec<String>,

  pub(crate) block: Block,

  pub(crate) index_optcel: Cell<Option<usize>>,

}




pub enum
Definition
{
  Fn(Fn),
  Var(Var),
  Static(Var),
  Const(Var),
  Argument(Var),
  Struct(Struct),
  Union(Union),
  Enum(Enum),
  Alias(Type),

}




#[derive(Clone,Copy)]
pub enum
Status
{
  Completed,
    Touched,
  Untouched,

}


pub struct
Declaration
{
  pub(crate) name: String,

  pub(crate) definition: Definition,

  pub(crate) status_cell: Cell<Status>,

}


impl
Declaration
{


pub fn
new(name: &str, def: Definition)-> Declaration
{
  Declaration{name: String::from(name), definition: def, status_cell: Cell::new(Status::Untouched)}
}


pub fn
print(&self, lib: &Library)
{
    match &self.definition
    {
  Definition::Fn(f)=>
        {
          print!("fn\n{}",&self.name);

          f.signature.print_with_name_list(&f.parameter_name_list,lib);

          print!("\n");

          f.block.print(lib);
        },
  Definition::Var(v)=>
        {
          print!("var\n{}: ",&self.name);

          v.r#type.print(lib);

            if let Some(ei) = &v.expression_index_opt
            {
              print!(" = ");

                if let Some(e) = lib.get_expression(*ei)
                {
                  e.print(lib);
                }
            }
        },
  Definition::Static(v)=>
        {
          print!("static\n{}: ",&self.name);

          v.r#type.print(lib);

            if let Some(ei) = &v.expression_index_opt
            {
              print!(" = ");

                if let Some(e) = lib.get_expression(*ei)
                {
                  e.print(lib);
                }
            }
        },
  Definition::Const(v)=>
        {
          print!("const\n{}: ",&self.name);

          v.r#type.print(lib);

            if let Some(ei) = &v.expression_index_opt
            {
              print!(" = ");

                if let Some(e) = lib.get_expression(*ei)
                {
                  e.print(lib);
                }
            }
        },
  Definition::Argument(v)=>
        {
          print!("arg\n{}: ",&self.name);

          v.r#type.print(lib);

            if let Some(ei) = &v.expression_index_opt
            {
              print!(" = ");

                if let Some(e) = lib.get_expression(*ei)
                {
                  e.print(lib);
                }
            }
        },
  Definition::Struct(st)=>
        {
          print!("struct\n{}",&self.name);

          st.print(lib);
        },
  Definition::Union(un)=>
        {
          print!("union\n{}",&self.name);

          un.print(lib);
        },
  Definition::Enum(en)=>
        {
          print!("enum\n{}",&self.name);

          en.print(lib);
        },
  Definition::Alias(ty)=>
        {
          print!("alias\n{}: ",&self.name);

          ty.print(lib);
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
  If(ConditionalBlock,Vec<ConditionalBlock>,Option<Block>),
  For(Block),
  While(ConditionalBlock),
  Loop(Block),
  Break,
  Continue,
  Return(Option<ExpressionIndex>),
  Expression(ExpressionIndex),

}


impl
Statement
{


pub fn
make_from_string(s: &str, lib: &mut Library)-> Result<Statement,()>
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

          return self::read_statement::read_statement(&e_dir,lib);
        }
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
print(&self, lib: &Library)
{
    match self
    {
  Statement::Empty=>{print!(";");},
  Statement::Declaration(decl)=>{decl.print(lib);},
  Statement::Block(blk)=>{blk.print(lib);},
  Statement::If(top,elif_ls,el_opt)=>
        {
          print!("if ");

          top.print(lib);

            for condblk in elif_ls
            {
              print!("else if ");

              condblk.print(lib);
            }


            if let Some(blk) = el_opt
            {
              print!("else ");

              blk.print(lib);
            }
        },
  Statement::For(blks)=>{},
  Statement::While(condblk)=>
        {
          print!("while ");

          condblk.print(lib);
        },
  Statement::Loop(blk)=>
        {
          print!("loop\n");

          blk.print(lib);
        },
  Statement::Break=>{print!("break");},
  Statement::Continue=>{print!("continue");},
  Statement::Return(ei_opt)=>
        {
          print!("return ");

            if let Some(ei) = ei_opt
            {
                if let Some(e) = lib.get_expression(*ei)
                {
                  e.print(lib);
                }
            }
        },
  Statement::Expression(ei)=>
        {
            if let Some(e) = lib.get_expression(*ei)
            {
              e.print(lib);
            }
        },
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
new()-> Block
{
  Block{statement_list: Vec::new()}
}


pub fn
get_statement_list(&self)-> &Vec<Statement>
{
  &self.statement_list
}


pub fn
print(&self, lib: &Library)
{
  print!("{{\n");

    for stmt in &self.statement_list
    {
      stmt.print(lib);

      print!("\n");
    }


  print!("}}\n");
}


}




pub struct
ConditionalBlock
{
  pub(crate) expression_index: ExpressionIndex,
  pub(crate) block: Block,

}


impl
ConditionalBlock
{


pub fn
print(&self, lib: &Library)
{
    if let Some(e) = lib.get_expression(self.expression_index)
    {
      e.print(lib);

      print!("\n");

      self.block.print(lib);
    }
}


}




pub struct
Program
{
  pub(crate) declaration_list: Vec<Declaration>,

}


impl
Program
{


pub fn
new()-> Program
{
  Program{declaration_list: Vec::new()}
}


pub fn
make_from_string(s: &str, lib: &mut Library)-> Result<Program,()>
{
  use crate::syntax::dictionary::Dictionary;

  let       dic = self::dictionary::get_dictionary();
  let  expr_dic = super::expression::dictionary::get_dictionary();
  let    ty_dic = super::typesystem::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![expr_dic,ty_dic];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"declaration",Some(dics))
    {
      let  mut prog = Program::new();

      let  mut cur = crate::syntax::Cursor::new(&dir);

        while let Some(decl_d) = cur.get_directory()
        {
            if let Ok(decl) = crate::language::statement::read_declaration::read_declaration(decl_d,lib)
            {
              prog.declaration_list.push(decl);

              cur.advance(1);
            }

          else
            {
              return Err(());
            }
        }


      return Ok(prog);
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
fix(&mut self)-> Result<(),()>
{
  let  mut decl_ls: Vec<&Declaration> = Vec::new();

    for decl in &self.declaration_list
    {
      decl_ls.push(decl);
    }


  Err(())
}


pub fn
print(&self, lib: &Library)
{
  print!("program\n\n");

    for st in &self.declaration_list
    {
      st.print(lib);

      print!("\n\n");
    }
}


}




