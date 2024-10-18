

const WORD_SIZE: usize = 8;


pub struct
Memory
{
  pub(crate) core: Vec<u8>,

}


impl
Memory
{


pub fn
new()-> Self
{
  Self{core: Vec::new()}
}


pub fn
get_size(&self)-> usize
{
  self.core.len()
}


pub fn
extend(&mut self, sz: usize)
{
  let  now = self.get_size();

  self.core.resize(now+sz,0);
}


pub fn
reduce(&mut self, sz: usize)
{
  self.core.truncate(sz);
}


pub fn  put_bool(&mut self, off: usize, val: bool){unsafe{*self.core.as_mut_ptr().add(off) = if val{1} else{0}};}

pub fn   put_i8(&mut self, off: usize, val:  i8){unsafe{*(self.core.as_mut_ptr().add(off) as *mut  i8) = val};}
pub fn  put_i16(&mut self, off: usize, val: i16){unsafe{*(self.core.as_mut_ptr().add(off) as *mut i16) = val};}
pub fn  put_i32(&mut self, off: usize, val: i32){unsafe{*(self.core.as_mut_ptr().add(off) as *mut i32) = val};}
pub fn  put_i64(&mut self, off: usize, val: i64){unsafe{*(self.core.as_mut_ptr().add(off) as *mut i64) = val};}
pub fn   put_u8(&mut self, off: usize, val:  u8){unsafe{*(self.core.as_mut_ptr().add(off) as *mut  u8) = val};}
pub fn  put_u16(&mut self, off: usize, val: u16){unsafe{*(self.core.as_mut_ptr().add(off) as *mut u16) = val};}
pub fn  put_u32(&mut self, off: usize, val: u32){unsafe{*(self.core.as_mut_ptr().add(off) as *mut u32) = val};}
pub fn  put_u64(&mut self, off: usize, val: u64){unsafe{*(self.core.as_mut_ptr().add(off) as *mut u64) = val};}
pub fn  put_f32(&mut self, off: usize, val: f32){unsafe{*(self.core.as_mut_ptr().add(off) as *mut f32) = val};}
pub fn  put_f64(&mut self, off: usize, val: f64){unsafe{*(self.core.as_mut_ptr().add(off) as *mut f64) = val};}

pub fn
put_str(&mut self, base: usize, buf: &Vec<u8>)
{
    for off in 0..buf.len()
    {
      let  v = unsafe{*buf.get_unchecked(off)};

      let  dst = unsafe{&mut *self.core.as_mut_ptr().add(base+off)};

      *dst = v;
    }
}

pub fn  get_bool(&self, off: usize)-> bool{if unsafe{*self.core.as_ptr().add(off)} != 0{true} else{false}}

pub fn   get_i8(&self, off: usize)->  i8{unsafe{*(self.core.as_ptr().add(off) as *const  i8)}}
pub fn  get_i16(&self, off: usize)-> i16{unsafe{*(self.core.as_ptr().add(off) as *const i16)}}
pub fn  get_i32(&self, off: usize)-> i32{unsafe{*(self.core.as_ptr().add(off) as *const i32)}}
pub fn  get_i64(&self, off: usize)-> i64{unsafe{*(self.core.as_ptr().add(off) as *const i64)}}
pub fn   get_u8(&self, off: usize)->  u8{unsafe{*(self.core.as_ptr().add(off) as *const  u8)}}
pub fn  get_u16(&self, off: usize)-> u16{unsafe{*(self.core.as_ptr().add(off) as *const u16)}}
pub fn  get_u32(&self, off: usize)-> u32{unsafe{*(self.core.as_ptr().add(off) as *const u32)}}
pub fn  get_u64(&self, off: usize)-> u64{unsafe{*(self.core.as_ptr().add(off) as *const u64)}}
pub fn  get_f32(&self, off: usize)-> f32{unsafe{*(self.core.as_ptr().add(off) as *const f32)}}
pub fn  get_f64(&self, off: usize)-> f64{unsafe{*(self.core.as_ptr().add(off) as *const f64)}}

pub fn
get_str(&self, base: usize, sz: usize)-> Vec<u8>
{
  let  mut buf: Vec<u8> = Vec::new();

    for off in 0..sz
    {
      buf.push(unsafe{*self.core.as_ptr().add(base+off)});
    }


  buf
}




pub fn
addi(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(dst,o1+o2);
}

pub fn
subi(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(dst,o1-o2);
}

pub fn
muli(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(dst,o1*o2);
}

pub fn
divi(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(dst,o1/o2);
}

pub fn
remi(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(dst,o1%o2);
}

pub fn
addu(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1+o2);
}

