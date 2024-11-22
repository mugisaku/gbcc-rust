

use super::type_info::TypeInfo;




#[derive(Clone)]
pub enum
Constant
{
  Bool(bool),

  I8(i8), I16(i16), I32(i32), I64(i64), ISize(isize),
  U8(u8), U16(u16), U32(u32), U64(u64), USize(usize),

  F32(f32), F64(f64),

}


impl
Constant
{


pub fn
get_type_info(&self)-> TypeInfo
{
    match self
    {
  Self::Bool(_) =>{TypeInfo::Bool }
  Self::I8(_)   =>{TypeInfo::I8   }
  Self::I16(_)  =>{TypeInfo::I16  }
  Self::I32(_)  =>{TypeInfo::I32  }
  Self::I64(_)  =>{TypeInfo::I64  }
  Self::ISize(_)=>{TypeInfo::ISize}
  Self::U8(_)   =>{TypeInfo::U8   }
  Self::U16(_)  =>{TypeInfo::U16  }
  Self::U32(_)  =>{TypeInfo::U32  }
  Self::U64(_)  =>{TypeInfo::U64  }
  Self::USize(_)=>{TypeInfo::USize}
  Self::F32(_)  =>{TypeInfo::F32  }
  Self::F64(_)  =>{TypeInfo::F64  }
  _=>{panic!();}
    }
}


pub fn  get_bool(&self)-> bool{if let Self::Bool(b) = self{*b} else{panic!();}}

pub fn  get_i8(&self)->       i8{if let Self::I8(i)    = self{*i} else{panic!();}}
pub fn  get_i16(&self)->     i16{if let Self::I16(i)   = self{*i} else{panic!();}}
pub fn  get_i32(&self)->     i32{if let Self::I32(i)   = self{*i} else{panic!();}}
pub fn  get_i64(&self)->     i64{if let Self::I64(i)   = self{*i} else{panic!();}}
pub fn  get_isize(&self)-> isize{if let Self::ISize(i) = self{*i} else{panic!();}}

pub fn  get_u8(&self)->       u8{if let Self::U8(u)    = self{*u} else{panic!();}}
pub fn  get_u16(&self)->     u16{if let Self::U16(u)   = self{*u} else{panic!();}}
pub fn  get_u32(&self)->     u32{if let Self::U32(u)   = self{*u} else{panic!();}}
pub fn  get_u64(&self)->     u64{if let Self::U64(u)   = self{*u} else{panic!();}}
pub fn  get_usize(&self)-> usize{if let Self::USize(u) = self{*u} else{panic!();}}

pub fn
get_uint(&self)-> usize
{
    match self
    {
  Self::U8(u)   =>{*u as usize}
  Self::U16(u)  =>{*u as usize}
  Self::U32(u)  =>{*u as usize}
  Self::U64(u)  =>{*u as usize}
  Self::USize(u)=>{*u         }
  _=>{panic!();}
    }
}

pub fn  get_f32(&self)-> f32{if let Self::F32(f) = self{*f} else{panic!();}}
pub fn  get_f64(&self)-> f64{if let Self::F64(f) = self{*f} else{panic!();}}


pub fn
neg(&self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(-*i)}
  Self::I16(i)  =>{Self::I16(-*i)}
  Self::I32(i)  =>{Self::I32(-*i)}
  Self::I64(i)  =>{Self::I64(-*i)}
  Self::ISize(i)=>{Self::ISize(-*i)}
  Self::F32(f)  =>{Self::F32(-*f)}
  Self::F64(f)  =>{Self::F64(-*f)}
  _=>{panic!();}
    }
}


