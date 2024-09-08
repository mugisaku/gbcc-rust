

use super::get_aligned_size;
use super::get_default_aligned_size;

use super::declaration::{
  Declaration,
  Space,
  Symbol,

};

use super::expression::{
  collect_string,
  Expression,
  Path,

};

pub const WORD_SIZE: usize = 8;




#[derive(Clone)]
pub struct
TyInfo
{
  pub(crate)  size: usize,
  pub(crate) align: usize,

}


impl
TyInfo
{


pub fn
from_size(sz: usize)-> Self
{
  Self{
    size: sz, align: sz,
  }
}


pub fn
update_size(&mut self, new_sz: usize)
{
    if new_sz > self.size
    {
      self.size = new_sz;
    }
}


pub fn
update_align(&mut self, new_al: usize)
{
    if new_al > self.align
    {
      self.align = new_al;
    }
}


}




#[derive(Clone)]
pub struct
TySet
{
  pub(crate) field_list: Vec<Field>,
  pub(crate)   info_opt: Option<TyInfo>,

}


impl
TySet
{


pub fn
new()-> Self
{
  Self{
    field_list: Vec::new(),
    info_opt: None,
  }
}


pub fn
from(ls: Vec<Field>)-> Self
{
  Self{
    field_list: ls,
    info_opt: None,
  }
}


pub fn
find(&self, name: &str)-> Option<&Field>
{
    for fld in &self.field_list
    {
        if fld.name == name
        {
          return Some(fld);
        }
    }


  None
}


pub fn
add(&mut self, name: String, ty: Ty)-> bool
{
    if name.len() != 0
    {
        if let Some(_) = self.find(&name)
        {
          return false;
        }
    }


  let  index = self.field_list.len();

  let  fld = Field{
    index, name, ty, offset: 0,
  };


  self.field_list.push(fld);

  self. info_opt = None;

  true
}


pub fn
complete_as_struct(&mut self, sym_tbl: &Vec<Symbol>)-> bool
{
    if let Some(_) = &self.info_opt
    {
      return true;
    }


  let  mut ti = TyInfo::from_size(0);

    for i in 0..self.field_list.len()
    {
      let  fld = &mut self.field_list[i];

      let  ty = &mut fld.ty;

        if !ty.complete(sym_tbl)
        {
          return false;
        }


      let  cur_ti = ty.get_info();

        if cur_ti.size != 0
        {
          fld.offset = get_aligned_size(ti.size,cur_ti.align);

          ti.size += cur_ti.size;

          ti.update_align(cur_ti.align);
        }

      else
        {
          fld.offset = ti.size;
        }
    }


  self.info_opt = Some(ti);

  true
}


pub fn
complete_as_union(&mut self, sym_tbl: &Vec<Symbol>)-> bool
{
    if let Some(_) = &self.info_opt
    {
      return true;
    }


  let  mut ti = TyInfo::from_size(0);

    for i in 0..self.field_list.len()
    {
      let  fld = &mut self.field_list[i];

      let  ty = &mut fld.ty;

        if !ty.complete(sym_tbl)
        {
          return false;
        }


      let  cur_ti = ty.get_info();

        if cur_ti.size != 0
        {
          ti.update_size( cur_ti.size );
          ti.update_align(cur_ti.align);
        }
    }


  self.info_opt = Some(ti);

  true
}


pub fn
print(&self)
{
    for fld in &self.field_list
    {
      print!("{}: ",&fld.name);

      fld.ty.print();

      print!("({}), ",fld.offset);
    }
}


}




#[derive(Clone)]
pub enum
Ty
{
  Void,
  Bool,

  U8, U16, U32, U64, USize,
  ULiteral,
  I8, I16, I32, I64, ISize,
  ILiteral,

  F32, F64,
  FLiteral,

  StringLiteral,

  FunctionPort(Box<Ty>,Vec<Ty>,bool),

      Pointer(Box<Ty>),
    Reference(Box<Ty>),
  Dereference(Box<Ty>),

  Array(Box<Ty>,Expression,Option<usize>),
  Tuple(TySet),
  Struct(TySet),
  Union(TySet),
  Enum(Vec<Enumerator>,bool),

  External(Path,Option<Box<Ty>>),

}


