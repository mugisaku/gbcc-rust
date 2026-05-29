

use super::*;
use super::asm::*;
use super::symbol_table::*;


const  HALT_FLAG: usize = 1;


pub struct
Machine
{
  memory_ptr: *mut u8,
  memory_size: usize,

  frequency: usize,

  pc: usize,
  fp: usize,
  sp: usize,
  cp: usize,

  status: usize,

  call_depth: usize,

  time_counter: usize,

}


impl
Machine
{


pub const fn
new()-> Self
{
  Self{
    memory_ptr: std::ptr::null_mut(),
    memory_size: 0,

    frequency: 0,

    pc: 0,
    fp: 0,
    sp: 0,
    cp: 0,

    status: 0,

    call_depth: 0,

    time_counter: 0
  }
}


pub fn
get_byte(&self, offset: usize)-> u8
{
  unsafe{*self.memory_ptr.add(offset%self.memory_size)}
}


pub fn
get_byte_unchecked(&self, offset: usize)-> u8
{
  unsafe{*self.memory_ptr.add(offset)}
}


pub fn
put_byte(&self, offset: usize, byte: u8)
{
  unsafe{*self.memory_ptr.add(offset%self.memory_size) = byte;}
}


pub fn
put_byte_unchecked(&self, offset: usize, byte: u8)
{
  unsafe{*self.memory_ptr.add(offset) = byte;}
}


pub fn
connect_memory(&mut self, ptr: *mut u8, sz: usize)
{
  self.memory_ptr = ptr;
  self.memory_size = sz;
}


pub fn
reset(&mut self, freq: usize, exec: &Exec, entry_fn_name: &str)
{
  self.frequency = freq;

  self.pc = exec.find_entry_point(entry_fn_name).unwrap();
  self.fp = exec.get_stack_start();
  self.sp = exec.get_stack_start();
  self.cp = exec.get_callstack_start();
  self.status = 0;
  self.call_depth = 0;
}


pub fn
push(&mut self, v: u64)
{
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
  unsafe{*(self.memory_ptr.add(self.sp-WORD_SIZE) as *const u64)}
}


pub fn
ref_last_mut(&mut self)-> &mut u64
{
  unsafe{&mut *(self.memory_ptr.add(self.sp-WORD_SIZE) as *mut u64)}
}


pub fn
get_ptr(&self, off: usize)-> *const u8
{
  unsafe{self.memory_ptr.add(off) as *const u8}
}


fn  get_u8( &self, off: usize)-> u8 {unsafe{*(self.get_ptr(off)              )}}
fn  get_u16(&self, off: usize)-> u16{unsafe{*(self.get_ptr(off) as *const u16)}}
fn  get_u32(&self, off: usize)-> u32{unsafe{*(self.get_ptr(off) as *const u32)}}
fn  get_u64(&self, off: usize)-> u64{unsafe{*(self.get_ptr(off) as *const u64)}}


pub fn
get_mut_ptr(&mut self, off: usize)-> *mut u8
{
  unsafe{self.memory_ptr.add(off) as *mut u8}
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

  unsafe{*self.memory_ptr.add(pc)}
}


pub fn
get_imm_i8(&mut self)-> i8
{
  self.get_imm_u8() as i8
}


pub fn
get_imm_i16(&mut self)-> i16
{
  self.get_imm_u16() as i16
}


pub fn
get_imm_i32(&mut self)-> i32
{
  self.get_imm_u32() as i32
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
get_imm_f32(&mut self)-> f32
{
  f32::from_bits(self.get_imm_u32())
}


pub fn
extend_stack(&mut self, n: usize)
{
  self.sp += (WORD_SIZE*n);
}


pub fn
branch_if_zero(&mut self, offset: isize)
{
  let  cond = self.pop();

    if cond == 0
    {
      self.jump(offset);
    }
}


pub fn
branch_if_non_zero(&mut self, offset: isize)
{
  let  cond = self.pop();

    if cond != 0
    {
      self.jump(offset);
    }
}


pub fn
step(&mut self)
{
//  println!("PC: {}, FP: {}, SP: {}, CP: {}",self.pc,self.fp,self.sp,self.cp);

  let  b = self.get_next_byte();

//  println!("OP: {}",b);

    match b
    {
  (op) if op == Opcode::Nop as u8=>{}

  (op) if op == Opcode::Pushpc as u8=>
    {
      self.push(self.pc as u64);
    }
  (op) if op == Opcode::Pushfp as u8=>
    {
      self.push(self.fp as u64);
    }
  (op) if op == Opcode::Pushsp as u8=>
    {
      self.push(self.sp as u64);
    }
  (op) if op == Opcode::Push8 as u8=>
    {
      let  v = self.get_imm_i8() as i64 as u64;

      self.push(v);
    }
  (op) if op == Opcode::Push16 as u8=>
    {
      let  v = self.get_imm_i16() as i64 as u64;

      self.push(v);
    }
  (op) if op == Opcode::Push32 as u8=>
    {
      let  v = self.get_imm_i32() as i64 as u64;

      self.push(v);
    }
  (op) if op == Opcode::Push64 as u8=>
    {
      let  v = self.get_imm_u64();

      self.push(v);
    }
  (op) if op == Opcode::Xs8 as u8=>
    {
      let  v = self.get_imm_u8() as usize;

      self.extend_stack(v);
    }
  (op) if op == Opcode::Xs16 as u8=>
    {
      let  v = self.get_imm_u16() as usize;

      self.extend_stack(v);
    }
  (op) if op == Opcode::Xs32 as u8=>
    {
      let  v = self.get_imm_u32() as usize;

      self.extend_stack(v);
    }
  (op) if op == Opcode::Jmp8 as u8=>
    {
      let  offset = self.get_imm_i8() as isize;

      self.jump(offset);
    }
  (op) if op == Opcode::Jmp16 as u8=>
    {
      let  offset = self.get_imm_i16() as isize;

      self.jump(offset);
    }
  (op) if op == Opcode::Jmp32 as u8=>
    {
      let  offset = self.get_imm_i32() as isize;

      self.jump(offset);
    }
  (op) if op == Opcode::Brz8 as u8=>
    {
      let  offset = self.get_imm_i8() as isize;

      self.branch_if_zero(offset);
    }
  (op) if op == Opcode::Brz16 as u8=>
    {
      let  offset = self.get_imm_i16() as isize;

      self.branch_if_zero(offset);
    }
  (op) if op == Opcode::Brz32 as u8=>
    {
      let  offset = self.get_imm_i32() as isize;

      self.branch_if_zero(offset);
    }
  (op) if op == Opcode::Brnz8 as u8=>
    {
      let  offset = self.get_imm_i8() as isize;

      self.branch_if_non_zero(offset);
    }
  (op) if op == Opcode::Brnz16 as u8=>
    {
      let  offset = self.get_imm_i16() as isize;

      self.branch_if_non_zero(offset);
    }
  (op) if op == Opcode::Brnz32 as u8=>
    {
      let  offset = self.get_imm_i32() as isize;

      self.branch_if_non_zero(offset);
    }
  (op) if op == Opcode::Pop as u8=>{let  _ = self.pop();}
  (op) if op == Opcode::Dup as u8=>
    {
      let  v = self.get_last();

      self.push(v);
    }
  (op) if op == Opcode::Ld_i8 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u8(addr) as i8 as u64;

      self.push(v);
    }
  (op) if op == Opcode::Ld_i16 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u16(addr) as i16 as u64;

      self.push(v);
    }
  (op) if op == Opcode::Ld_i32 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u32(addr) as i32 as u64;

      self.push(v);
    }
  (op) if op == Opcode::Ld_i64 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u64(addr);

      self.push(v);
    }
  (op) if op == Opcode::St_i8 as u8=>
    {
      let     v = self.pop() as i8 as u8;
      let  addr = self.pop() as usize;

      self.put_u8(addr,v);
    }
  (op) if op == Opcode::St_i16 as u8=>
    {
      let     v = self.pop() as i16 as u16;
      let  addr = self.pop() as usize;

      self.put_u16(addr,v);
    }
  (op) if op == Opcode::St_i32 as u8=>
    {
      let     v = self.pop() as i32 as u32;
      let  addr = self.pop() as usize;

      self.put_u32(addr,v);
    }
  (op) if op == Opcode::St_i64 as u8=>
    {
      let     v = self.pop() as u64;
      let  addr = self.pop() as usize;

      self.put_u64(addr,v);
    }
  (op) if op == Opcode::Neg as u8=>{let  v = self.ref_last_mut();  *v = (-((*v) as i64)) as u64;}
  (op) if op == Opcode::Not as u8=>{let  v = self.ref_last_mut();  *v = !*v;}

  (op) if op == Opcode::Add as u8=>{let  (l,r) = self.pop2_i();  self.push((l+r) as u64);}
  (op) if op == Opcode::Sub as u8=>{let  (l,r) = self.pop2_i();  self.push((l-r) as u64);}
  (op) if op == Opcode::Mul as u8=>{let  (l,r) = self.pop2_i();  self.push((l*r) as u64);}
  (op) if op == Opcode::Div as u8=>{let  (l,r) = self.pop2_i();  self.push((l/r) as u64);}
  (op) if op == Opcode::Rem as u8=>{let  (l,r) = self.pop2_i();  self.push((l%r) as u64);}

  (op) if op == Opcode::Shl as u8=>{let  (l,r) = self.pop2();  self.push(l<<r);}
  (op) if op == Opcode::Shr as u8=>{let  (l,r) = self.pop2();  self.push(l>>r);}
  (op) if op == Opcode::And as u8=>{let  (l,r) = self.pop2();  self.push(l&r);}
  (op) if op == Opcode::Or  as u8=>{let  (l,r) = self.pop2();  self.push(l|r);}
  (op) if op == Opcode::Xor as u8=>{let  (l,r) = self.pop2();  self.push(l^r);}

  (op) if op == Opcode::Eq  as u8=>{let  (l,r) = self.pop2();  self.push_b(l == r);}
  (op) if op == Opcode::Neq as u8=>{let  (l,r) = self.pop2();  self.push_b(l != r);}

  (op) if op == Opcode::Lt   as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l <  r);}
  (op) if op == Opcode::Lteq as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l <= r);}
  (op) if op == Opcode::Gt   as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l >  r);}
  (op) if op == Opcode::Gteq as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l >= r);}

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
      let  sp_addr = self.cp-(WORD_SIZE*1);

      let  old_pc = self.pc                                 ;
                    self.pc = self.get_u64(pc_addr) as usize;

      self.put_u64(pc_addr,old_pc as u64);

      self.fp = self.get_u64(sp_addr) as usize;

      let  mut arg_addr = self.fp;

//      print!("called with args(");

        while arg_addr < self.sp
        {
//          print!("{},",self.get_u64(arg_addr));

          arg_addr += WORD_SIZE;
        }


//      print!(")\n");

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
    if self.frequency == 0
    {
      println!("machine is set zero frequency");

      return;
    }


  self.unhalt();

    for _ in 0..self.frequency
    {
      self.step();

        if self.is_halted()
        {
          return;
        }
    }
}


pub fn
keep_run(&mut self)
{
    if self.frequency == 0
    {
      println!("machine is set zero frequency");

      return;
    }


  use std::time::{Duration,Instant};
  use std::thread::sleep;

  self.unhalt();

    loop
    {
      let  now = Instant::now();

        for _ in 0..self.frequency
        {
          self.step();

            if self.is_halted()
            {
              return;
            }
        }


      let  tm = Duration::from_secs(1)-now.elapsed();

      sleep(tm);
    }
}


pub fn
print(&self)
{
}


}




