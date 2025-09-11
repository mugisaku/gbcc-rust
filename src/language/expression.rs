

use std::convert::From;


use super::constant::Constant;

use super::element::{
  Class,

};


use super::type_kind::*;


use super::opcode::{
  Instruction,
  ProgramNode,
};



use super::library::{
  Library,

};


pub enum
ValueKind
{
  Literal(Vec<u8>),
  Constant(Vec<u8>),
  Variable(Vec<Instruction>),
  Operated(Vec<Instruction>),

}


pub struct
Value
{
  kind: ValueKind,

  type_code: TypeCode,

}


impl
Value
{


pub fn
new_constant(c: Constant)-> Self
{
  Self{kind: ValueKind::Literal(c.memory), type_code: c.type_code}
}


pub fn
new_void_literal()-> Self
{
  Self{kind: ValueKind::Literal(Vec::new()), type_code: TypeCode::new_void()}
}


pub fn
new_bool_literal(v: bool)-> Self
{
  let  c = Constant::from(v);

  Self{kind: ValueKind::Literal(c.memory), type_code: c.type_code}
}


pub fn
new_usize_literal(v: usize)-> Self
{
  let  c = Constant::from(v);

  Self{kind: ValueKind::Literal(c.memory), type_code: c.type_code}
}


pub fn
new_f64_literal(v: f64)-> Self
{
  Self{kind: ValueKind::Literal(v.to_be_bytes().to_vec()), type_code: TypeCode::new_f64()}
}


pub fn
get_memory(&self)-> Option<&[u8]>
{
    match &self.kind
    {
  ValueKind::Literal(m)=> {Some(m)}
  ValueKind::Constant(m)=>{Some(m)}
  _=>{None}
    }
}


pub fn
to_operated_from(m: &[u8], type_code: TypeCode)-> Self
{
  let  mut pc = Vec::<Instruction>::new();

       if type_code.is_bool(){pc.push(Instruction::pushb(to_bool(m)));}
  else if type_code.is_i8()
       || type_code.is_i16()
       || type_code.is_i32()
       || type_code.is_i64()
       || type_code.is_isize(){pc.push(Instruction::pushi(to_i64(m)));}
  else if type_code.is_u8()
       || type_code.is_u16()
       || type_code.is_u32()
       || type_code.is_u64()
       || type_code.is_usize(){pc.push(Instruction::pushu(to_u64(m)));}
  else if type_code.is_f32()
       || type_code.is_f64(){pc.push(Instruction::pushf(to_f64(m)));}
  else{panic!();}


  Self{
    kind: ValueKind::Operated(pc),
    type_code,
  }
}


pub fn
to_operated(self)-> Self
{
    match self.kind
    {
  ValueKind::Literal(m)=> {Self::to_operated_from(&m,self.type_code)}
  ValueKind::Constant(m)=>{Self::to_operated_from(&m,self.type_code)}
  ValueKind::Variable(pc)=>
    {
      panic!();
    }
  ValueKind::Operated(pc)=>{Self{kind: ValueKind::Operated(pc), type_code: self.type_code}}
    }
}


}




pub fn
to_bytes1(src: &[u8])-> [u8; 1]
{
  unsafe{[
    *src.get_unchecked(0),
  ]}
}


pub fn
to_be_bytes2(src: &[u8])-> [u8; 2]
{
  unsafe{[
    *src.get_unchecked(0),
    *src.get_unchecked(1),
  ]}
}


pub fn
to_be_bytes4(src: &[u8])-> [u8; 4]
{
  unsafe{[
    *src.get_unchecked(0),
    *src.get_unchecked(1),
    *src.get_unchecked(2),
    *src.get_unchecked(3),
  ]}
}


pub fn
to_be_bytes8(src: &[u8])-> [u8; 8]
{
  unsafe{[
    *src.get_unchecked(0),
    *src.get_unchecked(1),
    *src.get_unchecked(2),
    *src.get_unchecked(3),
    *src.get_unchecked(4),
    *src.get_unchecked(5),
    *src.get_unchecked(6),
    *src.get_unchecked(7),
  ]}
}


