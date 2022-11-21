

use std::rc::Rc;
use super::statement::*;
use super::typesystem::*;
use super::typesystem::null_ti;
use crate::syntax::parser::Directory;
use crate::syntax::parser::Cursor;
use crate::syntax::parser::ObjectData;


pub struct
Space
{
  variables: Vec<Variable>,
  functions: Vec<Function>,

}


impl
Space
{


pub fn
from(dir: &Directory)-> Space
{
  let mut  cur = Cursor::from(dir);

  let mut  vars: Vec<Variable> = Vec::new();
  let mut   fns: Vec<Function> = Vec::new();

    while let Some(te_dir) = cur.seek_directory("top_element")
    {
      let mut  te_cur = Cursor::from(&te_dir);

        if let Some(fndef_dir) = te_cur.seek_directory("function_definition")
        {
          let  f = Function::from(&fndef_dir);

          fns.push(f);
        }


      cur.advance(1);
    }


  Space{ variables: vars, functions: fns}
}


pub fn
get_function_table(&self)-> &Vec<Function>
{
  &self.functions
}


pub fn
print(&self)
{
  println!("variables:{}","{");

    for var in &self.variables
    {
      var.print();

      println!("{}","}");
    }


  println!("{}","}");

  println!("functions:{}","{");

    for f in &self.functions
    {
      f.print();

      println!("");
    }


  println!("{}","}");
}


}




pub struct
VariableDeclaration
{
  name: Rc<String>,
}


impl
VariableDeclaration
{


pub fn
from(dir: &Directory)-> VariableDeclaration
{
  VariableDeclaration{ name: Rc::new(String::new())}
}


pub fn
print(&self)
{
  print!("var\n{}",*self.name);
}


}


pub struct
Variable
{
  name: Rc<String>,

  
}


impl
Variable
{


pub fn
print(&self)
{
  print!("var\n{}",*self.name);
}


}



pub struct
Function
{
  name: Rc<String>,

  parameter_list: Vec<VariableDeclaration>,

  return_value_type: Option<TypeInfo>,

  block: Block,

}



impl
Function
{


pub fn
read_parameter_list(dir: &Directory)-> Vec<VariableDeclaration>
{
  let mut  ls: Vec<VariableDeclaration> = Vec::new();

  ls
}


pub fn
read_return_value_type(dir: &Directory)-> TypeInfo
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

    if let Some(d) = cur.get_directory_with_name("type_expression")
    {
      return TypeInfo::from(d);
    }


  null_ti.clone()
}


pub fn
from(dir: &Directory)-> Function
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

    if let Some(s) = cur.get_identifier()
    {
      let  name = s.clone();

      cur.advance(1);

        if let Some(paras_dir) = cur.get_directory_with_name("parameter_list")
        {
          let  paras = Self::read_parameter_list(paras_dir);

          cur.advance(1);

          let mut  return_value_type: Option<TypeInfo> = None;

            if let Some(retvalty_dir) = cur.get_directory_with_name("return_value_type")
            {
              return_value_type = Some(Self::read_return_value_type(retvalty_dir));

              cur.advance(1);
            }


            if let Some(blk_dir) = cur.get_directory_with_name("block_statement")
            {
              let  blk = Block::from(blk_dir);

              return  Function{ name, parameter_list: paras, return_value_type, block: blk};
            }
        }
    }


  let mut  f = Function{ name: Rc::new(String::new()), parameter_list: Vec::new(), return_value_type: None, block: Block::new()};

  f
}


pub fn
get_name(&self)-> &Rc<String>
{
  &self.name
}


pub fn
get_block(&self)-> &Block
{
  &self.block
}


pub fn
print(&self)
{
  print!("fn\n{}(",*self.name);

    for p in &self.parameter_list
    {
      print!("{}",*p.name);
    }


  print!(")");

    if let Some(ti) = &self.return_value_type
    {
      print!("-> ");

      ti.print();
    }


  print!("\n");

  self.block.print();
}


}




