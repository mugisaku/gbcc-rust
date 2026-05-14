

use std::rc::Rc;

use super::*;
use super::ty::*;


#[derive(Clone)]
pub enum
Opcode
{
  Nop,

  Push0,
  Push1,
  Push2,
  Push3,
  Push4,
  Push5,
  Push6,
  Push7,
  Push8,

  Pushpc,
  Pushfp,
  Pushsp,

  Pushi8,
  Pushi16,
  Pushi32,
  Pushu8,
  Pushu16,
  Pushu32,
  Pushu64,
  Pushf32,

  Xs8,
  Xs16,
  Xs32,

  Jmp8,
  Jmp16,
  Jmp32,

  Brz8,
  Brz16,
  Brz32,

  Brnz8,
  Brnz16,
  Brnz32,

  Pop,
  Dup,

  Sx8, Sx16, Sx32,
  Tr8, Tr16, Tr32,

  B32toF,
  FtoB32,

  Ld8, Ld16, Ld32, Ld64,
  St8, St16, St32, St64,

  Neg, Negf,
  Not, Notl,

  Itof, Ftoi,


  Addi, Subi, Muli, Divi, Remi,
  Addu, Subu, Mulu, Divu, Remu,
  Addf, Subf, Mulf, Divf, Remf,

  Shl, Shr, And, Or, Xor,

  Eq,  Neq,
  Eqf, Neqf,

  Lti, Lteqi, Gti, Gteqi,
  Ltu, Ltequ, Gtu, Gtequ,
  Ltf, Lteqf, Gtf, Gteqf,

  Land, Lor,

  Prcal,
    Cal,

  Ret,
  Hlt,

  Pri,

}


impl
Opcode
{


pub fn
to_str(&self)-> &'static str
{
    match self
    {
  Self::Nop=>{"nop"}

  Self::Push0=>{"push0"}
  Self::Push1=>{"push1"}
  Self::Push2=>{"push2"}
  Self::Push3=>{"push3"}
  Self::Push4=>{"push4"}
  Self::Push5=>{"push5"}
  Self::Push6=>{"push6"}
  Self::Push7=>{"push7"}
  Self::Push8=>{"push8"}

  Self::Pushpc=>{"pushpc"}
  Self::Pushfp=>{"pushfp"}
  Self::Pushsp=>{"pushsp"}

  Self::Pushi8 =>{"pushi8"}
  Self::Pushi16=>{"pushi16"}
  Self::Pushi32=>{"pushi32"}
  Self::Pushu8 =>{"pushu8"}
  Self::Pushu16=>{"pushu16"}
  Self::Pushu32=>{"pushu32"}
  Self::Pushu64=>{"pushu64"}
  Self::Pushf32=>{"pushf32"}


  Self::Xs8 =>{"xs8"}
  Self::Xs16=>{"xs16"}
  Self::Xs32=>{"xs32"}

  Self::Jmp8 =>{"jmp8"}
  Self::Jmp16=>{"jmp16"}
  Self::Jmp32=>{"jmp32"}

  Self::Brz8 =>{"brz8"}
  Self::Brz16=>{"brz16"}
  Self::Brz32=>{"brz32"}

  Self::Brnz8 =>{"brnz8"}
  Self::Brnz16=>{"brnz16"}
  Self::Brnz32=>{"brnz32"}

  Self::Pop=>{"pop"}
  Self::Dup=>{"dup"}

  Self::Sx8 =>{"sx8"}
  Self::Sx16=>{"sx16"}
  Self::Sx32=>{"sx32"}
  Self::Tr8 =>{"tr8"}
  Self::Tr16=>{"tr16"}
  Self::Tr32=>{"tr32"}

  Self::B32toF=>{"b32tof"}
  Self::FtoB32=>{"ftob32"}

  Self::Ld8 =>{"ld8"}
  Self::Ld16=>{"ld16"}
  Self::Ld32=>{"ld32"}
  Self::Ld64=>{"ld64"}
  Self::St8 =>{"st8"}
  Self::St16=>{"st16"}
  Self::St32=>{"st32"}
  Self::St64=>{"st64"}

  Self::Neg =>{"neg"}
  Self::Negf=>{"negf"}
  Self::Not =>{"not"}
  Self::Notl=>{"notl"}

  Self::Itof=>{"itof"}
  Self::Ftoi=>{"ftoi"}


  Self::Addi=>{"addi"}
  Self::Subi=>{"subi"}
  Self::Muli=>{"muli"}
  Self::Divi=>{"divi"}
  Self::Remi=>{"remi"}
  Self::Addu=>{"addu"}
  Self::Subu=>{"subu"}
  Self::Mulu=>{"mulu"}
  Self::Divu=>{"divu"}
  Self::Remu=>{"remu"}
  Self::Addf=>{"addf"}
  Self::Subf=>{"subf"}
  Self::Mulf=>{"mulf"}
  Self::Divf=>{"divf"}
  Self::Remf=>{"remf"}

  Self::Shl=>{"shl"}
  Self::Shr=>{"shr"}
  Self::And=>{"and"}
  Self::Or =>{"or"}
  Self::Xor=>{"xor"}

  Self::Eq  =>{"eq"}
  Self::Neq =>{"neq"}
  Self::Eqf =>{"eqf"}
  Self::Neqf=>{"neqf"}

  Self::Lti  =>{"lti"}
  Self::Lteqi=>{"lteqi"}
  Self::Gti  =>{"gti"}
  Self::Gteqi=>{"gteqi"}
  Self::Ltu  =>{"ltu"}
  Self::Ltequ=>{"ltequ"}
  Self::Gtu  =>{"gtu"}
  Self::Gtequ=>{"gtequ"}
  Self::Ltf  =>{"ltf"}
  Self::Lteqf=>{"lteqf"}
  Self::Gtf  =>{"gtf"}
  Self::Gteqf=>{"gteqf"}

  Self::Land=>{"land"}
  Self::Lor=>{"lor"}

  Self::Prcal=>{"prcal"}
  Self::Cal=>{"cal"}

  Self::Ret=>{"ret"}
  Self::Hlt=>{"hlt"}

  Self::Pri=>{"pri"}
    }
}


pub fn
print(&self)
{
  print!("{}",self.to_str());
}


}


