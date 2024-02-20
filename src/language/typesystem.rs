

use super::get_aligned_size;

use super::declaration::{
  Declaration,
  DeclarationLink,

};

use super::expression::{
  Expression,
  ExpressionKeeper,

};

pub const WORD_SIZE: usize = 8;




#[derive(Clone)]
pub struct
TypeItemKeeper
{
  pub(crate) type_item: Box<TypeItem>,
  pub(crate) type_info_opt: Option<TypeInfo>,

}


impl
TypeItemKeeper
{


pub fn
new(ti: TypeItem)-> TypeItemKeeper
{
  TypeItemKeeper{
    type_item: Box::new(ti),
    type_info_opt: None,
  }
}


pub fn
get_type_info_mut(&mut self, decln: &DeclarationLink)-> Option<&TypeInfo>
{
    if let None = &self.type_info_opt
    {
        if let Ok(ti) = self.type_item.try_get_info_mut(decln)
        {
          self.type_info_opt = Some(ti);
        }
    }


    if let Some(ti) = &self.type_info_opt
    {
      return Some(ti);
    }


  None
}


}




#[derive(Clone)]
pub enum
TypeItem
{
  ByName(String),

  Void,
  Bool,
  Char,
  U8, U16, U32, U64, USize,
  I8, I16, I32, I64, ISize,
  F32, F64,

  FunctionReference(TypeItemKeeper,Vec<Parameter>),

  Tuple(Vec<Parameter>),

  Pointer(TypeItemKeeper),
  Reference(TypeItemKeeper),

  Struct(Vec<Parameter>),
  Union(Vec<Parameter>),
  Enum(Vec<EnumParameter>),

}


