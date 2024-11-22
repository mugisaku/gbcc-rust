

use super::type_info::TypeInfo;
use super::constant::Constant;


#[derive(Clone)]
pub enum
Literal
{
   Bool(bool),
    Int(i64),
   Uint(u64),
  Float(f64),

}


impl
Literal
{


pub fn
get_constant(&self, ti: TypeInfo)-> Constant
{
    match ti
    {
  TypeInfo::Bool =>{Constant::Bool(self.get_bool())}
  TypeInfo::I8   =>{Constant::I8(   self.get_i8()   )}
  TypeInfo::I16  =>{Constant::I16(  self.get_i16()  )}
  TypeInfo::I32  =>{Constant::I32(  self.get_i32()  )}
  TypeInfo::I64  =>{Constant::I64(  self.get_i64()  )}
  TypeInfo::ISize=>{Constant::ISize(self.get_isize())}
  TypeInfo::U8   =>{Constant::U8(   self.get_u8()   )}
  TypeInfo::U16  =>{Constant::U16(  self.get_u16()  )}
  TypeInfo::U32  =>{Constant::U32(  self.get_u32()  )}
  TypeInfo::U64  =>{Constant::U64(  self.get_u64()  )}
  TypeInfo::USize=>{Constant::USize(self.get_usize())}
  TypeInfo::F32  =>{Constant::F32(  self.get_f32()  )}
  TypeInfo::F64  =>{Constant::F64(  self.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
to_int(u: u64)-> i64
{
    if u <= i64::MAX as u64
    {
      return u as i64;
    }


  panic!();
}


pub fn
to_uint(i: i64)-> u64
{
    if i >= 0
    {
      return i as u64;
    }


  panic!();
}


pub fn
get_bool(&self)-> bool
{
    match self
    {
  Self::Bool(b)=>{*b}
  _=>{panic!();}
    }
}


pub fn
test_i64(v: i64, max: i64, min: i64)-> bool
{
  (v <= max) && (v <= min)
}


pub fn
test_u64(v: u64, max: u64)-> bool
{
  (v <= max)
}


pub fn
get_i8(&self)-> i8
{
    match self
    {
  Self::Uint(u)=>{if Self::test_u64(*u,i8::MAX as u64)               {*u as i8} else{panic!();}}
  Self::Int(i) =>{if Self::test_i64(*i,i8::MAX as i64,i8::MIN as i64){*i as i8} else{panic!();}}
  _=>{panic!();}
    }
}


pub fn
get_i16(&self)-> i16
{
    match self
    {
  Self::Uint(u)=>{if Self::test_u64(*u,i16::MAX as u64)                {*u as i16} else{panic!();}}
  Self::Int(i) =>{if Self::test_i64(*i,i16::MAX as i64,i16::MIN as i64){*i as i16} else{panic!();}}
  _=>{panic!();}
    }
}


pub fn
get_i32(&self)-> i32
{
    match self
    {
  Self::Uint(u)=>{if Self::test_u64(*u,i32::MAX as u64)                {*u as i32} else{panic!();}}
  Self::Int(i) =>{if Self::test_i64(*i,i32::MAX as i64,i32::MIN as i64){*i as i32} else{panic!();}}
  _=>{panic!();}
    }
}


pub fn
get_i64(&self)-> i64
{
    match self
    {
  Self::Uint(u)=>{if Self::test_u64(*u,i64::MAX as u64){*u as i64} else{panic!();}}
  Self::Int(i) =>{*i}
  _=>{panic!();}
    }
}


pub fn
get_isize(&self)-> isize
{
  self.get_i64() as isize
}


pub fn
get_u8(&self)-> u8
{
    match self
    {
  Self::Uint(u)=>{if Self::test_u64(*u,u8::MAX as u64)  {*u as u8} else{panic!();}}
  Self::Int(i) =>{if Self::test_i64(*i,u8::MAX as i64,0){*i as u8} else{panic!();}}
  _=>{panic!();}
    }
}


pub fn
get_u16(&self)-> u16
{
    match self
    {
  Self::Uint(u)=>{if Self::test_u64(*u,u16::MAX as u64)  {*u as u16} else{panic!();}}
  Self::Int(i) =>{if Self::test_i64(*i,u16::MAX as i64,0){*i as u16} else{panic!();}}
  _=>{panic!();}
    }
}


pub fn
get_u32(&self)-> u32
{
    match self
    {
  Self::Uint(u)=>{if Self::test_u64(*u,u32::MAX as u64)  {*u as u32} else{panic!();}}
  Self::Int(i) =>{if Self::test_i64(*i,u32::MAX as i64,0){*i as u32} else{panic!();}}
  _=>{panic!();}
    }
}


pub fn
get_u64(&self)-> u64
{
    match self
    {
  Self::Uint(u)=>{*u}
  Self::Int(i) =>{if Self::test_i64(*i,u64::MAX as i64,0){*i as u64} else{panic!();}}
  _=>{panic!();}
    }
}


pub fn
get_usize(&self)-> usize
{
  self.get_u64() as usize
}


pub fn
get_f32(&self)-> f32
{
    match self
    {
  Self::Float(f)=>{if (*f <= f32::MAX as f64) && (*f >= f32::MIN as f64){*f as f32} else{panic!();}}
  _=>{panic!();}
    }
}


pub fn
get_f64(&self)-> f64
{
    match self
    {
  Self::Float(f)=>{*f}
  _=>{panic!();}
    }
}


pub fn
get_int(&self)-> i64
{
    match self
    {
  Self::Uint(u)=>{Self::to_int(*u)}
  Self::Int(i)=>{*i}
  _=>{panic!();}
    }
}


pub fn
get_uint(&self)-> u64
{
    match self
    {
  Self::Int(i)=>{Self::to_uint(*i)}
  Self::Uint(u)=>{*u}
  _=>{panic!();}
    }
}


pub fn
get_float(&self)-> f64
{
    match self
    {
  Self::Float(f)=>{*f}
  _=>{panic!();}
    }
}


pub fn
neg(&self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int(-*i)}
  Self::Uint(u) =>{Self::Int(-Self::to_int(*u))}
  Self::Float(f)=>{Self::Float(-*f)}
  _=>{panic!();}
    } 
}


pub fn
not(&self)-> Self
{
    match self
    {
  Self::Int(i) =>{Self::Int(!*i)}
  Self::Uint(u)=>{Self::Uint(!*u)}
  _=>{panic!();}
    } 
}


pub fn
logical_not(&self)-> Self
{
    match self
    {
  Self::Bool(b)=>{Self::Bool(!*b)}
  _=>{panic!();}
    } 
}


pub fn
add(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int(  *i+r.get_int())}
  Self::Uint(u) =>{Self::Uint( *u+r.get_uint())}
  Self::Float(f)=>{Self::Float(*f+r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
sub(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int(  *i-r.get_int())}
  Self::Uint(u) =>{Self::Uint( *u-r.get_uint())}
  Self::Float(f)=>{Self::Float(*f-r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
mul(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int(  *i*r.get_int())}
  Self::Uint(u) =>{Self::Uint( *u*r.get_uint())}
  Self::Float(f)=>{Self::Float(*f*r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
div(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int(  *i/r.get_int())}
  Self::Uint(u) =>{Self::Uint( *u/r.get_uint())}
  Self::Float(f)=>{Self::Float(*f/r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
rem(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int(  *i%r.get_int())}
  Self::Uint(u) =>{Self::Uint( *u%r.get_uint())}
  Self::Float(f)=>{Self::Float(*f%r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
shl(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int( *i<<r.get_uint())}
  Self::Uint(u) =>{Self::Uint(*u<<r.get_uint())}
  _=>{panic!();}
    } 
}


pub fn
shr(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int( *i>>r.get_uint())}
  Self::Uint(u) =>{Self::Uint(*u>>r.get_uint())}
  _=>{panic!();}
    } 
}


pub fn
and(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int( *i&r.get_int())}
  Self::Uint(u) =>{Self::Uint(*u&r.get_uint())}
  _=>{panic!();}
    } 
}


pub fn
or(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int( *i|r.get_int())}
  Self::Uint(u) =>{Self::Uint(*u|r.get_uint())}
  _=>{panic!();}
    } 
}


pub fn
xor(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Int( *i^r.get_int())}
  Self::Uint(u) =>{Self::Uint(*u^r.get_uint())}
  _=>{panic!();}
    } 
}


pub fn
eq(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Bool(*i == r.get_int())}
  Self::Uint(u) =>{Self::Bool(*u == r.get_uint())}
  Self::Float(f)=>{Self::Bool(*f == r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
neq(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Bool(*i != r.get_int())}
  Self::Uint(u) =>{Self::Bool(*u != r.get_uint())}
  Self::Float(f)=>{Self::Bool(*f != r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
lt(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Bool(*i < r.get_int())}
  Self::Uint(u) =>{Self::Bool(*u < r.get_uint())}
  Self::Float(f)=>{Self::Bool(*f < r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
lteq(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Bool(*i <= r.get_int())}
  Self::Uint(u) =>{Self::Bool(*u <= r.get_uint())}
  Self::Float(f)=>{Self::Bool(*f <= r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
gt(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Bool(*i > r.get_int())}
  Self::Uint(u) =>{Self::Bool(*u > r.get_uint())}
  Self::Float(f)=>{Self::Bool(*f > r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
gteq(&self, r: &Self)-> Self
{
    match self
    {
  Self::Int(i)  =>{Self::Bool(*i >= r.get_int())}
  Self::Uint(u) =>{Self::Bool(*u >= r.get_uint())}
  Self::Float(f)=>{Self::Bool(*f >= r.get_float())}
  _=>{panic!();}
    } 
}


pub fn
logical_and(&self, r: &Self)-> Self
{
  Self::Bool(self.get_bool() && r.get_bool())
}


pub fn
logical_or(&self, r: &Self)-> Self
{
  Self::Bool(self.get_bool() || r.get_bool())
}


}