impl
std::convert::From<u8> for Opcode
{


fn
from(b: u8)-> Self
{
    match b
    {
  (op) if op == Self::Nop as u8=>{Self::Nop}
  (op) if op == Self::Push0 as u8=>{Self::Push0}
  (op) if op == Self::Push1 as u8=>{Self::Push1}
  (op) if op == Self::Push2 as u8=>{Self::Push2}
  (op) if op == Self::Push3 as u8=>{Self::Push3}
  (op) if op == Self::Push4 as u8=>{Self::Push4}
  (op) if op == Self::Push5 as u8=>{Self::Push5}
  (op) if op == Self::Push6 as u8=>{Self::Push6}
  (op) if op == Self::Push7 as u8=>{Self::Push7}
  (op) if op == Self::Push8 as u8=>{Self::Push8}
  (op) if op == Self::Pushpc as u8=>{Self::Pushpc}
  (op) if op == Self::Pushfp as u8=>{Self::Pushfp}
  (op) if op == Self::Pushsp as u8=>{Self::Pushsp}
  (op) if op == Self::Xs8 as u8=>{Self::Xs8}
  (op) if op == Self::Xs16 as u8=>{Self::Xs16}
  (op) if op == Self::Xs32 as u8=>{Self::Xs32}
  (op) if op == Self::Jmp8 as u8=>{Self::Jmp8}
  (op) if op == Self::Jmp16 as u8=>{Self::Jmp16}
  (op) if op == Self::Jmp32 as u8=>{Self::Jmp32}
  (op) if op == Self::Brz8 as u8=>{Self::Brz8}
  (op) if op == Self::Brz16 as u8=>{Self::Brz16}
  (op) if op == Self::Brz32 as u8=>{Self::Brz32}
  (op) if op == Self::Brnz8 as u8=>{Self::Brnz8}
  (op) if op == Self::Brnz16 as u8=>{Self::Brnz16}
  (op) if op == Self::Brnz32 as u8=>{Self::Brnz32}
  (op) if op == Self::Pop as u8=>{Self::Pop}
  (op) if op == Self::Dup as u8=>{Self::Dup}
  (op) if op == Self::Ld8 as u8=>{Self::Ld8}
  (op) if op == Self::Ld16 as u8=>{Self::Ld16}
  (op) if op == Self::Ld32 as u8=>{Self::Ld32}
  (op) if op == Self::Ld64 as u8=>{Self::Ld64}
  (op) if op == Self::St8 as u8=>{Self::St8}
  (op) if op == Self::St16 as u8=>{Self::St16}
  (op) if op == Self::St32 as u8=>{Self::St32}
  (op) if op == Self::St64 as u8=>{Self::St64}
  (op) if op == Self::Sx8 as u8=>{Self::Sx8}
  (op) if op == Self::Sx16 as u8=>{Self::Sx16}
  (op) if op == Self::Sx32 as u8=>{Self::Sx32}
  (op) if op == Self::Tr8 as u8=>{Self::Tr8}
  (op) if op == Self::Tr16 as u8=>{Self::Tr16}
  (op) if op == Self::Tr32 as u8=>{Self::Tr32}
  (op) if op == Self::B32toF as u8=>{Self::B32toF}
  (op) if op == Self::FtoB32 as u8=>{Self::FtoB32}
  (op) if op == Self::Neg  as u8=>{Self::Neg}
  (op) if op == Self::Negf as u8=>{Self::Negf}
  (op) if op == Self::Not  as u8=>{Self::Not}
  (op) if op == Self::Notl as u8=>{Self::Notl}
  (op) if op == Self::Itof as u8=>{Self::Itof}
  (op) if op == Self::Ftoi as u8=>{Self::Ftoi}
  (op) if op == Self::Addi as u8=>{Self::Addi}
  (op) if op == Self::Subi as u8=>{Self::Subi}
  (op) if op == Self::Muli as u8=>{Self::Muli}
  (op) if op == Self::Divi as u8=>{Self::Divi}
  (op) if op == Self::Remi as u8=>{Self::Remi}
  (op) if op == Self::Addu as u8=>{Self::Addu}
  (op) if op == Self::Subu as u8=>{Self::Subu}
  (op) if op == Self::Mulu as u8=>{Self::Mulu}
  (op) if op == Self::Divu as u8=>{Self::Divu}
  (op) if op == Self::Remu as u8=>{Self::Remu}
  (op) if op == Self::Addf as u8=>{Self::Addf}
  (op) if op == Self::Subf as u8=>{Self::Subf}
  (op) if op == Self::Mulf as u8=>{Self::Mulf}
  (op) if op == Self::Divf as u8=>{Self::Divf}
  (op) if op == Self::Remf as u8=>{Self::Remf}
  (op) if op == Self::Shl as u8=>{Self::Shl}
  (op) if op == Self::Shr as u8=>{Self::Shr}
  (op) if op == Self::And as u8=>{Self::And}
  (op) if op == Self::Or  as u8=>{Self::Or}
  (op) if op == Self::Xor as u8=>{Self::Xor}
  (op) if op == Self::Eq  as u8=>{Self::Eq}
  (op) if op == Self::Neq as u8=>{Self::Neq}
  (op) if op == Self::Eqf  as u8=>{Self::Eqf}
  (op) if op == Self::Neqf as u8=>{Self::Neqf}
  (op) if op == Self::Lti   as u8=>{Self::Lti}
  (op) if op == Self::Lteqi as u8=>{Self::Lteqi}
  (op) if op == Self::Gti   as u8=>{Self::Gti}
  (op) if op == Self::Gteqi as u8=>{Self::Gteqi}
  (op) if op == Self::Ltu   as u8=>{Self::Ltu}
  (op) if op == Self::Ltequ as u8=>{Self::Ltequ}
  (op) if op == Self::Gtu   as u8=>{Self::Gtu}
  (op) if op == Self::Gtequ as u8=>{Self::Gtequ}
  (op) if op == Self::Ltf   as u8=>{Self::Ltf}
  (op) if op == Self::Lteqf as u8=>{Self::Lteqf}
  (op) if op == Self::Gtf   as u8=>{Self::Gtf}
  (op) if op == Self::Gteqf as u8=>{Self::Gteqf}
  (op) if op == Self::Land as u8=>{Self::Land}
  (op) if op == Self::Lor  as u8=>{Self::Lor}
  (op) if op == Self::Prcal as u8=>{Self::Prcal}
  (op) if op == Self::Cal as u8=>{Self::Cal}
  (op) if op == Self::Ret as u8=>{Self::Ret}
  (op) if op == Self::Hlt as u8=>{Self::Hlt}
  (op) if op == Self::Pri as u8=>{Self::Pri}
  _=>{panic!();}
    }
}


}




