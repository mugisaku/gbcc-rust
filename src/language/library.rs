

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
DeclarationIndex
{
  pub(crate) value: usize,
}




pub struct
Library
{
  pub(crate)  expression_list: Vec<Expression>,
  pub(crate)      string_list: Vec<String>,

  pub(crate) global_declaration_list: Vec<Declaration>,
  pub(crate)        declaration_list: Vec<Declaration>,

  pub(crate) block_list: Vec<Block>,

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
               declaration_list: Vec::new(),
        global_declaration_list: Vec::new(),
              block_list: Vec::new(),
  };

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
              lib.push_global_declaration(decl);

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
push_global_declaration(&mut self, d: Declaration)-> DeclarationIndex
{
  let  di = DeclarationIndex{value: self.global_declaration_list.len()};

  self.global_declaration_list.push(d);

  di
}


pub fn
get_global_declaration(&self, i: DeclarationIndex)-> Option<&Declaration>
{
    if i.value < self.global_declaration_list.len()
    {
      return Some(&self.global_declaration_list[i.value]);
    }


  None
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

    for decl in &self.global_declaration_list
    {
      decl.print(self);

      print!("\n\n");
    }


  print!("}}\n\n");
}




}





