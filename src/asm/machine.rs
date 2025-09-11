

use super::memory::{
  Memory,

};

use super::instruction::{
  Instruction,

};


use super::execution_image::{
  ExecutionImage,

};


use super::instruction;


const WORD_SIZE: usize = 8;

const EXEIMG_START: usize = 512;
const INITIAL_FP_VALUE: usize = 1024*16;
const INITIAL_SP_VALUE: usize = 1024*16;
const INITIAL_CFP_VALUE: usize = 2048*16;

const FREEZE_FLAG: usize = 1;
const  PAUSE_FLAG: usize = 2;
const DIRECT_FLAG: usize = 4;
const   JUMP_FLAG: usize = 8;


pub struct
Machine
{
  pub(crate) memory: Memory,

  pub(crate) pc: usize,

  pub(crate) fp: usize,
  pub(crate) sp: usize,

  pub(crate) cfp: usize,//call frame pointer
  pub(crate) rv: usize,//return value

  pub(crate) rc: usize,//report counter

  pub(crate) flags: usize,

}


impl
Machine
{


pub fn
new()-> Self
{
  Self{
    memory: Memory::new_with_size(65536),

    pc: EXEIMG_START,

    fp: INITIAL_FP_VALUE,
    sp: INITIAL_SP_VALUE,

    cfp: INITIAL_CFP_VALUE,
    rv: 0,

    rc: 0,

    flags: 0,
  }
}




pub fn
reset(&mut self)
{
  self.pc = EXEIMG_START;
  self.fp = INITIAL_FP_VALUE;
  self.sp = INITIAL_SP_VALUE;
  self.cfp = INITIAL_CFP_VALUE;
  self.rv = 0;
  self.rc = 0;
  self.flags = 0;

  self.memory.fill0();
}


pub fn
install(&mut self, ximg: &ExecutionImage)
{
    for i in 0..ximg.binary.len()
    {
      let  b = unsafe{ximg.binary.get_unchecked(i)};

      self.memory.put::<u8>(EXEIMG_START+i,*b);
    }


  self.pc = EXEIMG_START+ximg.entry_point;

    for (name,pos) in &ximg.symbol_list
    {
      println!("{} allocated on {}.",name,EXEIMG_START+*pos);
    }
}


pub fn
read_imm8(&mut self)-> u8
{
    if self.pc < self.memory.get_size()
    {
      let  t = self.memory.get::<u8>(self.pc);

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

  t
}


pub fn
read_imm32(&mut self)-> u32
{
  let  mut t = (self.read_imm16() as u32)<<16;

  t |= (self.read_imm16() as u32);

  t
}


pub fn
read_imm64(&mut self)-> u64
{
  let  mut t = (self.read_imm32() as u64)<<32;

  t |= (self.read_imm32() as u64);

  t
}


pub fn    set_freeze_flag(&mut self){self.flags |=  FREEZE_FLAG;}
pub fn  unset_freeze_flag(&mut self){self.flags &= !FREEZE_FLAG;}
pub fn   test_freeze_flag(&self)-> bool{self.flags&FREEZE_FLAG != 0}

pub fn    set_pause_flag(&mut self){self.flags |=  PAUSE_FLAG;}
pub fn  unset_pause_flag(&mut self){self.flags &= !PAUSE_FLAG;}
pub fn   test_pause_flag(&self)->bool {self.flags&PAUSE_FLAG !=0}

pub fn    set_direct_flag(&mut self){self.flags |=  DIRECT_FLAG;}
pub fn  unset_direct_flag(&mut self){self.flags &= !DIRECT_FLAG;}
pub fn   test_direct_flag(&self)-> bool{self.flags&DIRECT_FLAG != 0}

pub fn    set_jump_flag(&mut self){self.flags |=  JUMP_FLAG;}
pub fn  unset_jump_flag(&mut self){self.flags &= !JUMP_FLAG;}
pub fn   test_jump_flag(&self)-> bool{self.flags&JUMP_FLAG != 0}


pub fn
push(&mut self, v: usize)
{
    if self.test_direct_flag()
    {
      self.unset_direct_flag();

        if self.test_jump_flag()
        {
          self.unset_jump_flag();

          self.pc = ((self.pc as isize)+(v as isize)) as usize;

          println!("  **jump");
        }
    }

  else
    {
      self.memory.put(self.sp,v);

      self.sp += WORD_SIZE;
    }
}


pub fn
pop<T: std::clone::Clone>(&mut self)-> T
{
  self.sp -= WORD_SIZE;

  self.memory.get::<T>(self.sp)
}


pub fn
top<T: std::clone::Clone>(&self)-> T
{
  self.memory.get::<T>(self.sp-WORD_SIZE)
}


pub fn
jmp(&mut self)
{
  self.set_direct_flag();
    self.set_jump_flag();
}


pub fn
brz(&mut self)
{
  let  v = self.pop::<u64>();

  self.set_direct_flag();

    if v == 0
    {
println!("  **branched then value is zero");
      self.set_jump_flag();
    }

  else
    {
println!("  **not branched then value is not zero");
    }
}


pub fn
brnz(&mut self)
{
  let  v = self.pop::<u64>();

  self.set_direct_flag();

    if v != 0
    {
println!("  **branched then value is NOT zero");
      self.set_jump_flag();
    }

  else
    {
println!("  **not branched then value is zero");
    }
}




pub fn
glo(&mut self)
{
  let  i = self.pop::<usize>();

  let  addr = EXEIMG_START+(WORD_SIZE*i);
  let   ptr = EXEIMG_START+self.memory.get::<usize>(addr);

println!("  **glo: {}, addr: {}",i,ptr);
  self.push(ptr);
}


pub fn
arg(&mut self)
{
  let  i = self.pop::<usize>();

  self.push(self.fp-(WORD_SIZE*(1+i)) as usize);
}


pub fn
loc(&mut self)
{
  let  i = 1+self.pop::<usize>();

  self.push(self.fp+(WORD_SIZE*i) as usize);
}


pub fn
spx(&mut self)
{
  let  u = self.pop::<usize>();

  self.sp += WORD_SIZE*u;
}




pub fn
prcal(&mut self)
{
  let  fn_addr = self.pop::<usize>();
println!("  **prcal: fn_addr: {}",fn_addr);
  self.memory.put(self.cfp,fn_addr);
  self.memory.put(self.cfp+(WORD_SIZE*1),self.fp);
  self.memory.put(self.cfp+(WORD_SIZE*2),self.sp);

  self.cfp += (WORD_SIZE*3);
}


pub fn
cal(&mut self)
{
  self.memory.put::<usize>(self.sp,self.pc);

  self.fp = self.sp             ;
            self.sp += WORD_SIZE;

  self.pc = self.memory.get::<usize>(self.cfp-(WORD_SIZE*3));

println!("  **call");
}


pub fn
ret(&mut self)
{
  self.rv = self.pop::<usize>();

  self.cfp -= (WORD_SIZE*3);

  self.pc = self.memory.get::<usize>(self.fp);
  self.fp = self.memory.get::<usize>(self.cfp+(WORD_SIZE*1));
  self.sp = self.memory.get::<usize>(self.cfp+(WORD_SIZE*2));

  self.push(self.rv);

println!("  **return {}",self.rv);
}


pub fn
hlt(&mut self)
{
  self.rv = self.pop::<usize>();

  self.set_freeze_flag();
}


pub fn
pri(&self)
{
  println!("  **RRI {}",self.top::<i64>());
}


pub fn
pru(&self)
{
  println!("  **RRU {}",self.top::<u64>());
}


pub fn
prf(&self)
{
  println!("  **RRF {}",self.top::<f64>());
}


pub fn
repo(&mut self)
{
  println!("  **REPORT {}",self.rc);

  self.rc += 1;
}
	

pub fn
print_program(&self, n: usize)
{
    for i in 0..n
    {
      let  b = self.memory.get::<u8>(self.pc+i);

      println!("PRINT PROGRAM: pc: {}, byte: {}",self.pc+i,b);
    }
}
	

pub fn
inc_sp(&mut self)
{
  self.sp += WORD_SIZE;
}


pub fn
dec_sp(&mut self)
{
  self.sp -= WORD_SIZE;
}


pub fn
step(&mut self)-> Option<()>
{
    if self.test_freeze_flag()
    {
      println!("MACHINE IS HALTED. final value is {}.",self.rv);

      return None;
    }


    if self.test_pause_flag()
    {
      println!("MACHINE IS PAUSING.");

      return None;
    }


    if self.pc < self.memory.get_size()
    {
      let  pc = self.pc     ;
                self.pc += 1;

      let  opcode = self.memory.get::<u8>(pc);

print!("pc: {:05}, fp: {:05}, sp: {:05}, opcode: ",pc,self.fp,self.sp);
Instruction::print_symbol(opcode);
println!("");

        match opcode
        {
      instruction::NOP=>{}
      instruction::ADDI=>{  self.memory.add::<i64>(self.sp);  self.dec_sp();}
      instruction::SUBI=>{  self.memory.sub::<i64>(self.sp);  self.dec_sp();}
      instruction::MULI=>{  self.memory.mul::<i64>(self.sp);  self.dec_sp();}
      instruction::DIVI=>{  self.memory.div::<i64>(self.sp);  self.dec_sp();}
      instruction::REMI=>{  self.memory.rem::<i64>(self.sp);  self.dec_sp();}
      instruction::ADDU=>{  self.memory.add::<u64>(self.sp);  self.dec_sp();}
      instruction::SUBU=>{  self.memory.sub::<u64>(self.sp);  self.dec_sp();}
      instruction::MULU=>{  self.memory.mul::<u64>(self.sp);  self.dec_sp();}
      instruction::DIVU=>{  self.memory.div::<u64>(self.sp);  self.dec_sp();}
      instruction::REMU=>{  self.memory.rem::<u64>(self.sp);  self.dec_sp();}
      instruction::ADDF=>{  self.memory.add::<f64>(self.sp);  self.dec_sp();}
      instruction::SUBF=>{  self.memory.sub::<f64>(self.sp);  self.dec_sp();}
      instruction::MULF=>{  self.memory.mul::<f64>(self.sp);  self.dec_sp();}
      instruction::DIVF=>{  self.memory.div::<f64>(self.sp);  self.dec_sp();}
      instruction::REMF=>{  self.memory.rem::<f64>(self.sp);  self.dec_sp();}

      instruction::SHL=>{  self.memory.shl::<u64>(self.sp);  self.dec_sp();}
      instruction::SHR=>{  self.memory.shr::<u64>(self.sp);  self.dec_sp();}
      instruction::AND=>{  self.memory.and::<u64>(self.sp);  self.dec_sp();}
      instruction::OR =>{  self.memory.or::<u64>( self.sp);  self.dec_sp();}
      instruction::XOR=>{  self.memory.xor::<u64>(self.sp);  self.dec_sp();}

      instruction::EQ =>{  self.memory.eq::<u64>( self.sp);  self.dec_sp();}
      instruction::NEQ=>{  self.memory.neq::<u64>(self.sp);  self.dec_sp();}

      instruction::EQF =>{  self.memory.eq::<f64>( self.sp);  self.dec_sp();}
      instruction::NEQF=>{  self.memory.neq::<f64>(self.sp);  self.dec_sp();}

      instruction::LTI  =>{  self.memory.lt::<i64>(  self.sp);  self.dec_sp();}
      instruction::LTEQI=>{  self.memory.lteq::<i64>(self.sp);  self.dec_sp();}
      instruction::GTI  =>{  self.memory.gt::<i64>(  self.sp);  self.dec_sp();}
      instruction::GTEQI=>{  self.memory.gteq::<i64>(self.sp);  self.dec_sp();}
      instruction::LTU  =>{  self.memory.lt::<u64>(  self.sp);  self.dec_sp();}
      instruction::LTEQU=>{  self.memory.lteq::<u64>(self.sp);  self.dec_sp();}
      instruction::GTU  =>{  self.memory.gt::<u64>(  self.sp);  self.dec_sp();}
      instruction::GTEQU=>{  self.memory.gteq::<u64>(self.sp);  self.dec_sp();}
      instruction::LTF  =>{  self.memory.lt::<f64>(  self.sp);  self.dec_sp();}
      instruction::LTEQF=>{  self.memory.lteq::<f64>(self.sp);  self.dec_sp();}
      instruction::GTF  =>{  self.memory.gt::<f64>(  self.sp);  self.dec_sp();}
      instruction::GTEQF=>{  self.memory.gteq::<f64>(self.sp);  self.dec_sp();}

      instruction::LAND=>{  self.memory.logical_and(self.sp);  self.dec_sp();}
      instruction::LOR =>{  self.memory.logical_or( self.sp);  self.dec_sp();}

      instruction::NEG =>{self.memory.neg::<i64>(self.sp);}
      instruction::NEGF=>{self.memory.neg::<f64>(self.sp);}
      instruction::NOT =>{self.memory.not::<u64>(self.sp);}
      instruction::LNOT=>{self.memory.logical_not( self.sp);}
      instruction::ITOU=>{self.memory.itou(self.sp);}
      instruction::UTOI=>{self.memory.utoi(self.sp);}
      instruction::ITOF=>{self.memory.itof(self.sp);}
      instruction::FTOI=>{self.memory.ftoi(self.sp);}

      instruction::PUSH0  =>{self.push(0);}
      instruction::PUSH1  =>{self.push(1);}
      instruction::PUSH2  =>{self.push(2);}
      instruction::PUSH3  =>{self.push(3);}
      instruction::PUSH4  =>{self.push(4);}
      instruction::PUSH5  =>{self.push(5);}
      instruction::PUSH6  =>{self.push(6);}
      instruction::PUSH7  =>{self.push(7);}
      instruction::PUSH8  =>{self.push(8);}
      instruction::PUSHI8 =>{  let  imm = self.read_imm8()  as  i8;  self.push(imm as usize);}
      instruction::PUSHI16=>{  let  imm = self.read_imm16() as i16;  self.push(imm as usize);}
      instruction::PUSHI32=>{  let  imm = self.read_imm32() as i32;  self.push(imm as usize);}
      instruction::PUSHU8 =>{  let  imm = self.read_imm8() ;         self.push(imm as usize);}
      instruction::PUSHU16=>{  let  imm = self.read_imm16();         self.push(imm as usize);}
      instruction::PUSHU32=>{  let  imm = self.read_imm32();         self.push(imm as usize);}
      instruction::PUSHF32=>{  let  imm = self.read_imm32();         self.push((f32::from_bits(imm) as f64).to_bits() as usize);}
      instruction::PUSH64 =>{  let  imm = self.read_imm64();         self.push(imm as usize);}
      instruction::POP    =>{self.dec_sp();}
      instruction::DUP    =>{  let  v = self.top::<usize>();  self.push(v);}

      instruction::LDI8 =>{self.memory.put::<i64>(self.sp-WORD_SIZE,self.memory.ld::< i8>(self.sp) as i64);},
      instruction::LDI16=>{self.memory.put::<i64>(self.sp-WORD_SIZE,self.memory.ld::<i16>(self.sp) as i64);},
      instruction::LDI32=>{self.memory.put::<i64>(self.sp-WORD_SIZE,self.memory.ld::<i32>(self.sp) as i64);},
      instruction::LDU8 =>{self.memory.put::<u64>(self.sp-WORD_SIZE,self.memory.ld::< u8>(self.sp) as u64);},
      instruction::LDU16=>{self.memory.put::<u64>(self.sp-WORD_SIZE,self.memory.ld::<u16>(self.sp) as u64);},
      instruction::LDU32=>{self.memory.put::<u64>(self.sp-WORD_SIZE,self.memory.ld::<u32>(self.sp) as u64);},
      instruction::LDF32=>{self.memory.put::<f64>(self.sp-WORD_SIZE,self.memory.ld::<f32>(self.sp) as f64);},
      instruction::LD64 =>{self.memory.put::<u64>(self.sp-WORD_SIZE,self.memory.ld::<u64>(self.sp) as u64);},

      instruction::STI8 =>{  self.memory.st::< i8>(self.sp);  self.sp -= (WORD_SIZE*2);},
      instruction::STI16=>{  self.memory.st::<i16>(self.sp);  self.sp -= (WORD_SIZE*2);},
      instruction::STI32=>{  self.memory.st::<i32>(self.sp);  self.sp -= (WORD_SIZE*2);},
      instruction::STU8 =>{  self.memory.st::< u8>(self.sp);  self.sp -= (WORD_SIZE*2);},
      instruction::STU16=>{  self.memory.st::<u16>(self.sp);  self.sp -= (WORD_SIZE*2);},
      instruction::STU32=>{  self.memory.st::<u32>(self.sp);  self.sp -= (WORD_SIZE*2);},
      instruction::STF32=>{  self.memory.st::<f32>(self.sp);  self.sp -= (WORD_SIZE*2);},
      instruction::ST64 =>{  self.memory.st::<u64>(self.sp);  self.sp -= (WORD_SIZE*2);},

      instruction::GLO=>{self.glo();}
      instruction::ARG=>{self.arg();}
      instruction::LOC=>{self.loc();}
      instruction::SPX=>{self.spx();}

      instruction::PRCAL=>{self.prcal();}
      instruction::CAL =>{self.cal();}
      instruction::JMP =>{self.jmp();}
      instruction::BRZ =>{self.brz();}
      instruction::BRNZ=>{self.brnz();}
      instruction::RET =>{self.ret();}
      instruction::PRI=>{self.pri();}
      instruction::PRU=>{self.pru();}
      instruction::PRF=>{self.prf();}
      instruction::REPO=>{self.repo();}
      instruction::HLT =>{self.hlt();}

      _=>{  println!("unknown opcode {}, pc: {}",opcode,pc);  self.set_freeze_flag();  return None;}
        }


      return Some(());
    }


  None
}


pub fn
run(&mut self, n_opt: Option<usize>)
{
    if let Some(n) = n_opt
    {
      let  dur = std::time::Duration::from_millis(80);

      let  mut counter = n;

        loop
        {
            if counter != 0
            {
              counter -= 1;

                if let None = self.step()
                {
                  break;
                }
            }

          else
            {
              counter = n;

              std::thread::sleep(dur);
            }
        }
    }

  else
    {
        loop
        {
            if let None = self.step()
            {
              break;
            }
        }
    }
}


}