impl
TypeItem
{


pub fn  is_void(&self)-> bool{if let Self::Void = self{true} else{false}}
pub fn  is_bool(&self)-> bool{if let Self::Bool = self{true} else{false}}
pub fn  is_char(&self)-> bool{if let Self::Char = self{true} else{false}}
pub fn  is_u8(&self)-> bool{if let Self::U8 = self{true} else{false}}
pub fn  is_u16(&self)-> bool{if let Self::U16 = self{true} else{false}}
pub fn  is_u32(&self)-> bool{if let Self::U32 = self{true} else{false}}
pub fn  is_u64(&self)-> bool{if let Self::U64 = self{true} else{false}}
pub fn  is_usize(&self)-> bool{if let Self::USize = self{true} else{false}}
pub fn  is_i8(&self)-> bool{if let Self::I8 = self{true} else{false}}
pub fn  is_i16(&self)-> bool{if let Self::I16 = self{true} else{false}}
pub fn  is_i32(&self)-> bool{if let Self::I32 = self{true} else{false}}
pub fn  is_i64(&self)-> bool{if let Self::I64 = self{true} else{false}}
pub fn  is_isize(&self)-> bool{if let Self::ISize = self{true} else{false}}
pub fn  is_f32(&self)-> bool{if let Self::F32 = self{true} else{false}}
pub fn  is_f64(&self)-> bool{if let Self::F64 = self{true} else{false}}

pub fn  is_signed_integer(&self)->   bool{self.is_i8() || self.is_i16() || self.is_i32() || self.is_i64() || self.is_isize()}
pub fn  is_unsigned_integer(&self)-> bool{self.is_u8() || self.is_u16() || self.is_u32() || self.is_u64() || self.is_usize()}
pub fn  is_floating(&self)-> bool{self.is_f32() || self.is_f64()}


pub fn
try_from(s: &str)-> Result<TypeItem,()>
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




/*
pub fn
get_field_list_info(ls: &Vec<FieldInfo>)-> FieldListInfo
{
  let  mut inf = FieldListInfo::default();

    for f in ls
    {
      inf.max_size  = std::cmp::max(inf.max_size ,f.type_info.size );
      inf.max_align = std::cmp::max(inf.max_align,f.type_info.align);
    }


    if let Some(last) = ls.last()
    {
      let  sz = last.offset+last.type_info.size;

      inf.total_size = get_aligned_size(sz);
    }


  inf
}


pub fn
try_get_field_list_mut(para_ls: &mut Vec<Parameter>, decln: &DeclarationLink)-> Result<Vec<Field>,()>
{
  let  mut ls: Vec<Field> = Vec::new();

  let  mut off: usize = 0;

    for para in para_ls
    {
        if let Ok(ti) = para.type_item_keeper.type_item.try_get_info_mut(decln)
        {
          let  sz = ti.size;

          let  f = Field{
                 name: para.name.clone(),
            type_info: ti,
                index: ls.len(),
               offset: off,
          };


          off = get_aligned_size(off+sz);

          ls.push(f);
        }

      else
        {
          return Err(());
        }
    }


  Ok(ls)
}
*/


pub fn
try_get_info_mut(&mut self, decln: &DeclarationLink)-> Result<TypeInfo,()>
{
/*
    match self
    {
  TypeItem::Void=>{Ok(TypeInfo::new(0))},
  TypeItem::Bool=>{Ok(TypeInfo::new(1))},
  TypeItem::Char=>{Ok(TypeInfo::new(4))},
  TypeItem::U8=>{Ok(TypeInfo::new(1))},
  TypeItem::U16=>{Ok(TypeInfo::new(2))},
  TypeItem::U32=>{Ok(TypeInfo::new(4))},
  TypeItem::U64=>{Ok(TypeInfo::new(8))},
  TypeItem::USize=>{Ok(TypeInfo::new(WORD_SIZE))},
  TypeItem::I8=>{Ok(TypeInfo::new(1))},
  TypeItem::I16=>{Ok(TypeInfo::new(2))},
  TypeItem::I32=>{Ok(TypeInfo::new(4))},
  TypeItem::I64=>{Ok(TypeInfo::new(8))},
  TypeItem::ISize=>{Ok(TypeInfo::new(WORD_SIZE))},
  TypeItem::F32=>{Ok(TypeInfo::new(4))},
  TypeItem::F64=>{Ok(TypeInfo::new(8))},

  TypeItem::ByName(s)=>
        {
            if let Some(decl) = decln.find(s)
            {
//                if let &decl.definition
                {
                }
            }


          return Err(());
        },

  TypeItem::FunctionReference(ret_ti,para_ls)=>
        {
            if let Ok(f_ls) = Self::try_get_field_list_mut(para_ls,decln)
            {
              let  f_ls_inf = Self::get_field_list_info(&f_ls);

              Ok(TypeInfo{
                 name: String::new(),
                 size: WORD_SIZE,
                align: WORD_SIZE,
                field_list: Vec::new(),
              })
            }

          else
            {
              Err(())
            }
        }
  TypeItem::Tuple(ls)=>
        {
            if let Ok(f_ls) = Self::try_get_field_list_mut(ls,decln)
            {
              let  f_ls_inf = Self::get_field_list_info(&f_ls);

              Ok(TypeInfo{
                 name: String::new(),
                 size: f_ls_inf.total_size,
                align: f_ls_inf.max_align,
                field_list: f_ls,
              })
            }

          else
            {
              Err(())
            }
        }
  TypeItem::Pointer(_)=>{Ok(TypeInfo::new(WORD_SIZE))},
  TypeItem::Reference(_)=>{Ok(TypeInfo::new(WORD_SIZE))},
  TypeItem::Struct(ls)=>
        {
            if let Ok(f_ls) = Self::try_get_field_list_mut(ls,decln)
            {
              let  f_ls_inf = Self::get_field_list_info(&f_ls);

              Ok(TypeInfo{
                 name: String::new(),
                 size: f_ls_inf.total_size,
                align: f_ls_inf.max_align,
                field_list: f_ls,
              })
            }

          else
            {
              Err(())
            }
        },
  TypeItem::Union(ls)=>
        {
            if let Ok(f_ls) = Self::try_get_field_list_mut(ls,decln)
            {
              let  f_ls_inf = Self::get_field_list_info(&f_ls);

              Ok(TypeInfo{
                 name: String::new(),
                 size: f_ls_inf.max_size,
                align: f_ls_inf.max_align,
                field_list: f_ls,
              })
            }

          else
            {
              Err(())
            }
        },
  TypeItem::Enum(_)=>
        {
          Err(())
        },
    }
*/

Err(())
}


pub fn
print_list(ls: &Vec<Parameter>)
{
    for para in ls
    {
      let  s = &para.name;

        if s.len() != 0
        {
          print!("{}: ",s);
        }


      para.type_item_keeper.type_item.print();

      print!(",");
    }
}


pub fn
print(&self)
{
    match self
    {
  TypeItem::Void=>{print!("void");},
  TypeItem::Bool=>{print!("bool");},
  TypeItem::Char=>{print!("char");},
  TypeItem::U8=>{print!("u8");},
  TypeItem::U16=>{print!("u16");},
  TypeItem::U32=>{print!("u32");},
  TypeItem::U64=>{print!("u64");},
  TypeItem::USize=>{print!("usize");},
  TypeItem::I8=>{print!("i8");},
  TypeItem::I16=>{print!("i16");},
  TypeItem::I32=>{print!("i32");},
  TypeItem::I64=>{print!("i64");},
  TypeItem::ISize=>{print!("isize");},
  TypeItem::F32=>{print!("f32");},
  TypeItem::F64=>{print!("f64");},

  TypeItem::ByName(s)=>{print!("{}",s);},

  TypeItem::FunctionReference(ret_ti,para_ls)=>
        {
          print!("fn(");

          Self::print_list(para_ls);

          print!(")-> ");

          ret_ti.type_item.print();
        }
  TypeItem::Tuple(ls)=>
        {
          print!("(");

          Self::print_list(ls);

          print!(")");
        }
  TypeItem::Pointer(ti)=>{  print!("*`");  ti.type_item.print();},
  TypeItem::Reference(ti)=>{  print!("&");  ti.type_item.print();},
  TypeItem::Struct(ls)=>
        {
          print!("struct{{\n");

          Self::print_list(ls);

          print!("}}");
        },
  TypeItem::Union(ls)=>
        {
          print!("union{{\n");

          Self::print_list(ls);

          print!("}}");
        },
  TypeItem::Enum(ls)=>
        {
          print!("enum{{\n");

            for para in ls
            {
              print!("{}: {},\n",&para.name,para.value);
            }


          print!("}}");
        },
    }
}


}




