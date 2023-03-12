

use crate::syntax::parser::Directory;
use crate::syntax::parser::Cursor;
use crate::syntax::parser::ObjectData;
use crate::language::expression::Expression;




pub static  word_size: usize = 8;

pub static  bool_ti: TypeInfo = TypeInfo::Boolean(1);
pub static  char_ti: TypeInfo = TypeInfo::Character(1);
pub static   s8_ti: TypeInfo = TypeInfo::SignedInteger(1);
pub static  s16_ti: TypeInfo = TypeInfo::SignedInteger(2);
pub static  s32_ti: TypeInfo = TypeInfo::SignedInteger(4);
pub static  s64_ti: TypeInfo = TypeInfo::SignedInteger(8);
pub static   u8_ti: TypeInfo = TypeInfo::Integer(1);
pub static  u16_ti: TypeInfo = TypeInfo::Integer(2);
pub static  u32_ti: TypeInfo = TypeInfo::Integer(4);
pub static  u64_ti: TypeInfo = TypeInfo::Integer(8);
pub static  f32_ti: TypeInfo = TypeInfo::Floating(4);
pub static  f64_ti: TypeInfo = TypeInfo::Floating(8);
pub static  null_ti: TypeInfo = TypeInfo::Null;
pub static  nullptr_ti: TypeInfo = TypeInfo::NullPointer;
pub static  geneptr_ti: TypeInfo = TypeInfo::GeneralPointer;
pub static  void_ti: TypeInfo = TypeInfo::Void;
pub static  ulen_ti: TypeInfo = TypeInfo::Length;
pub static  slen_ti: TypeInfo = TypeInfo::SignedLength;


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
  Null,
  NullPointer,
  GeneralPointer,
  Void,
  Boolean(usize),
  Character(usize),
  SignedInteger(usize),
  Integer(usize),
  Floating(usize),
  Length,
  SignedLength,

  Array(Box<TypeInfo>,usize),
  Pointer(Box<TypeInfo>),
  Reference(Box<TypeInfo>),
  Function(Box<FunctionInfo>),

  Struct(StructInfo),
  Union(UnionInfo),
  Enum(EnumInfo),

}


impl
PartialEq for TypeInfo
{


fn
eq(&self, other: &Self)-> bool
{
  let  a =  self.make_id();
  let  b = other.make_id();

  a == b
}


}


impl
Clone for TypeInfo
{


fn
clone(&self)-> Self
{
    match self
    {
  TypeInfo::Null=>             {return TypeInfo::Null;},
  TypeInfo::NullPointer=>      {return TypeInfo::NullPointer;},
  TypeInfo::GeneralPointer=>   {return TypeInfo::GeneralPointer;},
  TypeInfo::Void=>             {return TypeInfo::Void;},
  TypeInfo::Boolean(sz)=>      {return TypeInfo::Boolean(*sz);},
  TypeInfo::Character(sz)=>    {return TypeInfo::Character(*sz);},
  TypeInfo::SignedInteger(sz)=>{return TypeInfo::SignedInteger(*sz);},
  TypeInfo::Integer(sz)=>      {return TypeInfo::Integer(*sz);}
  TypeInfo::Floating(sz)=>     {return TypeInfo::Floating(*sz);}
  TypeInfo::Length=>           {return TypeInfo::Length;}
  TypeInfo::SignedLength=>     {return TypeInfo::SignedLength;}
  TypeInfo::Array(ti,n)=>      {return TypeInfo::Array(ti.clone(),*n);},
  TypeInfo::Pointer(ti)=>      {return TypeInfo::Pointer(ti.clone());},
  TypeInfo::Reference(ti)=>    {return TypeInfo::Reference(ti.clone());},
  TypeInfo::Struct(s)=>        {return TypeInfo::Struct(s.clone());},
  TypeInfo::Union(u)=>         {return TypeInfo::Union(u.clone());},
  TypeInfo::Enum(e)=>          {return TypeInfo::Enum(e.clone());},
  TypeInfo::Function(f)=>      {return TypeInfo::Function(f.clone());},
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
  TypeInfo::Null=>             {return 0;},
  TypeInfo::NullPointer=>      {return 0;},
  TypeInfo::GeneralPointer=>   {return word_size;},
  TypeInfo::Void=>             {return 0;},
  TypeInfo::Boolean(sz)=>      {return *sz;},
  TypeInfo::Character(sz)=>    {return *sz;},
  TypeInfo::SignedInteger(sz)=>{return *sz;},
  TypeInfo::Integer(sz)=>      {return *sz;},
  TypeInfo::Floating(sz)=>     {return *sz;},
  TypeInfo::Length=>           {return word_size;},
  TypeInfo::SignedLength=>     {return word_size;},

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
  TypeInfo::Array(ti,n)=>{return ti.get_align();},
  TypeInfo::Pointer(ti)=>{return word_size;},
  TypeInfo::Reference(ti)=>{return word_size;},

  TypeInfo::Struct(s)=>{return s.get_align();},
  TypeInfo::Union(u)=>{return u.get_align();},
  TypeInfo::Enum(e)=>{return e.get_align();},
  TypeInfo::Function(f)=>{return word_size;},
  _=>{},
    }


  self.get_size()
}


pub fn
make_id(&self)-> String
{
  let mut  buf = String::new();

  self.print_id(&mut buf);

  buf
}


pub fn
print_id(&self, buf: &mut String)
{
    match self
    {
  TypeInfo::Null=>             {buf.push_str("null");},
  TypeInfo::NullPointer=>      {buf.push_str("nullptr");},
  TypeInfo::GeneralPointer=>   {buf.push_str("geneptr");},
  TypeInfo::Void=>             {buf.push_str("void");},
  TypeInfo::Boolean(sz)=>      {buf.push_str(format!("bool{}",8*sz).as_str());},
  TypeInfo::Character(sz)=>    {buf.push_str(format!("char{}",8*sz).as_str());},
  TypeInfo::SignedInteger(sz)=>{buf.push_str(format!("i{}",8*sz).as_str());},
  TypeInfo::Integer(sz)=>      {buf.push_str(format!("u{}",8*sz).as_str());},
  TypeInfo::Floating(sz)=>     {buf.push_str(format!("f{}",8*sz).as_str());},
  TypeInfo::Length=>           {buf.push_str("len");},
  TypeInfo::SignedLength=>     {buf.push_str("slen");},

  TypeInfo::Array(ti,n)=>
        {
          ti.print_id(buf);

          let  t = format!("[{}]",n);

          buf.push_str(&t);
        },
  TypeInfo::Pointer(ti)=>
        {
          ti.print_id(buf);

          buf.push('*');
        },
  TypeInfo::Reference(ti)=>
        {
          ti.print_id(buf);

          buf.push('&');
        },

  TypeInfo::Struct(s)=>{s.print_id(buf);},
  TypeInfo::Union(u)=>{u.print_id(buf);},
  TypeInfo::Enum(e)=>{e.print_id(buf);},
  TypeInfo::Function(f)=>{f.print_id(buf);},
    }
}


pub fn
print(&self)
{
    match self
    {
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
  _=>{}
    }


  print!("{}",self.make_id());
}


}


