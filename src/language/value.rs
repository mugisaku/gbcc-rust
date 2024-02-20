

use super::typesystem::{
  TypeItem,
  TypeItemKeeper,
  TypeInfo,

};


use super::declaration::{
  Declaration,
  DeclarationLink,

};


use super::expression::{
  PostfixOperator,
  PrefixOperator,
  BinaryOperator,
  Expression,
  ExpressionKeeper,

};




#[derive(Clone)]
pub enum
ValueData
{
  None,

  I64(i64),
  U64(u64),
  F64(f64),

  Image(String),

}


#[derive(Clone)]
pub struct
Value
{
  pub(crate) type_item: TypeItem,
  pub(crate)      data: ValueData,

}


impl
Value
{


pub fn
new_void()-> Value
{
  Value{
    type_item: TypeItem::Void,
    data: ValueData::None,
  }
}


pub fn
new_bool(b: bool)-> Value
{
  Value{
    type_item: TypeItem::U64,
    data: ValueData::U64(if b{1} else{0}),
  }
}


pub fn
new_char(c: char)-> Value
{
  Value{
    type_item: TypeItem::Char,
    data: ValueData::U64(c as u64),
  }
}


pub fn
new_u8(u: u64)-> Value
{
  Value{
    type_item: TypeItem::U8,
    data: ValueData::U64((u as u8).into()),
  }
}


pub fn
new_u16(u: u64)-> Value
{
  Value{
    type_item: TypeItem::U16,
    data: ValueData::U64((u as u16).into()),
  }
}


pub fn
new_u32(u: u64)-> Value
{
  Value{
    type_item: TypeItem::U32,
    data: ValueData::U64((u as u32).into()),
  }
}


pub fn
new_u64(u: u64)-> Value
{
  Value{
    type_item: TypeItem::U64,
    data: ValueData::U64(u),
  }
}


pub fn
new_usize(u: u64)-> Value
{
  Value{
    type_item: TypeItem::USize,
    data: ValueData::U64(u),
  }
}


pub fn
new_i8(i: i64)-> Value
{
  Value{
    type_item: TypeItem::I8,
    data: ValueData::I64((i as i8).into()),
  }
}


pub fn
new_i16(i: i64)-> Value
{
  Value{
    type_item: TypeItem::I16,
    data: ValueData::I64((i as i16).into()),
  }
}


pub fn
new_i32(i: i64)-> Value
{
  Value{
    type_item: TypeItem::I32,
    data: ValueData::I64((i as i32).into()),
  }
}


pub fn
new_i64(i: i64)-> Value
{
  Value{
    type_item: TypeItem::I64,
    data: ValueData::I64(i),
  }
}


pub fn
new_isize(i: i64)-> Value
{
  Value{
    type_item: TypeItem::ISize,
    data: ValueData::I64(i),
  }
}


pub fn
new_f32(f: f64)-> Value
{
  Value{
    type_item: TypeItem::F32,
    data: ValueData::F64((f as f32).into()),
  }
}


pub fn
new_f64(f: f64)-> Value
{
  Value{
    type_item: TypeItem::F64,
    data: ValueData::F64(f),
  }
}


pub fn
new_pointer(v: u64, ti: TypeItemKeeper)-> Value
{
  Value{
    type_item: TypeItem::Pointer(ti),
    data: ValueData::U64(v),
  }
}


pub fn
new_reference(v: u64, ti: TypeItemKeeper)-> Value
{
  Value{
    type_item: TypeItem::Reference(ti),
    data: ValueData::U64(v),
  }
}




pub fn
get_u64(&self)-> u64
{
    if let ValueData::U64(u) = self.data
    {
      return u;
    }


  panic!();
}


pub fn
get_i64(&self)-> i64
{
    if let ValueData::I64(i) = self.data
    {
      return i;
    }


  panic!();
}


pub fn
get_f64(&self)-> f64
{
    if let ValueData::F64(f) = self.data
    {
      return f;
    }


  panic!();
}


pub fn
get_bool(&self)-> bool
{
    if let ValueData::U64(u) = self.data
    {
      return u != 0;
    }


  panic!();
}


pub fn
get_char(&self)-> char
{
    if let ValueData::U64(u) = self.data
    {
        if let Some(c) = char::from_u32(u as u32)
        {
          return c;
        }
    }


  panic!();
}


pub fn
negi(&self)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   -self_i)},
  TypeItem::I16=>  {Self::new_i16(  -self_i)},
  TypeItem::I32=>  {Self::new_i32(  -self_i)},
  TypeItem::I64=>  {Self::new_i64(  -self_i)},
  TypeItem::ISize=>{Self::new_isize(-self_i)},
  _=>{Self::new_void()}
    }
}