pub fn
to_bool(src: &[u8])-> bool
{
    match src.len()
    {
  1=>{unsafe{*src.get_unchecked(0) != 0}}
  2=>{u16::from_be_bytes(to_be_bytes2(src)) != 0}
  4=>{u32::from_be_bytes(to_be_bytes4(src)) != 0}
  8=>{u64::from_be_bytes(to_be_bytes8(src)) != 0}
  _=>{panic!();}
    }
}


pub fn
to_i8(src: &[u8])-> i8
{
    match src.len()
    {
  1=>{unsafe{*src.get_unchecked(0) as i8}}
  2=>{i16::from_be_bytes(to_be_bytes2(src)) as i8}
  4=>{i32::from_be_bytes(to_be_bytes4(src)) as i8}
  8=>{i64::from_be_bytes(to_be_bytes8(src)) as i8}
  _=>{panic!();}
    }
}


pub fn
to_i16(src: &[u8])-> i16
{
    match src.len()
    {
  1=>{unsafe{*src.get_unchecked(0) as i8 as i16}}
  2=>{i16::from_be_bytes(to_be_bytes2(src)) as i16}
  4=>{i32::from_be_bytes(to_be_bytes4(src)) as i16}
  8=>{i64::from_be_bytes(to_be_bytes8(src)) as i16}
  _=>{panic!();}
    }
}


pub fn
to_i32(src: &[u8])-> i32
{
    match src.len()
    {
  1=>{unsafe{*src.get_unchecked(0) as i8 as i32}}
  2=>{i16::from_be_bytes(to_be_bytes2(src)) as i32}
  4=>{i32::from_be_bytes(to_be_bytes4(src)) as i32}
  8=>{i64::from_be_bytes(to_be_bytes8(src)) as i32}
  _=>{panic!();}
    }
}


pub fn
to_i64(src: &[u8])-> i64
{
    match src.len()
    {
  1=>{unsafe{*src.get_unchecked(0) as i8 as i64}}
  2=>{i16::from_be_bytes(to_be_bytes2(src)) as i64}
  4=>{i32::from_be_bytes(to_be_bytes4(src)) as i64}
  8=>{i64::from_be_bytes(to_be_bytes8(src)) as i64}
  _=>{panic!();}
    }
}


pub fn
to_u8(src: &[u8])-> u8
{
    match src.len()
    {
  1=>{unsafe{*src.get_unchecked(0) as u8}}
  2=>{u16::from_be_bytes(to_be_bytes2(src)) as u8}
  4=>{u32::from_be_bytes(to_be_bytes4(src)) as u8}
  8=>{u64::from_be_bytes(to_be_bytes8(src)) as u8}
  _=>{panic!();}
    }
}


pub fn
to_u16(src: &[u8])-> u16
{
    match src.len()
    {
  1=>{unsafe{*src.get_unchecked(0) as i8 as u16}}
  2=>{u16::from_be_bytes(to_be_bytes2(src)) as u16}
  4=>{u32::from_be_bytes(to_be_bytes4(src)) as u16}
  8=>{u64::from_be_bytes(to_be_bytes8(src)) as u16}
  _=>{panic!();}
    }
}


pub fn
to_u32(src: &[u8])-> u32
{
    match src.len()
    {
  1=>{unsafe{*src.get_unchecked(0) as i8 as u32}}
  2=>{u16::from_be_bytes(to_be_bytes2(src)) as u32}
  4=>{u32::from_be_bytes(to_be_bytes4(src)) as u32}
  8=>{u64::from_be_bytes(to_be_bytes8(src)) as u32}
  _=>{panic!();}
    }
}


pub fn
to_u64(src: &[u8])-> u64
{
    match src.len()
    {
  1=>{unsafe{*src.get_unchecked(0) as u64}}
  2=>{u16::from_be_bytes(to_be_bytes2(src)) as u64}
  4=>{u32::from_be_bytes(to_be_bytes4(src)) as u64}
  8=>{u64::from_be_bytes(to_be_bytes8(src))}
  _=>{panic!();}
    }
}


