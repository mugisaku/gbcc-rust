

use super::library::{
  Library,
  TypeIndex,
};


#[derive(Clone,Copy)]
pub struct
Value
{
  pub(crate) type_index: TypeIndex,

  pub(crate) data: ValueData,

}




#[derive(Clone,Copy)]
#[allow(dead_code)]
pub enum
ValueData
{
  Undefined,

  I64(i64),
  U64(u64),
  F64(f64),

}


#[allow(dead_code)]
impl
ValueData
{


pub fn
is_undefined(&self)-> bool
{
    if let ValueData::Undefined = self
    {
      return true;
    }


  false
}


pub fn
to_i8(&self)-> Option<i8>
{
    if let Some(i) = self.to_i64()
    {
      return Some(i as i8);
    }


  None
}


pub fn
to_i16(&self)-> Option<i16>
{
    if let Some(i) = self.to_i64()
    {
      return Some(i as i16);
    }


  None
}


pub fn
to_i32(&self)-> Option<i32>
{
    if let Some(i) = self.to_i64()
    {
      return Some(i as i32);
    }


  None
}


pub fn
to_i64(&self)-> Option<i64>
{
    match self
    {
  ValueData::I64(i)=>{Some(*i)}
  ValueData::U64(u)=>{Some(*u as i64)}
  ValueData::F64(f)=>{Some(*f as i64)}
  _=>{None}
    }
}


pub fn
get_i64(&self)-> Option<i64>
{
    match self
    {
  ValueData::I64(i)=>{Some(*i)}
  _=>{None}
    }
}


pub fn
to_u8(&self)-> Option<u8>
{
    if let Some(u) = self.to_u64()
    {
      return Some(u as u8);
    }


  None
}


pub fn
to_u16(&self)-> Option<u16>
{
    if let Some(u) = self.to_u64()
    {
      return Some(u as u16);
    }


  None
}


pub fn
to_u32(&self)-> Option<u32>
{
    if let Some(u) = self.to_u64()
    {
      return Some(u as u32);
    }


  None
}


pub fn
to_u64(&self)-> Option<u64>
{
    match self
    {
  ValueData::I64(i)=>{Some(*i as u64)}
  ValueData::U64(u)=>{Some(*u)}
  ValueData::F64(f)=>{Some(*f as u64)}
  _=>{None}
    }
}


pub fn
get_u64(&self)-> Option<u64>
{
    match self
    {
  ValueData::U64(u)=>{Some(*u)}
  _=>{None}
    }
}


pub fn
to_f32(&self)-> Option<f32>
{
    if let Some(f) = self.to_f64()
    {
      return Some(f as f32);
    }


  None
}


pub fn
to_f64(&self)-> Option<f64>
{
    match self
    {
  ValueData::I64(i)=>{Some(*i as f64)}
  ValueData::U64(u)=>{Some(*u as f64)}
  ValueData::F64(f)=>{Some(*f)}
  _=>{None}
    }
}


pub fn
get_f64(&self)-> Option<f64>
{
    match self
    {
  ValueData::F64(f)=>{Some(*f)}
  _=>{None}
    }
}




pub fn
print(&self)
{
    match self
    {
  ValueData::Undefined=> {print!("undefined",);}
  ValueData::I64(i)=>{print!("i64: {}",*i);}
  ValueData::U64(u)=>{print!("u64: {}",*u);}
  ValueData::F64(f)=>{print!("f64: {}",*f);}
    }
}


}




