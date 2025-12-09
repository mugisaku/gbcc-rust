



#[derive(Clone)]
pub enum
Opcode
{
  Nop,

  Pushi(i64),
  Pushu(u64),
  Pushf(f64),

  Pop,
  Dup,

  Ldi8, Ldi16, Ldi32,
  Ldu8, Ldu16, Ldu32,
  Ldf32,
  Ld64,

  Addfp,

  St8, St16, St32, St64,

  Neg, Negf,
  Not, Notl,

  Ref, Deref,

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

  Andl, Orl,

  Label(String),
    Jmp(String),
    Brz(String),
   Brnz(String),

  Prcal,
    Cal,

  Ret,
  Hlt,

}




const WORD_SIZE: usize = 8;

const  HALT_FLAG: usize = 1;
const DIRTY_FLAG: usize = 2;


pub struct
Machine<'a,'b>
{
        codes: &'a Vec<Opcode>,
  global_data: &'b Vec<u8>,

  cal_stack: Vec<u64>,

  memory: Vec<u8>,

  pc: usize,
  fp: usize,
  sp: usize,

  cal_depth: usize,

  status: usize,

}


impl<'a,'b>
Machine<'a,'b>
{


pub fn
new(codes: &'a Vec<Opcode>, global_data: &'b Vec<u8>)-> Self
{
  Self{
    codes,

    global_data,

    cal_stack: Vec::new(),

    memory: Vec::new(),

    pc: 0,
    fp: 0,
    sp: 0,

    cal_depth: 0,

    status: 0,
  }
}


fn
find_label(&self, name: &str)-> Option<usize>
{
    for i in 0..self.codes.len()
    {
        if let Opcode::Label(s) = unsafe{self.codes.get_unchecked(i)}
        {
            if s == name
            {
              return Some(i);
            }
        }
    }


  None
}


pub fn
reset(&mut self)
{
  self.pc = 0;
  self.status = 0;

  let  gd_sz = self.global_data.len();
  let  stack_start = (gd_sz+(WORD_SIZE-1))/WORD_SIZE*WORD_SIZE;

  self.fp = stack_start;
  self.sp = stack_start;

    if self.memory.len() < stack_start
    {
      self.memory.resize(stack_start,0);
    }


    for i in 0..gd_sz
    {
      let  b = unsafe{*self.global_data.get_unchecked(i)};

      unsafe{*self.memory.get_unchecked_mut(i) = b};
    }
}


pub fn
push(&mut self, v: u64)
{
    if self.sp+WORD_SIZE >= self.memory.len()
    {
      self.memory.resize(self.sp+(WORD_SIZE*256),0);
    }


  self.put_u64(self.sp,v);

  self.sp += WORD_SIZE;
}


pub fn
pop(&mut self)-> u64
{
  self.sp -= WORD_SIZE;

  self.get_u64(self.sp)
}


fn
pop2_i(&mut self)-> (i64,i64)
{
  let  l = self.pop() as i64;
  let  r = self.pop() as i64;

  (l,r)
}


fn
pop2(&mut self)-> (u64,u64)
{
  let  l = self.pop();
  let  r = self.pop();

  (l,r)
}


fn
pop2_f(&mut self)-> (f64,f64)
{
  let  l = f64::from_bits(self.pop());
  let  r = f64::from_bits(self.pop());

  (l,r)
}


pub fn
last(&self)-> &u64
{
  unsafe{&*(self.get_ptr(self.sp-WORD_SIZE) as *const u64)}
}


pub fn
last_mut(&mut self)-> &mut u64
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
    if off < self.global_data.len()
    {
    }


  unsafe{self.memory.as_mut_ptr().add(off) as *mut u8}
}


fn  put_u8( &mut self, off: usize, v: u8 ){unsafe{*(self.get_mut_ptr(off)            ) = v};}
fn  put_u16(&mut self, off: usize, v: u16){unsafe{*(self.get_mut_ptr(off) as *mut u16) = v};}
fn  put_u32(&mut self, off: usize, v: u32){unsafe{*(self.get_mut_ptr(off) as *mut u32) = v};}
fn  put_u64(&mut self, off: usize, v: u64){unsafe{*(self.get_mut_ptr(off) as *mut u64) = v};}


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
set_dirty_flag(&mut self)
{
  self.status |= DIRTY_FLAG;
}


pub fn
unset_dirty_flag(&mut self)
{
  self.status &= !DIRTY_FLAG;
}


pub fn
test_dirty_flag(&self)-> bool
{
  (self.status&DIRTY_FLAG) != 0
}


