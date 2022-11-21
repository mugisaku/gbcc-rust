

use std::rc::Rc;
use crate::syntax::parser::Directory;
use crate::syntax::parser::Cursor;


pub enum
PrimitiveTypeKind
{
  Null,
  NullPointer,
  GeneralPointer,
  Void,
  Boolean,
  Character,
  SignedInteger,
  Integer,
  Floating,
  Length,
  SignedLength,

}


pub struct
PrimitiveTypeInfo
{
  kind: PrimitiveTypeKind,

  size: usize,

  name: &'static str,

}


pub static  word_size: usize = 8;

pub static  bool_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Boolean, size: 1, name: "bool"};
pub static  char_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Character, size: 1, name: "char"};
pub static   s8_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::SignedInteger, size: 1, name: "s8"};
pub static  s16_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::SignedInteger, size: 2, name: "s16"};
pub static  s32_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::SignedInteger, size: 4, name: "s32"};
pub static  s64_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::SignedInteger, size: 8, name: "s64"};
pub static   u8_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Integer, size: 1, name: "u8"};
pub static  u16_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Integer, size: 2, name: "u16"};
pub static  u32_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Integer, size: 4, name: "u32"};
pub static  u64_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Integer, size: 8, name: "u64"};
pub static  f32_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Floating, size: 4, name: "f32"};
pub static  f64_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Floating, size: 8, name: "f64"};
pub static  null_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Null, size: 0, name: "null"};
pub static  nullptr_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::NullPointer, size: 0, name: "nullptr"};
pub static  geneptr_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::GeneralPointer, size: word_size, name: "geneptr"};
pub static  void_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Void, size: 0, name: "void"};
pub static  ulen_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::Length, size: word_size, name: "ulen"};
pub static  slen_pti: PrimitiveTypeInfo = PrimitiveTypeInfo{ kind: PrimitiveTypeKind::SignedLength, size: word_size, name: "slen"};

pub static  bool_ti: TypeInfo = TypeInfo::Primitive(&bool_pti);
pub static  char_ti: TypeInfo = TypeInfo::Primitive(&char_pti);
pub static   s8_ti: TypeInfo = TypeInfo::Primitive(&s8_pti);
pub static  s16_ti: TypeInfo = TypeInfo::Primitive(&s16_pti);
pub static  s32_ti: TypeInfo = TypeInfo::Primitive(&s32_pti);
pub static  s64_ti: TypeInfo = TypeInfo::Primitive(&s64_pti);
pub static   u8_ti: TypeInfo = TypeInfo::Primitive(&u8_pti);
pub static  u16_ti: TypeInfo = TypeInfo::Primitive(&u16_pti);
pub static  u32_ti: TypeInfo = TypeInfo::Primitive(&u32_pti);
pub static  u64_ti: TypeInfo = TypeInfo::Primitive(&u64_pti);
pub static  f32_ti: TypeInfo = TypeInfo::Primitive(&f32_pti);
pub static  f64_ti: TypeInfo = TypeInfo::Primitive(&f64_pti);
pub static  null_ti: TypeInfo = TypeInfo::Primitive(&null_pti);
pub static  nullptr_ti: TypeInfo = TypeInfo::Primitive(&nullptr_pti);
pub static  geneptr_ti: TypeInfo = TypeInfo::Primitive(&geneptr_pti);
pub static  void_ti: TypeInfo = TypeInfo::Primitive(&void_pti);
pub static  ulen_ti: TypeInfo = TypeInfo::Primitive(&ulen_pti);
pub static  slen_ti: TypeInfo = TypeInfo::Primitive(&slen_pti);


pub fn
get_aligned_size(sz: usize)-> usize
{
  (sz+(word_size-1))/word_size*word_size
}


pub fn
get_max(a: usize, b: usize)-> usize
{
  if a <= b{b} else{a}
}


