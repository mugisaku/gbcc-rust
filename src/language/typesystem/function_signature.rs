

use super::{
  TypeNote,
  get_max,
};

use super::r#struct::{
  Struct,
  print_member_list,

};


#[derive(Clone)]
pub struct
FunctionSignature
{
  pub(crate) parameter: Struct,

  pub(crate) return_type_note: TypeNote,

}


impl
FunctionSignature
{


pub fn
new()-> FunctionSignature
{
  FunctionSignature{parameter: Struct::new(), return_type_note: TypeNote::Void}
}


pub fn
print(&self)
{
  print!("(");

  print_member_list(&self.parameter.member_list);

  print!(")-> ");

  self.return_type_note.print();
}


}




