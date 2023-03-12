

use std::convert::From;

pub const WORD_SIZE: usize = 8;


pub fn
get_word_size_aligned(i: i64)-> i64
{
  let  wsz = WORD_SIZE as i64;

  (i+(wsz-1))/wsz*wsz
}


#[derive(Clone,Copy)]
pub union
Word
{
  i: i64,
  u: u64,
  f: f64,

  m_i8 :  i8,
  m_i16: i16,
  m_i32: i32,

  m_u8 :  u8,
  m_u16: u16,
  m_u32: u32,

  m_f32: f32,

}


impl
Word
{


pub fn from_i64(i: i64)-> Word{Word{i}}
pub fn from_u64(u: u64)-> Word{Word{u}}
pub fn from_f64(f: f64)-> Word{Word{f}}

pub fn get_i64(&self)-> i64{unsafe{self.i}}
pub fn get_u64(&self)-> u64{unsafe{self.u}}
pub fn get_f64(&self)-> f64{unsafe{self.f}}

pub fn get_i8(&self)->   i8{unsafe{self.m_i8}}
pub fn get_i16(&self)-> i16{unsafe{self.m_i16}}
pub fn get_i32(&self)-> i32{unsafe{self.m_i32}}
pub fn get_u8(&self)->   u8{unsafe{self.m_u8}}
pub fn get_u16(&self)-> u16{unsafe{self.m_u16}}
pub fn get_u32(&self)-> u32{unsafe{self.m_u32}}
pub fn get_f32(&self)-> f32{unsafe{self.m_f32}}

pub fn get_bool(&self)-> bool{unsafe{self.u != 0}}


}


impl  From<i8>  for Word{fn from(i: i8)->  Word{Word::from_i64(i as i64)}}
impl  From<i16> for Word{fn from(i: i16)-> Word{Word::from_i64(i as i64)}}
impl  From<i32> for Word{fn from(i: i32)-> Word{Word::from_i64(i as i64)}}
impl  From<i64> for Word{fn from(i: i64)-> Word{Word::from_i64(i)}}
impl  From<u8>  for Word{fn from(u: u8)->  Word{Word::from_u64(u as u64)}}
impl  From<u16> for Word{fn from(u: u16)-> Word{Word::from_u64(u as u64)}}
impl  From<u32> for Word{fn from(u: u32)-> Word{Word::from_u64(u as u64)}}
impl  From<u64> for Word{fn from(u: u64)-> Word{Word::from_u64(u)}}
impl  From<f32> for Word{fn from(f: f32)-> Word{Word::from_f64(f as f64)}}
impl  From<f64> for Word{fn from(f: f64)-> Word{Word::from_f64(f)}}
impl  From<bool> for Word{fn from(b: bool)-> Word{if b{Word::from_u64(1)}else{Word::from_u64(0)}}}




pub struct
Memory
{
  content: Vec<u8>,

  putlog_flag: bool,
  getlog_flag: bool,

}