#[derive(Clone)]
pub struct
Parameter
{
  pub(crate)             name: String,
  pub(crate) type_item_keeper: TypeItemKeeper,

}


#[derive(Clone)]
pub struct
EnumParameter
{
  pub(crate)  name: String,
  pub(crate) value:  usize,

}


#[derive(Clone)]
pub struct
FieldInfo
{
  pub(crate)      name: String,
  pub(crate) type_info: TypeInfo,

  pub(crate)  index: usize,
  pub(crate) offset: usize,

}


#[derive(Default,Clone)]
pub struct
TypeInfo
{
  pub(crate)  name: String,
  pub(crate)  size: usize,
  pub(crate) align: usize,

  pub(crate) field_info_list: Vec<FieldInfo>,

}


impl
TypeInfo
{


pub fn
new(name: &str, sz: usize)-> TypeInfo
{
  TypeInfo{
          name: name.to_string(),
          size: sz,
         align: sz,
    field_info_list: Vec::new(),
  }
}


}


enum
TypeSymbol
{
  Void, Bool, Char,
  U8, U16, U32, U64, USize,
  I8, I16, I32, I64, ISize,
  F32, F64,
  Pointer, Reference, Tuple, Struct, Union, Enum,
  FunctionReference,

}


const SYM_BASE: u8 = 0x80;
const VOID_SYM: u8 = SYM_BASE+TypeSymbol::Void as u8;
const BOOL_SYM: u8 = SYM_BASE+TypeSymbol::Bool as u8;
const CHAR_SYM: u8 = SYM_BASE+TypeSymbol::Char as u8;
const    U8_SYM: u8 = SYM_BASE+TypeSymbol::U8    as u8;
const   U16_SYM: u8 = SYM_BASE+TypeSymbol::U16   as u8;
const   U32_SYM: u8 = SYM_BASE+TypeSymbol::U32   as u8;
const   U64_SYM: u8 = SYM_BASE+TypeSymbol::U64   as u8;
const USIZE_SYM: u8 = SYM_BASE+TypeSymbol::USize as u8;
const    I8_SYM: u8 = SYM_BASE+TypeSymbol::I8    as u8;
const   I16_SYM: u8 = SYM_BASE+TypeSymbol::I16   as u8;
const   I32_SYM: u8 = SYM_BASE+TypeSymbol::I32   as u8;
const   I64_SYM: u8 = SYM_BASE+TypeSymbol::I64   as u8;
const ISIZE_SYM: u8 = SYM_BASE+TypeSymbol::ISize as u8;
const   F32_SYM: u8 = SYM_BASE+TypeSymbol::F32   as u8;
const   F64_SYM: u8 = SYM_BASE+TypeSymbol::F64   as u8;
const   POINTER_SYM: u8 = SYM_BASE+TypeSymbol::Pointer as u8;
const REFERENCE_SYM: u8 = SYM_BASE+TypeSymbol::Reference as u8;
const     TUPLE_SYM: u8 = SYM_BASE+TypeSymbol::Tuple as u8;
const    STRUCT_SYM: u8 = SYM_BASE+TypeSymbol::Struct as u8;
const     UNION_SYM: u8 = SYM_BASE+TypeSymbol::Union as u8;
const      ENUM_SYM: u8 = SYM_BASE+TypeSymbol::Enum as u8;