impl
Ty
{


pub fn
try_from(s: &str)-> Result<Ty,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::declaration::typesystem_dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"type_note",Some(dics))
    {
      let  cur = crate::syntax::Cursor::new(&dir);

        if let Some(t_dir) = cur.get_directory()
        {
//                  t_dir.print(0);

          return super::declaration::read_type::read_type(&t_dir);
        }
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
complete(&mut self, sym_tbl: &Vec<Symbol>)-> bool
{
    match self
    {
  Ty::Array(ty,e,sz_opt)=>
      {
          if let Some(_) = sz_opt
          {
            return true;
          }

        else
          if let Ok(sz) = e.get_size_value()
          {
            *sz_opt = Some(sz);

            return true;
          }


        false
      }
  Ty::Tuple(set)=>
      {
        set.complete_as_struct(sym_tbl)
      }
  Ty::Struct(set)=>
      {
        set.complete_as_struct(sym_tbl)
      }
  Ty::Union(set)=>
      {
        set.complete_as_union(sym_tbl)
      }
  Ty::Enum(ls,comp_flag)=>
      {
          if !*comp_flag
          {
          }


        true
      }
  Ty::FunctionPort(ret_ty,param_ty_ls,comp_flag)=>
      {
          if !*comp_flag
          {
              for ty in param_ty_ls
              {
                ty.print();
              }


            ret_ty.print();
          }


        true
      }
  Ty::External(path,ty_opt)=>
      {
          if let Some(ty) = ty_opt
          {
            return ty.complete(sym_tbl);
          }


        false
      }
  _=>{true}
    }
}


pub fn  is_void(&self)->  bool{if let Ty::Void = self{true} else{false}}
pub fn  is_bool(&self)->  bool{if let Ty::Bool = self{true} else{false}}
pub fn  is_u8(&self)->    bool{if let Ty::U8 = self{true} else{false}}
pub fn  is_u16(&self)->   bool{if let Ty::U16 = self{true} else{false}}
pub fn  is_u32(&self)->   bool{if let Ty::U32 = self{true} else{false}}
pub fn  is_u64(&self)->   bool{if let Ty::U64 = self{true} else{false}}
pub fn  is_usize(&self)-> bool{if let Ty::USize = self{true} else{false}}
pub fn  is_i8(&self)->    bool{if let Ty::I8 = self{true} else{false}}
pub fn  is_i16(&self)->   bool{if let Ty::I16 = self{true} else{false}}
pub fn  is_i32(&self)->   bool{if let Ty::I32 = self{true} else{false}}
pub fn  is_i64(&self)->   bool{if let Ty::I64 = self{true} else{false}}
pub fn  is_isize(&self)-> bool{if let Ty::ISize = self{true} else{false}}
pub fn  is_iltr(&self)-> bool{if let Ty::ILiteral = self{true} else{false}}
pub fn  is_ultr(&self)-> bool{if let Ty::ULiteral = self{true} else{false}}
pub fn  is_fltr(&self)-> bool{if let Ty::FLiteral = self{true} else{false}}
pub fn  is_f32(&self)->   bool{if let Ty::F32 = self{true} else{false}}
pub fn  is_f64(&self)->   bool{if let Ty::F64 = self{true} else{false}}
pub fn  is_sltr(&self)-> bool{if let Ty::StringLiteral = self{true} else{false}}

pub fn  is_pointer(&self)-> bool{if let Ty::Pointer(_) = self{true} else{false}}
pub fn  is_reference(&self)-> bool{if let Ty::Reference(_) = self{true} else{false}}
pub fn  is_dereference(&self)-> bool{if let Ty::Dereference(_) = self{true} else{false}}
pub fn  is_tuple(&self)-> bool{if let Ty::Tuple(_) = self{true} else{false}}
pub fn  is_struct(&self)-> bool{if let Ty::Struct(_) = self{true} else{false}}
pub fn  is_union(&self)-> bool{if let Ty::Union(_) = self{true} else{false}}
pub fn  is_enum(&self)-> bool{if let Ty::Enum(_,_) = self{true} else{false}}
pub fn  is_array(&self)-> bool{if let Ty::Array(_,_,_) = self{true} else{false}}
pub fn  is_function_port(&self)-> bool{if let Ty::FunctionPort(_,_,_) = self{true} else{false}}


pub fn
is_unsigned_integer(&self)-> bool
{
     self.is_u8()   
  || self.is_u16()  
  || self.is_u32()  
  || self.is_u64()  
  || self.is_usize()
  || self.is_ultr()
}


pub fn
is_signed_integer(&self)-> bool
{
     self.is_i8()   
  || self.is_i16()  
  || self.is_i32()  
  || self.is_i64()  
  || self.is_isize()
  || self.is_iltr()
}


pub fn
is_integer(&self)-> bool
{
       self.is_signed_integer()
  || self.is_unsigned_integer()
}


pub fn
is_floating(&self)-> bool
{
     self.is_f32()
  || self.is_f64()
  || self.is_fltr()
}


pub fn
is_word(&self)-> bool
{
       self.is_signed_integer()
  || self.is_unsigned_integer()
  ||         self.is_floating()
}


pub fn
get_info(&self)-> TyInfo
{
    match self
    {
  Ty::Void=>{TyInfo::from_size(0)}
  Ty::Bool=>{TyInfo::from_size(1)}
  Ty::I8=>   {TyInfo::from_size(1)}
  Ty::I16=>  {TyInfo::from_size(2)}
  Ty::I32=>  {TyInfo::from_size(4)}
  Ty::I64=>  {TyInfo::from_size(8)}
  Ty::ISize=>{TyInfo::from_size(8)}
  Ty::ILiteral=>{TyInfo::from_size(8)}
  Ty::U8=>   {TyInfo::from_size(1)}
  Ty::U16=>  {TyInfo::from_size(2)}
  Ty::U32=>  {TyInfo::from_size(4)}
  Ty::U64=>  {TyInfo::from_size(8)}
  Ty::USize=>{TyInfo::from_size(8)}
  Ty::ULiteral=>{TyInfo::from_size(8)}
  Ty::F32=>{TyInfo::from_size(4)}
  Ty::F64=>{TyInfo::from_size(8)}
  Ty::FLiteral=>{TyInfo::from_size(8)}
  Ty::Pointer(_)=>{TyInfo::from_size(8)}
  Ty::Reference(_)=>{TyInfo::from_size(8)}
  Ty::Dereference(_)=>{TyInfo::from_size(8)}
  Ty::Array(ty,e,sz_opt)=>
      {
          if let Some(sz) = sz_opt
          {
            let  base = ty.get_info();

            TyInfo{
               size: base.size*(*sz),
              align: base.align,
            }
          }

        else
          {
            panic!();
          }
      }
  Ty::Tuple(set)=>
      {
        panic!();
      }
  Ty::Struct(set)=>
      {
        panic!();
      }
  Ty::Union(set)=>
      {
        panic!();
      }
  Ty::Enum(_,_)=>{TyInfo::from_size(8)}
  Ty::FunctionPort(_,_,_)=>{TyInfo::from_size(8)}
  _=>{panic!();}
    }
}


pub fn
print(&self)
{
    match self
    {
  Ty::Void=>{print!("void");}
  Ty::Bool=>{print!("bool");}
  Ty::I8=>   {print!("i8");}
  Ty::I16=>  {print!("i16");}
  Ty::I32=>  {print!("i32");}
  Ty::I64=>  {print!("i64");}
  Ty::ISize=>{print!("isize");}
  Ty::ILiteral=>{print!("iltr");}
  Ty::U8=>   {print!("u8");}
  Ty::U16=>  {print!("u16");}
  Ty::U32=>  {print!("u32");}
  Ty::U64=>  {print!("u64");}
  Ty::USize=>{print!("usize");}
  Ty::ULiteral=>{print!("ultr");}
  Ty::F32=>{print!("f32");}
  Ty::F64=>{print!("f64");}
  Ty::FLiteral=>{print!("fltr");}
  Ty::StringLiteral=>{print!("strltr");}
  Ty::Pointer(ty)=>
      {
        print!("*");

        ty.print();
      }
  Ty::Reference(ty)=>
      {
        print!("&");

        ty.print();
      }
  Ty::Dereference(ty)=>
      {
        print!("*&");

        ty.print();
      }
  Ty::Array(ty,e,sz_opt)=>
      {
        ty.print();

        print!("[");

          if let Some(sz) = sz_opt
          {
            print!("{}",sz);
          }

        else
          {
            e.print();
          }


        print!("]");
      }
  Ty::Tuple(set)=>
      {
        print!("(");

        set.print();

        print!(")");
      }
  Ty::Struct(set)=>
      {
        print!("struct{{");

        set.print();

        print!("}}");
      }
  Ty::Union(set)=>
      {
        print!("union{{");

        set.print();

        print!("}}");
      }
  Ty::Enum(ls,_)=>
      {
      }
  Ty::FunctionPort(ret_ty,param_ty_ls,_)=>
      {
        print!("fn(");

          for ty in param_ty_ls
          {
            ty.print();

            print!(", ");
          }


        print!(")-> ");

        ret_ty.print();
      }
  Ty::External(path,_)=>
      {
        path.print();
      }
    }
}


}




#[derive(Clone)]
pub struct
Field
{
  pub(crate)   name: String,
  pub(crate)     ty: Ty,
  pub(crate)  index: usize,
  pub(crate) offset: usize,

}


impl
Field
{


pub fn
new()-> Self
{
  Self{
      name: String::new(),
        ty: Ty::Void,
     index: 0,
    offset: 0,
  }
}


}


#[derive(Clone)]
pub struct
Enumerator
{
  pub(crate)           name: String,
  pub(crate) expression_opt: Option<Expression>,
  pub(crate)      value_opt: Option<usize>,

}




