

use super::opcode;
use super::memory::Memory;

const  word_size: u64 = 8;


pub struct
Processor
{
  memory: Memory,

  pc: u64,
  ep: u64,
  sp: u64,
  bp: u64,

  halt_flag: bool,

}


impl
Processor
{


pub fn
new()-> Processor
{
  Processor{ memory: Memory::new(0), pc: 0, ep: 0, sp: 0, bp: 0, halt_flag: true}
}


pub fn
renew_memory(&mut self, sz: u64)
{
  self.memory.resize((sz+(word_size-1))/word_size*word_size);
}


pub fn
reset(&mut self)
{
  self.pc = 0;
  self.ep = 0;

            self.bp = self.memory.get_size()-word_size;
  self.sp = self.bp                                   ;

  self.unhalt();
}


pub fn
load_image(&mut self, img: &Vec<u8>)
{
  self.memory.write(img,0);
}


pub fn
extend_sp(&mut self)
{
  self.sp -= word_size;
}


pub fn
shorten_sp(&mut self)
{
  self.sp += word_size;
}


pub fn
pop_u64(&mut self)-> u64
{
  self.shorten_sp();

  self.memory.get_u64(self.sp)
}


pub fn
pop_i64(&mut self)-> i64
{
  self.shorten_sp();

  self.memory.get_i64(self.sp)
}


pub fn
pop_f64(&mut self)-> f64
{
  self.shorten_sp();

  self.memory.get_f64(self.sp)
}


pub fn
dup_u64(&mut self)-> u64
{
  let  v = self.pop_u64();

  self.push_u64(v);

  v
}


pub fn
push_u64(&mut self, v: u64)
{
  self.memory.put_u64(self.sp,v);

  self.extend_sp();
}


pub fn
push_i64(&mut self, v: i64)
{
  self.memory.put_i64(self.sp,v);

  self.extend_sp();
}


pub fn
push_f64(&mut self, v: f64)
{
  self.memory.put_f64(self.sp,v);

  self.extend_sp();
}




pub fn
read_opcode(&mut self)-> u8
{
  let  v = self.memory.get_u8(self.pc);

  self.pc += 1;

  v
}


pub fn
read_u8(&mut self)-> u8
{
  self.read_opcode()
}


pub fn
read_u16le(&mut self)-> u16
{
  let mut  v  =  self.read_opcode() as u16    ;
           v |= (self.read_opcode() as u16)<<8;

  v
}


pub fn
read_u32le(&mut self)-> u32
{
  let mut  v  =  self.read_opcode() as u32     ;
           v |= (self.read_opcode() as u32)<< 8;
           v |= (self.read_opcode() as u32)<<16;
           v |= (self.read_opcode() as u32)<<24;

  v
}


pub fn
read_u64le(&mut self)-> u64
{
  let mut  v  =  self.read_opcode() as u64     ;
           v |= (self.read_opcode() as u64)<< 8;
           v |= (self.read_opcode() as u64)<<16;
           v |= (self.read_opcode() as u64)<<24;
           v |= (self.read_opcode() as u64)<<32;
           v |= (self.read_opcode() as u64)<<40;
           v |= (self.read_opcode() as u64)<<48;
           v |= (self.read_opcode() as u64)<<56;

  v
}


pub fn
read_i8(&mut self)-> i8
{
  self.read_u8() as i8
}


pub fn
read_i16le(&mut self)-> i16
{
  self.read_u16le() as i16
}


pub fn
read_i32le(&mut self)-> i32
{
  self.read_u32le() as i32
}


pub fn
read_i64le(&mut self)-> i64
{
  self.read_u64le() as i64
}


pub fn
read_f32le(&mut self)-> f32
{
  self.read_u32le() as f32
}


pub fn
read_f64le(&mut self)-> f64
{
  self.read_u64le() as f64
}


pub fn
is_halted(&self)-> bool
{
  self.halt_flag
}


pub fn
halt(&mut self)
{
  self.halt_flag = true;
}


pub fn
unhalt(&mut self)
{
  self.halt_flag = false;
}


pub fn
get_old_pc(&self)-> u64
{
  self.memory.get_u64(self.bp)
}


pub fn
get_old_ep(&self)-> u64
{
  self.memory.get_u64(self.bp-word_size)
}


pub fn
get_old_bp(&self)-> u64
{
  self.memory.get_u64(self.bp-(word_size*2))
}


pub fn
get_return_value_address(&self)-> u64
{
  self.memory.get_u64(self.bp-(word_size*3))
}


pub fn
finish_return(&mut self)
{
  self.pc = self.get_old_pc();
  self.ep = self.get_old_ep();

  let  old_bp = self.get_old_bp();

  self.sp = self.bp         ;
            self.bp = old_bp;
}


pub fn
print_info(&self)
{
  print!("pc: {}, ep: {}, bp: {}, sp: {}\n",self.pc,self.ep,self.bp,self.sp);
}


pub fn
print_frame_info(&self)
{
  let  old_pc = self.get_old_bp();
  let  old_ep = self.get_old_ep();
  let  old_bp = self.get_old_bp();

  print!("old_pc: {}, old_ep: {}, old_bp: {}\n",old_pc,old_ep,old_bp);
}


pub fn
step(&mut self)
{
    if self.is_halted()
    {
      return;
    }


  print!("pc: {} = ",self.pc);

  let  opcode = self.read_opcode();

  print!("{}\n",opcode::get_name(opcode));

    match opcode
    {
  opcode::nop=>{},
  opcode::ldu8=>
        {
          let  addr = self.pop_u64();

          self.push_u64(self.memory.get_u8(addr) as u64);
        },
  opcode::ldu16=>
        {
          let  addr = self.pop_u64();

          self.push_u64(self.memory.get_u16(addr) as u64);
        },
  opcode::ldu32=>
        {
          let  addr = self.pop_u64();

          self.push_u64(self.memory.get_u32(addr) as u64);
        },
  opcode::lds8=>
        {
          let  addr = self.pop_u64();

          self.push_i64(self.memory.get_i8(addr) as i64);
        },
  opcode::lds16=>
        {
          let  addr = self.pop_u64();

          self.push_i64(self.memory.get_i16(addr) as i64);
        },
  opcode::lds32=>
        {
          let  addr = self.pop_u64();

          self.push_i64(self.memory.get_i32(addr) as i64);
        },
  opcode::ldf32=>
        {
          let  addr = self.pop_u64();

          self.push_f64(self.memory.get_f32(addr) as f64);
        },
  opcode::ld64=>
        {
          let  addr = self.pop_u64();

          self.push_u64(self.memory.get_u64(addr));
        },
  opcode::stu8=>
        {
          let  addr = self.pop_u64();
          let     v = self.pop_u64();

          self.memory.put_u8(addr,v as u8);
        },
  opcode::stu16=>
        {
          let  addr = self.pop_u64();
          let     v = self.pop_u64();

          self.memory.put_u16(addr,v as u16);
        },
  opcode::stu32=>
        {
          let  addr = self.pop_u64();
          let     v = self.pop_u64();

          self.memory.put_u32(addr,v as u32);
        },
  opcode::sts8=>
        {
          let  addr = self.pop_u64();
          let     v = self.pop_u64();

          self.memory.put_i8(addr,v as i8);
        },
  opcode::sts16=>
        {
          let  addr = self.pop_u64();
          let     v = self.pop_u64();

          self.memory.put_i16(addr,v as i16);
        },
  opcode::sts32=>
        {
          let  addr = self.pop_u64();
          let     v = self.pop_u64();

          self.memory.put_i32(addr,v as i32);
        },
  opcode::stf32=>
        {
          let  addr = self.pop_u64();
          let     v = self.pop_u64();

          self.memory.put_f32(addr,v as f32);
        },
  opcode::st64=>
        {
          let  addr = self.pop_u64();
          let     v = self.pop_u64();

          self.memory.put_u64(addr,v);
        },
  opcode::adds=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l+r);
        },
  opcode::subs=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l-r);
        },
  opcode::muls=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l*r);
        },
  opcode::divs=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l/r);
        },
  opcode::rems=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l%r);
        },
  opcode::addu=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(l+r);
        },
  opcode::subu=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(l-r);
        },
  opcode::mulu=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(l*r);
        },
  opcode::divu=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(l/r);
        },
  opcode::remu=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(l%r);
        },
  opcode::addf=>
        {
          let  r = self.pop_f64();
          let  l = self.pop_f64();

          self.push_f64(l+r);
        },
  opcode::subf=>
        {
          let  r = self.pop_f64();
          let  l = self.pop_f64();

          self.push_f64(l-r);
        },
  opcode::mulf=>
        {
          let  r = self.pop_f64();
          let  l = self.pop_f64();

          self.push_f64(l*r);
        },
  opcode::divf=>
        {
          let  r = self.pop_f64();
          let  l = self.pop_f64();

          self.push_f64(l/r);
        },
  opcode::remf=>
        {
          let  r = self.pop_f64();
          let  l = self.pop_f64();

          self.push_f64(l%r);
        },
  opcode::shl=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l<<r);
        },
  opcode::shr=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l>>r);
        },
  opcode::and=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l&r);
        },
  opcode::or=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l|r);
        },
  opcode::xor=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_i64(l^r);
        },
  opcode::eq=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(if l == r{1}else{0});
        },
  opcode::neq=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(if l != r{1}else{0});
        },
  opcode::lts=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_u64(if l < r{1}else{0});
        },
  opcode::lteqs=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_u64(if l < r{1}else{0});
        },
  opcode::gts=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_u64(if l <= r{1}else{0});
        },
  opcode::gteqs=>
        {
          let  r = self.pop_i64();
          let  l = self.pop_i64();

          self.push_u64(if l > r{1}else{0});
        },
  opcode::ltu=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(if l >= r{1}else{0});
        },
  opcode::ltequ=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(if l <= r{1}else{0});
        },
  opcode::gtu=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(if l > r{1}else{0});
        },
  opcode::gtequ=>
        {
          let  r = self.pop_u64();
          let  l = self.pop_u64();

          self.push_u64(if l >= r{1}else{0});
        },
  opcode::ltf=>
        {
          let  r = self.pop_f64();
          let  l = self.pop_f64();

          self.push_u64(if l < r{1}else{0});
        },
  opcode::lteqf=>
        {
          let  r = self.pop_f64();
          let  l = self.pop_f64();

          self.push_u64(if l <= r{1}else{0});
        },
  opcode::gtf=>
        {
          let  r = self.pop_f64();
          let  l = self.pop_f64();

          self.push_u64(if l > r{1}else{0});
        },
  opcode::gteqf=>
        {
          let  r = self.pop_f64();
          let  l = self.pop_f64();

          self.push_u64(if l >= r{1}else{0});
        },
  opcode::land=>
        {
          let  r = self.pop_u64() != 0;
          let  l = self.pop_u64() != 0;

          self.push_u64(if l && r{1}else{0});
        },
  opcode::lor=>
        {
          let  r = self.pop_u64() != 0;
          let  l = self.pop_u64() != 0;

          self.push_u64(if l || r{1}else{0});
        },
  opcode::lnot=>
        {
          let  v = self.pop_u64();

          self.push_u64(if v != 0{0}else{1});
        },
  opcode::not=>
        {
          let  v = self.pop_u64();

          self.push_u64(!v);
        },
  opcode::neg=>
        {
          let  v = self.pop_i64();

          self.push_i64(-v);
        },
  opcode::negf=>
        {
          let  v = self.pop_f64();

          self.push_f64(-v);
        },
  opcode::psh0=>
        {
          self.push_u64(0);
        },
  opcode::psh1=>
        {
          self.push_u64(1);
        },
  opcode::psh2=>
        {
          self.push_u64(2);
        },
  opcode::psh4=>
        {
          self.push_u64(4);
        },
  opcode::psh8=>
        {
          self.push_u64(8);
        },
  opcode::pshfw=>
        {
          self.push_i64(-1);
        },
  opcode::pshu8=>
        {
          let  v = self.read_u8();

          self.push_u64(v as u64);
        },
  opcode::pshu16=>
        {
          let  v = self.read_u16le();

          self.push_u64(v as u64);
        },
  opcode::pshu32=>
        {
          let  v = self.read_u32le();

          self.push_u64(v as u64);
        },
  opcode::pshs8=>
        {
          let  v = self.read_i8();

          self.push_i64(v as i64);
        },
  opcode::pshs16=>
        {
          let  v = self.read_i16le();

          self.push_i64(v as i64);
        },
  opcode::pshs32=>
        {
          let  v = self.read_i32le();

          self.push_i64(v as i64);
        },
  opcode::pshf32=>
        {
          let  v = self.read_f32le();

          self.push_f64(v as f64);
        },
  opcode::pshb64=>
        {
          let  v = self.read_u64le();

          self.push_u64(v);
        },
  opcode::xsp=>
        {
          let  v = self.pop_u64();

          self.sp -= v;
        },
  opcode::ssp=>
        {
          let  v = self.pop_u64();

          self.sp += v;
        },
  opcode::maa=>
        {
          let  v = self.pop_u64();

          self.push_u64(self.bp+v);
        },
  opcode::swp=>
        {
          let  a = self.pop_u64();
          let  b = self.pop_u64();

          self.push_u64(a);
          self.push_u64(b);
        },
  opcode::jmp=>
        {
          self.pc = self.pop_i64() as u64;
        },
  opcode::br=>
        {
          let  offset = self.pop_i64();
          let       v = self.pop_u64();

            if v == 0
            {
              self.pc = (self.pc as i64+offset) as u64;
            }
        },
  opcode::cal=>
        {
          let  calstk_sz = self.pop_u64();

          let  retval_addr = self.sp+calstk_sz;
 
          let  fn_addr = self.memory.get_u64(retval_addr);
 
          let  old_pc = self.pc          ;
                        self.pc = fn_addr;

          let  old_ep = self.ep;
          let  old_bp = self.bp;

          self.ep = self.pc;
          self.bp = self.sp;

          self.push_u64(old_pc);
          self.push_u64(old_ep);
          self.push_u64(old_bp);
          self.push_u64(retval_addr);
        },
  opcode::ret=>
        {
          self.finish_return();
        },
  opcode::hlt=>
        {
          self.halt();
        },
  opcode::retd=>
        {
          let  sz = self.pop_u64();

            if sz > 0
            {
              let  src_address = self.sp+sz;
              let  dst_address = self.get_return_value_address();

                for offset in 0..sz
                {
                  let  v = self.memory.get_u8(src_address+offset);

                  self.memory.put_u8(dst_address+offset,v);
                }
            }


          self.finish_return();
        },
  opcode::reti=>
        {
          let  sz = self.pop_u64();

            if sz > 0
            {
              let  src_address = self.pop_u64();
              let  dst_address = self.get_return_value_address();

                for offset in 0..sz
                {
                  let  v = self.memory.get_u8(src_address+offset);

                  self.memory.put_u8(dst_address+offset,v);
                }
            }


          self.finish_return();
        },
  opcode::prnu64=>
        {
          print!("{} is printed\n",self.dup_u64());
        },
  opcode::putlog0=>
        {
          self.memory.unset_putlog_flag();
        },
  opcode::putlog1=>
        {
          self.memory.set_putlog_flag();
        },
  _=>{print!("unkown opcode {}\n",opcode)},
    }
}




}





