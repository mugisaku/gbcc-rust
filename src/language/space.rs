

use super::statement::*;
use super::type_expression::*;
use crate::syntax::{
  Directory
};




pub enum
Component
{
  Function(Function),
  Variable(Variable),
  Constant(Variable),
  Struct(Vec<PreParameter>),
  Union(Vec<PreParameter>),
  Enum(Vec<Enumerator>),

}


impl
Component
{


pub fn
print(&self)
{
    match self
    {
  Component::Function(f)=>{},
  Component::Variable(v)=>{},
  Component::Constant(v)=>{},
  Component::Struct(pls)=>{},
  Component::Union(pls)=>{},
  Component::Enum(els)=>{},
    }
}


}




pub struct
Space
{
  components: Vec<Component>,

}


impl
Space
{


pub fn
build(dir: &Directory)-> Result<Space,()>
{
  let mut  cur = Cursor::from(dir);

  let mut  compos: Vec<Component> = Vec::new();

    while let Some(te_dir) = cur.seek_directory("top_element")
    {
      let mut  te_cur = Cursor::from(&te_dir);

        if let Some(fndef_dir) = te_cur.seek_directory("function_definition")
        {
            if let Ok(f) = Function::build(&fndef_dir)
            {
              compos.push(Component::Function(f));
            }

          else
            {
              return Err(());
            }
        }


      cur.advance(1);
    }


  Ok(Space{ components: compos})
}


pub fn
get_component_list(&self)-> &Vec<Component>
{
  &self.components
}


pub fn
print(&self)
{
    for compo in &self.components
    {
      compo.print();

      println!("\n");
    }
}


}




pub struct
Variable
{
  name: String,

  type_expression: TypeExpression,
  
}


impl
Variable
{


pub fn
get_name(&self)-> &str
{
  &self.name
}


pub fn
get_type_expression(&self)-> &TypeExpression
{
  &self.type_expression
}


pub fn
print(&self)
{
  print!("var\n{}: ",self.name);

  self.type_expression.print();
}


}



pub struct
Function
{
  name: String,

  signature: FunctionSignature,

  block: Block,

}



impl
Function
{


pub fn
build(dir: &Directory)-> Result<Function,()>
{
  let mut  cur = Cursor::from(dir);

  cur.advance(1);

    if let Some(s) = cur.get_identifier()
    {
      let  name = String::from(s.as_str());

      cur.advance(1);

        if let Some(sig_dir) = cur.get_directory_with_name("function_signature")
        {
            if let Ok(sig) = FunctionSignature::build(sig_dir)
            {
              cur.advance(1);

                if let Some(blk_dir) = cur.get_directory_with_name("block_statement")
                {
                    if let Ok(blk) = Block::build(blk_dir)
                    {
                      return Ok(Function{ name, signature: sig, block: blk});
                    }
                }
            }
        }
    }


  Err(())
}


pub fn
get_name(&self)-> &str
{
  &self.name
}


pub fn
get_signature(&self)-> &FunctionSignature
{
  &self.signature
}


pub fn
get_block(&self)-> &Block
{
  &self.block
}


pub fn
print(&self)
{
  print!("fn\n{}",self.name);

  self.signature.print();

  print!("\n");

  self.block.print();
}


}




