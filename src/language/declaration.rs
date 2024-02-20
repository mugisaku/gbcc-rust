

pub mod read_expression;
pub mod read_statement;
pub mod read_declaration;
pub mod read_type;
pub mod expression_dictionary;
pub mod statement_dictionary;
pub mod typesystem_dictionary;

use std::rc::Rc;

use super::{
  WORD_SIZE,
  get_aligned_size,
};

use super::expression::{
  Expression,
  ExpressionKeeper,
};

use super::typesystem::{
  TypeItem,
  TypeItemKeeper,
  Parameter,
  EnumParameter,
  FieldInfo,

};

use super::value::Value;
use super::statement::{
  Statement,

};




pub trait
ObjectManager
{


fn  find_definition(&self, name: &str)-> Option<&Definition>;
fn  find_struct(&self, name: &str)-> Option<&Vec<Parameter>>;
fn   find_union(&self, name: &str)-> Option<&Vec<Parameter>>;
fn    find_enum(&self, name: &str)-> Option<&Vec<EnumParameter>>;
fn   find_alias(&self, name: &str)-> Option<&TypeItem>;


}




pub struct
IDDistributor
{
  pub(crate)   plain_id: usize,
  pub(crate) if_list_id: usize,
  pub(crate)   while_id: usize,
  pub(crate)     for_id: usize,
  pub(crate)    loop_id: usize,

  pub(crate) expression_id: usize,
  pub(crate)     string_id: usize,

}


impl
IDDistributor
{


pub fn
new()-> IDDistributor
{
  IDDistributor{
    plain_id: 0,
  if_list_id: 0,
    while_id: 0,
      for_id: 0,
     loop_id: 0,

    expression_id: 0,
        string_id: 0,
  }
}


}




pub struct
Storage
{
  pub(crate) type_item_keeper: TypeItemKeeper,

  pub(crate) expression_keeper_opt: Option<ExpressionKeeper>,

}


impl
Storage
{


pub fn
print(&self)
{
  self.type_item_keeper.type_item.print();

    if let Some(ek) = &self.expression_keeper_opt
    {
      print!(" = ");

      ek.expression.print();
    }
}


}




pub struct
Function
{
  pub(crate) parameter_list: Vec<Parameter>,

  pub(crate) return_type_item_keeper: TypeItemKeeper,

  pub(crate) statement_list: Vec<Statement>,

}


impl
Function
{


pub fn
find_parameter(&self, name: &str)-> Option<&Parameter>
{
    for para in &self.parameter_list
    {
        if para.name == name
        {
          return Some(para);
        }
    }


  None
}


pub fn
find_declaration(&self, name: &str)-> Option<&Declaration>
{
    for stmt in &self.statement_list
    {
        if let Statement::Declaration(decl) = stmt
        {
            if decl.name == name
            {
              return Some(decl);
            }
        }
    }


  None
}


}




pub enum
Definition
{
  Space(Space),
  Fn(Function),
  Var(Storage),
  Static(Storage),
  Const(Storage),
  Type(TypeItemKeeper),

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
  Definition::Space(sp)=>
        {
          print!("space\n{}(",&self.name);

          sp.print();
        }
  Definition::Fn(f)=>
        {
          print!("fn\n{}(",&self.name);

            for p in &f.parameter_list
            {
              print!("{}: ",&p.name);

              p.type_item_keeper.type_item.print();

              print!(", ");
            }


          print!(")-> ");

          f.return_type_item_keeper.type_item.print();

          print!("\n{{\n");

          super::statement::print_statement_list(&f.statement_list,1);

          print!("}}");
        },
  Definition::Var(s)=>
        {
          print!("var  {}: ",&self.name);

          s.print();
        },
  Definition::Static(s)=>
        {
          print!("static  {}: ",&self.name);

          s.print();
        },
  Definition::Const(s)=>
        {
          print!("const  {}: ",&self.name);

          s.print();
        },
  Definition::Type(tk)=>
        {
          print!("{}: ",&self.name);

          tk.type_item.print();
        },
    }
}


}




pub struct
Space
{
  pub(crate) declaration_list: Vec<Declaration>,

}


impl
Space
{


pub fn 
new()-> Self
{
  let  mut sp = Self{
    declaration_list: Vec::new(),
  };


  sp
}




pub fn
find_declaration(&self, name: &str)-> Option<&Declaration>
{
    for decl in &self.declaration_list
    {
        if decl.name == name
        {
          return Some(decl);
        }
    }


  None
}




pub fn
append_from_str(&mut self, s: &str)
{
  use crate::syntax::dictionary::Dictionary;

  let       dic = self::statement_dictionary::get_dictionary();
  let  expr_dic = self::expression_dictionary::get_dictionary();
  let    ty_dic = self::typesystem_dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![expr_dic,ty_dic];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"primary_statement",Some(dics))
    {
      let  mut cur = crate::syntax::Cursor::new(&dir);

        while let Some(decl_d) = cur.get_directory()
        {
            if let Ok(di) = crate::language::declaration::read_declaration::read_declaration(decl_d)
            {
              self.declaration_list.push(di);

              cur.advance(1);
            }

          else
            {
            }
        }


      return;
    }


  println!("make_from_string error: parse is failed");
}


pub fn
print(&self)
{
  print!("\n{{\n");

    for decl in &self.declaration_list
    {
      decl.print();

      print!("\n\n");
    }


  print!("}}\n\n");
}


}




pub struct
DeclarationLink
{
  pub(crate) table_list: Vec<Vec<*mut Declaration>>,

}


impl
DeclarationLink
{


pub fn
new()-> DeclarationLink
{
  DeclarationLink{
    table_list: vec![Vec::new()],
  }
}


pub fn
new_table(&mut self)
{
  self.table_list.push(Vec::new());
}


pub fn
delete_last_table(&mut self)
{
  let  _ = self.table_list.pop();
}


pub fn
push(&mut self, decl: &mut Declaration)
{
    if let Some(tbl) = self.table_list.last_mut()
    {
      tbl.push(decl as *mut Declaration);
    }
}


pub fn
find(&self, name: &str)-> Option<&mut Declaration>
{
  let  l = self.table_list.len();

    for i in 0..l
    {
      let  tbl = &self.table_list[l-1-i];

        for ptr in tbl
        {
            unsafe
            {
              let  decl = &mut **ptr;

                if decl.name == name
                {
                  return Some(decl);
                }
            }
        }
    }


  None
}


}




