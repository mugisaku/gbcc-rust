

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
  BlockIndex,
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




pub struct
Library
{
  pub(crate)  expression_list: Vec<Expression>,
  pub(crate)      string_list: Vec<String>,
  pub(crate)        type_list: Vec<Type>,
  pub(crate) declaration_list: Vec<Declaration>,
  pub(crate)       block_list: Vec<Block>,

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
              block_list: Vec::new(),
  };


  lib.type_list.push(Type::Undefined);
  lib.type_list.push(Type::Void);
  lib.type_list.push(Type::Bool);
  lib.type_list.push(Type::Char);
  lib.type_list.push(Type::I8);
  lib.type_list.push(Type::I16);
  lib.type_list.push(Type::I32);
  lib.type_list.push(Type::I64);
  lib.type_list.push(Type::ISize);
  lib.type_list.push(Type::U8);
  lib.type_list.push(Type::U16);
  lib.type_list.push(Type::U32);
  lib.type_list.push(Type::U64);
  lib.type_list.push(Type::USize);
  lib.type_list.push(Type::F32);
  lib.type_list.push(Type::F64);

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

  self.declaration_list.push(d);

  di
}


pub fn
get_declaration(&self, i: DeclarationIndex)-> Option<&Declaration>
{
    if i.value < self.declaration_list.len()
    {
      return Some(&self.declaration_list[i.value]);
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
        if self.type_list[i] == t
        {
          return TypeIndex{value: i};
        }
    }


  self.type_list.push(t);

  TypeIndex{value: last_i}
}


pub fn
get_type(&self, i: TypeIndex)-> Option<&Type>
{
    if i.value < self.type_list.len()
    {
      return Some(&self.type_list[i.value]);
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
push_block(&mut self, blk: Block)-> BlockIndex
{
  let  last_i = self.block_list.len();

  self.block_list.push(blk);

  BlockIndex{value: last_i}
}


pub fn
get_block(&self, i: BlockIndex)-> Option<&Block>
{
    if i.value < self.block_list.len()
    {
      return Some(&self.block_list[i.value]);
    }


  None
}


pub fn
print_block(&self, i: BlockIndex)
{
    if let Some(blk) = self.get_block(i)
    {
      blk.print(self);
    }
}




pub fn
print(&self)
{
  print!("library\n{{\n");

    for decl in &self.declaration_list
    {
      decl.print(self);

      print!("\n\n");
    }


  print!("}}\n\n");
}




}