const FUNCTION_REFERENCE_SYM: u8 = SYM_BASE+TypeSymbol::FunctionReference as u8;


pub struct
TypeCode(Vec<u8>);


impl
TypeCode
{


pub fn  new_void()-> TypeCode{TypeCode(vec![VOID_SYM])}
pub fn  new_bool()-> TypeCode{TypeCode(vec![BOOL_SYM])}
pub fn  new_char()-> TypeCode{TypeCode(vec![CHAR_SYM])}

pub fn  new_u8()-> TypeCode{TypeCode(vec![U8_SYM])}
pub fn  new_u16()-> TypeCode{TypeCode(vec![U16_SYM])}
pub fn  new_u32()-> TypeCode{TypeCode(vec![U32_SYM])}
pub fn  new_u64()-> TypeCode{TypeCode(vec![U64_SYM])}
pub fn  new_usize()-> TypeCode{TypeCode(vec![USIZE_SYM])}

pub fn  new_i8()-> TypeCode{TypeCode(vec![I8_SYM])}
pub fn  new_i16()-> TypeCode{TypeCode(vec![I16_SYM])}
pub fn  new_i32()-> TypeCode{TypeCode(vec![I32_SYM])}
pub fn  new_i64()-> TypeCode{TypeCode(vec![I64_SYM])}
pub fn  new_isize()-> TypeCode{TypeCode(vec![ISIZE_SYM])}

pub fn  new_f32()-> TypeCode{TypeCode(vec![F32_SYM])}
pub fn  new_f64()-> TypeCode{TypeCode(vec![F64_SYM])}


fn
write_name(&mut self, name: &str)
{
  let  l = name.len();

    if l > 255
    {
      panic!();
    }


  self.0.push(l as u8);

    for c in name.chars()
    {
      self.0.push(c as u8);
    }
}


fn
write_u32(&mut self, i: u32)
{
  self.0.push(((i>>24)&0xFF) as u8);
  self.0.push(((i>>16)&0xFF) as u8);
  self.0.push(((i>> 8)&0xFF) as u8);
  self.0.push(((i>> 0)&0xFF) as u8);
}


fn
write_i64(&mut self, i: i64)
{
  self.0.push(((i>>56)&0xFF) as u8);
  self.0.push(((i>>48)&0xFF) as u8);
  self.0.push(((i>>40)&0xFF) as u8);
  self.0.push(((i>>32)&0xFF) as u8);
  self.0.push(((i>>24)&0xFF) as u8);
  self.0.push(((i>>16)&0xFF) as u8);
  self.0.push(((i>> 8)&0xFF) as u8);
  self.0.push(((i>> 0)&0xFF) as u8);
}


fn
write_code(&mut self, src: &Vec<u8>)
{
  let  l = src.len();

    if l > 0xFFFF
    {
      panic!();
    }


  self.0.push(((l>>8)&0xFF) as u8);
  self.0.push(( l    &0xFF) as u8);

    for c in src
    {
      self.0.push(*c);
    }
}


fn
write_code_list(&mut self, src: &Vec<TypeCode>)
{
  let  n = src.len();

    if n > 0xFF
    {
      panic!();
    }


  self.0.push(n as u8);

    for tc in src
    {
      self.write_code(&tc.0);
    }
}


fn
write_field_code_list(&mut self, src: &Vec<(&str,TypeCode)>)
{
  let  mut n = src.len();

    if n > 0xFF
    {
      panic!();
    }


  self.0.push(n as u8);

    for (name,tc) in src
    {
      self.write_name(name);
      self.write_code(&tc.0);
    }
}




pub fn
to_pointer(&self)-> TypeCode
{
  let  mut tc = TypeCode(vec![POINTER_SYM]);

  tc.write_code(&self.0);

  tc
}


pub fn
to_reference(&self)-> TypeCode
{
  let  mut tc = TypeCode(vec![REFERENCE_SYM]);

  tc.write_code(&self.0);

  tc
}


pub fn
new_tuple(ls: Vec<TypeCode>)-> TypeCode
{
  let  mut tc = TypeCode(vec![TUPLE_SYM]);

  tc.write_code_list(&ls);

  tc
}


pub fn
new_struct(name: &str, ls: Vec<(&str,TypeCode)>)-> TypeCode
{
  let  mut tc = TypeCode(vec![STRUCT_SYM]);

  tc.write_name(name);

  tc.write_field_code_list(&ls);

  tc
}


pub fn
new_union(name: &str, ls: Vec<(&str,TypeCode)>)-> TypeCode
{
  let  mut tc = TypeCode(vec![UNION_SYM]);

  tc.write_name(name);

  tc.write_field_code_list(&ls);

  tc
}


pub fn
new_enum(name: &str, ls: Vec<(&str,i64)>)-> TypeCode
{
  let  mut tc = TypeCode(vec![ENUM_SYM]);

  tc.write_name(name);

  tc.write_u32(ls.len() as u32);

    for (name,i) in ls
    {
      tc.write_name(name);
      tc.write_i64(i);
    }


  tc
}


pub fn
new_function_reference(params: Vec<TypeCode>, ret_tc: TypeCode)-> TypeCode
{
  let  mut tc = TypeCode(vec![FUNCTION_REFERENCE_SYM]);

  tc.write_code_list(&params);
  tc.write_code(&ret_tc.0);

  tc
}


fn
get_pointer_pair(&self)-> (*const u8, *const u8)
{
  let  a = self.0.as_ptr();
  let  b = unsafe{a.clone().add(self.0.len())};

  (a,b)
}


fn
read_name(it: &mut *const u8, end: *const u8)-> Result<String,()>
{
    if *it != end
    {
      let  len = unsafe{**it};

      *it = unsafe{it.add(1)};

      let  mut s = String::new();

        for _ in 0..len
        {
            if *it != end
            {
              s.push(unsafe{**it} as char);

              *it = unsafe{it.add(1)};
            }

          else
            {
              return Err(());
            }
        }


      return Ok(s);
    }


  Err(())
}


fn
read_i64(it: &mut *const u8, end: *const u8)-> Result<i64,()>
{
  let  mut i: i64 = 0;

  unsafe{
    if *it != end{            i |= (**it) as i64;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as i64;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as i64;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as i64;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as i64;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as i64;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as i64;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as i64;  *it = it.add(1);
      return Ok(i);
    }}}}}}}}
  }


  Err(())
}


