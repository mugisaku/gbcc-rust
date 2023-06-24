

use super::{
  Type,
};


use crate::language::library::{
  ExpressionIndex,
  StringIndex,
  TypeIndex,
  Library
};


#[derive(PartialEq)]
pub struct
FunctionSignature
{
  pub(crate) parameter_list: Vec<TypeIndex>,

  pub(crate) return_type_index: TypeIndex,

}


impl
FunctionSignature
{


pub fn
new()-> FunctionSignature
{
  FunctionSignature{parameter_list: Vec::new(), return_type_index: TypeIndex{value: 0}}
}


pub fn
print(&self, lib: &Library)
{
  print!("(");

    for p in &self.parameter_list
    {
      lib.print_type(*p);

      print!(", ");
    }


  print!(")-> ");

  lib.print_type(self.return_type_index);
}


pub fn
print_with_name_list(&self, ls: &Vec<String>, lib: &Library)
{
    if ls.len() != self.parameter_list.len()
    {
      println!("print_with_name_list error: length of list is not matched");

      return;
    }


  print!("(");

    for i in 0..ls.len()
    {
      print!("{}: ",&ls[i]);

      lib.print_type(self.parameter_list[i]);

      print!(", ");
    }


  print!(")-> ");

  lib.print_type(self.return_type_index);
}


}