pub fn
not(&self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(!*i)}
  Self::I16(i)  =>{Self::I16(!*i)}
  Self::I32(i)  =>{Self::I32(!*i)}
  Self::I64(i)  =>{Self::I64(!*i)}
  Self::ISize(i)=>{Self::ISize(!*i)}
  Self::U8(u)   =>{Self::U8(!*u)}
  Self::U16(u)  =>{Self::U16(!*u)}
  Self::U32(u)  =>{Self::U32(!*u)}
  Self::U64(u)  =>{Self::U64(!*u)}
  Self::USize(u)=>{Self::USize(!*u)}
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
  Self::I8(i)   =>{Self::I8(   *i+r.get_i8()   )}
  Self::I16(i)  =>{Self::I16(  *i+r.get_i16()  )}
  Self::I32(i)  =>{Self::I32(  *i+r.get_i32()  )}
  Self::I64(i)  =>{Self::I64(  *i+r.get_i64()  )}
  Self::ISize(i)=>{Self::ISize(*i+r.get_isize())}
  Self::U8(u)   =>{Self::U8(   *u+r.get_u8()   )}
  Self::U16(u)  =>{Self::U16(  *u+r.get_u16()  )}
  Self::U32(u)  =>{Self::U32(  *u+r.get_u32()  )}
  Self::U64(u)  =>{Self::U64(  *u+r.get_u64()  )}
  Self::USize(u)=>{Self::USize(*u+r.get_usize())}
  Self::F32(f)  =>{Self::F32(  *f+r.get_f32()  )}
  Self::F64(f)  =>{Self::F64(  *f+r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
sub(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(   *i-r.get_i8()   )}
  Self::I16(i)  =>{Self::I16(  *i-r.get_i16()  )}
  Self::I32(i)  =>{Self::I32(  *i-r.get_i32()  )}
  Self::I64(i)  =>{Self::I64(  *i-r.get_i64()  )}
  Self::ISize(i)=>{Self::ISize(*i-r.get_isize())}
  Self::U8(u)   =>{Self::U8(   *u-r.get_u8()   )}
  Self::U16(u)  =>{Self::U16(  *u-r.get_u16()  )}
  Self::U32(u)  =>{Self::U32(  *u-r.get_u32()  )}
  Self::U64(u)  =>{Self::U64(  *u-r.get_u64()  )}
  Self::USize(u)=>{Self::USize(*u-r.get_usize())}
  Self::F32(f)  =>{Self::F32(  *f-r.get_f32()  )}
  Self::F64(f)  =>{Self::F64(  *f-r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
mul(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(   *i*r.get_i8()   )}
  Self::I16(i)  =>{Self::I16(  *i*r.get_i16()  )}
  Self::I32(i)  =>{Self::I32(  *i*r.get_i32()  )}
  Self::I64(i)  =>{Self::I64(  *i*r.get_i64()  )}
  Self::ISize(i)=>{Self::ISize(*i*r.get_isize())}
  Self::U8(u)   =>{Self::U8(   *u*r.get_u8()   )}
  Self::U16(u)  =>{Self::U16(  *u*r.get_u16()  )}
  Self::U32(u)  =>{Self::U32(  *u*r.get_u32()  )}
  Self::U64(u)  =>{Self::U64(  *u*r.get_u64()  )}
  Self::USize(u)=>{Self::USize(*u*r.get_usize())}
  Self::F32(f)  =>{Self::F32(  *f*r.get_f32()  )}
  Self::F64(f)  =>{Self::F64(  *f*r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
div(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(   *i/r.get_i8()   )}
  Self::I16(i)  =>{Self::I16(  *i/r.get_i16()  )}
  Self::I32(i)  =>{Self::I32(  *i/r.get_i32()  )}
  Self::I64(i)  =>{Self::I64(  *i/r.get_i64()  )}
  Self::ISize(i)=>{Self::ISize(*i/r.get_isize())}
  Self::U8(u)   =>{Self::U8(   *u/r.get_u8()   )}
  Self::U16(u)  =>{Self::U16(  *u/r.get_u16()  )}
  Self::U32(u)  =>{Self::U32(  *u/r.get_u32()  )}
  Self::U64(u)  =>{Self::U64(  *u/r.get_u64()  )}
  Self::USize(u)=>{Self::USize(*u/r.get_usize())}
  Self::F32(f)  =>{Self::F32(  *f/r.get_f32()  )}
  Self::F64(f)  =>{Self::F64(  *f/r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
rem(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(   *i%r.get_i8()   )}
  Self::I16(i)  =>{Self::I16(  *i%r.get_i16()  )}
  Self::I32(i)  =>{Self::I32(  *i%r.get_i32()  )}
  Self::I64(i)  =>{Self::I64(  *i%r.get_i64()  )}
  Self::ISize(i)=>{Self::ISize(*i%r.get_isize())}
  Self::U8(u)   =>{Self::U8(   *u%r.get_u8()   )}
  Self::U16(u)  =>{Self::U16(  *u%r.get_u16()  )}
  Self::U32(u)  =>{Self::U32(  *u%r.get_u32()  )}
  Self::U64(u)  =>{Self::U64(  *u%r.get_u64()  )}
  Self::USize(u)=>{Self::USize(*u%r.get_usize())}
  Self::F32(f)  =>{Self::F32(  *f%r.get_f32()  )}
  Self::F64(f)  =>{Self::F64(  *f%r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
shl(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(   *i<<r.get_uint())}
  Self::I16(i)  =>{Self::I16(  *i<<r.get_uint())}
  Self::I32(i)  =>{Self::I32(  *i<<r.get_uint())}
  Self::I64(i)  =>{Self::I64(  *i<<r.get_uint())}
  Self::ISize(i)=>{Self::ISize(*i<<r.get_uint())}
  Self::U8(u)   =>{Self::U8(   *u<<r.get_uint())}
  Self::U16(u)  =>{Self::U16(  *u<<r.get_uint())}
  Self::U32(u)  =>{Self::U32(  *u<<r.get_uint())}
  Self::U64(u)  =>{Self::U64(  *u<<r.get_uint())}
  Self::USize(u)=>{Self::USize(*u<<r.get_uint())}
  _=>{panic!();}
    }
}


pub fn
shr(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(   *i>>r.get_uint())}
  Self::I16(i)  =>{Self::I16(  *i>>r.get_uint())}
  Self::I32(i)  =>{Self::I32(  *i>>r.get_uint())}
  Self::I64(i)  =>{Self::I64(  *i>>r.get_uint())}
  Self::ISize(i)=>{Self::ISize(*i>>r.get_uint())}
  Self::U8(u)   =>{Self::U8(   *u>>r.get_uint())}
  Self::U16(u)  =>{Self::U16(  *u>>r.get_uint())}
  Self::U32(u)  =>{Self::U32(  *u>>r.get_uint())}
  Self::U64(u)  =>{Self::U64(  *u>>r.get_uint())}
  Self::USize(u)=>{Self::USize(*u>>r.get_uint())}
  _=>{panic!();}
    }
}


pub fn
and(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(   *i&r.get_i8()   )}
  Self::I16(i)  =>{Self::I16(  *i&r.get_i16()  )}
  Self::I32(i)  =>{Self::I32(  *i&r.get_i32()  )}
  Self::I64(i)  =>{Self::I64(  *i&r.get_i64()  )}
  Self::ISize(i)=>{Self::ISize(*i&r.get_isize())}
  Self::U8(u)   =>{Self::U8(   *u&r.get_u8()   )}
  Self::U16(u)  =>{Self::U16(  *u&r.get_u16()  )}
  Self::U32(u)  =>{Self::U32(  *u&r.get_u32()  )}
  Self::U64(u)  =>{Self::U64(  *u&r.get_u64()  )}
  Self::USize(u)=>{Self::USize(*u&r.get_usize())}
  _=>{panic!();}
    }
}


pub fn
or(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(   *i|r.get_i8()   )}
  Self::I16(i)  =>{Self::I16(  *i|r.get_i16()  )}
  Self::I32(i)  =>{Self::I32(  *i|r.get_i32()  )}
  Self::I64(i)  =>{Self::I64(  *i|r.get_i64()  )}
  Self::ISize(i)=>{Self::ISize(*i|r.get_isize())}
  Self::U8(u)   =>{Self::U8(   *u|r.get_u8()   )}
  Self::U16(u)  =>{Self::U16(  *u|r.get_u16()  )}
  Self::U32(u)  =>{Self::U32(  *u|r.get_u32()  )}
  Self::U64(u)  =>{Self::U64(  *u|r.get_u64()  )}
  Self::USize(u)=>{Self::USize(*u|r.get_usize())}
  _=>{panic!();}
    }
}


pub fn
xor(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::I8(   *i^r.get_i8()   )}
  Self::I16(i)  =>{Self::I16(  *i^r.get_i16()  )}
  Self::I32(i)  =>{Self::I32(  *i^r.get_i32()  )}
  Self::I64(i)  =>{Self::I64(  *i^r.get_i64()  )}
  Self::ISize(i)=>{Self::ISize(*i^r.get_isize())}
  Self::U8(u)   =>{Self::U8(   *u^r.get_u8()   )}
  Self::U16(u)  =>{Self::U16(  *u^r.get_u16()  )}
  Self::U32(u)  =>{Self::U32(  *u^r.get_u32()  )}
  Self::U64(u)  =>{Self::U64(  *u^r.get_u64()  )}
  Self::USize(u)=>{Self::USize(*u^r.get_usize())}
  _=>{panic!();}
    }
}