fn
read_u32(it: &mut *const u8, end: *const u8)-> Result<u32,()>
{
  let  mut i: u32 = 0;

  unsafe{
    if *it != end{            i |= (**it) as u32;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as u32;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as u32;  *it = it.add(1);
    if *it != end{  i <<= 8;  i |= (**it) as u32;  *it = it.add(1);
      return Ok(i);
    }}}}
  }


  Err(())
}


fn
read_code(it: &mut *const u8, end: *const u8)-> Result<TypeCode,()>
{
    if *it != end
    {
      let  mut len = (unsafe{**it} as usize)<<8;

      *it = unsafe{it.add(1)};

        if *it == end
        {
          panic!();
        }


      len |= (unsafe{**it} as usize);

      *it = unsafe{it.add(1)};

      let  mut s: Vec<u8> = Vec::new();

        for _ in 0..len
        {
            if *it != end
            {
              s.push(unsafe{**it});

              *it = unsafe{it.add(1)};
            }

          else
            {
              panic!();
            }
        }


      return Ok(TypeCode(s));
    }


  Err(())
}


fn
read_code_list(it: &mut *const u8, end: *const u8)-> Result<Vec<TypeCode>,()>
{
    if *it != end
    {
      let  n = unsafe{**it};

      *it = unsafe{it.add(1)};

        if *it == end
        {
          panic!();
        }


      let  mut ls: Vec<TypeCode> = Vec::new();

        for _ in 0..n
        {
            if let Ok(tc) = Self::read_code(it,end)
            {
              ls.push(tc);
            }

          else
            {
              return Err(());
            }
        }


      return Ok(ls);
    }


  Err(())
}


