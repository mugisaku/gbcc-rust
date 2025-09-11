

use std::convert::From;


use super::type_kind::TypeCode;


pub struct
Constant
{
  pub    memory: Vec<u8>,
  pub type_code: TypeCode,

}


impl
From<bool> for Constant
{


fn
from(b: bool)-> Self
{
  Self{memory: vec![if b{1u8} else{0u8}], type_code: TypeCode::new_bool()}
}


}


impl
From<i8> for Constant
{


fn
from(v: i8)-> Self
{
  Self{memory: vec![v as u8], type_code: TypeCode::new_i8()}
}


}


impl
From<i16> for Constant
{


fn
from(v: i16)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_i16()}
}


}


impl
From<i32> for Constant
{


fn
from(v: i32)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_i32()}
}


}


impl
From<i64> for Constant
{


fn
from(v: i64)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_i64()}
}


}



impl
From<isize> for Constant
{


fn
from(v: isize)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_isize()}
}


}


impl
From<u8> for Constant
{


fn
from(v: u8)-> Self
{
  Self{memory: vec![v], type_code: TypeCode::new_u8()}
}


}


impl
From<u16> for Constant
{


fn
from(v: u16)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_u16()}
}


}


impl
From<u32> for Constant
{


fn
from(v: u32)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_u32()}
}


}


impl
From<u64> for Constant
{


fn
from(v: u64)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_u64()}
}


}



impl
From<usize> for Constant
{


fn
from(v: usize)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_usize()}
}


}


impl
From<f32> for Constant
{


fn
from(v: f32)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_f32()}
}


}


impl
From<f64> for Constant
{


fn
from(v: f64)-> Self
{
  Self{memory: v.to_be_bytes().to_vec(), type_code: TypeCode::new_f64()}
}


}