pub fn
to_f32(src: &[u8])-> f32
{
    match src.len()
    {
  4=>{f32::from_be_bytes(to_be_bytes4(src)) as f32}
  8=>{f64::from_be_bytes(to_be_bytes8(src)) as f32}
  _=>{panic!();}
    }
}


pub fn
to_f64(src: &[u8])-> f64
{
    match src.len()
    {
  4=>{f32::from_be_bytes(to_be_bytes4(src)) as f64}
  8=>{f64::from_be_bytes(to_be_bytes8(src))}
  _=>{panic!();}
    }
}




#[derive(Clone)]
pub enum
UnaryOperator
{
  Neg,
  Not,
  LogicalNot,
  Deref,

}


impl
UnaryOperator
{


pub fn
operate(&self, v: Value)-> Result<Value,()>
{
/*
    if class_name == BOOL_S
    {
        if let Self::LogicalNot = self
        {
          return Ok(BOOL_S.to_string());
        }
    }

  else
    if is_i_class(class_name)
    || is_u_class(class_name)
    {
        if let Self::Not = self
        {
          return Ok(class_name.to_string());
        }
    }

  else
    if is_i_class(class_name)
    {
        if let Self::Neg = self
        {
          return Ok(class_name.to_string());
        }
    }

  else
    if (class_name == F32_S)
    || (class_name == F64_S)
    {
        if let Self::Neg = self
        {
          return Ok(class_name.to_string());
        }
    }
*/


  Err(())
}


pub fn
print(&self)
{
    match self
    {
  UnaryOperator::Neg=>{print!("-");},
  UnaryOperator::Not=>{print!("~");},
  UnaryOperator::LogicalNot=>{print!("!");},
  UnaryOperator::Deref=>{print!("*");},
    }
}


pub fn
print_mnemonic(&self)
{
    match self
    {
  UnaryOperator::Neg=>{print!("neg");},
  UnaryOperator::Not=>{print!("not");},
  UnaryOperator::LogicalNot=>{print!("logical_not");},
  UnaryOperator::Deref=>{print!("deref");},
    }
}


}




#[derive(Clone)]
pub enum
BinaryOperator
{
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Shl,
  Shr,
  And,
  Or,
  Xor,
  Eq,
  Neq,
  Lt,
  Lteq,
  Gt,
  Gteq,
  LogicalOr,
  LogicalAnd,

}


impl
BinaryOperator
{




pub fn
is_arithmetic(&self)-> bool
{
    match self
    {
  Self::Add
 |Self::Sub
 |Self::Mul
 |Self::Div
 |Self::Rem=>{true}
  _=>{false}
    }
}


pub fn
is_bitwise(&self)-> bool
{
    match self
    {
  Self::Shl
 |Self::Shr
 |Self::And
 |Self::Or
 |Self::Xor=>{true}
  _=>{false}
    }
}


pub fn
is_comparison(&self)-> bool
{
    match self
    {
  Self::Eq
 |Self::Neq
 |Self::Lt
 |Self::Lteq
 |Self::Gt
 |Self::Gteq=>{true}
  _=>{false}
    }
}


pub fn
is_logical(&self)-> bool
{
    match self
    {
  Self::LogicalAnd
 |Self::LogicalOr=>{true}
  _=>{false}
    }
}


pub fn
operate_bool_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_bool(lmem);
  let  rv = to_bool(rmem);

    match self
    {
  Self::LogicalAnd=>{Constant::from(lv && rv)}
  Self::LogicalOr=> {Constant::from(lv || rv)}
  _=>{panic!();}
    }
}


pub fn
operate_i8_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_i8(lmem);
  let  rv = to_i8(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}