fn
read_field_code_list(it: &mut *const u8, end: *const u8)-> Result<Vec<(String,TypeCode)>,()>
{
    if *it != end
    {
      let  n = unsafe{**it};

      *it = unsafe{it.add(1)};

      let  mut ls: Vec<(String,TypeCode)> = Vec::new();

        for _ in 0..n
        {
            if let Ok(name) = Self::read_name(it,end)
            {
                if let Ok(tc) = Self::read_code(it,end)
                {
                  ls.push((name,tc));
                }
            }

          else
            {
              return Err(());
            }
        }


      return Ok(ls);
    }


  Err(())
}


pub fn
print(&self)
{
  let  (mut it,end) = self.get_pointer_pair();

  Self::print_internal(&mut it,end);
}


pub fn
print_internal(it: &mut *const u8, end: *const u8)
{
    if *it != end
    {
        match unsafe{**it}
        {
      VOID_SYM=>{print!("void");}
      BOOL_SYM=>{print!("bool");}
      CHAR_SYM=>{print!("char");}
      I8_SYM=>   {print!("i8" );}
      I16_SYM=>  {print!("i16");}
      I32_SYM=>  {print!("i32");}
      I64_SYM=>  {print!("i64");}
      ISIZE_SYM=>{print!("isize");}
      U8_SYM=>   {print!("u8" );}
      U16_SYM=>  {print!("u16");}
      U32_SYM=>  {print!("u32");}
      U64_SYM=>  {print!("u64");}
      USIZE_SYM=>{print!("usize");}
      F32_SYM=>{print!("f32");}
      F64_SYM=>{print!("f64");}
      POINTER_SYM=>
          {
            print!("*");

            *it = unsafe{it.add(1)};

              if let Ok(tc) = Self::read_code(it,end)
              {
                tc.print();
              }
          }
      REFERENCE_SYM=>
          {
            print!("&");

            *it = unsafe{it.add(1)};

              if let Ok(tc) = Self::read_code(it,end)
              {
                tc.print();
              }
          }
      TUPLE_SYM=>
          {
            print!("(");

            *it = unsafe{it.add(1)};

              if let Ok(ls) = Self::read_code_list(it,end)
              {
                  for tc in ls
                  {
                    tc.print();
                    println!(",");
                  }
              }


            print!(")");
          }
      STRUCT_SYM=>
          {
            *it = unsafe{it.add(1)};

              if let Ok(name) = Self::read_name(it,end)
              {
                print!("struct {}{{",&name);

                  if let Ok(ls) = Self::read_field_code_list(it,end)
                  {
                      for (name,tc) in ls
                      {
                        print!("{}: ",&name);
                        tc.print();
                        println!(",");
                      }
                  }


                print!("}}");
              }
          }
      UNION_SYM=>
          {
            *it = unsafe{it.add(1)};

              if let Ok(name) = Self::read_name(it,end)
              {
                print!("union {}{{",&name);

                  if let Ok(ls) = Self::read_field_code_list(it,end)
                  {
                      for (name,tc) in ls
                      {
                        print!("{}: ",&name);
                        tc.print();
                        println!(",");
                      }
                  }


                print!("}}");
              }
          }
      ENUM_SYM=>
          {
            *it = unsafe{it.add(1)};

              if let Ok(name) = Self::read_name(it,end)
              {
                print!("enum {}{{",&name);

                  if let Ok(n) = Self::read_u32(it,end)
                  {
                      for _ in 0..n
                      {
                          if let Ok(en_name) = Self::read_name(it,end)
                          {
                              if let Ok(en_i) = Self::read_i64(it,end)
                              {
                                println!("{} = {},",&en_name,en_i);
                              }
                          }
                      }
                  }


                print!("}}");
              }
          }
      FUNCTION_REFERENCE_SYM=>
          {
            *it = unsafe{it.add(1)};

              if let Ok(ls) = Self::read_code_list(it,end)
              {
                  if let Ok(ret_tc) = Self::read_code(it,end)
                  {
                    print!("fn(");

                      for tc in ls
                      {
                        tc.print();
                        println!(",");
                      }


                    print!(")-> ");

                    ret_tc.print();
                  }
              }
          }
      _=>{}
        }
    }
}