#[derive(Clone,Default)]
struct
LineInfo
{
  offset: usize,
    size: usize,

}


#[derive(Clone)]
struct
Destination
{
  label: String,
  index: usize,

}




#[derive(Clone)]
pub enum
AsmLine
{
  Label(String),

  Opcode(Opcode),

   Pushi8(i8),
  Pushi16(i16),
  Pushi32(i32),
   Pushu8(u8),
  Pushu16(u16),
  Pushu32(u32),
  Pushu64(u64),
  Pushf32(f32),

   Xs8(u8),
  Xs16(u16),
  Xs32(u32),

   Jmp(i32,Destination),
   Brz(i32,Destination),
  Brnz(i32,Destination),

}


impl
AsmLine
{


pub fn
make_pushi(i: i64)-> Self
{
    if i >= 0
    {
      Self::make_pushu(i as u64)
    }

  else if i >= ( i8::MIN as i64){Self::Pushi8( i as i8)}
  else if i >= (i16::MIN as i64){Self::Pushi16(i as i16)}
  else if i >= (i32::MIN as i64){Self::Pushi32(i as i32)}
  else                          {Self::Pushu64(i as u64)}
}


pub fn
make_pushu(u: u64)-> Self
{
       if u == 0{Self::Opcode(Opcode::Push0)}
  else if u == 1{Self::Opcode(Opcode::Push1)}
  else if u == 2{Self::Opcode(Opcode::Push2)}
  else if u == 3{Self::Opcode(Opcode::Push3)}
  else if u == 4{Self::Opcode(Opcode::Push4)}
  else if u == 5{Self::Opcode(Opcode::Push5)}
  else if u == 6{Self::Opcode(Opcode::Push6)}
  else if u == 7{Self::Opcode(Opcode::Push7)}
  else if u == 8{Self::Opcode(Opcode::Push8)}
  else if u <= ( u8::MAX as u64){Self::Pushu8( u as u8)}
  else if u <= (u16::MAX as u64){Self::Pushu16(u as u16)}
  else if u <= (u32::MAX as u64){Self::Pushu32(u as u32)}
  else                          {Self::Pushu64(u       )}
}


pub fn
make_pushf(f: f64)-> Self
{
      if f.abs() <= (f32::MAX as f64){Self::Pushf32(f as f32)}
  else                               {Self::Pushu64(f.to_bits())}
}


pub fn
make_xs(n: usize)-> Self
{
       if n <= ( u8::MAX as usize){Self::Xs8( n as  u8)}
  else if n <= (u16::MAX as usize){Self::Xs16(n as u16)}
  else if n <= (u32::MAX as usize){Self::Xs32(n as u32)}
  else{panic!();}
}


pub fn
make_jmp(s: &str)-> Self
{
  let  dst = Destination{label: s.to_string(), index: 0};

  Self::Jmp(0,dst)
}


pub fn
make_brz(s: &str)-> Self
{
  let  dst = Destination{label: s.to_string(), index: 0};

  Self::Brz(0,dst)
}


pub fn
make_brnz(s: &str)-> Self
{
  let  dst = Destination{label: s.to_string(), index: 0};

  Self::Brnz(0,dst)
}


pub fn
get_size_of_i(i: i32)-> usize
{
    match i.abs()
    {
  (v) if v <= ( i8::MAX as i32)=>{1}
  (v) if v <= (i16::MAX as i32)=>{2}
  _=>{4}
    }
}

pub fn
get_size(&self)-> usize
{
    match self
    {
  Self::Label(_)=>{0}

  Self::Opcode(_)=>{1}

  Self::Pushi8(_)
 |Self::Pushu8(_)
 |Self::Xs8(_)=>{2}

  Self::Pushi16(_)
 |Self::Pushu16(_)
 |Self::Xs16(_)=>{3}

  Self::Pushi32(_)
 |Self::Pushu32(_)
 |Self::Pushf32(_)
 |Self::Xs32(_)=>{5}

  Self::Pushu64(_)=>{9}

  Self::Jmp(i,_) =>{1+Self::get_size_of_i(*i)}
  Self::Brz(i,_) =>{1+Self::get_size_of_i(*i)}
  Self::Brnz(i,_)=>{1+Self::get_size_of_i(*i)}
    }
}


pub fn
write_u8_to(op: Opcode, u: u8, buf: &mut Vec<u8>)
{
  buf.push(op as u8);

  buf.push(u);
}


pub fn
write_u16_to(op: Opcode, u: u16, buf: &mut Vec<u8>)
{
  buf.push(op as u8);

  let  bytes = u.to_be_bytes();

    for b in bytes
    {
      buf.push(b);
    }
}


pub fn
write_u32_to(op: Opcode, u: u32, buf: &mut Vec<u8>)
{
  buf.push(op as u8);

  let  bytes = u.to_be_bytes();

    for b in bytes
    {
      buf.push(b);
    }
}


pub fn
write_u64_to(op: Opcode, u: u64, buf: &mut Vec<u8>)
{
  buf.push(op as u8);

  let  bytes = u.to_be_bytes();

    for b in bytes
    {
      buf.push(b);
    }
}


pub fn
write_to(&self, buf: &mut Vec<u8>)
{
    match self
    {
  Self::Label(_)=>{}

  Self::Opcode(op)=>{buf.push(op.clone() as u8);}

  Self::Pushi8(i) =>{Self::write_u8_to(Opcode::Pushi8 ,*i as u8,buf);}
  Self::Pushi16(i)=>{Self::write_u16_to(Opcode::Pushi16,*i as u16,buf);}
  Self::Pushi32(i)=>{Self::write_u32_to(Opcode::Pushi32,*i as u32,buf);}
  Self::Pushu8(u) =>{Self::write_u8_to(Opcode::Pushu8 ,*u,buf);}
  Self::Pushu16(u)=>{Self::write_u16_to(Opcode::Pushu16,*u,buf);}
  Self::Pushu32(u)=>{Self::write_u32_to(Opcode::Pushu32,*u,buf);}
  Self::Pushu64(u)=>{Self::write_u64_to(Opcode::Pushu64,*u,buf);}
  Self::Pushf32(f)=>{Self::write_u32_to(Opcode::Pushf32,f.to_bits(),buf);}
  Self::Xs8(u) =>{Self::write_u8_to(Opcode::Xs8 ,*u,buf);}
  Self::Xs16(u)=>{Self::write_u16_to(Opcode::Xs16,*u,buf);}
  Self::Xs32(u)=>{Self::write_u32_to(Opcode::Xs32,*u,buf);}
  Self::Jmp(i,_)=>
    {
        match Self::get_size_of_i(*i)
        {
      1=>{Self::write_u8_to( Opcode::Jmp8 ,*i as  u8,buf);}
      2=>{Self::write_u16_to(Opcode::Jmp16,*i as u16,buf);}
      4=>{Self::write_u32_to(Opcode::Jmp32,*i as u32,buf);}
      _=>{panic!();}
        }
    }
  Self::Brz(i,_)=>
    {
        match Self::get_size_of_i(*i)
        {
      1=>{Self::write_u8_to( Opcode::Brz8 ,*i as  u8,buf);}
      2=>{Self::write_u16_to(Opcode::Brz16,*i as u16,buf);}
      4=>{Self::write_u32_to(Opcode::Brz32,*i as u32,buf);}
      _=>{panic!();}
        }
    }
  Self::Brnz(i,_)=>
    {
        match Self::get_size_of_i(*i)
        {
      1=>{Self::write_u8_to( Opcode::Brnz8 ,*i as  u8,buf);}
      2=>{Self::write_u16_to(Opcode::Brnz16,*i as u16,buf);}
      4=>{Self::write_u32_to(Opcode::Brnz32,*i as u32,buf);}
      _=>{panic!();}
        }
    }
    }
}


pub fn
print(&self)
{
    match self
    {
  Self::Label(s)=>{print!("[{}]",s);}

  Self::Opcode(op)=>{op.print();}

  Self::Pushi8(i) =>{print!("pushi8 {}",*i);}
  Self::Pushi16(i)=>{print!("pushi16 {}",*i);}
  Self::Pushi32(i)=>{print!("pushi32 {}",*i);}
  Self::Pushu8(u) =>{print!("pushu8 {}",*u);}
  Self::Pushu16(u)=>{print!("pushu16 {}",*u);}
  Self::Pushu32(u)=>{print!("pushu32 {}",*u);}
  Self::Pushu64(u)=>{print!("pushu64 {}",*u);}
  Self::Pushf32(f)=>{print!("pushf32 {}",*f);}

  Self::Xs8(u) =>{print!("xs8 {}",*u);}
  Self::Xs16(u)=>{print!("xs16 {}",*u);}
  Self::Xs32(u)=>{print!("xs32 {}",*u);}

  Self::Jmp(i,dst)=>{print!("jmp {}({})",&dst.label,*i);}
  Self::Brz(i,dst)=>{print!("brz {}({})",&dst.label,*i);}
  Self::Brnz(i,dst)=>{print!("brnz {}({})",&dst.label,*i);}
    }
}


}