pub enum
TypeInfo
{
  Primitive(&'static PrimitiveTypeInfo),

  Array(Box<TypeInfo>,usize),
  Pointer(Box<TypeInfo>),
  Reference(Box<TypeInfo>),

  Struct(StructInfo),
  Union(UnionInfo),
  Enum(EnumInfo),
  Function(FunctionInfo),

}


impl
Clone for TypeInfo
{


fn
clone(&self)-> Self
{
    match self
    {
  TypeInfo::Primitive(p)=> {return TypeInfo::Primitive(p);},
  TypeInfo::Array(ti,n)=>  {return TypeInfo::Array(ti.clone(),*n);},
  TypeInfo::Pointer(ti)=>  {return TypeInfo::Pointer(ti.clone());},
  TypeInfo::Reference(ti)=>{return TypeInfo::Reference(ti.clone());},
  TypeInfo::Struct(s)=>    {return TypeInfo::Struct(s.clone());},
  TypeInfo::Union(u)=>     {return TypeInfo::Union(u.clone());},
  TypeInfo::Enum(e)=>      {return TypeInfo::Enum(e.clone());},
  TypeInfo::Function(f)=>  {return TypeInfo::Function(f.clone());},
    }
}


}


impl
TypeInfo
{


pub fn
from(dir: &Directory)-> TypeInfo
{
  let mut  cur = Cursor::from(dir);

    if let Some(rcs) = cur.get_identifier()
    {
      let  s = rcs.as_str();

           if s ==  "s8"{return s8_ti.clone();}
      else if s == "s16"{return s16_ti.clone();}
      else if s == "s32"{return s32_ti.clone();}
      else if s == "s64"{return s64_ti.clone();}
      else if s ==  "u8"{return u8_ti.clone();}
      else if s == "u16"{return u16_ti.clone();}
      else if s == "u32"{return u32_ti.clone();}
      else if s == "u64"{return u64_ti.clone();}
      else if s == "f32"{return f32_ti.clone();}
      else if s == "f64"{return f64_ti.clone();}
      else if s == "bool"{return bool_ti.clone();}
      else if s == "char"{return char_ti.clone();}
      else if s == "ulen"{return ulen_ti.clone();}
      else if s == "slen"{return slen_ti.clone();}
      else if s == "null"{return null_ti.clone();}
      else if s == "void"{return void_ti.clone();}
      else if s == "nullptr"{return nullptr_ti.clone();}
      else if s == "geneptr"{return geneptr_ti.clone();}
    }


  null_ti.clone()
}


pub fn
get_size(&self)-> usize
{
    match self
    {
  TypeInfo::Primitive(p)=>{return p.size;},

  TypeInfo::Array(ti,n)=>{return ti.get_size()*n;},
  TypeInfo::Pointer(ti)=>{return word_size;},
  TypeInfo::Reference(ti)=>{return word_size;},

  TypeInfo::Struct(s)=>{return s.get_size();},
  TypeInfo::Union(u)=>{return u.get_size();},
  TypeInfo::Enum(e)=>{return e.get_size();},
  TypeInfo::Function(f)=>{return word_size;},
    }
}


pub fn
get_align(&self)-> usize
{
    match self
    {
  TypeInfo::Primitive(p)=>{return p.size;},

  TypeInfo::Array(ti,n)=>{return ti.get_align();},
  TypeInfo::Pointer(ti)=>{return word_size;},
  TypeInfo::Reference(ti)=>{return word_size;},

  TypeInfo::Struct(s)=>{return s.get_align();},
  TypeInfo::Union(u)=>{return u.get_align();},
  TypeInfo::Enum(e)=>{return e.get_align();},
  TypeInfo::Function(f)=>{return word_size;},
    }
}


pub fn
print(&self)
{
    match self
    {
  TypeInfo::Primitive(p)=>{print!("{}",p.name);},

  TypeInfo::Array(ti,n)=>
        {
          ti.print();
          print!("[{}]",n);
        },
  TypeInfo::Pointer(ti)=>
        {
          ti.print();
          print!("*");
        },
  TypeInfo::Reference(ti)=>
        {
          ti.print();
          print!("&");
        },

  TypeInfo::Struct(s)=>{s.print();},
  TypeInfo::Union(u)=>{u.print();},
  TypeInfo::Enum(e)=>{e.print();},
  TypeInfo::Function(f)=>{f.print();},
    }
}


}


pub struct
Member
{
       name: String,
  type_info: Box<TypeInfo>,

  offset: usize,

}


impl
Clone for Member
{


fn
clone(&self)-> Self
{
  Member{ name: self.name.clone(), type_info: self.type_info.clone(), offset: self.offset}
}


}


pub struct
StructInfo
{
  member_list: Vec<Member>,

   size: usize,
  align: usize,

}


impl
Clone for StructInfo
{


fn
clone(&self)-> Self
{
  StructInfo{ member_list: self.member_list.clone(), size: self.size, align: self.align}
}


}


impl
StructInfo
{


pub fn
new()-> StructInfo
{
  StructInfo{ member_list: Vec::new(), size: 0, align: 0}
}


pub fn
push(&mut self, name: String, type_info: Box<TypeInfo>)
{
  let  offset = self.size                                                ;
                self.size = get_aligned_size(offset+type_info.get_size());

  self.align = get_max(self.align,type_info.get_align());

  self.member_list.push(Member{ name, type_info, offset});
}


pub fn   get_size(&self)-> usize{return self.size;}
pub fn  get_align(&self)-> usize{return self.align;}


pub fn
print(&self)
{
}


}




pub struct
UnionInfo
{
  member_list: Vec<Member>,

   size: usize,
  align: usize,

}


impl
Clone for UnionInfo
{


fn
clone(&self)-> Self
{
  UnionInfo{ member_list: self.member_list.clone(), size: self.size, align: self.align}
}


}


impl
UnionInfo
{


pub fn
new()-> UnionInfo
{
  UnionInfo{ member_list: Vec::new(), size: 0, align: 0}
}


pub fn
push(&mut self, name: String, type_info: Box<TypeInfo>)
{
  self.size = get_aligned_size(type_info.get_size());

  self.align = get_max(self.align,type_info.get_align());

  self.member_list.push(Member{ name, type_info, offset: 0});
}


pub fn   get_size(&self)-> usize{return self.size;}
pub fn  get_align(&self)-> usize{return self.align;}


pub fn
print(&self)
{
}


}




pub struct
Enumerator
{
  name: Rc<String>,

  value: i64,

}


pub struct
EnumInfo
{
  member_list: Vec<Member>,

   size: usize,
  align: usize,

}


impl
Clone for EnumInfo
{


fn
clone(&self)-> Self
{
  EnumInfo{ member_list: self.member_list.clone(), size: self.size, align: self.align}
}


}


impl
EnumInfo
{


pub fn
new()-> EnumInfo
{
  EnumInfo{ member_list: Vec::new(), size: 0, align: 0}
}


pub fn   get_size(&self)-> usize{return self.size;}
pub fn  get_align(&self)-> usize{return self.align;}


pub fn
print(&self)
{
}


}




pub struct
FunctionInfo
{
  type_info: Box<TypeInfo>,

  parameter_list: StructInfo,

}



impl
Clone for FunctionInfo
{


fn
clone(&self)-> Self
{
  FunctionInfo{ type_info: self.type_info.clone(), parameter_list: self.parameter_list.clone()}
}


}


impl
FunctionInfo
{


pub fn
print(&self)
{
}


}