/*fn
read_type_info(it: &mut *const u8, end: *const u8)-> Result<TypeInfo,()>
{
    if *it != end
    {
        match unsafe{**it}
        {
      VOID_SYM=>{return Ok(TypeInfo::new("void",0));}
      BOOL_SYM=>{return Ok(TypeInfo::new("bool",1));}
      CHAR_SYM=>{return Ok(TypeInfo::new("char",1));}
      I8_SYM=>   {return Ok(TypeInfo::new("i8" ,  1));}
      I16_SYM=>  {return Ok(TypeInfo::new("i16",  2));}
      I32_SYM=>  {return Ok(TypeInfo::new("i32",  4));}
      I64_SYM=>  {return Ok(TypeInfo::new("i64",  8));}
      ISIZE_SYM=>{return Ok(TypeInfo::new("isize",8));}
      U8_SYM=>   {return Ok(TypeInfo::new("u8" ,  1));}
      U16_SYM=>  {return Ok(TypeInfo::new("u16",  2));}
      U32_SYM=>  {return Ok(TypeInfo::new("u32",  4));}
      U64_SYM=>  {return Ok(TypeInfo::new("u64",  8));}
      USIZE_SYM=>{return Ok(TypeInfo::new("usize",8));}
      F32_SYM=>{return Ok(TypeInfo::new("f32",4));}
      F64_SYM=>{return Ok(TypeInfo::new("f64",8));}
      TUPLE_SYM=>
          {
              if let Ok(ls) = Self::read_field_info_list(it,end)
              {
//                return Ok(TypeInfo::new("tuple",));
              }
          }
//      STRUCT_SYM=>{return Ok(TypeInfo::new("",));}
//      UNION_SYM=>{return Ok(TypeInfo::new("",));}
//      ENUM_SYM=>{return Ok(TypeInfo::new("",));}
      _=>{return Err(());}
        }
    }


  Err(())
}


pub fn
to_info(&self)-> Result<TypeInfo,()>
{
  let  mut it = self.0.as_ptr();
  let  end = unsafe{it.add(self.0.len())};

  Self::read_type_info(&mut it,end)
}
*/



}