pub fn
subu(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1-o2);
}

pub fn
mulu(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1*o2);
}

pub fn
divu(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1/o2);
}

pub fn
remu(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1%o2);
}

pub fn
addf(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(dst,o1+o2);
}

pub fn
subf(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(dst,o1-o2);
}

pub fn
mulf(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(dst,o1*o2);
}

pub fn
divf(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(dst,o1/o2);
}

pub fn
remf(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(dst,o1%o2);
}

pub fn
shl(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1<<o2);
}

pub fn
shr(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1>>o2);
}

pub fn
and(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1&o2);
}

pub fn
or(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1|o2);
}

pub fn
xor(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(dst,o1^o2);
}

pub fn
eq(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(dst,o1 == o2);
}

pub fn
neq(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(dst,o1 != o2);
}

pub fn
lti(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_bool(dst,o1 < o2);
}

pub fn
lteqi(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_bool(dst,o1 <= o2);
}

pub fn
gti(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_bool(dst,o1 > o2);
}

pub fn
gteqi(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_bool(dst,o1 >= o2);
}

pub fn
ltu(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(dst,o1 < o2);
}

pub fn
ltequ(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(dst,o1 <= o2);
}

pub fn
gtu(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(dst,o1 > o2);
}

pub fn
gtequ(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(dst,o1 >= o2);
}

pub fn
ltf(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_bool(dst,o1 < o2);
}

pub fn
lteqf(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_bool(dst,o1 <= o2);
}

pub fn
gtf(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_bool(dst,o1 > o2);
}

pub fn
gteqf(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_bool(dst,o1 >= o2);
}

pub fn
logical_and(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_bool(src1);
  let  o2 = self.get_bool(src2);

  self.put_bool(dst,o1 && o2);
}

pub fn
logical_or(&mut self, dst: usize, src1: usize, src2: usize)
{
  let  o1 = self.get_bool(src1);
  let  o2 = self.get_bool(src2);

  self.put_bool(dst,o1 || o2);
}

pub fn
logical_not(&mut self, dst: usize, src: usize)
{
  let  o = self.get_bool(src);

  self.put_bool(dst,!o);
}

pub fn
not(&mut self, dst: usize, src: usize)
{
  let  o = self.get_u64(src);

  self.put_u64(dst,!o);
}

pub fn
negi(&mut self, dst: usize, src: usize)
{
  let  o = self.get_i64(src);

  self.put_i64(dst,-o);
}

pub fn
negf(&mut self, dst: usize, src: usize)
{
  let  o = self.get_f64(src);

  self.put_f64(dst,-o);
}

pub fn
itou(&mut self, dst: usize, src: usize)
{
  let  o = self.get_i64(src);

  self.put_u64(dst,o as u64);
}

pub fn
utoi(&mut self, dst: usize, src: usize)
{
  let  o = self.get_u64(src);

  self.put_i64(dst,o as i64);
}

pub fn
ftoi(&mut self, dst: usize, src: usize)
{
  let  o = self.get_f64(src);

  self.put_i64(dst,o as i64);
}

pub fn
itof(&mut self, dst: usize, src: usize)
{
  let  o = self.get_i64(src);

  self.put_f64(dst,o as f64);
}

pub fn
cp(&mut self, dst: usize, src: usize, sz: usize)
{
    for off in 0..sz
    {
        unsafe
        {
          let  v = *self.core.get_unchecked(src+off);

          *self.core.get_unchecked_mut(dst+off) = v;
        }
    }
}


}





