

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


pub fn  put_bool(&mut self, val: bool, off: usize){unsafe{*self.core.as_mut_ptr().add(off) = if val{1} else{0}};}

pub fn   put_i8(&mut self, val:  i8, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut  i8) = val};}
pub fn  put_i16(&mut self, val: i16, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut i16) = val};}
pub fn  put_i32(&mut self, val: i32, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut i32) = val};}
pub fn  put_i64(&mut self, val: i64, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut i64) = val};}
pub fn   put_u8(&mut self, val:  u8, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut  u8) = val};}
pub fn  put_u16(&mut self, val: u16, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut u16) = val};}
pub fn  put_u32(&mut self, val: u32, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut u32) = val};}
pub fn  put_u64(&mut self, val: u64, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut u64) = val};}
pub fn  put_f32(&mut self, val: f32, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut f32) = val};}
pub fn  put_f64(&mut self, val: f64, off: usize){unsafe{*(self.core.as_mut_ptr().add(off) as *mut f64) = val};}

pub fn
put_str(&self, buf: &Vec<u8>, base: usize)
{
    for off in 0..buf.len()
    {
      let  v = *buf.get_unchecked(off);

      unsafe{*self.core.as_ptr().add(base+off)}) = v;
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
addi(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(o1+o2,dst);
}

pub fn
subi(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(o1-o2,dst);
}

pub fn
muli(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(o1*o2,dst);
}

pub fn
divi(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(o1/o2,dst);
}

pub fn
remi(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_i64(o1%o2,dst);
}

pub fn
addu(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1+o2,dst);
}

pub fn
subu(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1-o2,dst);
}

pub fn
mulu(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1*o2,dst);
}

pub fn
divu(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1/o2,dst);
}

pub fn
remu(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1%o2,dst);
}

pub fn
addf(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(o1+o2,dst);
}

pub fn
subf(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(o1-o2,dst);
}

pub fn
mulf(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(o1*o2,dst);
}

pub fn
divf(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(o1/o2,dst);
}

pub fn
remf(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_f64(o1%o2,dst);
}

pub fn
shl(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1<<o2,dst);
}

pub fn
shr(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1>>o2,dst);
}

pub fn
and(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1&o2,dst);
}

pub fn
or(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1|o2,dst);
}

pub fn
xor(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_u64(o1^o2,dst);
}

pub fn
eq(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(o1 == o2,dst);
}

pub fn
neq(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(o1 != o2,dst);
}

pub fn
lti(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_bool(o1 < o2,dst);
}

pub fn
lteqi(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_bool(o1 <= o2,dst);
}

pub fn
gti(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_bool(o1 > o2,dst);
}

pub fn
gteqi(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_i64(src1);
  let  o2 = self.get_i64(src2);

  self.put_bool(o1 >= o2,dst);
}

pub fn
ltu(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(o1 < o2,dst);
}

pub fn
lteq(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(o1 <= o2,dst);
}

pub fn
gtu(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(o1 > o2,dst);
}

pub fn
gtequ(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_u64(src1);
  let  o2 = self.get_u64(src2);

  self.put_bool(o1 >= o2,dst);
}

pub fn
ltf(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_bool(o1 < o2,dst);
}

pub fn
lteqf(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_bool(o1 <= o2,dst);
}

pub fn
gtf(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_bool(o1 > o2,dst);
}

pub fn
gteqf(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_f64(src1);
  let  o2 = self.get_f64(src2);

  self.put_bool(o1 >= o2,dst);
}

pub fn
logical_and(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_bool(src1);
  let  o2 = self.get_bool(src2);

  self.put_bool(o1 && o2,dst);
}

pub fn
logical_or(&mut self, src1: usize, src2: usize, dst: usize)
{
  let  o1 = self.get_bool(src1);
  let  o2 = self.get_bool(src2);

  self.put_bool(o1 || o2,dst);
}

pub fn
logical_not(&mut self, src: usize, dst: usize)
{
  let  o = self.get_bool(src);

  self.put_bool(!o,dst);
}

pub fn
not(&mut self, src: usize, dst: usize)
{
  let  o = self.get_u64(src);

  self.put_u64(!o,dst);
}

pub fn
negi(&mut self, src: usize, dst: usize)
{
  let  o = self.get_i64(src);

  self.put_i64(-o,dst);
}

pub fn
negf(&mut self, src: usize, dst: usize)
{
  let  o = self.get_f64(src);

  self.put_f64(-o,dst);
}

pub fn
itou(&mut self, src: usize, dst: usize)
{
  let  o = self.get_i64(src);

  self.put_u64(o as u64,dst);
}

pub fn
utoi(&mut self, src: usize, dst: usize)
{
  let  o = self.get_u64(src);

  self.put_i64(o as i64,dst);
}

pub fn
ftoi(&mut self, src: usize, dst: usize)
{
  let  o = self.get_f64(src);

  self.put_i64(o as i64,dst);
}

pub fn
itof(&mut self, src: usize, dst: usize)
{
  let  o = self.get_i64(src);

  self.put_f64(o as f64,dst);
}

pub fn
cp(&mut self, src: usize, dst: usize, sz: usize)
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




