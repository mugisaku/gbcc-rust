

use super::*;
use super::asm::*;
use super::symbol_table::*;


const  HALT_FLAG: usize = 1;


pub struct
Core
{
  id: usize,

  memory_ptr: *mut u8,

  fp_base: usize,
 
  pc: usize,
  fp: usize,
  sp: usize,

  status: usize,

  call_depth: usize,

  error_counter: usize,

}


impl
Core
{


pub const fn
new()-> Self
{
  Self{
    id: 0,
    memory_ptr: std::ptr::null_mut(),
    fp_base: 0,
    pc: 0,
    fp: 0,
    sp: 0,
    status: 0,
    call_depth: 0,
    error_counter: 0,
  }
}


pub fn
initialize(&mut self, id: usize, memory_ptr: *mut u8, fp_base: usize)
{
  self.id         = id;
  self.memory_ptr = memory_ptr;
  self.fp_base    = fp_base;
  self.call_depth = 0;
}


pub fn
reset(&mut self, pc: usize)
{
  self.pc = pc;
  self.fp = self.fp_base;
  self.sp = self.fp_base;
  self.status = 0;
  self.call_depth = 1;
  self.error_counter = 0;
}


pub fn
spawn(&mut self, n: usize)
{
}


pub fn
get_pc(&self)-> usize
{
  self.pc
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
  unsafe{*(self.get_ptr((self.sp-WORD_SIZE)&Exec::MEMORY_MASK8) as *const u64)}
}


pub fn
ref_last_mut(&mut self)-> &mut u64
{
  unsafe{&mut *(self.get_mut_ptr((self.sp-WORD_SIZE)&Exec::MEMORY_MASK8) as *mut u64)}
}


pub fn
get_ptr(&self, off: usize)-> *const u8
{
  unsafe{self.memory_ptr.add(off) as *const u8}
}


fn  get_u8( &self, off: usize)-> u8 {unsafe{*(self.get_ptr(off&Exec::MEMORY_MASK1)              )}}
fn  get_u16(&self, off: usize)-> u16{unsafe{*(self.get_ptr(off&Exec::MEMORY_MASK2) as *const u16)}}
fn  get_u32(&self, off: usize)-> u32{unsafe{*(self.get_ptr(off&Exec::MEMORY_MASK4) as *const u32)}}
fn  get_u64(&self, off: usize)-> u64{unsafe{*(self.get_ptr(off&Exec::MEMORY_MASK8) as *const u64)}}


pub fn
get_mut_ptr(&mut self, off: usize)-> *mut u8
{
  unsafe{self.memory_ptr.add(off) as *mut u8}
}


fn  put_u8( &mut self, off: usize, v: u8 ){unsafe{*(self.get_mut_ptr(off&Exec::MEMORY_MASK1)            ) = v};}
fn  put_u16(&mut self, off: usize, v: u16){unsafe{*(self.get_mut_ptr(off&Exec::MEMORY_MASK2) as *mut u16) = v};}
fn  put_u32(&mut self, off: usize, v: u32){unsafe{*(self.get_mut_ptr(off&Exec::MEMORY_MASK4) as *mut u32) = v};}
fn  put_u64(&mut self, off: usize, v: u64){unsafe{*(self.get_mut_ptr(off&Exec::MEMORY_MASK8) as *mut u64) = v};}


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
pop_pc(&mut self)-> usize
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
  let  pc = self.pop_pc();

  self.get_u8(pc)
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
step(&mut self, verbose: bool)
{
    if verbose
    {
      print!("ID: {}, PC: {}, FP: {}, SP: {}",self.id,self.pc,self.fp,self.sp);
    }


  let  cur_pc = self.pc;

  let  b = self.get_next_byte();

    if verbose
    {
      let  s = match Opcode::try_from(b)
        {
      Ok(op)=>{op.to_str().to_string()}
      Err(())=>{format!("invalid: {}",b)}
        };


      println!(" OP: {}",&s);
    }


    match b
    {
  (op) if op == Opcode::Nop as u8=>{}

  (op) if op == Opcode::Pushid as u8=>
    {
      self.push(self.id as u64);
    }
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
      let  v = self.get_imm_i8() as u64;

      self.push(v);
    }
  (op) if op == Opcode::Push16 as u8=>
    {
      let  v = self.get_imm_i16() as u64;

      self.push(v);
    }
  (op) if op == Opcode::Push32 as u8=>
    {
      let  v = self.get_imm_i32() as u64;

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
  (op) if op == Opcode::Ld_u8 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u8(addr) as u64;

      self.push(v);
    }
  (op) if op == Opcode::Ld_u16 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u16(addr) as u64;

      self.push(v);
    }
  (op) if op == Opcode::Ld_u32 as u8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u32(addr) as u64;

      self.push(v);
    }
  (op) if op == Opcode::St_i8 as u8=>
    {
      let     v = self.pop() as u8;
      let  addr = self.pop() as usize;

      self.put_u8(addr,v);
    }
  (op) if op == Opcode::St_i16 as u8=>
    {
      let     v = self.pop() as u16;
      let  addr = self.pop() as usize;

      self.put_u16(addr,v);
    }
  (op) if op == Opcode::St_i32 as u8=>
    {
      let     v = self.pop() as u32;
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

  (op) if op == Opcode::Lnot as u8=>{let  v = self.pop();  self.push(if v != 0{0} else{1});}
  (op) if op == Opcode::Land as u8=>{let  (l,r) = self.pop2();  self.push(if (l != 0) && (r != 0){1} else{0});}
  (op) if op == Opcode::Lor  as u8=>{let  (l,r) = self.pop2();  self.push(if (l != 0) || (r != 0){1} else{0});}

  (op) if op == Opcode::Eq  as u8=>{let  (l,r) = self.pop2();  self.push_b(l == r);}
  (op) if op == Opcode::Neq as u8=>{let  (l,r) = self.pop2();  self.push_b(l != r);}

  (op) if op == Opcode::Lt   as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l <  r);}
  (op) if op == Opcode::Lteq as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l <= r);}
  (op) if op == Opcode::Gt   as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l >  r);}
  (op) if op == Opcode::Gteq as u8=>{let  (l,r) = self.pop2_i();  self.push_b(l >= r);}
  (op) if op == Opcode::Cal as u8=>
    {
      let  arg_n = self.pop();

      let  new_pc_addr = self.sp-(WORD_SIZE*((arg_n+1) as usize));

      self.push(new_pc_addr as u64);
      self.push(self.pc as u64);
      self.push(self.fp as u64);

      self.fp = self.sp;
      self.pc = self.get_u64(new_pc_addr) as usize;

        if verbose
        {
          println!("arg_n {}, jumped to {}",arg_n,self.pc);
        }


      self.call_depth += 1;
    }
  (op) if op == Opcode::Ret as u8=>
    {
      let  retval = self.pop();

      self.sp = self.fp;

      self.fp = self.pop() as usize;
      self.pc = self.pop() as usize;
      self.sp = self.pop() as usize;

      self.call_depth -= 1;

        if self.call_depth == 0
        {
          self.halt();

          println!("execution is completed: value is {}",retval);

          return;
        }


      self.push(retval);
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
  _=>
    {
      self.error_counter += 1;

        if self.error_counter >= 8
        {
          self.halt();

          println!("stopped because it appears many errors");

          return;
        }
    }
    }
}


