

use super::collection::{
  DestinationTable,
  Collection,
};


use super::memory::{
  Memory,
  get_aligned,
};




#[derive(Clone,Copy)]
pub enum
AllocationID
{
     Global(usize),
      Local(usize),
  Parameter(usize),

}


pub trait
Operand
{


fn  new(name: &str)-> Self;
fn  get_name(&self)-> &String;
fn  get_id(&self)-> AllocationID;
fn  get_offset(&self)-> usize;
fn  set_data(&mut self, id: AllocationID, off: usize);


fn
link(&mut self, tbl: &DestinationTable)-> Result<(),()>
{
  let  name = self.get_name();

    if let Some(dst) = tbl.find(name)
    {
      self.set_data(dst.id,dst.offset);

      Ok(())
    }

  else
    {
      print!("Operand::link error: {} is not found",name);

      Err(())
    }
}


}




#[derive(Clone)]
pub struct
Source
{
  pub(crate)   name: String,
  pub(crate)     id: AllocationID,
  pub(crate) offset: usize,

}


impl
Operand for Source
{


fn
new(name: &str)-> Self
{
  Self{
      name: String::from(name),
        id: AllocationID::Global(0),
    offset: 0,
  }
}


fn  get_name(&self)-> &String{&self.name}
fn  get_id(&self)-> AllocationID{self.id}
fn  get_offset(&self)-> usize{self.offset}


fn
set_data(&mut self, id: AllocationID, off: usize)
{
  self.id     =  id;
  self.offset = off;
}


}


pub fn  new_operand_list()-> Vec<Source>{Vec::new()}


#[derive(Clone)]
pub struct
Destination
{
  pub(crate)   name: String,
  pub(crate)     id: AllocationID,
  pub(crate) offset: usize,

}


impl
Operand for Destination
{


fn
new(name: &str)-> Self
{
  Self{
      name: String::from(name),
        id: AllocationID::Global(0),
    offset: 0,
  }
}


fn  get_name(&self)-> &String{&self.name}
fn  get_id(&self)-> AllocationID{self.id}
fn  get_offset(&self)-> usize{self.offset}


fn
set_data(&mut self, id: AllocationID, off: usize)
{
  self.id     =  id;
  self.offset = off;
}


}




pub struct
SourceListWrapper
{
  pub(crate) list: Vec<Source>,

}


impl
SourceListWrapper
{


pub fn
add(mut self, name: &str)-> SourceListWrapper
{
  self.list.push(Source::new(name));

  self
}


pub fn
release(mut self)-> Vec<Source>
{
  self.list
}


}


pub fn  build_args()-> SourceListWrapper{SourceListWrapper{list: Vec::new()}}


#[derive(Clone)]
pub struct
Allocation
{
  pub(crate) name: String,

  pub(crate) size: usize,

  pub(crate) memory_opt: Option<Memory>,

  pub(crate) user_count: usize,

  pub(crate) offset: usize,

}


impl
Allocation
{


pub fn
new(name: &str, sz: usize)-> Allocation
{
  Allocation{
    name: String::from(name),
    size: sz,
    memory_opt: None,
    user_count: 0,
    offset: 0,
  }
}


pub fn
find_index_and_offset(ls: &Vec<Allocation>, name: &str)-> Option<(usize,usize)>
{
    for i in 0..ls.len()
    {
        if ls[i].name == name
        {
          return Some((i,ls[i].offset));
        }
    }


  None
}


pub fn
update_offsets(ls: &mut Vec<Allocation>, start: usize)-> usize
{
  let  mut off: usize = get_aligned(start);

    for alo in ls
    {
//        if alo.user_count != 0
        {
          alo.offset = off                            ;
                       off = get_aligned(off+alo.size);
        }
    }


  off
}


pub fn
update_offsets_neg(ls: &mut Vec<Allocation>, start: usize)-> usize
{
  let  mut off: usize = get_aligned(start);

    for alo in ls
    {
                   off += get_aligned(alo.size);
      alo.offset = off                         ;
    }


  off
}


pub fn
print(&self)
{
  print!("{}",&self.name);

//    if verbose > 0
    {
      print!("(off: {}, sz:{})",self.offset,self.size);

        if let Some(m) = &self.memory_opt
        {
          print!(" = {{");

          m.print();

          print!("}}");
        }
    }
}


}