pub fn
operate_i16_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_i16(lmem);
  let  rv = to_i16(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
operate_i32_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_i32(lmem);
  let  rv = to_i32(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
operate_i64_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_i64(lmem);
  let  rv = to_i64(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
operate_isize_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_i64(lmem) as isize;
  let  rv = to_i64(rmem) as isize;

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
operate_u8_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_u8(lmem);
  let  rv = to_u8(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}




pub fn
operate_u16_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_u16(lmem);
  let  rv = to_u16(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
operate_u32_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_u32(lmem);
  let  rv = to_u32(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
operate_u64_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_u64(lmem);
  let  rv = to_u64(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
operate_usize_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_u64(lmem) as usize;
  let  rv = to_u64(rmem) as usize;

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Shl=>{Constant::from(lv<<rv)}
  Self::Shr=>{Constant::from(lv>>rv)}
  Self::And=>{Constant::from(lv&rv)}
  Self::Or=> {Constant::from(lv|rv)}
  Self::Xor=>{Constant::from(lv^rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
operate_f32_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_f32(lmem);
  let  rv = to_f32(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
operate_f64_constant(&self, lmem: &[u8], rmem: &[u8])-> Constant
{
  let  lv = to_f64(lmem);
  let  rv = to_f64(rmem);

    match self
    {
  Self::Add=>{Constant::from(lv+rv)}
  Self::Sub=>{Constant::from(lv-rv)}
  Self::Mul=>{Constant::from(lv*rv)}
  Self::Div=>{Constant::from(lv/rv)}
  Self::Rem=>{Constant::from(lv%rv)}
  Self::Eq  =>{Constant::from(lv == rv)}
  Self::Neq =>{Constant::from(lv != rv)}
  Self::Lt  =>{Constant::from(lv <  rv)}
  Self::Lteq=>{Constant::from(lv <= rv)}
  Self::Gt  =>{Constant::from(lv >  rv)}
  Self::Gteq=>{Constant::from(lv >= rv)}
  _=>{panic!();}
    }
}


pub fn
to_constant(mut v: Value, type_code: &TypeCode)-> Value
{
  let  mut mem = if let ValueKind::Literal(mem) = v.kind{mem} else{panic!();};

    if v.type_code.is_i() && type_code.is_i()
    {
      let  i = to_i64(&mem);

        if type_code.is_i8()
        {
          mem = (i as i8).to_be_bytes().to_vec();
        }

      else
        if type_code.is_i16()
        {
          mem = (i as i16).to_be_bytes().to_vec();
        }

      else
        if type_code.is_i32()
        {
          mem = (i as i32).to_be_bytes().to_vec();
        }
    }

  else
    if v.type_code.is_u() && type_code.is_u()
    {
      let  u = to_u64(&mem);

        if type_code.is_u8()
        {
          mem = (u as u8).to_be_bytes().to_vec();
        }

      else
        if type_code.is_u16()
        {
          mem = (u as u16).to_be_bytes().to_vec();
        }

      else
        if type_code.is_u32()
        {
          mem = (u as u32).to_be_bytes().to_vec();
        }
    }

  else
    if v.type_code.is_f() && type_code.is_f()
    {
      let  f = to_f64(&mem);

        if type_code.is_f32()
        {
          mem = (f as f32).to_be_bytes().to_vec();
        }
    }

  else
    {
      panic!();
    }


  Value{kind: ValueKind::Constant(mem), type_code: type_code.clone()}
}


pub fn
to_operated(&self, lv: Value, rv: Value)-> Value
{
  let  kind = ValueKind::Operated(Vec::new());
/*
    if self.is_arithmetic() || self.is_bitwise()
    {
    }
	
  else
    if self.is_comparison()
    {
    }
	
  else
    if self.is_logical()
    {
    }

  else
*/
    {
      panic!();
    }
}


pub fn
operate(&self, mut lv: Value, mut rv: Value)-> Value
{
    if let ValueKind::Literal(lmem) = &lv.kind
    {
        if let ValueKind::Literal(rmem) = &rv.kind
        {
            if lv.type_code.is_bool()
            && rv.type_code.is_bool()
            {
              let  c = self.operate_bool_constant(lmem,rmem);

              return Value{kind: ValueKind::Literal(c.memory), type_code: c.type_code};
            }

          else
            if lv.type_code.is_i()
            && rv.type_code.is_i()
            {
              let  c = self.operate_i64_constant(lmem,rmem);

              return Value{kind: ValueKind::Literal(c.memory), type_code: c.type_code};
            }

          else
            if lv.type_code.is_u()
            && rv.type_code.is_u()
            {
              let  c = self.operate_u64_constant(lmem,rmem);

              return Value{kind: ValueKind::Literal(c.memory), type_code: c.type_code};
            }

          else
            if lv.type_code.is_f()
            && rv.type_code.is_f()
            {
              let  c = self.operate_f64_constant(lmem,rmem);

              return Value{kind: ValueKind::Literal(c.memory), type_code: c.type_code};
            }

          else
            {
              panic!();
            }
        }
    }


    if let ValueKind::Literal(_) = &lv.kind
    {
      lv = Self::to_constant(lv,&rv.type_code);
    }

  else
    if let ValueKind::Literal(_) = &rv.kind
    {
      rv = Self::to_constant(rv,&lv.type_code);
    }


    if &lv.type_code != &rv.type_code
    {
      panic!();
    }


    if let ValueKind::Constant(lmem) = &lv.kind
    {
        if let ValueKind::Constant(rmem) = &rv.kind
        {
          let  tc = &lv.type_code;

               if tc.is_bool() {return Value::new_constant( self.operate_bool_constant(lmem,rmem));}
          else if tc.is_i8()   {return Value::new_constant(   self.operate_i8_constant(lmem,rmem));}
          else if tc.is_i16()  {return Value::new_constant(  self.operate_i16_constant(lmem,rmem));}
          else if tc.is_i32()  {return Value::new_constant(  self.operate_i32_constant(lmem,rmem));}
          else if tc.is_i64()  {return Value::new_constant(  self.operate_i64_constant(lmem,rmem));}
          else if tc.is_isize(){return Value::new_constant(self.operate_isize_constant(lmem,rmem));}
          else if tc.is_u8()   {return Value::new_constant(   self.operate_u8_constant(lmem,rmem));}
          else if tc.is_u16()  {return Value::new_constant(  self.operate_u16_constant(lmem,rmem));}
          else if tc.is_u32()  {return Value::new_constant(  self.operate_u32_constant(lmem,rmem));}
          else if tc.is_u64()  {return Value::new_constant(  self.operate_u64_constant(lmem,rmem));}
          else if tc.is_usize(){return Value::new_constant(self.operate_usize_constant(lmem,rmem));}
          else if tc.is_f32()  {return Value::new_constant(  self.operate_f32_constant(lmem,rmem));}
          else if tc.is_f64()  {return Value::new_constant(  self.operate_f64_constant(lmem,rmem));}
          else
            {
              panic!();
            }
        }
    }


  self.to_operated(lv,rv)
}


pub fn
print(&self)
{
    match self
    {
  BinaryOperator::Add=>{print!("+");},
  BinaryOperator::Sub=>{print!("-");},
  BinaryOperator::Mul=>{print!("*");},
  BinaryOperator::Div=>{print!("/");},
  BinaryOperator::Rem=>{print!("%");},
  BinaryOperator::Shl=>{print!("<<");},
  BinaryOperator::Shr=>{print!(">>");},
  BinaryOperator::And=>{print!("&");},
  BinaryOperator::Or=>{print!("|");},
  BinaryOperator::Xor=>{print!("^");},
  BinaryOperator::Eq=>{print!("==");},
  BinaryOperator::Neq=>{print!("!=");},
  BinaryOperator::Lt=>{print!("<");},
  BinaryOperator::Lteq=>{print!("<=");},
  BinaryOperator::Gt=>{print!(">");},
  BinaryOperator::Gteq=>{print!(">=");},
  BinaryOperator::LogicalAnd=>{print!("&&");},
  BinaryOperator::LogicalOr=>{print!("||");},
    }
}


pub fn
print_mnemonic(&self)
{
    match self
    {
  BinaryOperator::Add=>{print!("add");},
  BinaryOperator::Sub=>{print!("sub");},
  BinaryOperator::Mul=>{print!("mul");},
  BinaryOperator::Div=>{print!("div");},
  BinaryOperator::Rem=>{print!("rem");},
  BinaryOperator::Shl=>{print!("shl");},
  BinaryOperator::Shr=>{print!("shr");},
  BinaryOperator::And=>{print!("and");},
  BinaryOperator::Or=>{print!("or");},
  BinaryOperator::Xor=>{print!("xor");},
  BinaryOperator::Eq=>{print!("eq");},
  BinaryOperator::Neq=>{print!("neq");},
  BinaryOperator::Lt=>{print!("lt");},
  BinaryOperator::Lteq=>{print!("lteq");},
  BinaryOperator::Gt=>{print!("gt");},
  BinaryOperator::Gteq=>{print!("gteq");},
  BinaryOperator::LogicalAnd=>{print!("logical_and");},
  BinaryOperator::LogicalOr=>{print!("logical_or");},
    }
}


}




#[derive(Clone)]
pub enum
AssignOperator
{
  Nop,
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Shl,
  Shr,
  And,
  Or,
  Xor,

}


impl
AssignOperator
{


pub fn
print(&self)
{
    match self
    {
  AssignOperator::Nop=>{print!("=");},
  AssignOperator::Add=>{print!("+=");},
  AssignOperator::Sub=>{print!("-=");},
  AssignOperator::Mul=>{print!("*=");},
  AssignOperator::Div=>{print!("/=");},
  AssignOperator::Rem=>{print!("%=");},
  AssignOperator::Shl=>{print!("<<=");},
  AssignOperator::Shr=>{print!(">>=");},
  AssignOperator::And=>{print!("&=");},
  AssignOperator::Or=>{print!("|=");},
  AssignOperator::Xor=>{print!("^=");},
    }
}


pub fn
get_relational_operator(&self)-> Option<BinaryOperator>
{
    match self
    {
  AssignOperator::Nop=>{None},
  AssignOperator::Add=>{Some(BinaryOperator::Add)},
  AssignOperator::Sub=>{Some(BinaryOperator::Sub)},
  AssignOperator::Mul=>{Some(BinaryOperator::Mul)},
  AssignOperator::Div=>{Some(BinaryOperator::Div)},
  AssignOperator::Rem=>{Some(BinaryOperator::Rem)},
  AssignOperator::Shl=>{Some(BinaryOperator::Shl)},
  AssignOperator::Shr=>{Some(BinaryOperator::Shr)},
  AssignOperator::And=>{Some(BinaryOperator::And)},
  AssignOperator::Or=>{Some(BinaryOperator::Or)},
  AssignOperator::Xor=>{Some(BinaryOperator::Xor)},
    }
}


}




#[derive(Clone,PartialEq)]
pub struct
Path
{
  pub(crate) identifier_list: Vec<String>,
}


impl
Path
{


pub fn
new()-> Path
{
  Path{identifier_list: Vec::new()}
}


pub fn
add(mut self, name: &str)-> Path
{
    if name.len() != 0
    {
      self.identifier_list.push(name.to_string());
    }


  self
}


pub fn
push(&mut self, name: &str)
{
    if name.len() != 0
    {
      self.identifier_list.push(name.to_string());
    }
}


pub fn
pop(&mut self)-> Option<String>
{
  self.identifier_list.pop()
}


pub fn
as_strings(&self)-> &Vec<String>
{
  &self.identifier_list
}


pub fn
to_string(&self)-> String
{
  let  mut s = String::new();

    if let Some(first) = self.identifier_list.first()
    {
      s.push_str(first);

        for i in 1..self.identifier_list.len()
        {
          s.push_str("::");
          s.push_str(&self.identifier_list[i]);
        }
    }


  s
}


pub fn
print(&self)
{
    if let Some(first) = self.identifier_list.first()
    {
      print!("{}",first);

        for i in 1..self.identifier_list.len()
        {
          print!("::{}",&self.identifier_list[i]);
        }
    }
}


}




#[derive(Clone)]
pub struct
TableElement
{
  pub(crate)       name: String,
  pub(crate) expression: Expression,

}


impl
TableElement
{


pub fn
new(name: String, expression: Expression)-> Self
{
  Self{
          name,
    expression,
  }
}


pub fn
print(&self)
{
  print!("{}: ",&self.name);

  self.expression.print();
}


}




#[derive(Clone)]
pub enum
Expression
{
  Void,

  Identifier(String),

  Bool(bool),
  Int(u64),
  Float(f64),

  String(String),

  Table(Vec<TableElement>),

  SubExpression(Box<Expression>),

  Access(Box<Expression>,String),
  Call(Box<Expression>,Vec<Expression>),
  Subscript(Box<Expression>,Box<Expression>),

   Unary( UnaryOperator,Box<Expression>),
  Binary(BinaryOperator,Box<Expression>,Box<Expression>),

}


impl
Expression
{


pub fn
read(s: &str)-> Expression
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"expression",Some(dics))
    {
      let  mut cur = crate::syntax::Cursor::new(&dir);

        if let Some(d_dir) = cur.get_directory_with_name("expression")
        {
          return super::read::read_expression(d_dir);
        }
    }


  panic!();
}


pub fn
evaluate(&self, lib: &Library)-> Result<Value,()>
{
    match self
    {
  Expression::Void=>
    {
      return Ok(Value::new_void_literal());
    }
  Expression::Identifier(s)=>
    {
    },
  Expression::Bool(b)=>
    {
      return Ok(Value::new_bool_literal(*b));
    }
  Expression::Int(u)=>
    {
      return Ok(Value::new_usize_literal(*u as usize));
    }
  Expression::Float(f)=>
    {
      return Ok(Value::new_f64_literal(*f));
    }
  Expression::String(s)=>
    {
    }
  Expression::Table(ls)=>
    {
    }
  Expression::SubExpression(e)=>
    {
      return e.evaluate(lib);
    }
  Expression::Unary(o,e)=>
    {
        if let Ok(v) = e.evaluate(lib)
        {
          return o.operate(v);
        }
    }
  Expression::Call(f,args)=>
    {
    }
  Expression::Subscript(target,index)=>
    {
    }
  Expression::Access(target,name)=>
    {
    },
  Expression::Binary(o,l,r)=>
    {
        if let Ok(lv) = l.evaluate(lib)
        {
            if let Ok(rv) = r.evaluate(lib)
            {
              return Ok(o.operate(lv,rv));
            }
        }
    }
    }


  Err(())
}


pub fn
get_usize(&self, lib: &Library)-> Result<usize,()>
{
    if let Ok(v) = self.evaluate(lib)
    {
        if v.type_code.is_usize()
        {
            match &v.kind
            {
          ValueKind::Literal(m)=> {return Ok(to_u64(m) as usize);}
          ValueKind::Constant(m)=>{return Ok(to_u64(m) as usize);}
          _=>{}
            }
        }
    }


  Err(())
}


pub fn
print(&self)
{
    match self
    {
  Expression::Void=>{print!("void");},
  Expression::Identifier(s)=>{print!("{}",s);},
  Expression::Bool(b)=>{print!("{}",b);},
  Expression::Int(u)=>{print!("{}",*u);},
  Expression::Float(f)=>{print!("{}",*f);},
  Expression::String(s)=>{print!("\"{}\"",s);},
  Expression::Table(ls)=>
        {
          print!("[");

            for e in ls
            {
              e.print();

              print!(", ");
            }


          print!("]");
        },
  Expression::SubExpression(e)=>
        {
          print!("(");
          e.print();
          print!(")");
        },
  Expression::Unary(o,e)=>
        {
          o.print();
          e.print();
        },
  Expression::Call(f,args)=>
        {
          f.print();

          print!("(");

            for e in args
            {
              e.print();

              print!(", ");
            }


          print!(")");
        },
  Expression::Subscript(target,index)=>
        {
          target.print();

          print!("[");

          index.print();

          print!("]");
        },
  Expression::Access(target,name)=>
        {
          target.print();

          print!(".{}",name);
        },
  Expression::Binary(o,l,r)=>
        {
          l.print();
          o.print();
          r.print();
        },
    }
}


}




