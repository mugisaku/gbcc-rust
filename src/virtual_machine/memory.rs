



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
read(&self, start: u64, size: u64)-> Vec<u8>
{
  let mut  buf: Vec<u8> = Vec::new();

    for offset in 0..size as usize
    {
      buf.push(self.content[(start as usize)+offset]);
    }


  buf
}


pub fn
write(&mut self, img: &Vec<u8>, pos: u64)
{
    for offset in 0..img.len()
    {
      self.content[pos as usize+offset] = img[offset];
    }
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
      *(self.content.as_ptr().add((addr as usize>>1)<<1) as *const u16)
    }
}


pub fn
get_u32(&self, addr: u64)-> u32
{
    unsafe
    {
      *(self.content.as_ptr().add((addr as usize>>2)<<2) as *const u32)
    }
}


pub fn
get_u64(&self, addr: u64)-> u64
{
    unsafe
    {
      *(self.content.as_ptr().add((addr as usize>>3)<<3) as *const u64)
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
      *(self.content.as_ptr().add((addr as usize>>1)<<1) as *const i16)
    }
}


pub fn
get_i32(&self, addr: u64)-> i32
{
    unsafe
    {
      *(self.content.as_ptr().add((addr as usize>>2)<<2) as *const i32)
    }
}


pub fn
get_i64(&self, addr: u64)-> i64
{
    unsafe
    {
      *(self.content.as_ptr().add((addr as usize>>3)<<3) as *const i64)
    }
}


pub fn
get_f32(&self, addr: u64)-> f32
{
    unsafe
    {
      *(self.content.as_ptr().add((addr as usize>>2)<<2) as *const f32)
    }
}


pub fn
get_f64(&self, addr: u64)-> f64
{
    unsafe
    {
      *(self.content.as_ptr().add((addr as usize>>3)<<3) as *const f64)
    }
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
      *(self.content.as_mut_ptr().add((addr as usize>>1)<<1) as *mut u16) = v;
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
      *(self.content.as_mut_ptr().add((addr as usize>>2)<<2) as *mut u32) = v;
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
      *(self.content.as_mut_ptr().add((addr as usize>>3)<<3) as *mut u64) = v;
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
      *(self.content.as_mut_ptr().add((addr as usize>>1)<<1) as *mut i16) = v;
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
      *(self.content.as_mut_ptr().add((addr as usize>>2)<<2) as *mut i32) = v;
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
      *(self.content.as_mut_ptr().add((addr as usize>>3)<<3) as *mut i64) = v;
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
      *(self.content.as_mut_ptr().add((addr as usize>>2)<<2) as *mut f32) = v;
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
      *(self.content.as_mut_ptr().add((addr as usize>>3)<<3) as *mut f64) = v;
    }
}




}





