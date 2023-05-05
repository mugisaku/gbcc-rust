

pub mod read_statement;
pub mod read_declaration;
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

  pub(crate) expression_opt: Option<Expression>,

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
  Static(Var),
  Const(Var),
  Struct(Struct),
  Union(Union),
  Enum(Enum),
  Alias(TypeNote),

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

            if let Some(e) = &v.expression_opt
            {
              print!(" = ");
              e.print();
            }
        },
  Definition::Static(v)=>
        {
          print!("static\n{}: ",&self.name);
          v.type_note.print();

            if let Some(e) = &v.expression_opt
            {
              print!(" = ");
              e.print();
            }
        },
  Definition::Const(v)=>
        {
          print!("const\n{}: ",&self.name);
          v.type_note.print();

            if let Some(e) = &v.expression_opt
            {
              print!(" = ");
              e.print();
            }
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
  Definition::Alias(ty)=>
        {
          print!("alias\n{}: ",&self.name);

          ty.print();
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
print(&self)
{
    match self
    {
  Statement::Empty=>{print!(";");},
  Statement::Declaration(decl)=>{decl.print();},
  Statement::Block(blk)=>{blk.print();},
  Statement::If(top,elif_ls,el_opt)=>
        {
          print!("if ");

          top.print();

            for condblk in elif_ls
            {
              print!("else if ");

              condblk.print();
            }


            if let Some(condblk) = el_opt
            {
              print!("else ");

              condblk.print();
            }
        },
  Statement::For(blks)=>{},
  Statement::While(condblk)=>
        {
          print!("while ");

          condblk.print();
        },
  Statement::Loop(blk)=>
        {
          print!("loop\n");

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
ConditionalBlock
{
  pub(crate) expression: Expression,
  pub(crate) block: Block,

}


impl
ConditionalBlock
{


pub fn
print(&self)
{
  self.expression.print();

  print!("\n");

  self.block.print();
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
make_from_string(s: &str)-> Result<Program,()>
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
            if let Ok(decl) = crate::language::statement::read_declaration::read_declaration(decl_d)
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
print(&self)
{
  print!("program\n\n");

    for st in &self.declaration_list
    {
      st.print();

      print!("\n\n");
    }
}


}