pub fn
negf(&self)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32=>{Self::new_f32(  -self_f)},
  TypeItem::F64=>{Self::new_f64(  -self_f)},
  _=>{Self::new_void()}
    }
}


pub fn
notu(&self)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   !self_u)},
  TypeItem::U16=>  {Self::new_u16(  !self_u)},
  TypeItem::U32=>  {Self::new_u32(  !self_u)},
  TypeItem::U64=>  {Self::new_u64(  !self_u)},
  TypeItem::USize=>{Self::new_usize(!self_u)},
  _=>{Self::new_void()}
    }
}


pub fn
noti(&self)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   !self_i)},
  TypeItem::I16=>  {Self::new_i16(  !self_i)},
  TypeItem::I32=>  {Self::new_i32(  !self_i)},
  TypeItem::I64=>  {Self::new_i64(  !self_i)},
  TypeItem::ISize=>{Self::new_isize(!self_i)},
  _=>{Self::new_void()}
    }
}


pub fn
log_not(&self)-> Value
{
  let  self_b = self.get_bool();

    match &self.type_item
    {
  TypeItem::Bool=>{Self::new_bool(!self_b)},
  _=>{Self::new_void()}
    }
}


pub fn
addr(&self)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::Reference(target)=>{Self::new_pointer(self_u,target.clone())},
  _=>{Self::new_void()}
    }
}


pub fn
deref(&self)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::Pointer(target)=>{Self::new_reference(self_u,target.clone())},
  _=>{Self::new_void()}
    }
}


pub fn
sz(&mut self, decln: &DeclarationLink)-> Value
{
    if let Ok(ti) = self.type_item.try_get_info_mut(decln)
    {
      return Self::new_usize(ti.size as u64)
    }


  panic!();
}


pub fn
accs(&self, name: &str)-> Value
{
    match &self.type_item
    {
  TypeItem::Reference(target)=>
        {
          Self::new_void()
        }
  _=>{Self::new_void()}
    }
}




pub fn
addu(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u+u)},
  TypeItem::U16=>  {Self::new_u16(  self_u+u)},
  TypeItem::U32=>  {Self::new_u32(  self_u+u)},
  TypeItem::U64=>  {Self::new_u64(  self_u+u)},
  TypeItem::USize=>{Self::new_usize(self_u+u)},
  _=>{Self::new_void()}
    }
}


pub fn
subu(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u-u)},
  TypeItem::U16=>  {Self::new_u16(  self_u-u)},
  TypeItem::U32=>  {Self::new_u32(  self_u-u)},
  TypeItem::U64=>  {Self::new_u64(  self_u-u)},
  TypeItem::USize=>{Self::new_usize(self_u-u)},
  _=>{Self::new_void()}
    }
}


pub fn
mulu(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u*u)},
  TypeItem::U16=>  {Self::new_u16(  self_u*u)},
  TypeItem::U32=>  {Self::new_u32(  self_u*u)},
  TypeItem::U64=>  {Self::new_u64(  self_u*u)},
  TypeItem::USize=>{Self::new_usize(self_u*u)},
  _=>{Self::new_void()}
    }
}


pub fn
divu(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u/u)},
  TypeItem::U16=>  {Self::new_u16(  self_u/u)},
  TypeItem::U32=>  {Self::new_u32(  self_u/u)},
  TypeItem::U64=>  {Self::new_u64(  self_u/u)},
  TypeItem::USize=>{Self::new_usize(self_u/u)},
  _=>{Self::new_void()}
    }
}


pub fn
remu(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u%u)},
  TypeItem::U16=>  {Self::new_u16(  self_u%u)},
  TypeItem::U32=>  {Self::new_u32(  self_u%u)},
  TypeItem::U64=>  {Self::new_u64(  self_u%u)},
  TypeItem::USize=>{Self::new_usize(self_u%u)},
  _=>{Self::new_void()}
    }
}


pub fn
addi(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i+i)},
  TypeItem::I16=>  {Self::new_i16(  self_i+i)},
  TypeItem::I32=>  {Self::new_i32(  self_i+i)},
  TypeItem::I64=>  {Self::new_i64(  self_i+i)},
  TypeItem::ISize=>{Self::new_isize(self_i+i)},
  _=>{Self::new_void()}
    }
}


