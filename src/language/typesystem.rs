

use super::get_aligned_size;

use super::declaration::{
  Declaration,
  Space,

};

use super::expression::{
  Expression,
  Path,

};

pub const WORD_SIZE: usize = 8;




#[derive(Clone)]
pub struct
FieldInfo
{
  pub(crate)      name: String,
  pub(crate) type_info: TypeInfo,

  pub(crate)  index: usize,
  pub(crate) offset: usize,

}


#[derive(Clone)]
pub enum
TypeInfoSubData
{
  None,
  External(Path,Option<Box<TypeInfo>>),
  Target(Box<TypeInfo>,Expression,Option<usize>),
  Named(Box<TypeInfo>,String),
  Signature(Vec<TypeInfo>,Box<TypeInfo>),
  FieldInfoList(String,Vec<FieldInfo>),

}


#[derive(Clone)]
pub struct
TypeInfo
{
  pub(crate) symbol: u8,

  pub(crate)  size: usize,
  pub(crate) align: usize,

  pub(crate) complete_flag: bool,

  pub(crate) sub_data: TypeInfoSubData,

}


impl
TypeInfo
{


pub fn
default()-> TypeInfo
{
  Self::new_void()
}


pub fn
new(sym: u8, sz: usize, comp: bool, subdat: TypeInfoSubData)-> TypeInfo
{
  TypeInfo{
        symbol: sym,
          size: sz,
         align: sz,
    complete_flag: comp,
    sub_data: subdat,
  }
}


pub fn  new_void()->  TypeInfo{Self::new( VOID_SYM,0,true,TypeInfoSubData::None)}
pub fn  new_bool()->  TypeInfo{Self::new( BOOL_SYM,1,true,TypeInfoSubData::None)}
pub fn  new_u8()->    TypeInfo{Self::new(   U8_SYM,1,true,TypeInfoSubData::None)}
pub fn  new_u16()->   TypeInfo{Self::new(  U16_SYM,2,true,TypeInfoSubData::None)}
pub fn  new_u32()->   TypeInfo{Self::new(  U32_SYM,4,true,TypeInfoSubData::None)}
pub fn  new_u64()->   TypeInfo{Self::new(  U64_SYM,8,true,TypeInfoSubData::None)}
pub fn  new_usize()-> TypeInfo{Self::new(USIZE_SYM,8,true,TypeInfoSubData::None)}
pub fn  new_i8()->    TypeInfo{Self::new(   I8_SYM,1,true,TypeInfoSubData::None)}
pub fn  new_i16()->   TypeInfo{Self::new(  I16_SYM,2,true,TypeInfoSubData::None)}
pub fn  new_i32()->   TypeInfo{Self::new(  I32_SYM,4,true,TypeInfoSubData::None)}
pub fn  new_i64()->   TypeInfo{Self::new(  I64_SYM,8,true,TypeInfoSubData::None)}
pub fn  new_isize()-> TypeInfo{Self::new(ISIZE_SYM,8,true,TypeInfoSubData::None)}
pub fn  new_ilit()-> TypeInfo{Self::new(ILIT_SYM,8,true,TypeInfoSubData::None)}
pub fn  new_ulit()-> TypeInfo{Self::new(ULIT_SYM,8,true,TypeInfoSubData::None)}
pub fn  new_flit()-> TypeInfo{Self::new(FLIT_SYM,8,true,TypeInfoSubData::None)}
pub fn  new_f32()->   TypeInfo{Self::new(  F32_SYM,4,true,TypeInfoSubData::None)}
pub fn  new_f64()->   TypeInfo{Self::new(  F64_SYM,8,true,TypeInfoSubData::None)}
pub fn  new_str_lit()-> TypeInfo{Self::new(STRLIT_SYM,16,true,TypeInfoSubData::None)}

pub fn
new_pointer(ti: TypeInfo)-> TypeInfo
{
  let  comp = ti.complete_flag;

  Self::new(POINTER_SYM,8,comp,TypeInfoSubData::Target(Box::new(ti),Expression::None,None))
}


pub fn
new_reference(ti: TypeInfo)-> TypeInfo
{
  let  comp = ti.complete_flag;

  Self::new(REFERENCE_SYM,8,comp,TypeInfoSubData::Target(Box::new(ti),Expression::None,None))
}


pub fn
new_array(ti: TypeInfo, n: usize)-> TypeInfo
{
  let  comp = ti.complete_flag;

  let   size = ti.size*n;
  let  align = ti.align;

  TypeInfo{
        symbol: ARRAY_SYM,
          size,
         align,
    complete_flag: comp,
    sub_data: TypeInfoSubData::Target(Box::new(ti),Expression::None,Some(n)),
  }
}


pub fn
new_tuple(mut ls: Vec<TypeInfo>)-> TypeInfo
{
  let  mut fi_ls: Vec<FieldInfo> = Vec::new();
  let  mut index = 0;

    for ti in ls
    {
      let  fi = FieldInfo{name: String::new(), type_info: ti, index, offset:0};

      fi_ls.push(fi);

      index  += 1;
    }


  TypeInfo{
        symbol: TUPLE_SYM,
          size: 0,
         align: 0,
    complete_flag: false,
    sub_data: TypeInfoSubData::FieldInfoList(String::new(),fi_ls),
  }
}


pub fn
new_struct(name: String, ls: Vec<(String,TypeInfo)>)-> TypeInfo
{
  let  mut fi_ls: Vec<FieldInfo> = Vec::new();
  let  mut index = 0;

    for e in ls
    {
      let  fi = FieldInfo{name: e.0, type_info: e.1, index, offset: 0};

      fi_ls.push(fi);

      index += 1;
    }


  TypeInfo{
        symbol: STRUCT_SYM,
          size: 0,
         align: 0,
    complete_flag: false,
    sub_data: TypeInfoSubData::FieldInfoList(name,fi_ls),
  }
}


pub fn
new_union(name: String, ls: Vec<(String,TypeInfo)>)-> TypeInfo
{
  let  mut fi_ls: Vec<FieldInfo> = Vec::new();
  let  mut index = 0;

    for e in ls
    {
      let  fi = FieldInfo{name: e.0, type_info: e.1, index, offset: 0};

      fi_ls.push(fi);

      index += 1;
    }


  TypeInfo{
        symbol: UNION_SYM,
          size: 0,
         align: 0,
    complete_flag: false,
    sub_data: TypeInfoSubData::FieldInfoList(name,fi_ls),
  }
}


pub fn
new_function_reference(ls: Vec<TypeInfo>, ret_ti: TypeInfo)-> TypeInfo
{
  let  mut param_ls: Vec<TypeInfo> = Vec::new();

    for e in ls
    {
      param_ls.push(e);
    }


  TypeInfo{
        symbol: FUNCTION_REFERENCE_SYM,
          size: 8,
         align: 8,
    complete_flag: false,
    sub_data: TypeInfoSubData::Signature(param_ls,Box::new(ret_ti)),
  }
}


pub fn
new_external(path: Path)-> TypeInfo
{
  TypeInfo{
        symbol: EXTERNAL_SYM,
          size: 0,
         align: 0,
    complete_flag: false,
    sub_data: TypeInfoSubData::External(path,None),
  }
}


pub fn
new_named(ti: TypeInfo, name: String)-> TypeInfo
{
  let   size = ti.size;
  let  align = ti.align;
  let  complete_flag = ti.complete_flag;

  TypeInfo{
        symbol: NAMED_SYM,
          size,
         align,
    complete_flag,
    sub_data: TypeInfoSubData::Named(Box::new(ti),name),
  }
}


pub fn
try_from(s: &str)-> Result<TypeInfo,()>
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




pub fn  is_void(&self)->  bool{self.symbol ==  VOID_SYM}
pub fn  is_bool(&self)->  bool{self.symbol ==  BOOL_SYM}
pub fn  is_u8(&self)->    bool{self.symbol ==    U8_SYM}
pub fn  is_u16(&self)->   bool{self.symbol ==   U16_SYM}
pub fn  is_u32(&self)->   bool{self.symbol ==   U32_SYM}
pub fn  is_u64(&self)->   bool{self.symbol ==   U64_SYM}
pub fn  is_usize(&self)-> bool{self.symbol == USIZE_SYM}
pub fn  is_i8(&self)->    bool{self.symbol ==    I8_SYM}
pub fn  is_i16(&self)->   bool{self.symbol ==   I16_SYM}
pub fn  is_i32(&self)->   bool{self.symbol ==   I32_SYM}
pub fn  is_i64(&self)->   bool{self.symbol ==   I64_SYM}
pub fn  is_isize(&self)-> bool{self.symbol == ISIZE_SYM}
pub fn  is_ilit(&self)-> bool{self.symbol == ILIT_SYM}
pub fn  is_ulit(&self)-> bool{self.symbol == ULIT_SYM}
pub fn  is_flit(&self)-> bool{self.symbol == FLIT_SYM}
pub fn  is_f32(&self)->   bool{self.symbol ==   F32_SYM}
pub fn  is_f64(&self)->   bool{self.symbol ==   F64_SYM}
pub fn  is_str_lit(&self)-> bool{self.symbol == STRLIT_SYM}

pub fn  is_pointer(&self)-> bool{self.symbol == POINTER_SYM}
pub fn  is_reference(&self)-> bool{self.symbol == REFERENCE_SYM}
pub fn  is_tuple(&self)-> bool{self.symbol == TUPLE_SYM}
pub fn  is_struct(&self)-> bool{self.symbol == STRUCT_SYM}
pub fn  is_union(&self)-> bool{self.symbol == UNION_SYM}
pub fn  is_enum(&self)-> bool{self.symbol == ENUM_SYM}
pub fn  is_array(&self)-> bool{self.symbol == ARRAY_SYM}
pub fn  is_function_reference(&self)-> bool{self.symbol == FUNCTION_REFERENCE_SYM}




pub fn
is_unsigned_integer(&self)-> bool
{
     self.is_u8()   
  || self.is_u16()  
  || self.is_u32()  
  || self.is_u64()  
  || self.is_usize()
  || self.is_ulit()
}


pub fn
is_signed_integer(&self)-> bool
{
     self.is_i8()   
  || self.is_i16()  
  || self.is_i32()  
  || self.is_i64()  
  || self.is_isize()
  || self.is_ilit()
}


pub fn
is_floating(&self)-> bool
{
     self.is_f32()
  || self.is_f64()
  || self.is_flit()
}


pub fn
is_integer(&self)-> bool
{
       self.is_signed_integer()
  || self.is_unsigned_integer()
}


pub fn
is_word(&self)-> bool
{
       self.is_signed_integer()
  || self.is_unsigned_integer()
  ||         self.is_floating()
}




pub fn
pointer_target(&self)-> Option<&Self>
{
    if self.is_pointer()
    {
        if let TypeInfoSubData::Target(ti_box,_,_) = &self.sub_data
        {
          return Some(&**ti_box);
        }
    }


  None
}


pub fn
reference_target(&self)-> Option<&Self>
{
    if self.is_reference()
    {
        if let TypeInfoSubData::Target(ti_box,_,_) = &self.sub_data
        {
          return Some(&**ti_box);
        }
    }


  None
}


pub fn
complete_for_struct(&mut self)-> bool
{
    if let TypeInfoSubData::FieldInfoList(_,ls) = &mut self.sub_data
    {
      let  mut max_align: usize = 0;
      let  mut    offset: usize = 0;

        for fld in ls
        {
            if fld.type_info.complete() == false
            {
              return false;
            }


          let   ti_size = fld.type_info.size;
          let  ti_align = fld.type_info.align;

            if ti_align != 0
            {
              offset = (offset+(ti_align-1))/ti_align*ti_align;
            }


          fld.offset = offset           ;
                       offset += ti_size;

          max_align = std::cmp::max(max_align,ti_align);
        }


      self.size  =    offset;
      self.align = max_align;
      self.complete_flag = true;
    }


  self.complete_flag
}


pub fn
complete_for_union(&mut self)-> bool
{
    if let TypeInfoSubData::FieldInfoList(_,ls) = &mut self.sub_data
    {
      let  mut max_align: usize = 0;
      let  mut  max_size: usize = 0;

        for fld in ls
        {
            if fld.type_info.complete() == false
            {
              return false;
            }


          let   ti_size = fld.type_info.size;
          let  ti_align = fld.type_info.align;

          max_size  = std::cmp::max(max_size ,ti_size );
          max_align = std::cmp::max(max_align,ti_align);
        }


      self.size  =  max_size;
      self.align = max_align;
      self.complete_flag = true;
    }


  self.complete_flag
}


pub fn
complete(&mut self)-> bool
{
    if self.complete_flag
    {
      return true;
    }


    match self.symbol
    {
  POINTER_SYM=>
      {
          if let TypeInfoSubData::Target(ti,_,_) = &mut self.sub_data
          {
            self.complete_flag = ti.complete();
          }
      }
  REFERENCE_SYM=>
      {
          if let TypeInfoSubData::Target(ti,_,_) = &mut self.sub_data
          {
            self.complete_flag = ti.complete();
          }
      }
  ARRAY_SYM=>
      {
          if let TypeInfoSubData::Target(ti,_,n_opt) = &mut self.sub_data
          {
              if let None = n_opt
              {
              }


              if let Some(n) = n_opt
              {
                self.complete_flag = ti.complete();

                self.size  = ti.size*(*n);
                self.align = ti.align;
              }
          }
      }
  TUPLE_SYM=> {self.complete_for_struct();}
  STRUCT_SYM=>{self.complete_for_struct();}
  UNION_SYM=> {self.complete_for_union();}
  ENUM_SYM=>
      {
      }
  FUNCTION_REFERENCE_SYM=>
      {
          if let TypeInfoSubData::Signature(param_ls,ret_ti) = &mut self.sub_data
          {
              for p in param_ls
              {
                  if p.complete() == false
                  {
                    return false;
                  }
              }


           self.complete_flag = ret_ti.complete();
          }
      }
  EXTERNAL_SYM=>
      {
          if let TypeInfoSubData::External(path,ti_opt) = &mut self.sub_data
          {
              if let Some(ti) = ti_opt
              {
              }
          }
      }
  _=>{}
    }


  self.complete_flag
}




fn
print_name(buf: &mut Vec<u8>, name: &str)
{
    for byte in name.as_bytes()
    {
      buf.push(*byte);
    }
}


fn
print_usize(buf: &mut Vec<u8>, n: usize)
{
  buf.push(((n>>56)&0xFF) as u8);
  buf.push(((n>>48)&0xFF) as u8);
  buf.push(((n>>40)&0xFF) as u8);
  buf.push(((n>>32)&0xFF) as u8);
  buf.push(((n>>24)&0xFF) as u8);
  buf.push(((n>>16)&0xFF) as u8);
  buf.push(((n>> 8)&0xFF) as u8);
  buf.push(((n>> 0)&0xFF) as u8);
}


fn
print_id(&self, buf: &mut Vec<u8>)
{
    if !self.complete_flag
    {
      panic!();
    }


  buf.push(self.symbol);

    match self.symbol
    {
  POINTER_SYM=>
      {
          if let TypeInfoSubData::Target(ti,_,_) = &self.sub_data
          {
            ti.print_id(buf);
          }
      }
  REFERENCE_SYM=>
      {
          if let TypeInfoSubData::Target(ti,_,_) = &self.sub_data
          {
            ti.print_id(buf);
          }
      }
  ARRAY_SYM=>
      {
          if let TypeInfoSubData::Target(ti,_,n_opt) = &self.sub_data
          {
            Self::print_usize(buf,n_opt.unwrap());

            ti.print_id(buf);
          }
      }
  TUPLE_SYM=>
      {
          if let TypeInfoSubData::FieldInfoList(name,ls) = &self.sub_data
          {
            Self::print_name(buf,name);

              for fi in ls
              {
                fi.type_info.print_id(buf);
              }
          }
      }
  STRUCT_SYM=>
      {
          if let TypeInfoSubData::FieldInfoList(name,ls) = &self.sub_data
          {
            Self::print_name(buf,name);

              for fi in ls
              {
                Self::print_name(buf,&fi.name);

                fi.type_info.print();
              }
          }
      }
  UNION_SYM=>
      {
          if let TypeInfoSubData::FieldInfoList(name,ls) = &self.sub_data
          {
            Self::print_name(buf,name);

              for fi in ls
              {
                Self::print_name(buf,&fi.name);

                fi.type_info.print();
              }
          }
      }
  ENUM_SYM=>
      {
      }
  FUNCTION_REFERENCE_SYM=>
      {
          if let TypeInfoSubData::Signature(param_ls,ret_ti) = &self.sub_data
          {
              for para in param_ls
              {
                para.print_id(buf);
              }


            ret_ti.print_id(buf);
          }
      }
  EXTERNAL_SYM=>
      {
          if let TypeInfoSubData::External(_,ti_opt) = &self.sub_data
          {
              if let Some(ti) = ti_opt
              {
                ti.print_id(buf);
              }
          }
      }
  _=>{}
    }
}


pub fn
get_id(&self)-> Vec<u8>
{
  let  mut buf: Vec<u8> = Vec::new();

  self.print_id(&mut buf);

  buf
}


pub fn
get_size(&self)-> usize
{
  self.size
}


pub fn
get_align(&self)-> usize
{
  self.align
}


pub fn
print(&self)
{
    match self.symbol
    {
  VOID_SYM=>{print!("void");}
  BOOL_SYM=>{print!("bool");}
  I8_SYM=>   {print!("i8");}
  I16_SYM=>  {print!("i16");}
  I32_SYM=>  {print!("i32");}
  I64_SYM=>  {print!("i64");}
  ISIZE_SYM=>{print!("isize");}
  ILIT_SYM=>{print!("ilit");}
  U8_SYM=>   {print!("u8");}
  U16_SYM=>  {print!("u16");}
  U32_SYM=>  {print!("u32");}
  U64_SYM=>  {print!("u64");}
  USIZE_SYM=>{print!("usize");}
  ULIT_SYM=>{print!("ulit");}
  F32_SYM=>{print!("f32");}
  F64_SYM=>{print!("f64");}
  FLIT_SYM=>{print!("flit");}
  POINTER_SYM=>
      {
        print!("*");

          if let TypeInfoSubData::Target(ti,_,_) = &self.sub_data
          {
            ti.print();
          }
      }
  REFERENCE_SYM=>
      {
        print!("&");

          if let TypeInfoSubData::Target(ti,_,_) = &self.sub_data
          {
            ti.print();
          }
      }
  ARRAY_SYM=>
      {
          if let TypeInfoSubData::Target(ti,e,n_opt) = &self.sub_data
          {
            ti.print();

            print!("[");

              if let Some(n) = n_opt
              {
                print!("{}",*n);
              }

            else
              {
                e.print();
              }


            print!("]");
          }
      }
  TUPLE_SYM=>
      {
        print!("(");

          if let TypeInfoSubData::FieldInfoList(_,ls) = &self.sub_data
          {
              for fi in ls
              {
                fi.type_info.print();

                print!(", ");
              }
          }

        print!(")");
      }
  STRUCT_SYM=>
      {
          if let TypeInfoSubData::FieldInfoList(name,ls) = &self.sub_data
          {
            print!("struct {}{{",name);

              for fi in ls
              {
                print!("{}: ",&fi.name);

                fi.type_info.print();

                print!("({}), ",fi.offset);
              }


            print!("}}(sz: {}, al: {})",self.size,self.align);
          }
      }
  UNION_SYM=>
      {
          if let TypeInfoSubData::FieldInfoList(name,ls) = &self.sub_data
          {
            print!("union {}{{",name);

              for fi in ls
              {
                print!("{}: ",&fi.name);

                fi.type_info.print();

                print!(", ");
              }


            print!("}}");
          }
      }
  ENUM_SYM=>
      {
      }
  FUNCTION_REFERENCE_SYM=>
      {
          if let TypeInfoSubData::Signature(param_ls,ret_ti) = &self.sub_data
          {
            print!("fn(");

              for para in param_ls
              {
                para.print();

                print!(", ");
              }


            print!(")-> ");

            ret_ti.print();
          }
      }
  EXTERNAL_SYM=>
      {
          if let TypeInfoSubData::External(path,_) = &self.sub_data
          {
            path.print();
          }
      }
  _=>{}
    }
}


}


