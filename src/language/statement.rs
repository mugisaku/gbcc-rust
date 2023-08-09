

pub mod read_statement;
pub mod read_declaration;
pub mod dictionary;

use super::library::{
  ExpressionIndex,
  StringIndex,
  TypeIndex,
  DeclarationIndex,
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
Storage
{
  pub(crate) type_index: TypeIndex,

  pub(crate) expression_index_opt: Option<ExpressionIndex>,

}


impl
Storage
{


pub fn
print(&self, lib: &Library)
{
  lib.print_type(self.type_index);

    if let Some(ei) = &self.expression_index_opt
    {
      print!(" = ");

        if let Some(e) = lib.get_expression(*ei)
        {
          e.print(lib);
        }
    }
}


}




pub struct
Function
{
  pub(crate) signature: FunctionSignature,

  pub(crate) parameter_name_list: Vec<String>,

  pub(crate) block_index: BlockIndex,

}




pub enum
Definition
{
  Fn(Function),
  Var(Storage),
  Static(Storage),
  Const(Storage),
  Argument(Storage),
  Struct(Struct),
  Union(Union),
  Enum(Enum),
  Alias(TypeIndex),

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
print(&self, lib: &Library)
{
    match &self.definition
    {
  Definition::Fn(f)=>
        {
          print!("fn\n{}",&self.name);

          f.signature.print_with_name_list(&f.parameter_name_list,lib);

          print!("\n");

            if let Some(blk) = lib.get_block(f.block_index)
            {
              blk.print(lib);
            }
        },
  Definition::Var(s)=>
        {
          print!("var\n{}: ",&self.name);

          s.print(lib);
        },
  Definition::Static(s)=>
        {
          print!("static\n{}: ",&self.name);

          s.print(lib);
        },
  Definition::Const(s)=>
        {
          print!("const\n{}: ",&self.name);

          s.print(lib);
        },
  Definition::Argument(s)=>
        {
          print!("arg\n{}: ",&self.name);

          s.print(lib);
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
  Definition::Alias(ti)=>
        {
          print!("alias\n{}: ",&self.name);

          lib.print_type(*ti);
        },
    }
}


}




pub enum
Statement
{
  Empty,
  Declaration(DeclarationIndex),
  Block(BlockIndex),
  If((ExpressionIndex,BlockIndex),Vec<(ExpressionIndex,BlockIndex)>,Option<BlockIndex>),
  For(BlockIndex),
  While((ExpressionIndex,BlockIndex)),
  Loop(BlockIndex),
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
  Statement::Declaration(di)=>{lib.print_declaration(*di);},
  Statement::Block(bi)=>{lib.print_block(*bi);},
  Statement::If((top_ei,top_bi),(elif_ls),el_bi_opt)=>
        {
          print!("if ");

          lib.print_expression(*top_ei);

          lib.print_block(*top_bi);

            for (ei,bi) in elif_ls
            {
              print!("else if ");

              lib.print_expression(*ei);

              lib.print_block(*bi);
            }


            if let Some(bi) = el_bi_opt
            {
              print!("else ");

              lib.print_block(*bi);
            }
        },
  Statement::For(blks)=>{},
  Statement::While((ei,bi))=>
        {
          print!("while ");

          lib.print_block(*bi);
        },
  Statement::Loop(bi)=>
        {
          print!("loop\n");

          lib.print_block(*bi);
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




#[derive(PartialEq,Clone,Copy)]
pub struct
BlockIndex
{
  pub(crate) value: usize,

}


pub struct
Block
{
  pub(crate) parent_block_index_opt: Option<BlockIndex>,

  pub(crate) statement_list: Vec<Statement>,

}


impl
Block
{


pub fn
new(parent_block_index_opt: Option<BlockIndex>)-> Block
{
  Block{parent_block_index_opt, statement_list: Vec::new()}
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