pub fn
step(&mut self)-> Option<u64>
{
    match unsafe{self.codes.get_unchecked(self.get_pc())}
    {
  Opcode::Nop=>{}

  Opcode::Pushi(i)=>{self.push(*i as u64);}
  Opcode::Pushu(u)=>{self.push(*u as u64);}
  Opcode::Pushf(f)=>{self.push(f.to_bits());}


  Opcode::Pop=>{let  _ = self.pop();}
  Opcode::Dup=>
    {
      let  v = *self.last();

      self.push(v);
    }
  Opcode::Ldi8=>
    {
      let  addr = self.pop() as usize;
      let     v = ((self.get_u8(addr) as i8) as i64) as u64;

      self.push(v);
    }
  Opcode::Ldi16=>
    {
      let  addr = self.pop() as usize;
      let     v = ((self.get_u16(addr) as i16) as i64) as u64;

      self.push(v);
    }
  Opcode::Ldi32=>
    {
      let  addr = self.pop() as usize;
      let     v = ((self.get_u32(addr) as i32) as i64) as u64;

      self.push(v);
    }
  Opcode::Ldu8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u8(addr) as u64;

      self.push(v);
    }
  Opcode::Ldu16=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u16(addr) as u64;

      self.push(v);
    }
  Opcode::Ldu32=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u32(addr) as u64;

      self.push(v);
    }
  Opcode::Ldf32=>
    {
      let  addr = self.pop() as usize;
      let     v = (f32::from_bits(self.get_u32(addr)) as f64).to_bits();

      self.push(v);
    }
  Opcode::Ld64=>
    {
      let  addr = self.pop() as usize;
      let     v = self.get_u64(addr);

      self.push(v);
    }
  Opcode::Addfp=>
    {
      let  off = self.pop() as isize;

      self.push(((self.fp as isize)+off) as u64);
    }
  Opcode::St8=>
    {
      let  addr = self.pop() as usize;
      let     v = self.pop() as u8;

      self.put_u8(addr,v);
    }
  Opcode::St16=>
    {
      let  addr = self.pop() as usize;
      let     v = self.pop() as u16;

      self.put_u16(addr,v);
    }
  Opcode::St32=>
    {
      let  addr = self.pop() as usize;
      let     v = self.pop() as u32;

      self.put_u32(addr,v);
    }
  Opcode::St64=>
    {
      let  addr = self.pop() as usize;
      let     v = self.pop() as u64;

      self.put_u64(addr,v);
    }
  Opcode::Neg =>{let  v = self.last_mut();  *v = (-((*v) as i64)) as u64;}
  Opcode::Negf=>{let  v = self.last_mut();  *v = (-f64::from_bits(*v)).to_bits();}
  Opcode::Not =>{let  v = self.last_mut();  *v = !*v;}
  Opcode::Notl=>{let  v = self.last_mut();  *v = if *v != 0{0} else{1};}

  Opcode::Itof=>{let  v = self.last_mut();  *v = ((*v) as i64 as f64).to_bits();}
  Opcode::Ftoi=>{let  v = self.last_mut();  *v = (f64::from_bits(*v) as f64).to_bits();}

  Opcode::Addi=>{let  (l,r) = self.pop2_i();  self.push((l+r) as u64);}
  Opcode::Subi=>{let  (l,r) = self.pop2_i();  self.push((l-r) as u64);}
  Opcode::Muli=>{let  (l,r) = self.pop2_i();  self.push((l*r) as u64);}
  Opcode::Divi=>{let  (l,r) = self.pop2_i();  self.push((l/r) as u64);}
  Opcode::Remi=>{let  (l,r) = self.pop2_i();  self.push((l%r) as u64);}
  Opcode::Addu=>{let  (l,r) = self.pop2();  self.push(l+r);}
  Opcode::Subu=>{let  (l,r) = self.pop2();  self.push(l-r);}
  Opcode::Mulu=>{let  (l,r) = self.pop2();  self.push(l*r);}
  Opcode::Divu=>{let  (l,r) = self.pop2();  self.push(l/r);}
  Opcode::Remu=>{let  (l,r) = self.pop2();  self.push(l%r);}
  Opcode::Addf=>{let  (l,r) = self.pop2_f();  self.push((l+r).to_bits());}
  Opcode::Subf=>{let  (l,r) = self.pop2_f();  self.push((l-r).to_bits());}
  Opcode::Mulf=>{let  (l,r) = self.pop2_f();  self.push((l*r).to_bits());}
  Opcode::Divf=>{let  (l,r) = self.pop2_f();  self.push((l/r).to_bits());}
  Opcode::Remf=>{let  (l,r) = self.pop2_f();  self.push((l%r).to_bits());}

  Opcode::Shl=>{let  (l,r) = self.pop2();  self.push(l<<r);}
  Opcode::Shr=>{let  (l,r) = self.pop2();  self.push(l>>r);}
  Opcode::And=>{let  (l,r) = self.pop2();  self.push(l&r);}
  Opcode::Or =>{let  (l,r) = self.pop2();  self.push(l|r);}
  Opcode::Xor=>{let  (l,r) = self.pop2();  self.push(l^r);}

  Opcode::Eq =>{let  (l,r) = self.pop2();  self.push(if l == r{1} else{0});}
  Opcode::Neq=>{let  (l,r) = self.pop2();  self.push(if l != r{1} else{0});}
  Opcode::Eqf =>{let  (l,r) = self.pop2_f();  self.push(if l == r{1} else{0});}
  Opcode::Neqf=>{let  (l,r) = self.pop2_f();  self.push(if l != r{1} else{0});}

  Opcode::Lti  =>{let  (l,r) = self.pop2();  self.push(if l <  r{1} else{0});}
  Opcode::Lteqi=>{let  (l,r) = self.pop2();  self.push(if l <= r{1} else{0});}
  Opcode::Gti  =>{let  (l,r) = self.pop2();  self.push(if l >  r{1} else{0});}
  Opcode::Gteqi=>{let  (l,r) = self.pop2();  self.push(if l >= r{1} else{0});}
  Opcode::Ltu  =>{let  (l,r) = self.pop2();  self.push(if l <  r{1} else{0});}
  Opcode::Ltequ=>{let  (l,r) = self.pop2();  self.push(if l <= r{1} else{0});}
  Opcode::Gtu  =>{let  (l,r) = self.pop2();  self.push(if l >  r{1} else{0});}
  Opcode::Gtequ=>{let  (l,r) = self.pop2();  self.push(if l >= r{1} else{0});}
  Opcode::Ltf  =>{let  (l,r) = self.pop2();  self.push(if l <  r{1} else{0});}
  Opcode::Lteqf=>{let  (l,r) = self.pop2();  self.push(if l <= r{1} else{0});}
  Opcode::Gtf  =>{let  (l,r) = self.pop2();  self.push(if l >  r{1} else{0});}
  Opcode::Gteqf=>{let  (l,r) = self.pop2();  self.push(if l >= r{1} else{0});}

  Opcode::Andl=>{let  (l,r) = self.pop2();  self.push(if (l != 0) && (r != 0){1} else{0});}
  Opcode::Orl =>{let  (l,r) = self.pop2();  self.push(if (l != 0) || (r != 0){1} else{0});}

  Opcode::Jmp(s)=>
    {
        if let Some(i) = self.find_label(s)
        {
          self.set_pc(i);
        }
    }
  Opcode::Brz(s)=>
    {
        if let Some(i) = self.find_label(s)
        {
            if self.pop() == 0
            {
              self.set_pc(i);
            }
        }
    }
  Opcode::Brnz(s)=>
    {
        if let Some(i) = self.find_label(s)
        {
            if self.pop() != 0
            {
              self.set_pc(i);
            }
        }
    }
  Opcode::Prcal=>
    {
      let  f_addr = self.pop();

      self.cal_stack.push(f_addr);
      self.cal_stack.push(self.fp as u64);
      self.cal_stack.push(self.sp as u64);
    }
  Opcode::Cal=>
    {
      self.cal_stack.push(self.pc as u64);

      self.pc = self.cal_stack[self.cal_stack.len()-3] as usize;

      self.fp = self.sp;

      self.cal_depth += 1;
    }
  Opcode::Ret=>
    {
      let  v = self.pop();

      self.cal_depth -= 1;

        if self.cal_depth == 0
        {
          return Some(v);
        }

      else
        {
          self.pc = self.cal_stack.pop().unwrap() as usize;
          self.sp = self.cal_stack.pop().unwrap() as usize;
          self.fp = self.cal_stack.pop().unwrap() as usize;
          let  _ = self.cal_stack.pop();
        }
    }
  Opcode::Hlt=>
    {
      self.halt();
    }
  _=>{panic!();}
    }


  None
}


pub fn
run(&mut self)-> Option<u64>
{
    if !self.is_halted()
    {
        if let Some(v) = self.step()
        {
          return Some(v);
        }
    }


  None
}


}