pub fn
subi(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i-i)},
  TypeItem::I16=>  {Self::new_i16(  self_i-i)},
  TypeItem::I32=>  {Self::new_i32(  self_i-i)},
  TypeItem::I64=>  {Self::new_i64(  self_i-i)},
  TypeItem::ISize=>{Self::new_isize(self_i-i)},
  _=>{Self::new_void()}
    }
}


pub fn
muli(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i*i)},
  TypeItem::I16=>  {Self::new_i16(  self_i*i)},
  TypeItem::I32=>  {Self::new_i32(  self_i*i)},
  TypeItem::I64=>  {Self::new_i64(  self_i*i)},
  TypeItem::ISize=>{Self::new_isize(self_i*i)},
  _=>{Self::new_void()}
    }
}


pub fn
divi(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i/i)},
  TypeItem::I16=>  {Self::new_i16(  self_i/i)},
  TypeItem::I32=>  {Self::new_i32(  self_i/i)},
  TypeItem::I64=>  {Self::new_i64(  self_i/i)},
  TypeItem::ISize=>{Self::new_isize(self_i/i)},
  _=>{Self::new_void()}
    }
}


pub fn
remi(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i%i)},
  TypeItem::I16=>  {Self::new_i16(  self_i%i)},
  TypeItem::I32=>  {Self::new_i32(  self_i%i)},
  TypeItem::I64=>  {Self::new_i64(  self_i%i)},
  TypeItem::ISize=>{Self::new_isize(self_i%i)},
  _=>{Self::new_void()}
    }
}


pub fn
addf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32=>{Self::new_f32(self_f+f)},
  TypeItem::F64=>{Self::new_f64(self_f+f)},
  _=>{Self::new_void()}
    }
}


pub fn
subf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32=>{Self::new_f32(self_f-f)},
  TypeItem::F64=>{Self::new_f64(self_f-f)},
  _=>{Self::new_void()}
    }
}


pub fn
mulf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32=>{Self::new_f32(self_f*f)},
  TypeItem::F64=>{Self::new_f64(self_f*f)},
  _=>{Self::new_void()}
    }
}


pub fn
divf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32=>{Self::new_f32(self_f/f)},
  TypeItem::F64=>{Self::new_f64(self_f/f)},
  _=>{Self::new_void()}
    }
}


pub fn
remf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32=>{Self::new_f32(self_f%f)},
  TypeItem::F64=>{Self::new_f64(self_f%f)},
  _=>{Self::new_void()}
    }
}


pub fn
shlu(&self, amount: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u<<amount)},
  TypeItem::U16=>  {Self::new_u16(  self_u<<amount)},
  TypeItem::U32=>  {Self::new_u32(  self_u<<amount)},
  TypeItem::U64=>  {Self::new_u64(  self_u<<amount)},
  TypeItem::USize=>{Self::new_usize(self_u<<amount)},
  _=>{Self::new_void()}
    }
}


pub fn
shli(&self, amount: u64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i<<amount)},
  TypeItem::I16=>  {Self::new_i16(  self_i<<amount)},
  TypeItem::I32=>  {Self::new_i32(  self_i<<amount)},
  TypeItem::I64=>  {Self::new_i64(  self_i<<amount)},
  TypeItem::ISize=>{Self::new_isize(self_i<<amount)},
  _=>{Self::new_void()}
    }
}


pub fn
shru(&self, amount: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u>>amount)},
  TypeItem::U16=>  {Self::new_u16(  self_u>>amount)},
  TypeItem::U32=>  {Self::new_u32(  self_u>>amount)},
  TypeItem::U64=>  {Self::new_u64(  self_u>>amount)},
  TypeItem::USize=>{Self::new_usize(self_u>>amount)},
  _=>{Self::new_void()}
    }
}


pub fn
shri(&self, amount: u64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i>>amount)},
  TypeItem::I16=>  {Self::new_i16(  self_i>>amount)},
  TypeItem::I32=>  {Self::new_i32(  self_i>>amount)},
  TypeItem::I64=>  {Self::new_i64(  self_i>>amount)},
  TypeItem::ISize=>{Self::new_isize(self_i>>amount)},
  _=>{Self::new_void()}
    }
}