#[derive(Clone)]
pub struct
AsmEvalText
{
  lines: Vec<AsmLine>,

  ty: Rc<Ty>,

  is_deref: bool,

}


impl
AsmEvalText
{


pub fn
new()-> Self
{
  Self{lines: Vec::new(), ty: find_ty("void").unwrap(), is_deref: false}
}


pub fn
get_ty_name(&self)-> &String
{
  self.ty.get_name()
}


pub fn
get_ty(&self)-> Rc<Ty>
{
  Rc::clone(&self.ty)
}


pub fn
set_ty(&mut self, ty: &Rc<Ty>)
{
  self.ty = Rc::clone(ty);
}


pub fn
set_ty_by_name(&mut self, name: &str)
{
  self.ty = find_ty(name).unwrap();
}


pub fn
is_deref(&self)-> bool
{
  self.is_deref
}


pub fn
push_opcode(&mut self, opcode: Opcode)
{
  self.lines.push(AsmLine::Opcode(opcode));
}


pub fn
push_2opcodes(&mut self, a: Opcode, b: Opcode)
{
  self.push_opcode(a);
  self.push_opcode(b);
}


pub fn
push_bool(&mut self, b: bool)
{
  self.lines.push(AsmLine::Opcode(if b{Opcode::Push1} else{Opcode::Push0}));

  self.set_ty_by_name("bool");
}


pub fn
push_i8(&mut self, i: i8)
{
  self.lines.push(AsmLine::make_pushi(i as i64));

  self.set_ty_by_name("i8");
}


pub fn
push_i16(&mut self, i: i16)
{
  self.lines.push(AsmLine::make_pushi(i as i64));

  self.set_ty_by_name("i16");
}


pub fn
push_i32(&mut self, i: i32)
{
  self.lines.push(AsmLine::make_pushi(i as i64));

  self.set_ty_by_name("i32");
}


pub fn
push_i64(&mut self, i: i64)
{
  self.lines.push(AsmLine::make_pushi(i as i64));

  self.set_ty_by_name("i64");
}


pub fn
push_isize(&mut self, i: isize)
{
  self.lines.push(AsmLine::make_pushi(i as i64));

  self.set_ty_by_name("isize");
}


pub fn
push_u8(&mut self, u: u8)
{
  self.lines.push(AsmLine::make_pushu(u as u64));

  self.set_ty_by_name("u8");
}


pub fn
push_u16(&mut self, u: u16)
{
  self.lines.push(AsmLine::make_pushu(u as u64));

  self.set_ty_by_name("u16");
}


pub fn
push_u32(&mut self, u: u32)
{
  self.lines.push(AsmLine::make_pushu(u as u64));

  self.set_ty_by_name("u32");
}


pub fn
push_u64(&mut self, u: u64)
{
  self.lines.push(AsmLine::make_pushu(u as u64));

  self.set_ty_by_name("u64");
}


pub fn
push_usize(&mut self, u: usize)
{
  self.lines.push(AsmLine::make_pushu(u as u64));

  self.set_ty_by_name("usize");
}


pub fn
push_f32(&mut self, f: f32)
{
  self.lines.push(AsmLine::make_pushf(f as f64));

  self.set_ty_by_name("f32");
}


pub fn
push_f64(&mut self, f: f64)
{
  self.lines.push(AsmLine::make_pushf(f as f64));

  self.set_ty_by_name("f64");
}


pub fn
push_global_var(&mut self, off: usize, ty: Rc<Ty>)
{
  self.push_usize(off);

  self.ty = ty;
  self.is_deref = true;
}


pub fn
push_fn(&mut self, off: usize, ty: Rc<Ty>)
{
  self.push_usize(off);
  self.push_opcode(Opcode::Ld64);

  self.ty = ty;
  self.is_deref = true;
}


pub fn
push_local_var(&mut self, off: usize, ty: Rc<Ty>)
{
  self.push_opcode(Opcode::Pushfp);
  self.push_usize(off);
  self.push_opcode(Opcode::Addi);

  self.ty = ty;
  self.is_deref = true;
}


pub fn
push_call(&mut self, args: Vec<Self>)
{
  let  ty = Rc::clone(&self.ty);

    if let TyKind::Function{parameter_tys, return_ty} = ty.get_kind()
    {
        if parameter_tys.len() != args.len()
        {
          panic!();
        }


      self.push_opcode(Opcode::Prcal);

        for a in args
        {
          self.lines.extend(a.lines);
        }


      self.push_opcode(Opcode::Cal);

      self.ty = Rc::clone(&return_ty);

      self.is_deref = false;
    }

  else
    {panic!();}
}


pub fn
push_load(&mut self)
{
    if !self.is_deref
    {
      panic!();
    }


    match self.ty.get_name()
    {
  (s) if s == "bool" =>{self.push_opcode(Opcode::Ld8);}
  (s) if s == "i8"   =>{self.push_2opcodes(Opcode::Ld8 ,Opcode::Sx8 );}
  (s) if s == "i16"  =>{self.push_2opcodes(Opcode::Ld16,Opcode::Sx16);}
  (s) if s == "i32"  =>{self.push_2opcodes(Opcode::Ld32,Opcode::Sx32);}
  (s) if s == "i64"  =>{self.push_opcode(Opcode::Ld64);}
  (s) if s == "isize"=>{self.push_opcode(Opcode::Ld64);}
  (s) if s == "u8"   =>{self.push_opcode(Opcode::Ld8 );}
  (s) if s == "u16"  =>{self.push_opcode(Opcode::Ld16);}
  (s) if s == "u32"  =>{self.push_opcode(Opcode::Ld32);}
  (s) if s == "u64"  =>{self.push_opcode(Opcode::Ld64);}
  (s) if s == "usize"=>{self.push_opcode(Opcode::Ld64);}
  (s) if s == "f32"  =>{self.push_2opcodes(Opcode::Ld32,Opcode::B32toF);}
  (s) if s == "f64"  =>{self.push_opcode(Opcode::Ld64);}
  _=>{panic!();}
    }


  self.is_deref = false;
}


pub fn
push_store(&mut self)
{
    if !self.is_deref
    {
      panic!();
    }


    match self.ty.get_name()
    {
  (s) if s == "bool" =>{self.push_opcode(Opcode::St8);}
  (s) if s == "i8"   =>{self.push_opcode(Opcode::St8 );}
  (s) if s == "i16"  =>{self.push_opcode(Opcode::St16);}
  (s) if s == "i32"  =>{self.push_opcode(Opcode::St32);}
  (s) if s == "i64"  =>{self.push_opcode(Opcode::St64);}
  (s) if s == "isize"=>{self.push_opcode(Opcode::St64);}
  (s) if s == "u8"   =>{self.push_opcode(Opcode::St8 );}
  (s) if s == "u16"  =>{self.push_opcode(Opcode::St16);}
  (s) if s == "u32"  =>{self.push_opcode(Opcode::St32);}
  (s) if s == "u64"  =>{self.push_opcode(Opcode::St64);}
  (s) if s == "usize"=>{self.push_opcode(Opcode::St64);}
  (s) if s == "f32"  =>{self.push_2opcodes(Opcode::FtoB32,Opcode::Ld32);}
  (s) if s == "f64"  =>{self.push_opcode(Opcode::St64);}
  _=>{panic!();}
    }


  self.set_ty_by_name("void");
}


pub fn
push_unary(&mut self, op: &str)
{
  let  ty_name = self.ty.get_name().clone();

    match op
    {
  (s) if s == "-"=>
    {
        if self.is_deref
        {
          self.push_load();
        }


        if (&ty_name == "i8")
        || (&ty_name == "i16")
        || (&ty_name == "i32")
        || (&ty_name == "i64")
        || (&ty_name == "isize")
        {
          self.push_opcode(Opcode::Neg);
        }

      else
        if (&ty_name == "f32")
        || (&ty_name == "f64")
        {
          self.push_opcode(Opcode::Negf);
        }
    }
  (s) if s == "!"=>
    {
        if self.is_deref
        {
          self.push_load();
        }


        if (&ty_name == "i8")
        || (&ty_name == "i16")
        || (&ty_name == "i32")
        || (&ty_name == "i64")
        || (&ty_name == "isize")
        || (&ty_name == "u8")
        || (&ty_name == "u16")
        || (&ty_name == "u32")
        || (&ty_name == "u64")
        || (&ty_name == "usize")
        {
          self.push_opcode(Opcode::Not);
        }

      else
        if (&ty_name == "bool")
        {
          self.push_opcode(Opcode::Notl);
        }
    }
  (s) if s == "&"=>
    {
      todo!();
    }
  (s) if s == "*"=>
    {
/*
        match self.ty.get_kind()
        {
      TyKind::Pointer(ty)=>
        {
          self.ty_name = ty_name.clone();
          self.is_deref = true;
        }
      TyKind::Reference(ty_name)=>
        {
          self.ty_name = ty_name.clone();
          self.is_deref = true;
        }
      _=>{panic!();}
        }
*/
todo!();
    }
  _=>{panic!();}
    }
}


fn
push_ari_or_cmp(&mut self, other: Self, i_op: Opcode, u_op: Opcode, f_op: Opcode, is_cmp: bool)
{
    if self.ty.get_name() != other.ty.get_name()
    {
      panic!();
    }


  let  op = match self.ty.get_name()
    {
  (s) if s == "i8"   =>{i_op}
  (s) if s == "i16"  =>{i_op}
  (s) if s == "i32"  =>{i_op}
  (s) if s == "i64"  =>{i_op}
  (s) if s == "isize"=>{i_op}
  (s) if s == "u8"   =>{u_op}
  (s) if s == "u16"  =>{u_op}
  (s) if s == "u32"  =>{u_op}
  (s) if s == "u64"  =>{u_op}
  (s) if s == "usize"=>{u_op}
  (s) if s == "f32"  =>{f_op}
  (s) if s == "f64"  =>{f_op}
  _=>{Opcode::Hlt}
    };


    if let Opcode::Hlt = op
    {
      panic!();
    }


  self.lines.extend(other.lines);

  self.push_opcode(op);

    if is_cmp
    {
      self.set_ty_by_name("bool");
    }
}


fn
push_log(&mut self, other: Self, op: Opcode)
{
    if self.ty.get_name() != other.ty.get_name()
    {
      panic!();
    }


    if self.ty.get_name() != "bool"
    {
      panic!();
    }


  self.lines.extend(other.lines);

  self.push_opcode(op);
}


pub fn
push_binary(&mut self, mut other: Self, op: &str)
{
    if self.is_deref
    {
      self.push_load();
    }


    if other.is_deref
    {
      other.push_load();
    }


    match op
    {
  (s) if s ==  "+"=>{self.push_ari_or_cmp(other,Opcode::Addi ,Opcode::Addu ,Opcode::Addf ,false)}
  (s) if s ==  "-"=>{self.push_ari_or_cmp(other,Opcode::Subi ,Opcode::Subu ,Opcode::Subf ,false)}
  (s) if s ==  "*"=>{self.push_ari_or_cmp(other,Opcode::Muli ,Opcode::Mulu ,Opcode::Mulf ,false)}
  (s) if s ==  "/"=>{self.push_ari_or_cmp(other,Opcode::Divi ,Opcode::Divu ,Opcode::Divf ,false)}
  (s) if s ==  "%"=>{self.push_ari_or_cmp(other,Opcode::Remi ,Opcode::Remu ,Opcode::Remf ,false)}
  (s) if s == "<<"=>{self.push_ari_or_cmp(other,Opcode::Shl  ,Opcode::Shl  ,Opcode::Hlt  ,false)}
  (s) if s == ">>"=>{self.push_ari_or_cmp(other,Opcode::Shr  ,Opcode::Shr  ,Opcode::Hlt  ,false)}
  (s) if s ==  "&"=>{self.push_ari_or_cmp(other,Opcode::And  ,Opcode::And  ,Opcode::Hlt  ,false)}
  (s) if s ==  "|"=>{self.push_ari_or_cmp(other,Opcode::Or   ,Opcode::Or   ,Opcode::Hlt  ,false)}
  (s) if s ==  "^"=>{self.push_ari_or_cmp(other,Opcode::Xor  ,Opcode::Xor  ,Opcode::Hlt  ,false)}
  (s) if s == "=="=>{self.push_ari_or_cmp(other,Opcode::Eq   ,Opcode::Eq   ,Opcode::Eqf  ,true)}
  (s) if s == "!="=>{self.push_ari_or_cmp(other,Opcode::Neq  ,Opcode::Neq  ,Opcode::Neqf ,true)}
  (s) if s ==  "<"=>{self.push_ari_or_cmp(other,Opcode::Lti  ,Opcode::Ltu  ,Opcode::Ltf  ,true)}
  (s) if s == "<="=>{self.push_ari_or_cmp(other,Opcode::Lteqi,Opcode::Ltequ,Opcode::Lteqf,true)}
  (s) if s ==  ">"=>{self.push_ari_or_cmp(other,Opcode::Gti  ,Opcode::Gtu  ,Opcode::Gtf  ,true)}
  (s) if s == ">="=>{self.push_ari_or_cmp(other,Opcode::Gteqi,Opcode::Gtequ,Opcode::Gteqf,true)}
  (s) if s == "&&"=>{self.push_log(other,Opcode::Land)}
  (s) if s == "||"=>{self.push_log(other,Opcode::Lor )}
  _=>{panic!();}
    }
}


pub fn
push_text(&mut self, mut other: Self)
{
  self.lines.append(&mut other.lines);
}


}




