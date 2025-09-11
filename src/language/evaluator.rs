

use super::expression::{
  Expression,
  AssignOperator,
  UnaryOperator,
  BinaryOperator,

};


use super::memory::{
  Memory,

};


const WORD_SIZE: usize = 8;


pub struct
ExpressionEvaluator
{
  pub(crate) memory: Memory,
  pub(crate) code_string: Vec<u8>,
  pub(crate) pc: usize,
  pub(crate) bp: usize,
  pub(crate) sp: usize,

}


impl
ExpressionEvaluator
{


pub fn
new()-> Self
{
  Self{
    memory: Memory::new_with_size(256),
    code_string: Vec::new(),
    pc: 0,
    bp: 0,
    sp: 0,
  }
}




pub fn
reset(&mut self, e: &Expression)
{
  self.pc = 0;
  self.bp = 0;
  self.sp = 0;

/*
  let  (ls,ti) = compile(e,dir);

  self.instruction_list = ls;

  self.final_value_dst = Destination{address_source: FieldIndex::Global(0)};
  self.final_value_type_info = ti;
*/
}


pub fn
read_imm8(&mut self)-> u8
{
    if self.pc < self.code_string.len()
    {
      let  t = self.code_string[self.pc];

      self.pc += 1;

      return t;
    }

  else
    {
      panic!();
    }
}


pub fn
read_imm16(&mut self)-> u16
{
  let  mut t = (self.read_imm8() as u16)<<8;

  t |= self.read_imm8() as u16;

  u16::from_be(t)
}


pub fn
read_imm32(&mut self)-> u32
{
  let  mut t = (self.read_imm8() as u32)<<24;

  t |= (self.read_imm8() as u32)<<16;
  t |= (self.read_imm8() as u32)<< 8;
  t |= (self.read_imm8() as u32)    ;

  u32::from_be(t)
}


pub fn
read_imm64(&mut self)-> u64
{
  let  mut t = (self.read_imm8() as u64)<<56;

  t |= (self.read_imm8() as u64)<<48;
  t |= (self.read_imm8() as u64)<<40;
  t |= (self.read_imm8() as u64)<<32;
  t |= (self.read_imm8() as u64)<<24;
  t |= (self.read_imm8() as u64)<<16;
  t |= (self.read_imm8() as u64)<< 8;
  t |= (self.read_imm8() as u64)    ;

  u64::from_be(t)
}


pub fn
push<T: std::clone::Clone>(&mut self, v: T)
{
  self.sp += WORD_SIZE;

  self.memory.put::<T>(self.sp,v);
}


pub fn
pop<T: std::clone::Clone>(&mut self)-> T
{
  let  v = self.memory.get::<T>(self.sp);

  self.sp -= WORD_SIZE;

  v
}


pub fn
top<T: std::clone::Clone>(&self)-> T
{
  self.memory.get::<T>(self.sp)
}


pub fn
ldi8(&mut self)
{
  let  addr = self.pop::<usize>();

  let  v = self.memory.get::<i8>(addr);

  self.push(v as i64);
}


pub fn
ldi16(&mut self)
{
  let  addr = self.pop::<usize>();

  let  v = self.memory.get::<i16>(addr);

  self.push(v as i64);
}


pub fn
ldi32(&mut self)
{
  let  addr = self.pop::<usize>();

  let  v = self.memory.get::<i32>(addr);

  self.push(v as i64);
}


pub fn
ldu8(&mut self)
{
  let  addr = self.pop::<usize>();

  let  v = self.memory.get::<u8>(addr);

  self.push(v as u64);
}


pub fn
ldu16(&mut self)
{
  let  addr = self.pop::<usize>();

  let  v = self.memory.get::<u16>(addr);

  self.push(v as u64);
}


pub fn
ldu32(&mut self)
{
  let  addr = self.pop::<usize>();

  let  v = self.memory.get::<u32>(addr);

  self.push(v as u64);
}


pub fn
ld64(&mut self)
{
  let  addr = self.pop::<usize>();

  let  v = self.memory.get::<u64>(addr);

  self.push(v);
}


pub fn
ldf32(&mut self)
{
  let  addr = self.pop::<usize>();

  let  v = self.memory.get::<f32>(addr);

  self.push(v as f64);
}


pub fn
sti8(&mut self)
{
  let     v = self.pop::<i64>();
  let  addr = self.pop::<usize>();

  self.memory.put(addr,v as i8);
}


pub fn
sti16(&mut self)
{
  let     v = self.pop::<i64>();
  let  addr = self.pop::<usize>();

  self.memory.put(addr,v as i16);
}


pub fn
sti32(&mut self)
{
  let     v = self.pop::<i64>();
  let  addr = self.pop::<usize>();

  self.memory.put(addr,v as i32);
}


pub fn
stu8(&mut self)
{
  let     v = self.pop::<u64>();
  let  addr = self.pop::<usize>();

  self.memory.put(addr,v as u8);
}


pub fn
stu16(&mut self)
{
  let     v = self.pop::<u64>();
  let  addr = self.pop::<usize>();

  self.memory.put(addr,v as u16);
}


pub fn
stu32(&mut self)
{
  let     v = self.pop::<u64>();
  let  addr = self.pop::<usize>();

  self.memory.put(addr,v as u32);
}


pub fn
st64(&mut self)
{
  let     v = self.pop::<u64>();
  let  addr = self.pop::<usize>();

  self.memory.put(addr,v);
}


pub fn
stf32(&mut self)
{
  let     v = self.pop::<f64>();
  let  addr = self.pop::<usize>();

  self.memory.put(addr,v as f32);
}


pub fn
brz(&mut self)
{
  let  v = self.pop::<u64>();

    if v == 0
    {
      self.pc = ((self.pc as isize)+self.pop::<isize>()) as usize;
    }
}


pub fn
brnz(&mut self)
{
  let  v = self.pop::<u64>();

    if v != 0
    {
      self.pc = ((self.pc as isize)+self.pop::<isize>()) as usize;
    }
}




pub fn
_size_of_return_value(&self)->usize
{
  0
}


pub fn
cal(&mut self)
{
/*
  let  src_addr = self.get_address(&src.address_source);

  let  old_pc = self.pc;
  let  old_bp = self.bp;

  self.pc = self.memory.get::<u64>(src_addr) as usize;
*/
}


pub fn
get_address_of_return_value(&self)->usize
{
  0
}


pub fn
ret(&mut self)
{
}


pub fn
step(&mut self)-> Option<()>
{
    if self.pc < self.code_string.len()
    {
      let  instr = self.code_string[self.pc];

      self.pc += 1;

        match instr
        {
      opcode::NOP=>{}
      opcode::ADDI=>{  let  ro = self.pop::<i64>();  self.memory.add(self.sp,ro);}
      opcode::SUBI=>{  let  ro = self.pop::<i64>();  self.memory.sub(self.sp,ro);}
      opcode::MULI=>{  let  ro = self.pop::<i64>();  self.memory.mul(self.sp,ro);}
      opcode::DIVI=>{  let  ro = self.pop::<i64>();  self.memory.div(self.sp,ro);}
      opcode::REMI=>{  let  ro = self.pop::<i64>();  self.memory.rem(self.sp,ro);}
      opcode::ADDU=>{  let  ro = self.pop::<u64>();  self.memory.add(self.sp,ro);}
      opcode::SUBU=>{  let  ro = self.pop::<u64>();  self.memory.sub(self.sp,ro);}
      opcode::MULU=>{  let  ro = self.pop::<u64>();  self.memory.mul(self.sp,ro);}
      opcode::DIVU=>{  let  ro = self.pop::<u64>();  self.memory.div(self.sp,ro);}
      opcode::REMU=>{  let  ro = self.pop::<u64>();  self.memory.rem(self.sp,ro);}
      opcode::ADDF=>{  let  ro = self.pop::<f64>();  self.memory.add(self.sp,ro);}
      opcode::SUBF=>{  let  ro = self.pop::<f64>();  self.memory.sub(self.sp,ro);}
      opcode::MULF=>{  let  ro = self.pop::<f64>();  self.memory.mul(self.sp,ro);}
      opcode::DIVF=>{  let  ro = self.pop::<f64>();  self.memory.div(self.sp,ro);}
      opcode::REMF=>{  let  ro = self.pop::<f64>();  self.memory.rem(self.sp,ro);}

      opcode::SHL=>{  let  ro = self.pop::<u64>();  self.memory.shl(self.sp,ro);}
      opcode::SHR=>{  let  ro = self.pop::<u64>();  self.memory.shr(self.sp,ro);}
      opcode::AND=>{  let  ro = self.pop::<u64>();  self.memory.and(self.sp,ro);}
      opcode::OR =>{  let  ro = self.pop::<u64>();  self.memory.or( self.sp,ro);}
      opcode::XOR=>{  let  ro = self.pop::<u64>();  self.memory.xor(self.sp,ro);}

      opcode::EQ =>{  let  ro = self.pop::<u64>();  self.memory.eq( self.sp,ro);}
      opcode::NEQ=>{  let  ro = self.pop::<u64>();  self.memory.neq(self.sp,ro);}

      opcode::EQF =>{  let  ro = self.pop::<f64>();  self.memory.eq( self.sp,ro);}
      opcode::NEQF=>{  let  ro = self.pop::<f64>();  self.memory.neq(self.sp,ro);}

      opcode::LTI  =>{  let  ro = self.pop::<i64>();  self.memory.lt(  self.sp,ro);}
      opcode::LTEQI=>{  let  ro = self.pop::<i64>();  self.memory.lteq(self.sp,ro);}
      opcode::GTI  =>{  let  ro = self.pop::<i64>();  self.memory.gt(  self.sp,ro);}
      opcode::GTEQI=>{  let  ro = self.pop::<i64>();  self.memory.gteq(self.sp,ro);}
      opcode::LTU  =>{  let  ro = self.pop::<u64>();  self.memory.lt(  self.sp,ro);}
      opcode::LTEQU=>{  let  ro = self.pop::<u64>();  self.memory.lteq(self.sp,ro);}
      opcode::GTU  =>{  let  ro = self.pop::<u64>();  self.memory.gt(  self.sp,ro);}
      opcode::GTEQU=>{  let  ro = self.pop::<u64>();  self.memory.gteq(self.sp,ro);}
      opcode::LTF  =>{  let  ro = self.pop::<f64>();  self.memory.lt(  self.sp,ro);}
      opcode::LTEQF=>{  let  ro = self.pop::<f64>();  self.memory.lteq(self.sp,ro);}
      opcode::GTF  =>{  let  ro = self.pop::<f64>();  self.memory.gt(  self.sp,ro);}
      opcode::GTEQF=>{  let  ro = self.pop::<f64>();  self.memory.gteq(self.sp,ro);}

      opcode::LAND=>{  let  ro = self.pop::<bool>();  self.memory.logical_and(self.sp,ro);}
      opcode::LOR =>{  let  ro = self.pop::<bool>();  self.memory.logical_or( self.sp,ro);}

      opcode::NEG =>{self.memory.neg::<i64>(self.sp);}
      opcode::NEGF=>{self.memory.neg::<f64>(self.sp);}
      opcode::NOT =>{self.memory.not::<u64>(self.sp);}
      opcode::LNOT=>{self.memory.logical_not( self.sp);}
      opcode::ITOU=>{self.memory.itou(self.sp);}
      opcode::UTOI=>{self.memory.utoi(self.sp);}
      opcode::ITOF=>{self.memory.itof(self.sp);}
      opcode::FTOI=>{self.memory.ftoi(self.sp);}

      opcode::PUSH0  =>{self.push::<u64>(0);}
      opcode::PUSH1  =>{self.push::<u64>(1);}
      opcode::PUSHI8 =>{  let  imm = self.read_imm8()  as  i8;  self.push(imm as i64);}
      opcode::PUSHI16=>{  let  imm = self.read_imm16() as i16;  self.push(imm as i64);}
      opcode::PUSHI32=>{  let  imm = self.read_imm32() as i32;  self.push(imm as i64);}
      opcode::PUSHU8 =>{  let  imm = self.read_imm8() ;         self.push(imm as u64);}
      opcode::PUSHU16=>{  let  imm = self.read_imm16();         self.push(imm as u64);}
      opcode::PUSHU32=>{  let  imm = self.read_imm32();         self.push(imm as u64);}
      opcode::PUSHF32=>{  let  imm = self.read_imm32();         self.push(f32::from_bits(imm) as f64);}
      opcode::PUSH64 =>{  let  imm = self.read_imm64();         self.push(imm);}
      opcode::DUP    =>{  let  v = self.top::<u64>();  self.push(v);}

      opcode::LDI8 =>{self.ldi8();},
      opcode::LDI16=>{self.ldi16();},
      opcode::LDI32=>{self.ldi32();},
      opcode::LDU8 =>{self.ldu8();},
      opcode::LDU16=>{self.ldu16();},
      opcode::LDU32=>{self.ldu32();},
      opcode::LDF32=>{self.ldf32();},
      opcode::LD64 =>{self.ld64();},
      opcode::LDPC =>{self.push(self.pc);},
      opcode::LDBP =>{self.push(self.bp);},
      opcode::LDSP =>{self.push(self.sp);},

      opcode::STI8 =>{self.sti8();},
      opcode::STI16=>{self.sti16();},
      opcode::STI32=>{self.sti32();},
      opcode::STU8 =>{self.stu8();},
      opcode::STU16=>{self.stu16();},
      opcode::STU32=>{self.stu32();},
      opcode::STF32=>{self.stf32();},
      opcode::ST64 =>{self.st64();},
      opcode::STPC =>{self.pc = self.pop::<usize>();},
      opcode::STBP =>{self.bp = self.pop::<usize>();},
      opcode::STSP =>{self.sp = self.pop::<usize>();},

      opcode::CAL =>{self.cal();}
      opcode::BRZ =>{self.brz();}
      opcode::BRNZ=>{self.brnz();}
      opcode::RET =>{self.ret();}
      _=>{panic!();}
        }


      return Some(());
    }


  None
}


pub fn
run(&mut self)
{
    loop
    {
        if let None = self.step()
        {
          break;
        }
    }
}


pub fn
get_final_value_as_usize(&self)-> usize
{
//  let  addr = self.get_address(&self.final_value_dst.address_source);

  self.memory.get::<u64>(0) as usize
}


pub fn
print_final_value(&self)
{
  print!("return value index is ");

//  self.final_value_dst.address_source.print();

  println!("");

  print!("return value type is ");

  println!("");

//  let  addr = self.get_address(&self.final_value_dst.address_source);

//  print!(" = {}({})",self.memory.get::<u64>(addr),addr);
}


}




