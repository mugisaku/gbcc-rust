

pub mod read_expression;
pub mod read_statement;
pub mod read_declaration;
pub mod read_type;
pub mod expression_dictionary;
pub mod statement_dictionary;
pub mod typesystem_dictionary;

use std::cell::Cell;

use super::{
  WORD_SIZE,
  get_aligned_size,
};

use super::expression::{
  Path,
  Expression,

};

use super::typesystem::{
  TypeInfo,
  FieldInfo,

};

use super::operation::{
  Source,
  Destination,
  Operation,

};

use super::statement::{
  Statement,

};




pub struct
Storage
{
  pub(crate) type_info: TypeInfo,

  pub(crate) expression: Expression,

}


impl
Storage
{


pub fn
print(&self)
{
  self.type_info.print();

    if let Expression::None = &self.expression
    {
    }

  else
    {
      print!(" = ");

      self.expression.print();
    }
}


}




pub struct
Function
{
  pub(crate) parameter_space: Space,

  pub(crate) return_type_info: TypeInfo,

  pub(crate) statement_list: Vec<Statement>,

}


impl
Function
{


pub fn
new()-> Function
{
  Function{
    parameter_space: Space{declaration_list: Vec::new()},

    return_type_info: TypeInfo::new_void(),

    statement_list: Vec::new(),
  }
}


pub fn
set_statement_list(mut self, ls: Vec<Statement>)-> Function
{
  self.statement_list = ls;

  self
}


pub fn
add_statement(mut self, stmt: Statement)-> Function
{
  self.statement_list.push(stmt);

  self
}


pub fn
add_parameter(mut self, name: &str, ti: TypeInfo)-> Function
{
  let  decl = Declaration::new(name.to_string(),Component::Type(ti));

  self.parameter_space.declaration_list.push(decl);

  self
}


pub fn
set_parameter_space(mut self, ls: Vec<Declaration>)-> Function
{
  self.parameter_space = Space{declaration_list: ls};

  self
}


pub fn
set_return_type_info(mut self, ti: TypeInfo)-> Function
{
  self.return_type_info = ti;

  self
}


pub fn
get_reference_type_info(&self)-> TypeInfo
{
  let  mut ls: Vec<TypeInfo> = Vec::new();

    for p in &self.parameter_space.declaration_list
    {
        if let Component::Type(ti) = &p.component
        {
          ls.push(ti.clone());
        }
    }


  TypeInfo::new_function_reference(ls,self.return_type_info.clone())
}


pub fn
find_parameter(&self, name: &str)-> Option<&Declaration>
{
    for para in &self.parameter_space.declaration_list
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
Component
{
  Dummy,

  Fn(Function),
  Var(Storage),
  Static(Storage),
  Const(Storage),
  Space(Space),
  Struct(Space),
  Union(Space),
  Enum(Space),
  Enumerator(Expression),
  Type(TypeInfo),

}




pub struct
Declaration
{
  pub(crate) name: String,

  pub(crate) component: Component,

}


impl
Declaration
{


pub fn
new(name: String, com: Component)-> Self
{
  Self{name: name, component: com}
}


pub fn
get_storage_mut(&mut self)-> Option<&mut Storage>
{
    match &mut self.component
    {
  Component::Var(st)=>{Some(st)}
  Component::Static(st)=>{Some(st)}
  Component::Const(st)=>{Some(st)}
  _=>{None}
    }
}


pub fn
get_storage(&self)-> Option<&Storage>
{
    match &self.component
    {
  Component::Var(st)=>{Some(st)}
  Component::Static(st)=>{Some(st)}
  Component::Const(st)=>{Some(st)}
  _=>{None}
    }
}


pub fn
get_space_mut(&mut self)-> Option<&mut Space>
{
    match &mut self.component
    {
  Component::Space(sp)=> {Some(sp)}
  Component::Struct(sp)=>{Some(sp)}
  Component::Union(sp)=> {Some(sp)}
  Component::Enum(sp)=>  {Some(sp)}
  _=>{None}
    }
}


pub fn
get_space(&self)-> Option<&Space>
{
    match &self.component
    {
  Component::Space(sp)=> {Some(sp)}
  Component::Struct(sp)=>{Some(sp)}
  Component::Union(sp)=> {Some(sp)}
  Component::Enum(sp)=>  {Some(sp)}
  _=>{None}
    }
}


pub fn
print(&self)
{
    match &self.component
    {
  Component::Dummy=>{print!("{}(DUMMY)",&self.name)}
  Component::Space(sp)=>
        {
          print!("space\n{}",&self.name);

          sp.print();
        }
  Component::Fn(f)=>
        {
          print!("fn\n{}(",&self.name);

            for p in &f.parameter_space.declaration_list
            {
              print!("{}: ",&p.name);

                if let Component::Type(ti) = &p.component
                {
                  ti.print();
                }


              print!(", ");
            }


          print!(")-> ");

          f.return_type_info.print();

          print!("\n{{\n");

          super::statement::Statement::print_statement_list(&f.statement_list,1);

          print!("}}");
        },
  Component::Var(s)=>
        {
          print!("var  {}: ",&self.name);

          s.print();
        },
  Component::Static(s)=>
        {
          print!("static  {}: ",&self.name);

          s.print();
        },
  Component::Const(s)=>
        {
          print!("const  {}: ",&self.name);

          s.print();
        },
  Component::Struct(sp)=>
        {
          print!("{}: ",&self.name);
        },
  Component::Union(sp)=>
        {
          print!("{}: ",&self.name);
        },
  Component::Enum(sp)=>
        {
          print!("{}: ",&self.name);
        },
  Component::Enumerator(e)=>
        {
          print!("{} = ",&self.name);

          e.print();
        },
  Component::Type(ti)=>
        {
          print!("{}: ",&self.name);

          ti.print();
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
add_new_dummy(&mut self, name: &str)
{
  self.declaration_list.push(Declaration::new(name.to_string(),Component::Dummy))
}


pub fn 
add_new_space(&mut self, name: &str)-> &mut Space
{
  let  decl = Declaration::new(name.to_string(),Component::Space(Space::new()));

  self.declaration_list.push(decl);

    if let Component::Space(sp) = &mut self.declaration_list.last_mut().unwrap().component
    {
      return sp;
    }


  panic!();
}


pub fn 
get_space_mut(&mut self, name: &str)-> &mut Space
{
    for decl in &mut self.declaration_list
    {
        if decl.name == name
        {
            if let Component::Space(sp) = &mut decl.component
            {
              return sp;
            }


          break;
        }
    }


  panic!();
}


pub fn
find_declaration_by_name(&self, name: &str)-> Option<&Declaration>
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