#[derive(Clone)]
pub struct
AsmText
{
  lines: Vec<(AsmLine,LineInfo)>,

}


impl
AsmText
{


pub fn
new()-> Self
{
  let  li = LineInfo::default();

  Self{
    lines: vec![(AsmLine::Opcode(Opcode::Nop),li)],
  }
}


pub fn
set_xs(&mut self, sz: usize)
{
    if let Some((ln,_)) = self.lines.first_mut()
    {
      *ln = AsmLine::make_xs(sz/WORD_SIZE);
    }
}


pub fn
push_label(&mut self, s: &str)
{
  let  li = LineInfo::default();

  self.lines.push((AsmLine::Label(s.to_string()),li));
}


pub fn
push_opcode(&mut self, opcode: Opcode)
{
  let  ln = AsmLine::Opcode(opcode);

  let  li = LineInfo::default();

  self.lines.push((ln,li));
}


pub fn
push_line(&mut self, ln: AsmLine)
{
  let  li = LineInfo::default();

  self.lines.push((ln,li));
}


pub fn
push_eval_text(&mut self, et: AsmEvalText)
{
    for ln in et.lines
    {
      self.push_line(ln);
    }
}


pub fn
push_jmp(&mut self, s: &str)
{
  self.push_line(AsmLine::make_jmp(s));
}


pub fn
push_brz(&mut self, s: &str)
{
  self.push_line(AsmLine::make_brz(s));
}


pub fn
push_brnz(&mut self, s: &str)
{
  self.push_line(AsmLine::make_brnz(s));
}


pub fn
push_assign(&mut self, mut l: AsmEvalText, r: AsmEvalText, op: &str)
{
    if !l.is_deref
    {
      panic!();
    }


    if &l.ty.get_name() != &r.ty.get_name()
    {
      panic!();
    }


  l.push_opcode(Opcode::Dup);
  l.push_load();

       if op ==  "+="{l.push_binary(r,"+");}
  else if op ==  "-="{l.push_binary(r,"-");}
  else if op ==  "*="{l.push_binary(r,"*");}
  else if op ==  "/="{l.push_binary(r,"/");}
  else if op ==  "%="{l.push_binary(r,"%");}
  else if op == "<<="{l.push_binary(r,"<<");}
  else if op == ">>="{l.push_binary(r,">>");}
  else if op ==  "&="{l.push_binary(r,"&");}
  else if op ==  "|="{l.push_binary(r,"|");}
  else if op ==  "^="{l.push_binary(r,"^");}
  else if op ==   "="{                     }
  else{panic!()}

  l.push_store();

  self.push_eval_text(l);
}


pub fn
to_bytes(&self)-> Vec<u8>
{
  let  mut bytes = Vec::<u8>::new();

    for (ln,_) in &self.lines
    {
      ln.write_to(&mut bytes);
    }


  bytes
}


fn
get_label_index(&self, s: &str)-> usize
{
    for i in 0..self.lines.len()
    {
        if let AsmLine::Label(ln_s) = &self.lines[i].0
        {
            if ln_s == s
            {
              return i;
            }
        }
    }


  panic!();
}


fn
prepare_for_finalize(&mut self)-> Vec<usize>
{
  let  mut ls = Vec::<usize>::new();

  let  mut off = 0usize;

    for i in 0..self.lines.len()
    {
      let  mut i = 0usize;

        match &self.lines[i].0
        {
      AsmLine::Jmp(_,dst) =>{i = self.get_label_index(&dst.label);}
      AsmLine::Brnz(_,dst)=>{i = self.get_label_index(&dst.label);}
      AsmLine::Brz(_,dst) =>{i = self.get_label_index(&dst.label);}
      _=>{}
        }


        match &mut self.lines[i].0
        {
      AsmLine::Jmp(jmp_off,dst)=>
        {
          *jmp_off = i32::MAX;
          dst.index = i;
          ls.push(i);
        }
      AsmLine::Brnz(jmp_off,dst)=>
        {
          *jmp_off = i32::MAX;
          dst.index = i;
          ls.push(i);
        }
      AsmLine::Brz(jmp_off,dst)=>
        {
          *jmp_off = i32::MAX;
          dst.index = i;
          ls.push(i);
        }
      _=>{}
        }


      let  sz = self.lines[i].0.get_size();

      self.lines[i].1.offset = off;
      self.lines[i].1.size   =  sz;

      off += sz;
    }


  ls
}


fn
update_info(&mut self)-> bool
{
  let  mut off = 0usize;
  let  mut flag = 0usize;

    for (ln,li) in &mut self.lines
    {
      let  sz = ln.get_size();

        if li.offset != off
        {
          li.offset = off;

          flag |= 1;
        }


      li.size = sz;

      off += sz;
    }


  flag != 0
}


fn
calculate_offset(&self, base_i: usize, dst_i: usize)-> i32
{
  let   dst = &self.lines[ dst_i].1;
  let  base = &self.lines[base_i].1;

  let  dist = (dst.offset as isize)-((base.offset+base.size) as isize);

  dist as i32
}


fn
update_jump_offset(&mut self, ls: &Vec<usize>)
{
    for i in ls
    {
      let  mut off = 0i32;

        match &self.lines[*i].0
        {
      AsmLine::Jmp(_,dst)=> {off = self.calculate_offset(*i,dst.index);}
      AsmLine::Brnz(_,dst)=>{off = self.calculate_offset(*i,dst.index);}
      AsmLine::Brz(_,dst)=> {off = self.calculate_offset(*i,dst.index);}
      _=>{panic!();}
        }


        match &mut self.lines[*i].0
        {
      AsmLine::Jmp(jmp_off,_)=> {*jmp_off = off;}
      AsmLine::Brnz(jmp_off,_)=>{*jmp_off = off;}
      AsmLine::Brz(jmp_off,_)=> {*jmp_off = off;}
      _=>{panic!();}
        }
    }
}


pub fn
finalize(&mut self)
{
  let  ls = self.prepare_for_finalize();

  self.update_jump_offset(&ls);

    while self.update_info()
    {
      self.update_jump_offset(&ls);
    }
}


pub fn
print(&self)
{
  let  mut n = 0usize;

    for (ln,_) in &self.lines
    {
        if let AsmLine::Label(s) = ln
        {
          print!("[{}] ",s);
        }

      else
        {
          ln.print();

          n += 1;
        }


      println!("");
    }
}


}