enum
TypeSymbol
{
  Void, Bool,
  U8, U16, U32, U64, USize,
  I8, I16, I32, I64, ISize,
  ILiteral,
  ULiteral,
  F32, F64,
  FLiteral,
  StringLiteral,
  Pointer, Reference, Array, Tuple, Struct, Union, Enum,
  FunctionReference,
  External,
  Named,

}


const SYM_BASE: u8 = 0x80;
pub const VOID_SYM: u8 = SYM_BASE+TypeSymbol::Void as u8;
pub const BOOL_SYM: u8 = SYM_BASE+TypeSymbol::Bool as u8;
pub const    U8_SYM: u8 = SYM_BASE+TypeSymbol::U8    as u8;
pub const   U16_SYM: u8 = SYM_BASE+TypeSymbol::U16   as u8;
pub const   U32_SYM: u8 = SYM_BASE+TypeSymbol::U32   as u8;
pub const   U64_SYM: u8 = SYM_BASE+TypeSymbol::U64   as u8;
pub const USIZE_SYM: u8 = SYM_BASE+TypeSymbol::USize as u8;
pub const    I8_SYM: u8 = SYM_BASE+TypeSymbol::I8    as u8;
pub const   I16_SYM: u8 = SYM_BASE+TypeSymbol::I16   as u8;
pub const   I32_SYM: u8 = SYM_BASE+TypeSymbol::I32   as u8;
pub const   I64_SYM: u8 = SYM_BASE+TypeSymbol::I64   as u8;
pub const ISIZE_SYM: u8 = SYM_BASE+TypeSymbol::ISize as u8;
pub const ILIT_SYM: u8 = SYM_BASE+TypeSymbol::ILiteral as u8;
pub const ULIT_SYM: u8 = SYM_BASE+TypeSymbol::ULiteral as u8;
pub const   F32_SYM: u8 = SYM_BASE+TypeSymbol::F32   as u8;
pub const   F64_SYM: u8 = SYM_BASE+TypeSymbol::F64   as u8;
pub const FLIT_SYM: u8 = SYM_BASE+TypeSymbol::FLiteral as u8;
pub const STRLIT_SYM: u8 = SYM_BASE+TypeSymbol::StringLiteral as u8;
pub const   POINTER_SYM: u8 = SYM_BASE+TypeSymbol::Pointer as u8;
pub const REFERENCE_SYM: u8 = SYM_BASE+TypeSymbol::Reference as u8;
pub const     ARRAY_SYM: u8 = SYM_BASE+TypeSymbol::Array as u8;
pub const     TUPLE_SYM: u8 = SYM_BASE+TypeSymbol::Tuple as u8;
pub const    STRUCT_SYM: u8 = SYM_BASE+TypeSymbol::Struct as u8;
pub const     UNION_SYM: u8 = SYM_BASE+TypeSymbol::Union as u8;
pub const      ENUM_SYM: u8 = SYM_BASE+TypeSymbol::Enum as u8;

pub const FUNCTION_REFERENCE_SYM: u8 = SYM_BASE+TypeSymbol::FunctionReference as u8;
pub const EXTERNAL_SYM: u8 = SYM_BASE+TypeSymbol::External as u8;
pub const NAMED_SYM: u8 = SYM_BASE+TypeSymbol::Named as u8;




