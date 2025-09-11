

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
new_with_size(sz: usize)-> Self
{
  let  mut core: Vec<u8> = Vec::new();

  core.resize(sz,0);

  Self{core}
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


pub fn  put<T>(&mut self, off: usize, val: T){unsafe{*(self.core.as_mut_ptr().add(off) as *mut T) = val};}

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



pub fn  get<T: std::clone::Clone>(&self, off: usize)-> T{unsafe{(*(self.core.as_ptr().add(off) as *const T)).clone()}}

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
add<T: std::clone::Clone+std::ops::Add>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo+ro);
}

pub fn
sub<T: std::clone::Clone+std::ops::Sub>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo-ro);
}

pub fn
mul<T: std::clone::Clone+std::ops::Mul>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo*ro);
}

pub fn
div<T: std::clone::Clone+std::ops::Div>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo/ro);
}

pub fn
rem<T: std::clone::Clone+std::ops::Rem>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo%ro);
}

pub fn
shl<T: std::clone::Clone+std::ops::Shl>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo<<ro);
}

pub fn
shr<T: std::clone::Clone+std::ops::Shr>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo>>ro);
}

pub fn
and<T: std::clone::Clone+std::ops::BitAnd>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo&ro);
}

pub fn
or<T: std::clone::Clone+std::ops::BitOr>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo|ro);
}

pub fn
xor<T: std::clone::Clone+std::ops::BitXor>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo^ro);
}

pub fn
eq<T: std::clone::Clone+std::cmp::PartialOrd>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo == ro);
}

pub fn
neq<T: std::clone::Clone+std::cmp::PartialOrd>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo != ro);
}

pub fn
lt<T: std::clone::Clone+std::cmp::PartialOrd>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo < ro);
}

pub fn
lteq<T: std::clone::Clone+std::cmp::PartialOrd>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo <= ro);
}

pub fn
gt<T: std::clone::Clone+std::cmp::PartialOrd>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo > ro);
}

pub fn
gteq<T: std::clone::Clone+std::cmp::PartialOrd>(&mut self, sp: usize, ro: T)
{
  let  lo = self.get::<T>(sp);

  self.put(sp,lo >= ro);
}

pub fn
logical_and(&mut self, sp: usize, ro: bool)
{
  let  lo = self.get::<bool>(sp);

  self.put(sp,lo && ro);
}

pub fn
logical_or(&mut self, sp: usize, ro: bool)
{
  let  lo = self.get::<bool>(sp);

  self.put(sp,lo || ro);
}

pub fn
logical_not(&mut self, sp: usize)
{
  let  o = self.get::<bool>(sp);

  self.put(sp,!o);
}

pub fn
not<T: std::clone::Clone+std::ops::Not>(&mut self, sp: usize)
{
  let  o = self.get::<T>(sp);

  self.put(sp,!o);
}

pub fn
neg<T: std::clone::Clone+std::ops::Neg>(&mut self, sp: usize)
{
  let  o = self.get::<T>(sp);

  self.put(sp,-o);
}

pub fn
itou(&mut self, sp: usize)
{
  let  o = self.get::<i64>(sp);

  self.put(sp,o as u64);
}

pub fn
utoi(&mut self, sp: usize)
{
  let  o = self.get::<u64>(sp);

  self.put(sp,o as i64);
}

pub fn
ftoi(&mut self, sp: usize)
{
  let  o = self.get::<f64>(sp);

  self.put(sp,o as i64);
}

pub fn
itof(&mut self, sp: usize)
{
  let  o = self.get::<i64>(sp);

  self.put(sp,o as f64);
}


pub fn
copy(&mut self, dst: usize, src: usize, sz: usize)
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





