

use std::cell::Cell;

use super::{
  WORD_SIZE,
  get_aligned_size,
};

use super::expression::Expression;
use super::typesystem::{
  Type,
  TypeInfo,
  Field,
};

use super::value::Value;
use super::statement::{
  Storage,
  Statement,
  Block,
  Declaration,
  Definition,
};


#[derive(PartialEq,Clone,Copy)]
pub struct
ExpressionIndex
{
  pub(crate) value: usize,
}


#[derive(PartialEq,Clone,Copy)]
pub struct
StringIndex
{
  pub(crate) value: usize,
}


#[derive(PartialEq,Clone,Copy)]
pub struct
TypeIndex
{
  pub(crate) value: usize,
}


#[derive(PartialEq,Clone,Copy)]
pub struct
DeclarationIndex
{
  pub(crate) value: usize,
}


#[derive(PartialEq,Clone,Copy)]
pub struct
SpaceIndex
{
  pub(crate) value: usize,
}


#[derive(PartialEq,Clone,Copy)]
pub struct
Address
{
  pub(crate) value: usize,
}


impl
Address
{


pub fn
get_aligned(&self)-> Address
{
  Address{value: get_aligned_size(self.value)}
}


}


pub struct
Space
{
  parent_index: SpaceIndex,

  name: String,

  index_list: Vec<DeclarationIndex>,

}


impl
Space
{


pub fn
new(parent_si: SpaceIndex, name: &str)-> Space
{
  Space{parent_index: parent_si, name: String::from(name), index_list: Vec::new()}
}


}




pub struct
Library
{
  pub(crate)  expression_list: Vec<Expression>,
  pub(crate)      string_list: Vec<String>,
  pub(crate)        type_list: Vec<(Type,SpaceIndex)>,
  pub(crate) declaration_list: Vec<(Declaration,SpaceIndex)>,

  pub(crate) space_list: Vec<Space>,

  pub(crate) current_space_index: SpaceIndex,
  pub(crate) space_index_stack: Vec<SpaceIndex>,

}


