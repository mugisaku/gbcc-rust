

use super::{
  Type,
};


use crate::language::library::{
  ExpressionIndex,
  StringIndex,
  Library
};


pub struct
FunctionSignature
{
  pub(crate) parameter_list: Vec<Type>,

  pub(crate) return_type: Type,

}


impl
FunctionSignature
{


pub fn
new()-> FunctionSignature
{
  FunctionSignature{parameter_list: Vec::new(), return_type: Type::Void}
}


pub fn
print(&self, lib: &Library)
{
  print!("(");

    for p in &self.parameter_list
    {
      p.print(lib);

      print!(", ");
    }


  print!(")-> ");

  self.return_type.print(lib);
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

      self.parameter_list[i].print(lib);

      print!(", ");
    }


  print!(")-> ");

  self.return_type.print(lib);
}


}




