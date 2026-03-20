

use super::*;
use super::asm::*;
use super::symbol_table::*;


const  HALT_FLAG: usize = 1;


pub struct
Machine
{
  memory: Vec<u8>,

  pc: usize,
  fp: usize,
  sp: usize,
  cp: usize,

  status: usize,

  call_depth: usize,

}


impl
Machine
{


pub fn
new()-> Self
{
  Self{
    memory: Vec::new(),

    pc: 0,
    fp: 0,
    sp: 0,
    cp: 0,

    status: 0,

    call_depth: 0,
  }
}


pub fn
resize_memory(&mut self, new_size: usize)
{
  self.memory.resize(new_size/WORD_SIZE*WORD_SIZE,0);
}


pub fn
reset(&mut self, exeimg: &ExecImage)
{
  const  CALL_STACK_SIZE: usize = 256;
  const LOCAL_STACK_SIZE: usize = 256;

  let  bytes = exeimg.get_bytes();

  self.memory.resize(bytes.len()+LOCAL_STACK_SIZE+CALL_STACK_SIZE,0);

    for i in 0..bytes.len()
    {
//println!("{} {} <= {}",i,self.memory[i],bytes[i]);
      self.memory[i] = bytes[i];
    }


  let  base = get_word_aligned(bytes.len());

  self.pc = exeimg.get_entry_point();
  self.fp = base;
  self.sp = base;
  self.cp = base+LOCAL_STACK_SIZE;
  self.status = 0;
  self.call_depth = 0;
}


pub fn
push(&mut self, v: u64)
{
    if (self.sp+WORD_SIZE) >= self.memory.len()
    {
      self.memory.resize(self.sp+(WORD_SIZE*256),0);
    }


  self.put_u64(self.sp,v);

  self.sp += WORD_SIZE;
}


pub fn
push_b(&mut self, v: bool)
{
  self.push(if v{1} else{0});
}


pub fn
pop(&mut self)-> u64
{
  self.sp -= WORD_SIZE;

  self.get_u64(self.sp)
}


fn
pop2(&mut self)-> (u64,u64)
{
  let  r = self.pop();
  let  l = self.pop();

  (l,r)
}


fn
pop2_i(&mut self)-> (i64,i64)
{
  let  (l,r) = self.pop2();

  (l as i64,r as i64)
}


fn
pop2_f(&mut self)-> (f64,f64)
{
  let  (l,r) = self.pop2();

  (f64::from_bits(l),f64::from_bits(r))
}


pub fn
get_last(&self)-> u64
{
  unsafe{*(self.get_ptr(self.sp-WORD_SIZE) as *const u64)}
}


pub fn
ref_last_mut(&mut self)-> &mut u64
{
  unsafe{&mut *(self.get_mut_ptr(self.sp-WORD_SIZE) as *mut u64)}
}


pub fn
get_ptr(&self, off: usize)-> *const u8
{
  unsafe{self.memory.as_ptr().add(off) as *const u8}
}


fn  get_u8( &self, off: usize)-> u8 {unsafe{*(self.get_ptr(off)              )}}
fn  get_u16(&self, off: usize)-> u16{unsafe{*(self.get_ptr(off) as *const u16)}}
fn  get_u32(&self, off: usize)-> u32{unsafe{*(self.get_ptr(off) as *const u32)}}
fn  get_u64(&self, off: usize)-> u64{unsafe{*(self.get_ptr(off) as *const u64)}}


pub fn
get_mut_ptr(&mut self, off: usize)-> *mut u8
{
  unsafe{self.memory.as_mut_ptr().add(off) as *mut u8}
}


fn  put_u8( &mut self, off: usize, v: u8 ){unsafe{*(self.get_mut_ptr(off)            ) = v};}
fn  put_u16(&mut self, off: usize, v: u16){unsafe{*(self.get_mut_ptr(off) as *mut u16) = v};}
fn  put_u32(&mut self, off: usize, v: u32){unsafe{*(self.get_mut_ptr(off) as *mut u32) = v};}
fn  put_u64(&mut self, off: usize, v: u64){unsafe{*(self.get_mut_ptr(off) as *mut u64) = v};}


pub fn
jump(&mut self, offset: isize)
{
  self.pc = ((self.pc as isize)+offset) as usize;
}


pub fn
set_pc(&mut self, pc: usize)
{
  self.pc = pc;
}


pub fn
get_pc(&mut self)-> usize
{
  let  old_pc = self.pc     ;
                self.pc += 1;

  old_pc
}


pub fn
halt(&mut self)
{
  self.status |= HALT_FLAG;
}


pub fn
unhalt(&mut self)
{
  self.status &= !HALT_FLAG;
}


pub fn
is_halted(&self)-> bool
{
  (self.status&HALT_FLAG) != 0
}


pub fn
get_next_byte(&mut self)-> u8
{
  let  pc = self.get_pc();

  *unsafe{self.memory.get_unchecked(pc)}
}


pub fn
get_imm_u8(&mut self)-> u8
{
  self.get_next_byte()
}


pub fn
get_imm_u16(&mut self)-> u16
{
   ((self.get_imm_u8() as u16)<<8)
  |((self.get_imm_u8() as u16)   )
}


pub fn
get_imm_u32(&mut self)-> u32
{
   ((self.get_imm_u16() as u32)<<16)
  |((self.get_imm_u16() as u32)    )
}


pub fn
get_imm_u64(&mut self)-> u64
{
   ((self.get_imm_u32() as u64)<<32)
  |((self.get_imm_u32() as u64)    )
}


pub fn
get_imm(&mut self)-> u64
{
    match self.get_next_byte()
    {
  (k) if k == ImmKind::U8 as u8=>
    {
      self.get_imm_u8() as u64
    }
  (k) if k == ImmKind::U16 as u8=>
    {
      self.get_imm_u16() as u64
    }
  (k) if k == ImmKind::U32 as u8=>
    {
      self.get_imm_u32() as u64
    }
  (k) if k == ImmKind::I8 as u8=>
    {
      self.get_imm_u8() as i8 as u64
    }
  (k) if k == ImmKind::I16 as u8=>
    {
      self.get_imm_u16() as i16 as u64
    }
  (k) if k == ImmKind::I32 as u8=>
    {
      self.get_imm_u32() as i32 as u64
    }
  (k) if k == ImmKind::F32 as u8=>
    {
      (f32::from_bits(self.get_imm_u32()) as f64).to_bits()
    }
  (k) if k == ImmKind::I64 as u8=>
    {
      self.get_imm_u64()
    }
  (k) if k == ImmKind::U64 as u8=>
    {
      self.get_imm_u64()
    }
  (k) if k == ImmKind::F64 as u8=>
    {
      self.get_imm_u64()
    }
  _=>{panic!();}
    }
}


pub fn
step(&mut self)
{
  println!("PC: {}, FP: {}, SP: {}, CP: {}",self.pc,self.fp,self.sp,self.cp);

  let  b = self.get_next_byte();

//  println!("OP: {}",b);

    match b
    {
  (op) if op == Opcode::Nop as u8=>{}

  (op) if op == Opcode::Push0 as u8=>{self.push(0);}
  (op) if op == Opcode::Push1 as u8=>{self.push(1);}
  (op) if op == Opcode::Push2 as u8=>{self.push(2);}
  (op) if op == Opcode::Push3 as u8=>{self.push(3);}
  (op) if op == Opcode::Push4 as u8=>{self.push(4);}
  (op) if op == Opcode::Push5 as u8=>{self.push(5);}
  (op) if op == Opcode::Push6 as u8=>{self.push(6);}
  (op) if op == Opcode::Push7 as u8=>{self.push(7);}
  (op) if op == Opcode::Push8 as u8=>{self.push(8);}
  (op) if op == Opcode::Pop as u8=>{let  _ = self.pop();}
  (op) if op == Opcode::Dup as u8=>
    {
      let  v = self.get_last();

      self.push(v);
    }
  (op) if op == Opcode::Xs as u8=>
    {
      let  v = self.pop() as usize;

      self.sp += (WORD_SIZE*v);
    }
  (op) if op == Opcode::Lpc as u8=>
    {
      self.push(self.pc as u64);
    }
  (op) if op == Opcode::Lfp as u8=>
    {
      self.push(self.fp as u64);
    }
  (op) if op == Opcode::Lsp as u8=>
    {
      self.push(self.sp as u64);
    }
  (op) if op == Opcode::Li as u8=>
    {
      let  v = self.get_imm();

      self.push(v);
    }
  (op) if op == Opcode::Ld8 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u8(addr) as u64;

      self.push(v);
    }
  (op) if op == Opcode::Ld16 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u16(addr) as u64;

      self.push(v);
    }
  (op) if op == Opcode::Ld32 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u32(addr) as u64;

      self.push(v);
    }
  (op) if op == Opcode::Ld64 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u64(addr);

      self.push(v);
    }
  (op) if op == Opcode::St8 as u8=>
    {
      let     v = self.pop() as u8;
      let  addr = self.pop() as usize;

      self.put_u8(addr,v);
    }
  (op) if op == Opcode::St16 as u8=>
    {
      let     v = self.pop() as u16;
      let  addr = self.pop() as usize;

      self.put_u16(addr,v);
    }
  (op) if op == Opcode::St32 as u8=>
    {
      let     v = self.pop() as u32;
      let  addr = self.pop() as usize;

      self.put_u32(addr,v);
    }
  (op) if op == Opcode::St64 as u8=>
    {
      let     v = self.pop() as u64;
      let  addr = self.pop() as usize;

      self.put_u64(addr,v);
    }
  (op) if op == Opcode::Sx8 as u8=>
    {
      let  v = self.get_last() as u8 as i8 as i64 as u64;

      *self.ref_last_mut() = v;
    }
  (op) if op == Opcode::Sx16 as u8=>
    {
      let  v = self.get_last() as u16 as i16 as i64 as u64;

      *self.ref_last_mut() = v;
    }
  (op) if op == Opcode::Sx32 as u8=>
    {
      let  v = self.get_last() as u32 as i32 as i64 as u64;

      *self.ref_last_mut() = v;
    }
  (op) if op == Opcode::Tr8 as u8=>
    {
      let  v = self.get_last() as u8 as u64;

      *self.ref_last_mut() = v;
    }
  (op) if op == Opcode::Tr16 as u8=>
    {
      let  v = self.get_last() as u16 as u64;

      *self.ref_last_mut() = v;
    }
  (op) if op == Opcode::Tr32 as u8=>
    {
      let  v = self.get_last() as u32 as u64;

      *self.ref_last_mut() = v;
    }
  (op) if op == Opcode::B32toF as u8=>
    {
      let  v = f32::from_bits(self.get_last() as u32) as f64;

      *self.ref_last_mut() = v.to_bits();
    }
  (op) if op == Opcode::FtoB32 as u8=>
    {
      let  v = f64::from_bits(self.get_last()) as f32;

      *self.ref_last_mut() = v.to_bits() as u64;
    }
  (op) if op == Opcode::Neg  as u8=>{let  v = self.ref_last_mut();  *v = (-((*v) as i64)) as u64;}
  (op) if op == Opcode::Negf as u8=>{let  v = self.ref_last_mut();  *v = (-f64::from_bits(*v)).to_bits();}
  (op) if op == Opcode::Not  as u8=>{let  v = self.ref_last_mut();  *v = !*v;}
  (op) if op == Opcode::Notl as u8=>{let  v = self.ref_last_mut();  *v = if *v != 0{0} else{1};}

  (op) if op == Opcode::Itof as u8=>{let  v = self.ref_last_mut();  *v = ((*v) as i64 as f64).to_bits();}
  (op) if op == Opcode::Ftoi as u8=>{let  v = self.ref_last_mut();  *v = (f64::from_bits(*v) as f64).to_bits();}

  (op) if op == Opcode::Addi as u8=>{let  (l,r) = self.pop2_i();  self.push((l+r) as u64);}
  (op) if op == Opcode::Subi as u8=>{let  (l,r) = self.pop2_i();  self.push((l-r) as u64);}
  (op) if op == Opcode::Muli as u8=>{let  (l,r) = self.pop2_i();  self.push((l*r) as u64);}
  (op) if op == Opcode::Divi as u8=>{let  (l,r) = self.pop2_i();  self.push((l/r) as u64);}
  (op) if op == Opcode::Remi as u8=>{let  (l,r) = self.pop2_i();  self.push((l%r) as u64);}
  (op) if op == Opcode::Addu as u8=>{let  (l,r) = self.pop2();  self.push(l+r);}
  (op) if op == Opcode::Subu as u8=>{let  (l,r) = self.pop2();  self.push(l-r);}
  (op) if op == Opcode::Mulu as u8=>{let  (l,r) = self.pop2();  self.push(l*r);}
  (op) if op == Opcode::Divu as u8=>{let  (l,r) = self.pop2();  self.push(l/r);}
  (op) if op == Opcode::Remu as u8=>{let  (l,r) = self.pop2();  self.push(l%r);}
  (op) if op == Opcode::Addf as u8=>{let  (l,r) = self.pop2_f();  self.push((l+r).to_bits());}
  (op) if op == Opcode::Subf as u8=>{let  (l,r) = self.pop2_f();  self.push((l-r).to_bits());}
  (op) if op == Opcode::Mulf as u8=>{let  (l,r) = self.pop2_f();  self.push((l*r).to_bits());}
  (op) if op == Opcode::Divf as u8=>{let  (l,r) = self.pop2_f();  self.push((l/r).to_bits());}
  (op) if op == Opcode::Remf as u8=>{let  (l,r) = self.pop2_f();  self.push((l%r).to_bits());}

  (op) if op == Opcode::Shl as u8=>{let  (l,r) = self.pop2();  self.push(l<<r);}
  (op) if op == Opcode::Shr as u8=>{let  (l,r) = self.pop2();  self.push(l>>r);}
  (op) if op == Opcode::And as u8=>{let  (l,r) = self.pop2();  self.push(l&r);}
  (op) if op == Opcode::Or  as u8=>{let  (l,r) = self.pop2();  self.push(l|r);}
  (op) if op == Opcode::Xor as u8=>{let  (l,r) = self.pop2();  self.push(l^r);}

  (op) if op == Opcode::Eq  as u8=>{let  (l,r) = self.pop2();  self.push_b(l == r);}
  (op) if op == Opcode::Neq as u8=>{let  (l,r) = self.pop2();  self.push_b(l != r);}
  (op) if op == Opcode::Eqf  as u8=>{let  (l,r) = self.pop2_f();  self.push_b(l == r);}
  (op) if op == Opcode::Neqf as u8=>{let  (l,r) = self.pop2_f();  self.push_b(l != r);}

  (op) if op == Opcode::Lti   as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l <  r);}
  (op) if op == Opcode::Lteqi as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l <= r);}
  (op) if op == Opcode::Gti   as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l >  r);}
  (op) if op == Opcode::Gteqi as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l >= r);}
  (op) if op == Opcode::Ltu   as u8=>{let  (l,r) = self.pop2();  self.push_b(l <  r);}
  (op) if op == Opcode::Ltequ as u8=>{let  (l,r) = self.pop2();  self.push_b(l <= r);}
  (op) if op == Opcode::Gtu   as u8=>{let  (l,r) = self.pop2();  self.push_b(l >  r);}
  (op) if op == Opcode::Gtequ as u8=>{let  (l,r) = self.pop2();  self.push_b(l >= r);}
  (op) if op == Opcode::Ltf   as u8=>{let  (l,r) = self.pop2_f();  self.push_b(l <  r);}
  (op) if op == Opcode::Lteqf as u8=>{let  (l,r) = self.pop2_f();  self.push_b(l <= r);}
  (op) if op == Opcode::Gtf   as u8=>{let  (l,r) = self.pop2_f();  self.push_b(l >  r);}
  (op) if op == Opcode::Gteqf as u8=>{let  (l,r) = self.pop2_f();  self.push_b(l >= r);}

  (op) if op == Opcode::Andl as u8=>{let  (l,r) = self.pop2();  self.push_b((l != 0) && (r != 0));}
  (op) if op == Opcode::Orl  as u8=>{let  (l,r) = self.pop2();  self.push_b((l != 0) || (r != 0));}

  (op) if op == Opcode::Jmp as u8=>
    {
      let  offset = self.get_imm() as isize;

      self.jump(offset);
    }
  (op) if op == Opcode::Brz as u8=>
    {
      let  offset = self.get_imm() as isize;

      let  cond = self.pop();

        if cond == 0
        {
          self.jump(offset);
        }
    }
  (op) if op == Opcode::Brnz as u8=>
    {
      let  offset = self.get_imm() as isize;

      let  cond = self.pop();

        if cond != 0
        {
          self.jump(offset);
        }
    }
  (op) if op == Opcode::Prcal as u8=>
    {
      let  f_addr = self.pop();

      self.put_u64(self.cp+(WORD_SIZE*0),f_addr);
      self.put_u64(self.cp+(WORD_SIZE*1),self.fp as u64);
      self.put_u64(self.cp+(WORD_SIZE*2),self.sp as u64);

      self.cp += WORD_SIZE*3;
    }
  (op) if op == Opcode::Cal as u8=>
    {
      let  pc_addr = self.cp-(WORD_SIZE*3);

      let  old_pc = self.pc                                 ;
                    self.pc = self.get_u64(pc_addr) as usize;

      self.put_u64(pc_addr,old_pc as u64);

      self.fp = self.sp;

      self.call_depth += 1;
    }
  (op) if op == Opcode::Ret as u8=>
    {
      let  v = self.pop();

        if self.call_depth == 0
        {
          self.halt();

          println!("execution is completed: value is {}",v);

          return;
        }


      self.pc = self.get_u64(self.cp-(WORD_SIZE*3)) as usize;
      self.fp = self.get_u64(self.cp-(WORD_SIZE*2)) as usize;
      self.sp = self.get_u64(self.cp-(WORD_SIZE*1)) as usize;

      self.cp -= WORD_SIZE*3;

      self.call_depth -= 1;

      self.push(v);
    }
  (op) if op == Opcode::Hlt as u8=>
    {
      self.halt();
    }
  (op) if op == Opcode::Pri as u8=>
    {
      let  v = self.pop();

      println!("**PRINT** {}",v);
    }
  _=>{panic!();}
    }
}


pub fn
run(&mut self)
{
  use std::{thread,time};

  self.unhalt();

  let  tm = time::Duration::from_millis(80);

    loop
    {
      self.step();

        if self.is_halted()
        {
          break;
        }


      thread::sleep(tm);
    }
}


}




