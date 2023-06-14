

use super::fixer::DeclarationIndex;


#[derive(Clone,Copy)]
pub enum
Value
{
  Undefined,

   I8(i8),
  I16(i16),
  I32(i32),
  I64(i64),
   U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  F32(f32),
  F64(f64),
  Bool(bool),

  Pointer(usize),
  Reference(usize),

  DeclarationIndex(DeclarationIndex),

}


impl
Value
{


pub fn
is_undefined(&self)-> bool
{
    if let Value::Undefined = self
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
  Value::I8(i)=> {Some(*i as i64)}
  Value::I16(i)=>{Some(*i as i64)}
  Value::I32(i)=>{Some(*i as i64)}
  Value::I64(i)=>{Some(*i)}
  Value::U8(u)=> {Some(*u as i64)}
  Value::U16(u)=>{Some(*u as i64)}
  Value::U32(u)=>{Some(*u as i64)}
  Value::U64(u)=>{Some(*u as i64)}
  Value::F32(f)=>{Some(*f as i64)}
  Value::F64(f)=>{Some(*f as i64)}
  _=>{None}
    }
}


pub fn
get_i64(&self)-> Option<i64>
{
    match self
    {
  Value::I8(i)=> {Some(*i as i64)}
  Value::I16(i)=>{Some(*i as i64)}
  Value::I32(i)=>{Some(*i as i64)}
  Value::I64(i)=>{Some(*i)}
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
  Value::I8(i)=> {Some(*i as u64)}
  Value::I16(i)=>{Some(*i as u64)}
  Value::I32(i)=>{Some(*i as u64)}
  Value::I64(i)=>{Some(*i as u64)}
  Value::U8(u)=> {Some(*u as u64)}
  Value::U16(u)=>{Some(*u as u64)}
  Value::U32(u)=>{Some(*u as u64)}
  Value::U64(u)=>{Some(*u)}
  Value::F32(f)=>{Some(*f as u64)}
  Value::F64(f)=>{Some(*f as u64)}
  _=>{None}
    }
}


pub fn
get_u64(&self)-> Option<u64>
{
    match self
    {
  Value::U8(u)=> {Some(*u as u64)}
  Value::U16(u)=>{Some(*u as u64)}
  Value::U32(u)=>{Some(*u as u64)}
  Value::U64(u)=>{Some(*u)}
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
  Value::I8(i)=> {Some(*i as f64)}
  Value::I16(i)=>{Some(*i as f64)}
  Value::I32(i)=>{Some(*i as f64)}
  Value::I64(i)=>{Some(*i as f64)}
  Value::U8(u)=> {Some(*u as f64)}
  Value::U16(u)=>{Some(*u as f64)}
  Value::U32(u)=>{Some(*u as f64)}
  Value::U64(u)=>{Some(*u as f64)}
  Value::F32(f)=>{Some(*f as f64)}
  Value::F64(f)=>{Some(*f)}
  _=>{None}
    }
}


pub fn
get_f64(&self)-> Option<f64>
{
    match self
    {
  Value::F32(f)=>{Some(*f as f64)}
  Value::F64(f)=>{Some(*f)}
  _=>{None}
    }
}




pub fn
to_bool(&self)-> Option<bool>
{
    match self
    {
  Value::I8(i)=> {Some(*i != 0)}
  Value::I16(i)=>{Some(*i != 0)}
  Value::I32(i)=>{Some(*i != 0)}
  Value::I64(i)=>{Some(*i != 0)}
  Value::U8(u)=> {Some(*u != 0)}
  Value::U16(u)=>{Some(*u != 0)}
  Value::U32(u)=>{Some(*u != 0)}
  Value::U64(u)=>{Some(*u != 0)}
  Value::Bool(b)=>{Some(*b)}
  _=>{None}
    }
}




pub fn
get_size(&self)-> Option<usize>
{
    match self
    {
  Value::I8(_)=> {Some(1)}
  Value::I16(_)=>{Some(2)}
  Value::I32(_)=>{Some(4)}
  Value::I64(_)=>{Some(8)}
  Value::U8(_)=> {Some(1)}
  Value::U16(_)=>{Some(2)}
  Value::U32(_)=>{Some(4)}
  Value::U64(_)=>{Some(8)}
  Value::F32(_)=>{Some(4)}
  Value::F64(_)=>{Some(8)}
  Value::Bool(_)=>{Some(1)}
  Value::Pointer(_)=>{Some(8)}
  Value::Reference(_)=>{Some(8)}
  _=>{None}
    }
}


pub fn
get_id(&self)-> usize
{
    match self
    {
  Value::Undefined=> {0}
  Value::I8(_)=> {1}
  Value::I16(_)=>{2}
  Value::I32(_)=>{3}
  Value::I64(_)=>{4}
  Value::U8(_)=> {5}
  Value::U16(_)=>{6}
  Value::U32(_)=>{7}
  Value::U64(_)=>{8}
  Value::F32(_)=>{9}
  Value::F64(_)=>{10}
  Value::Bool(_)=>{11}
  Value::Pointer(_)=>{12}
  Value::Reference(_)=>{13}
  _=>{0}
    }
}




pub fn
print(&self)
{
    match self
    {
  Value::Undefined=> {print!("undefined",);}
  Value::I8(i)=> {print!("i8: {}",*i);}
  Value::I16(i)=>{print!("i16: {}",*i);}
  Value::I32(i)=>{print!("i32: {}",*i);}
  Value::I64(i)=>{print!("i64: {}",*i);}
  Value::U8(u)=> {print!("u8: {}",*u);}
  Value::U16(u)=>{print!("u16: {}",*u);}
  Value::U32(u)=>{print!("u32: {}",*u);}
  Value::U64(u)=>{print!("u64: {}",*u);}
  Value::F32(f)=>{print!("f32: {}",*f);}
  Value::F64(f)=>{print!("f64: {}",*f);}
  Value::Bool(b)=>{print!("bool: {}",*b);}
  Value::Pointer(v)=>{print!("pointer: {}",*v);}
  Value::Reference(v)=>{print!("reference: {}",*v);}
  Value::DeclarationIndex(di)=>
        {
          print!("declaration");
        }
    }
}


}




