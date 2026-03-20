

use crate::node::*;
use super::*;




pub static  VOID_STR: &'static str = "void";
pub static UNDEF_STR: &'static str = "undef";
pub static  BOOL_STR: &'static str = "bool";
pub static   INT_STR: &'static str = "int";
pub static FLOAT_STR: &'static str = "float";


#[derive(Clone,PartialEq)]
pub enum
Ty
{
  Undef,
  Void,
  Bool,
  Int, Float,

  Pointer(Box<Ty>),
  Function{parameter_ty_list: Vec<Ty>, return_ty: Box<Ty>},

  Canonicalized(String),

}


impl
Ty
{


pub fn
read(s: &str)-> Result<Self,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

    if let Ok(nd) = crate::syntax::parse::parse_from_string(s,dic,"type",None)
    {
      return Ok(read_ty(&nd));
    }


  Err(())
}


pub fn  is_undef(&self)-> bool{if let Self::Undef = self{true} else{false}}
pub fn   is_void(&self)-> bool{if let Self::Void  = self{true} else{false}}
pub fn   is_bool(&self)-> bool{if let Self::Bool  = self{true} else{false}}
pub fn     is_int(&self)-> bool{if let Self::Int   = self{true} else{false}}
pub fn   is_float(&self)-> bool{if let Self::Float = self{true} else{false}}
pub fn   is_pointer(&self)-> bool{if let Self::Pointer(_) = self{true} else{false}}
pub fn   is_function(&self)-> bool{if let Self::Function{..} = self{true} else{false}}
pub fn   is_canonicalized(&self)-> bool{if let Self::Canonicalized(_) = self{true} else{false}}


pub fn
canonicalize(self)-> Self
{
  Self::Canonicalized(self.get_canonical_name())
}


pub fn
print_canonical_name_to(&self, buf: &mut String)
{
    match self
    {
  Self::Undef=>{buf.push_str(UNDEF_STR);}
  Self::Void=> {buf.push_str(VOID_STR);}
  Self::Bool=> {buf.push_str(BOOL_STR);}
  Self::Int=>  {buf.push_str(INT_STR);}
  Self::Float=>{buf.push_str(FLOAT_STR);}
  Self::Pointer(ty)=>{  buf.push_str("*");  ty.print_canonical_name_to(buf);}
  Self::Function{parameter_ty_list,return_ty}=>
    {
      buf.push_str("fn(");

        for ty in parameter_ty_list
        {
          ty.print_canonical_name_to(buf);

          buf.push_str(",");
        }


      buf.push_str(")->");

      return_ty.print_canonical_name_to(buf);
    }
  Self::Canonicalized(s)=>{buf.push_str(s);}
    }
}


pub fn
get_canonical_name(&self)-> String
{
  let  mut buf = String::new();

  self.print_canonical_name_to(&mut buf);

  buf
}


pub fn
print(&self)
{
  let  buf = self.get_canonical_name();

  print!("{}",&buf);
}


}




pub fn
read_ty(start_nd: &Node)-> Ty
{
  let  mut cur = start_nd.cursor();

    if let Some(s) = cur.get_semi_string()
    {
      print!("{}",s);

      cur.advance(1);
    }


    if let Some(s) = cur.get_identifier()
    {
           if s ==  "void"{return Ty::Void;}
      else if s == "undef"{return Ty::Undef;}
      else if s ==  "bool"{return Ty::Bool;}
      else if s ==   "int"{return Ty::Int;}
      else if s == "float"{return Ty::Float;}
    }


  panic!();
}