impl
Library
{


pub fn 
new()-> Library
{
  let  mut lib = Library{
         expression_list: Vec::new(),
             string_list: Vec::new(),
               type_list: Vec::new(),
        declaration_list: Vec::new(),
              space_list: Vec::new(),
       space_index_stack: Vec::new(),

    current_space_index: SpaceIndex{value: 0},
  };


  let  si = SpaceIndex{value: 0};

  lib.type_list.push((Type::Undefined,si));
  lib.type_list.push((Type::Void,si));
  lib.type_list.push((Type::Bool,si));
  lib.type_list.push((Type::Char,si));
  lib.type_list.push((Type::I8,si));
  lib.type_list.push((Type::I16,si));
  lib.type_list.push((Type::I32,si));
  lib.type_list.push((Type::I64,si));
  lib.type_list.push((Type::ISize,si));
  lib.type_list.push((Type::U8,si));
  lib.type_list.push((Type::U16,si));
  lib.type_list.push((Type::U32,si));
  lib.type_list.push((Type::U64,si));
  lib.type_list.push((Type::USize,si));
  lib.type_list.push((Type::F32,si));
  lib.type_list.push((Type::F64,si));

  lib.open_space("__GLOBAL_SPACE__");

  lib
}


pub fn
make_from_string(s: &str)-> Result<Library,()>
{
  use crate::syntax::dictionary::Dictionary;

  let       dic = super::statement::dictionary::get_dictionary();
  let  expr_dic = super::expression::dictionary::get_dictionary();
  let    ty_dic = super::typesystem::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![expr_dic,ty_dic];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"primary_statement",Some(dics))
    {
      let  mut lib = Self::new();

      let  mut cur = crate::syntax::Cursor::new(&dir);

        while let Some(decl_d) = cur.get_directory()
        {
            if let Ok(decl) = crate::language::statement::read_declaration::read_declaration(decl_d,&mut lib)
            {
              lib.push_declaration(decl);

              cur.advance(1);
            }

          else
            {
              return Err(());
            }
        }


      return Ok(lib);
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
open_space(&mut self, name: &str)-> SpaceIndex
{
  let  si = SpaceIndex{value: self.space_list.len()};

  self.space_index_stack.push(self.current_space_index);

  self.space_list.push(Space::new(self.current_space_index,name));

  self.current_space_index = si;

  si
}


pub fn
close_space(&mut self)
{
    if let Some(_) = self.space_index_stack.pop()
    {
        if let Some(si) = self.space_index_stack.last()
        {
          self.current_space_index = *si;
        }
    }
}


pub fn
get_space(&self, i: SpaceIndex)-> Option<&Space>
{
    if i.value < self.space_list.len()
    {
      return Some(&self.space_list[i.value]);
    }


  None
}


pub fn
get_current_space_mut(&mut self)-> Option<&mut Space>
{
  let  i = self.current_space_index.value;

    if i < self.space_list.len()
    {
      return Some(&mut self.space_list[i]);
    }


  None
}


pub fn
print_space(&self, i: SpaceIndex)
{
    if let Some(sp) = self.get_space(i)
    {
      print!("{}\n{{\n",&sp.name);

        for di in &sp.index_list
        {
            if di.value < self.declaration_list.len()
            {
              let  decl = &self.declaration_list[di.value].0;

              print!("{}\n",&decl.name);
            }
        }


      print!("}}\n");
    }
}


pub fn
push_expression(&mut self, e: Expression)-> ExpressionIndex
{
  let  i = self.expression_list.len();

  self.expression_list.push(e);

  ExpressionIndex{value: i}
}


pub fn
get_expression(&self, i: ExpressionIndex)-> Option<&Expression>
{
    if i.value < self.expression_list.len()
    {
      return Some(&self.expression_list[i.value]);
    }


  None
}


pub fn
print_expression(&self, i: ExpressionIndex)
{
    if let Some(e) = self.get_expression(i)
    {
      e.print(self);
    }
}


pub fn
push_declaration(&mut self, d: Declaration)-> DeclarationIndex
{
  let  di = DeclarationIndex{value: self.declaration_list.len()};

  self.declaration_list.push((d,self.current_space_index));

    if let Some(sp) = self.get_current_space_mut()
    {
      sp.index_list.push(di);
    }


  di
}


pub fn
get_declaration(&self, i: DeclarationIndex)-> Option<&Declaration>
{
    if i.value < self.declaration_list.len()
    {
      return Some(&self.declaration_list[i.value].0);
    }


  None
}


pub fn
print_declaration(&self, i: DeclarationIndex)
{
    if let Some(d) = self.get_declaration(i)
    {
      d.print(self);
    }
}


pub fn
push_string(&mut self, s: String)-> StringIndex
{
  let  last_i = self.string_list.len();

    for i in 0..last_i
    {
        if self.string_list[i] == s
        {
          return StringIndex{value: i};
        }
    }


  self.string_list.push(s);

  StringIndex{value: last_i}
}


pub fn
get_string(&self, i: StringIndex)-> Option<&String>
{
    if i.value < self.string_list.len()
    {
      return Some(&self.string_list[i.value]);
    }


  None
}


pub fn
print_string(&self, i: StringIndex)
{
    if let Some(s) = self.get_string(i)
    {
      print!("{}",s);
    }
}


pub fn
get_embedded_type_index(t: Type)-> TypeIndex
{
    match t
    {
  Type::Void=>     {TypeIndex{value:  1}},
  Type::Bool=>     {TypeIndex{value:  2}},
  Type::Char=>     {TypeIndex{value:  3}},
  Type::I8=>       {TypeIndex{value:  4}},
  Type::I16=>      {TypeIndex{value:  5}},
  Type::I32=>      {TypeIndex{value:  6}},
  Type::I64=>      {TypeIndex{value:  7}},
  Type::ISize=>    {TypeIndex{value:  8}},
  Type::U8=>       {TypeIndex{value:  9}},
  Type::U16=>      {TypeIndex{value: 10}},
  Type::U32=>      {TypeIndex{value: 11}},
  Type::U64=>      {TypeIndex{value: 12}},
  Type::USize=>    {TypeIndex{value: 13}},
  Type::F32=>      {TypeIndex{value: 14}},
  Type::F64=>      {TypeIndex{value: 15}},
  _=>{TypeIndex{value:  0}},
    }
}


pub fn
push_type(&mut self, t: Type)-> TypeIndex
{
    match t
    {
  Type::FromExpression(_)=>{TypeIndex{value: 0}},
  Type::FunctionPointer(_)=>{self.push_type_internal(t)},
  Type::Pointer(_)=>{self.push_type_internal(t)},
  Type::Reference(_)=>{self.push_type_internal(t)},
  Type::Tuple(_)=>{self.push_type_internal(t)},
  Type::Array(_,_)=>{self.push_type_internal(t)},
  Type::Symbol(_)=>{self.push_type_internal(t)},
  _=>{Self::get_embedded_type_index(t)}
    }
}


pub fn
push_type_list(&mut self, t_ls: Vec<Type>)-> Vec<TypeIndex>
{
  let  mut ti_ls: Vec<TypeIndex> = Vec::new();

    for t in t_ls
    {
      ti_ls.push(self.push_type(t));
    }


  ti_ls
}


pub fn
push_type_internal(&mut self, t: Type)-> TypeIndex
{
  let  last_i = self.type_list.len();

    for i in 0..last_i
    {
        if self.type_list[i].0 == t
        {
          return TypeIndex{value: i};
        }
    }


  self.type_list.push((t,self.current_space_index));

  TypeIndex{value: last_i}
}


pub fn
get_type(&self, i: TypeIndex)-> Option<&Type>
{
    if i.value < self.type_list.len()
    {
      return Some(&self.type_list[i.value].0);
    }


  None
}


pub fn
get_space_by_type_index(&self, i: TypeIndex)-> Option<&Space>
{
    if i.value < self.type_list.len()
    {
      let  si = self.type_list[i.value].1;

      return Some(&self.space_list[si.value]);
    }


  None
}


pub fn
print_type(&self, i: TypeIndex)
{
    if let Some(t) = self.get_type(i)
    {
      t.print(self);
    }
}




pub fn
find_declaration_index_in_space(&self, si: SpaceIndex, name: &str)-> Option<DeclarationIndex>
{
    if si.value < self.space_list.len()
    {
      let  sp = &self.space_list[si.value];

        for i in &sp.index_list
        {
            if let Some(d) = self.get_declaration(*i)
            {
                if d.name == name
                {
                  return Some(*i);
                }
            }
        }


        if si != sp.parent_index
        {
          return self.find_declaration_index_in_space(sp.parent_index,name);
        }
    }


  None
}




pub fn
print(&self)
{
  print!("library\n{{\n");

    for (decl,si) in &self.declaration_list
    {
        if let Some(sp) = self.get_space(*si)
        {
          print!("({})\n",&sp.name);
        }


      decl.print(self);

      print!("\n\n");
    }


  print!("}}\n\n");

    for sp in &self.space_list
    {
      print!("space {}{{\n",&sp.name);

        for di in &sp.index_list
        {
            if let Some(decl) = self.get_declaration(*di)
            {
              print!("  {} ",&decl.name);

                match &decl.definition
                {
              Definition::Fn(_)=>{println!("fn");},
              Definition::Var(_)=>{println!("var");},
              Definition::Static(_)=>{println!("static");},
              Definition::Const(_)=>{println!("const");},
              Definition::Argument(_)=>{println!("argument");},
              Definition::Struct(_)=>{println!("struct");},
              Definition::Union(_)=>{println!("union");},
              Definition::Enum(_)=>{println!("enum");},
              Definition::Alias(_)=>{println!("alias");},
                }
            }
        }


      print!("}}\n\n");
    }


/*
    for (decl,_,_) in &self.declaration_list
    {
      print!("decl {}\n",&decl.name);
    }
*/
}




}





