

use std::convert::From;

pub const WORD_SIZE: usize = 8;


pub fn
get_aligned(u: usize)-> usize
{
  (u+(WORD_SIZE-1))/WORD_SIZE*WORD_SIZE
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


#[allow(dead_code)]
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
impl  From<usize> for Word{fn from(u: usize)-> Word{Word::from_u64(u as u64)}}
impl  From<f32> for Word{fn from(f: f32)-> Word{Word::from_f64(f as f64)}}
impl  From<f64> for Word{fn from(f: f64)-> Word{Word::from_f64(f)}}
impl  From<bool> for Word{fn from(b: bool)-> Word{if b{Word::from_u64(1)}else{Word::from_u64(0)}}}




#[derive(Clone)]
pub struct
Memory
{
  content: Vec<u8>,

}


#[allow(dead_code)]
impl
Memory
{


pub fn
new(sz: usize)-> Memory
{
  let  mut mem = Memory{content: Vec::new()};

  mem.content.resize(sz,0);

  mem
}


pub fn
from_word(w: Word)-> Memory
{
  let  mut mem = Memory{content: Vec::new()};

  mem.content.resize(WORD_SIZE,0);

  mem.put_word(0,w);

  mem
}


pub fn
from_memory(src: &Memory, start: usize, sz: usize)-> Memory
{
  let  mut dst = Memory::new(sz);

    for i in 0..sz
    {
      dst.content[start+i] = src.content[start+i];
    }


  dst
}


pub fn
resize(&mut self, sz: usize)
{
  self.content.resize(sz,0);
}


pub fn
get_size(&self)-> usize
{
  self.content.len()
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
read(&mut self, dst_start: usize, src: &Memory, src_start: usize, src_sz_opt: Option<usize>)-> Result<(),()>
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
          self.content[(dst_start+offset)] = src.content[(src_start+offset)];
        }


      return Ok(());
    }


  Err(())
}


pub fn
copy(&mut self, dst_start: usize, src_start: usize, sz: usize)-> Result<(),()>
{
    if (dst_start+sz) > self.get_size()
    {
      println!("range of src is invalid");

      return Err(());
    }


    if (src_start+sz) > self.get_size()
    {
      println!("range of src is invalid");

      return Err(());
    }


    for offset in 0..sz
    {
      let  byte = self.content[(src_start+offset)]       ;
                  self.content[(dst_start+offset)] = byte;
    }


  Ok(())
}




pub fn  align2(addr: usize)-> usize{(addr>>1)<<1}
pub fn  align4(addr: usize)-> usize{(addr>>2)<<2}
pub fn  align8(addr: usize)-> usize{(addr>>3)<<3}


pub fn
get_u8(&self, addr: usize)-> u8
{
  self.content[addr]
}


pub fn
get_u16(&self, addr: usize)-> u16
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align2(addr)) as *const u16)
    }
}


pub fn
get_u32(&self, addr: usize)-> u32
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align4(addr)) as *const u32)
    }
}


pub fn
get_u64(&self, addr: usize)-> u64
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align8(addr)) as *const u64)
    }
}


pub fn
get_i8(&self, addr: usize)-> i8
{
    unsafe
    {
      *(self.content.as_ptr().add(addr) as *const i8)
    }
}


pub fn
get_i16(&self, addr: usize)-> i16
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align2(addr)) as *const i16)
    }
}


pub fn
get_i32(&self, addr: usize)-> i32
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align4(addr)) as *const i32)
    }
}


pub fn
get_i64(&self, addr: usize)-> i64
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align8(addr)) as *const i64)
    }
}


pub fn
get_f32(&self, addr: usize)-> f32
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align4(addr)) as *const f32)
    }
}


pub fn
get_f64(&self, addr: usize)-> f64
{
    unsafe
    {
      *(self.content.as_ptr().add(Self::align8(addr)) as *const f64)
    }
}


pub fn
get_word(&self, addr: usize)-> Word
{
  Word::from_u64(self.get_u64(addr))
}




pub fn
put_u8(&mut self, addr: usize, v: u8)
{
  self.content[addr] = v;
}


pub fn
put_u16(&mut self, addr: usize, v: u16)
{
    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align2(addr)) as *mut u16) = v;
    }
}


pub fn
put_u32(&mut self, addr: usize, v: u32)
{
    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align4(addr)) as *mut u32) = v;
    }
}


pub fn
put_u64(&mut self, addr: usize, v: u64)
{
    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align8(addr)) as *mut u64) = v;
    }
}


pub fn
put_i8(&mut self, addr: usize, v: i8)
{
    unsafe
    {
      *(self.content.as_mut_ptr().add(addr) as *mut i8) = v;
    }
}


pub fn
put_i16(&mut self, addr: usize, v: i16)
{
    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align2(addr)) as *mut i16) = v;
    }
}


pub fn
put_i32(&mut self, addr: usize, v: i32)
{
    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align4(addr)) as *mut i32) = v;
    }
}


pub fn
put_i64(&mut self, addr: usize, v: i64)
{
    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align8(addr)) as *mut i64) = v;
    }
}


pub fn
put_f32(&mut self, addr: usize, v: f32)
{
    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align4(addr)) as *mut f32) = v;
    }
}


pub fn
put_f64(&mut self, addr: usize, v: f64)
{
    unsafe
    {
      *(self.content.as_mut_ptr().add(Self::align8(addr)) as *mut f64) = v;
    }
}


pub fn
put_word(&mut self, addr: usize, w: Word)
{
  self.put_u64(addr,w.get_u64());
}


pub fn
print(&self)
{
  print!("{{");

    for c in &self.content
    {
      print!("{},",c);
    }


  print!("}}");
}




}