pub fn
andu(&self, bits: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u&bits)},
  TypeItem::U16=>  {Self::new_u16(  self_u&bits)},
  TypeItem::U32=>  {Self::new_u32(  self_u&bits)},
  TypeItem::U64=>  {Self::new_u64(  self_u&bits)},
  TypeItem::USize=>{Self::new_usize(self_u&bits)},
  _=>{Self::new_void()}
    }
}


pub fn
oru(&self, bits: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u|bits)},
  TypeItem::U16=>  {Self::new_u16(  self_u|bits)},
  TypeItem::U32=>  {Self::new_u32(  self_u|bits)},
  TypeItem::U64=>  {Self::new_u64(  self_u|bits)},
  TypeItem::USize=>{Self::new_usize(self_u|bits)},
  _=>{Self::new_void()}
    }
}


pub fn
xoru(&self, bits: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8=>   {Self::new_u8(   self_u^bits)},
  TypeItem::U16=>  {Self::new_u16(  self_u^bits)},
  TypeItem::U32=>  {Self::new_u32(  self_u^bits)},
  TypeItem::U64=>  {Self::new_u64(  self_u^bits)},
  TypeItem::USize=>{Self::new_usize(self_u^bits)},
  _=>{Self::new_void()}
    }
}


pub fn
andi(&self, bits: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i&bits)},
  TypeItem::I16=>  {Self::new_i16(  self_i&bits)},
  TypeItem::I32=>  {Self::new_i32(  self_i&bits)},
  TypeItem::I64=>  {Self::new_i64(  self_i&bits)},
  TypeItem::ISize=>{Self::new_isize(self_i&bits)},
  _=>{Self::new_void()}
    }
}


pub fn
ori(&self, bits: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i|bits)},
  TypeItem::I16=>  {Self::new_i16(  self_i|bits)},
  TypeItem::I32=>  {Self::new_i32(  self_i|bits)},
  TypeItem::I64=>  {Self::new_i64(  self_i|bits)},
  TypeItem::ISize=>{Self::new_isize(self_i|bits)},
  _=>{Self::new_void()}
    }
}


pub fn
xori(&self, bits: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8=>   {Self::new_i8(   self_i^bits)},
  TypeItem::I16=>  {Self::new_i16(  self_i^bits)},
  TypeItem::I32=>  {Self::new_i32(  self_i^bits)},
  TypeItem::I64=>  {Self::new_i64(  self_i^bits)},
  TypeItem::ISize=>{Self::new_isize(self_i^bits)},
  _=>{Self::new_void()}
    }
}


pub fn
equ(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8
 |TypeItem::U16
 |TypeItem::U32
 |TypeItem::U64
 |TypeItem::USize=>{Self::new_bool(self_u == u)},
  _=>{Self::new_void()}
    }
}


pub fn
nequ(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8
 |TypeItem::U16
 |TypeItem::U32
 |TypeItem::U64
 |TypeItem::USize=>{Self::new_bool(self_u != u)},
  _=>{Self::new_void()}
    }
}


pub fn
ltu(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8
 |TypeItem::U16
 |TypeItem::U32
 |TypeItem::U64
 |TypeItem::USize=>{Self::new_bool(self_u < u)},
  _=>{Self::new_void()}
    }
}


pub fn
ltequ(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8
 |TypeItem::U16
 |TypeItem::U32
 |TypeItem::U64
 |TypeItem::USize=>{Self::new_bool(self_u <= u)},
  _=>{Self::new_void()}
    }
}


pub fn
gtu(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8
 |TypeItem::U16
 |TypeItem::U32
 |TypeItem::U64
 |TypeItem::USize=>{Self::new_bool(self_u > u)},
  _=>{Self::new_void()}
    }
}


pub fn
gtequ(&self, u: u64)-> Value
{
  let  self_u = self.get_u64();

    match &self.type_item
    {
  TypeItem::U8
 |TypeItem::U16
 |TypeItem::U32
 |TypeItem::U64
 |TypeItem::USize=>{Self::new_bool(self_u >= u)},
  _=>{Self::new_void()}
    }
}


pub fn
eqi(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8
 |TypeItem::I16
 |TypeItem::I32
 |TypeItem::I64
 |TypeItem::ISize=>{Self::new_bool(self_i == i)},
  _=>{Self::new_void()}
    }
}


pub fn
neqi(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8
 |TypeItem::I16
 |TypeItem::I32
 |TypeItem::I64
 |TypeItem::ISize=>{Self::new_bool(self_i != i)},
  _=>{Self::new_void()}
    }
}