pub fn
eq(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::Bool(*i == r.get_i8()   )}
  Self::I16(i)  =>{Self::Bool(*i == r.get_i16()  )}
  Self::I32(i)  =>{Self::Bool(*i == r.get_i32()  )}
  Self::I64(i)  =>{Self::Bool(*i == r.get_i64()  )}
  Self::ISize(i)=>{Self::Bool(*i == r.get_isize())}
  Self::U8(u)   =>{Self::Bool(*u == r.get_u8()   )}
  Self::U16(u)  =>{Self::Bool(*u == r.get_u16()  )}
  Self::U32(u)  =>{Self::Bool(*u == r.get_u32()  )}
  Self::U64(u)  =>{Self::Bool(*u == r.get_u64()  )}
  Self::USize(u)=>{Self::Bool(*u == r.get_usize())}
  Self::F32(f)  =>{Self::Bool(*f == r.get_f32()  )}
  Self::F64(f)  =>{Self::Bool(*f == r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
neq(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::Bool(*i != r.get_i8()   )}
  Self::I16(i)  =>{Self::Bool(*i != r.get_i16()  )}
  Self::I32(i)  =>{Self::Bool(*i != r.get_i32()  )}
  Self::I64(i)  =>{Self::Bool(*i != r.get_i64()  )}
  Self::ISize(i)=>{Self::Bool(*i != r.get_isize())}
  Self::U8(u)   =>{Self::Bool(*u != r.get_u8()   )}
  Self::U16(u)  =>{Self::Bool(*u != r.get_u16()  )}
  Self::U32(u)  =>{Self::Bool(*u != r.get_u32()  )}
  Self::U64(u)  =>{Self::Bool(*u != r.get_u64()  )}
  Self::USize(u)=>{Self::Bool(*u != r.get_usize())}
  Self::F32(f)  =>{Self::Bool(*f != r.get_f32()  )}
  Self::F64(f)  =>{Self::Bool(*f != r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
lt(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::Bool(*i < r.get_i8()   )}
  Self::I16(i)  =>{Self::Bool(*i < r.get_i16()  )}
  Self::I32(i)  =>{Self::Bool(*i < r.get_i32()  )}
  Self::I64(i)  =>{Self::Bool(*i < r.get_i64()  )}
  Self::ISize(i)=>{Self::Bool(*i < r.get_isize())}
  Self::U8(u)   =>{Self::Bool(*u < r.get_u8()   )}
  Self::U16(u)  =>{Self::Bool(*u < r.get_u16()  )}
  Self::U32(u)  =>{Self::Bool(*u < r.get_u32()  )}
  Self::U64(u)  =>{Self::Bool(*u < r.get_u64()  )}
  Self::USize(u)=>{Self::Bool(*u < r.get_usize())}
  Self::F32(f)  =>{Self::Bool(*f < r.get_f32()  )}
  Self::F64(f)  =>{Self::Bool(*f < r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
lteq(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::Bool(*i <= r.get_i8()   )}
  Self::I16(i)  =>{Self::Bool(*i <= r.get_i16()  )}
  Self::I32(i)  =>{Self::Bool(*i <= r.get_i32()  )}
  Self::I64(i)  =>{Self::Bool(*i <= r.get_i64()  )}
  Self::ISize(i)=>{Self::Bool(*i <= r.get_isize())}
  Self::U8(u)   =>{Self::Bool(*u <= r.get_u8()   )}
  Self::U16(u)  =>{Self::Bool(*u <= r.get_u16()  )}
  Self::U32(u)  =>{Self::Bool(*u <= r.get_u32()  )}
  Self::U64(u)  =>{Self::Bool(*u <= r.get_u64()  )}
  Self::USize(u)=>{Self::Bool(*u <= r.get_usize())}
  Self::F32(f)  =>{Self::Bool(*f <= r.get_f32()  )}
  Self::F64(f)  =>{Self::Bool(*f <= r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
gt(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::Bool(*i > r.get_i8()   )}
  Self::I16(i)  =>{Self::Bool(*i > r.get_i16()  )}
  Self::I32(i)  =>{Self::Bool(*i > r.get_i32()  )}
  Self::I64(i)  =>{Self::Bool(*i > r.get_i64()  )}
  Self::ISize(i)=>{Self::Bool(*i > r.get_isize())}
  Self::U8(u)   =>{Self::Bool(*u > r.get_u8()   )}
  Self::U16(u)  =>{Self::Bool(*u > r.get_u16()  )}
  Self::U32(u)  =>{Self::Bool(*u > r.get_u32()  )}
  Self::U64(u)  =>{Self::Bool(*u > r.get_u64()  )}
  Self::USize(u)=>{Self::Bool(*u > r.get_usize())}
  Self::F32(f)  =>{Self::Bool(*f > r.get_f32()  )}
  Self::F64(f)  =>{Self::Bool(*f > r.get_f64()  )}
  _=>{panic!();}
    }
}


pub fn
gteq(&self, r: &Self)-> Self
{
    match self
    {
  Self::I8(i)   =>{Self::Bool(*i >= r.get_i8()   )}
  Self::I16(i)  =>{Self::Bool(*i >= r.get_i16()  )}
  Self::I32(i)  =>{Self::Bool(*i >= r.get_i32()  )}
  Self::I64(i)  =>{Self::Bool(*i >= r.get_i64()  )}
  Self::ISize(i)=>{Self::Bool(*i >= r.get_isize())}
  Self::U8(u)   =>{Self::Bool(*u >= r.get_u8()   )}
  Self::U16(u)  =>{Self::Bool(*u >= r.get_u16()  )}
  Self::U32(u)  =>{Self::Bool(*u >= r.get_u32()  )}
  Self::U64(u)  =>{Self::Bool(*u >= r.get_u64()  )}
  Self::USize(u)=>{Self::Bool(*u >= r.get_usize())}
  Self::F32(f)  =>{Self::Bool(*f >= r.get_f32()  )}
  Self::F64(f)  =>{Self::Bool(*f >= r.get_f64()  )}
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