impl
Memory
{


pub fn
new(sz: usize)-> Memory
{
  let mut  mem = Memory{ content: Vec::new(), putlog_flag: false, getlog_flag: false};

  mem.content.resize(sz,0);

  mem
}


pub fn
from_word(w: Word)-> Memory
{
  let mut  mem = Memory{ content: Vec::new(), putlog_flag: false, getlog_flag: false};

  mem.content.resize(WORD_SIZE,0);

  mem.put_word(0,w);

  mem
}


pub fn
resize(&mut self, sz: u64)
{
  self.content.resize(sz as usize,0);
}


pub fn
get_size(&self)-> u64
{
  self.content.len() as u64
}


pub fn
zerofill(&mut self)
{
    for v in &mut self.content
    {
      *v = 0;
    }
}


pub fn
read(&mut self, dst_start: u64, src: &Memory, src_start: u64, src_sz_opt: Option<u64>)-> Result<(),()>
{
  let  src_sz = if let Some(sz) = src_sz_opt{sz} else{src.get_size()};

    if (src_start+src_sz) > src.get_size()
    {
      println!("range of src is invalid");

      return Err(());
    }


    if (dst_start+src_sz) <= self.get_size()
    {
        for offset in 0..src_sz
        {
          self.content[(dst_start+offset) as usize] = src.content[(src_start+offset) as usize];
        }


      return Ok(());
    }


  Err(())
}


pub fn
test_getlog_flag(&self)-> bool
{
  self.getlog_flag
}


pub fn
test_putlog_flag(&self)-> bool
{
  self.putlog_flag
}


pub fn
set_putlog_flag(&mut self)
{
  self.putlog_flag = true;
}


pub fn
unset_putlog_flag(&mut self)
{
  self.putlog_flag = false;
}




pub fn  align2(addr: u64)-> usize{(addr as usize>>1)<<1}
pub fn  align4(addr: u64)-> usize{(addr as usize>>2)<<2}
pub fn  align8(addr: u64)-> usize{(addr as usize>>3)<<3}


pub fn
get_u8(&self, addr: u64)-> u8
{
  self.content[addr as usize]
}


pub fn
get_u16(&self, addr: u64)-> u16
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align2(addr)) as *const u16)
    }
}


pub fn
get_u32(&self, addr: u64)-> u32
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align4(addr)) as *const u32)
    }
}


pub fn
get_u64(&self, addr: u64)-> u64
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align8(addr)) as *const u64)
    }
}


pub fn
get_i8(&self, addr: u64)-> i8
{
    unsafe
    {
      *(self.content.as_ptr().add(addr as usize) as *const i8)
    }
}


pub fn
get_i16(&self, addr: u64)-> i16
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align2(addr)) as *const i16)
    }
}


pub fn
get_i32(&self, addr: u64)-> i32
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align4(addr)) as *const i32)
    }
}


pub fn
get_i64(&self, addr: u64)-> i64
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align8(addr)) as *const i64)
    }
}


pub fn
get_f32(&self, addr: u64)-> f32
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align4(addr)) as *const f32)
    }
}


pub fn
get_f64(&self, addr: u64)-> f64
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align8(addr)) as *const f64)
    }
}


pub fn
get_word(&self, addr: u64)-> Word
{
  Word::from_u64(self.get_u64(addr))
}




pub fn
put_u8(&mut self, addr: u64, v: u8)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


  self.content[addr as usize] = v;
}


pub fn
put_u16(&mut self, addr: u64, v: u16)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align2(addr)) as *mut u16) = v;
    }
}


pub fn
put_u32(&mut self, addr: u64, v: u32)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align4(addr)) as *mut u32) = v;
    }
}


pub fn
put_u64(&mut self, addr: u64, v: u64)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align8(addr)) as *mut u64) = v;
    }
}


pub fn
put_i8(&mut self, addr: u64, v: i8)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


    unsafe
    {
      *(self.content.as_mut_ptr().add(addr as usize) as *mut i8) = v;
    }
}


pub fn
put_i16(&mut self, addr: u64, v: i16)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align2(addr)) as *mut i16) = v;
    }
}


pub fn
put_i32(&mut self, addr: u64, v: i32)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align4(addr)) as *mut i32) = v;
    }
}


pub fn
put_i64(&mut self, addr: u64, v: i64)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align8(addr)) as *mut i64) = v;
    }
}


pub fn
put_f32(&mut self, addr: u64, v: f32)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align4(addr)) as *mut f32) = v;
    }
}


pub fn
put_f64(&mut self, addr: u64, v: f64)
{
    if self.putlog_flag
    {
      print!("put( addr: {}, value: {})\n",addr,v);
    }


    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align8(addr)) as *mut f64) = v;
    }
}


pub fn
put_word(&mut self, addr: u64, w: Word)
{
  self.put_u64(addr,w.get_u64());
}




}