pub fn
lti(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8
 |TypeItem::I16
 |TypeItem::I32
 |TypeItem::I64
 |TypeItem::ISize=>{Self::new_bool(self_i < i)},
  _=>{Self::new_void()}
    }
}


pub fn
lteqi(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8
 |TypeItem::I16
 |TypeItem::I32
 |TypeItem::I64
 |TypeItem::ISize=>{Self::new_bool(self_i <= i)},
  _=>{Self::new_void()}
    }
}


pub fn
gti(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8
 |TypeItem::I16
 |TypeItem::I32
 |TypeItem::I64
 |TypeItem::ISize=>{Self::new_bool(self_i > i)},
  _=>{Self::new_void()}
    }
}


pub fn
gteqi(&self, i: i64)-> Value
{
  let  self_i = self.get_i64();

    match &self.type_item
    {
  TypeItem::I8
 |TypeItem::I16
 |TypeItem::I32
 |TypeItem::I64
 |TypeItem::ISize=>{Self::new_bool(self_i >= i)},
  _=>{Self::new_void()}
    }
}


pub fn
eqf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32
 |TypeItem::F64=>{Self::new_bool(self_f == f)},
  _=>{Self::new_void()}
    }
}


pub fn
neqf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32
 |TypeItem::F64=>{Self::new_bool(self_f != f)},
  _=>{Self::new_void()}
    }
}


pub fn
ltf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32
 |TypeItem::F64=>{Self::new_bool(self_f < f)},
  _=>{Self::new_void()}
    }
}


pub fn
lteqf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32
 |TypeItem::F64=>{Self::new_bool(self_f <= f)},
  _=>{Self::new_void()}
    }
}


pub fn
gtf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32
 |TypeItem::F64=>{Self::new_bool(self_f > f)},
  _=>{Self::new_void()}
    }
}


pub fn
gteqf(&self, f: f64)-> Value
{
  let  self_f = self.get_f64();

    match &self.type_item
    {
  TypeItem::F32
 |TypeItem::F64=>{Self::new_bool(self_f >= f)},
  _=>{Self::new_void()}
    }
}


pub fn
log_and(&self, b: bool)-> Value
{
  let  self_b = self.get_bool();

    match &self.type_item
    {
  TypeItem::Bool=>{Self::new_bool(self_b && b)},
  _=>{Self::new_void()}
    }
}


pub fn
log_or(&self, b: bool)-> Value
{
  let  self_b = self.get_bool();

    match &self.type_item
    {
  TypeItem::Bool=>{Self::new_bool(self_b || b)},
  _=>{Self::new_void()}
    }
}




pub fn
assign(&self)
{
}




pub fn
print_b(&self)
{
    if let ValueData::U64(u) = self.data
    {
      print!("{}",if u != 0{true} else{false});
    }
}


pub fn
print_u(&self)
{
    if let ValueData::U64(u) = self.data
    {
      print!("{}",u);
    }
}


pub fn
print_i(&self)
{
    if let ValueData::I64(i) = self.data
    {
      print!("{}",i);
    }
}


pub fn
print_f(&self)
{
    if let ValueData::F64(f) = self.data
    {
      print!("{}",f);
    }
}


pub fn
print_img(&self)
{
    if let ValueData::Image(img) = &self.data
    {
      print!("{{...{}}}",img.len());
    }
}


pub fn
print(&self)
{
  let  ti = &self.type_item;

  ti.print();

  print!(" ");

       if let TypeItem::Bool = ti{self.print_b();}
  else if let TypeItem::Char = ti{self.print_u();}
  else if let TypeItem::U8 = ti{self.print_u();}
  else if let TypeItem::U16 = ti{self.print_u();}
  else if let TypeItem::U32 = ti{self.print_u();}
  else if let TypeItem::U64 = ti{self.print_u();}
  else if let TypeItem::USize = ti{self.print_u();}
  else if let TypeItem::I8 = ti{self.print_i();}
  else if let TypeItem::I16 = ti{self.print_i();}
  else if let TypeItem::I32 = ti{self.print_i();}
  else if let TypeItem::I64 = ti{self.print_i();}
  else if let TypeItem::ISize = ti{self.print_i();}
  else if let TypeItem::F32 = ti{self.print_f();}
  else if let TypeItem::F64 = ti{self.print_f();}
}


}