pub fn
run(&mut self, mut n: usize, verbose: bool)
{
  self.unhalt();

    while n != 0
    {
      self.step(verbose);

        if self.is_halted()
        {
          return;
        }


      n -= 1;
    }
}


}




pub const  CORE_NUMBER: usize =       4;
pub const   STACK_SIZE: usize = 0x10000;


pub struct
Machine
{
  memory_ptr: *mut u8,

  frequency: usize,

  cores: [Core; CORE_NUMBER],

  verbose: bool,

}


impl
Machine
{


pub const fn
new()-> Self
{
  Self{
    memory_ptr: std::ptr::null_mut(),

    frequency: 0,

    cores: [Core::new(),
            Core::new(),
            Core::new(),
            Core::new(),
           ],

    verbose: false,

  }
}


pub fn
set_verbose(&mut self)
{
  self.verbose = true;
}


pub fn
reset(&mut self, freq: usize, exec: &mut Exec, entry_fn_name: &str)
{
  self.memory_ptr = exec.get_mut_ptr(0);

  self.frequency = freq;

  let  mut stack_start = exec.find_const("STACK_START").unwrap() as usize;

  let  pc = exec.find_entry_point(entry_fn_name).unwrap();

    for i in 0..CORE_NUMBER
    {
      self.cores[i].initialize(i,self.memory_ptr,stack_start);

        if self.verbose
        {
          println!("core {} initialized. stack_start: {}",i,stack_start);
        }


      stack_start += STACK_SIZE;
    }


  self.cores[0].reset(pc);
}


pub fn
run(&mut self)-> usize
{
    if self.frequency == 0
    {
      println!("machine is set zero frequency");

      return 0;
    }


  let  mut living_n = 0;

    for core in &mut self.cores
    {
        if core.call_depth != 0
        {
          living_n += 1;

          core.run(self.frequency,self.verbose);
        }
    }


  living_n
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

    loop
    {
      let  now = Instant::now();

        if self.run() == 0
        {
          break;
        }


      let  tm = Duration::from_secs(1)-now.elapsed();

      sleep(tm);
    }
}


}