pub struct
Parameter
{
  name: String,

  type_info: TypeInfo,

  offset: usize,

}


impl
Clone for Parameter
{


fn
clone(&self)-> Self
{
  Parameter{ name: self.name.clone(), type_info: self.type_info.clone(), offset: self.offset}
}


}


impl
Parameter
{


pub fn
new(name: &str, ti: TypeInfo, off: usize)-> Parameter
{
  Parameter{ name: String::from(name), type_info: ti, offset: off}
}


pub fn
get_name(&self)-> &str
{
  &self.name
}


pub fn
get_type_info(&self)-> &TypeInfo
{
  &self.type_info
}


pub fn
get_offset(&self)-> usize
{
  self.offset
}


pub fn
print(&self)
{
  print!("{}: ",self.name);

  self.type_info.print();

  print!("(offset: {})",self.offset);
}


}




pub struct
StructInfo
{
  member_list: Vec<Parameter>,

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
push(&mut self, name: String, type_info: TypeInfo)
{
  let  offset = self.size                                                ;
                self.size = get_aligned_size(offset+type_info.get_size());

  self.align = get_max(self.align,type_info.get_align());

  self.member_list.push(Parameter{ name, type_info, offset});
}


pub fn   get_size(&self)-> usize{self.size}
pub fn  get_align(&self)-> usize{self.align}

pub fn  get_member_list(&self)-> &Vec<Parameter>{&self.member_list}


pub fn
print_id(&self, buf: &mut String)
{
    for m in &self.member_list
    {
      m.type_info.print_id(buf);
    }
}


pub fn
print(&self)
{
  print!("struct{{");

    for m in &self.member_list
    {
      m.print();
      print!(",\n");
    }


  print!("}}");
}


}




pub struct
UnionInfo
{
  member_list: Vec<Parameter>,

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
push(&mut self, name: String, type_info: TypeInfo)
{
  self.size = get_aligned_size(type_info.get_size());

  self.align = get_max(self.align,type_info.get_align());

  self.member_list.push(Parameter{ name, type_info, offset: 0});
}


pub fn   get_size(&self)-> usize{return self.size;}
pub fn  get_align(&self)-> usize{return self.align;}


pub fn
print_id(&self, buf: &mut String)
{
    for m in &self.member_list
    {
      m.type_info.print_id(buf);
    }
}


pub fn
print(&self)
{
  print!("union{{");

    for m in &self.member_list
    {
      m.print();
      print!(",\n");
    }


  print!("}}");
}


}




pub struct
Enumerator
{
  name: String,

  value: i64,

}


pub struct
EnumInfo
{
  member_list: Vec<Parameter>,

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
print_id(&self, buf: &mut String)
{
    for m in &self.member_list
    {
      m.type_info.print_id(buf);
    }
}


pub fn
print(&self)
{
}


}




pub struct
FunctionInfo
{
  parameter_list: Vec<Parameter>,

  return_type_info: TypeInfo,

}



impl
Clone for FunctionInfo
{


fn
clone(&self)-> Self
{
  FunctionInfo{ parameter_list: self.parameter_list.clone(), return_type_info: self.return_type_info.clone()}
}


}


impl
FunctionInfo
{


pub fn
new(parals: Vec<Parameter>, retti: TypeInfo)-> FunctionInfo
{
  FunctionInfo{ parameter_list: parals, return_type_info: retti}
}


pub fn
get_return_type_info(&self)-> &TypeInfo
{
  &self.return_type_info
}


pub fn
get_parameter_list(&self)-> &Vec<Parameter>
{
  &self.parameter_list
}


pub fn
get_stack_size(&self)-> usize
{
  let mut  sz = get_aligned_size(self.return_type_info.get_size());

    for p in &self.parameter_list
    {
      sz += get_aligned_size(p.get_type_info().get_size());
    }


  sz
}


pub fn
print_id(&self, buf: &mut String)
{
  buf.push_str("fn");

    for p in &self.parameter_list
    {
      p.get_type_info().print_id(buf);
    }


  self.return_type_info.print_id(buf);
}


pub fn
print(&self)
{
  print!("(");

    for p in &self.parameter_list
    {
      p.print();
    }


  print!(")-> ");

  self.return_type_info.print();
}


}




