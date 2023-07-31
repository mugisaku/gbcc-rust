

use super::memory::{
  Memory,
  Word,
  WORD_SIZE,
  get_aligned,
};

use super::allocation::{
  AllocationKind,
  AllocationLink,
  Allocation,
};

use super::block::{
  Terminator,
  Block
};

use super::line::{
  Line,
};

use super::function::Function;






#[derive()]
pub struct
Collection
{
  pub(crate) allocation_list: Vec<Allocation>,
  pub(crate)   function_list: Vec<Function>,

  pub(crate) allocation_area_begin: usize,
  pub(crate) allocation_area_end: usize,

}


impl
Collection
{


pub fn
new()-> Collection
{
  Collection{
    allocation_list: Vec::new(),
      function_list: Vec::new(),

    allocation_area_begin: 0,
    allocation_area_end: 0,
  }
}


pub fn
add_allocation(&mut self, name: &str, sz: usize, mem_opt: Option<Memory>)-> AllocationLink
{
  let  i = self.allocation_list.len();

  let  ln = AllocationLink::Global(i);

  let  a = Allocation{
    name: String::from(name),
    size: sz,
    kind: AllocationKind::Global,
    memory_opt: mem_opt,
    user_count: 0,
    offset: 0
  };


  self.allocation_list.push(a);

  ln
}


pub fn
add_function(&mut self, f: Function)
{
  self.function_list.push(f);
}


pub fn
get_allocation(&self, i: usize)-> Option<&Allocation>
{
    if i < self.allocation_list.len()
    {
      return Some(&self.allocation_list[i]);
    }


  None
}


pub fn
find_allocation(&self, name: &str)-> Option<&Allocation>
{
    for alo in &self.allocation_list
    {
        if alo.name == name
        {
          return Some(alo);
        }
    }


  None
}


pub fn
get_function(&self, i: usize)-> Option<&Function>
{
    if i < self.function_list.len()
    {
      return Some(&self.function_list[i]);
    }


  None
}


fn
assign_allocation_offset(&mut self, start_off: usize)
{
  let  mut off = get_aligned(start_off);

  self.allocation_area_begin = off;

    for alo in &mut self.allocation_list
    {
//        if alo.user_count != 0
        {
          alo.offset = off;

          off = get_aligned(off+alo.size);
        }
    }


  self.allocation_area_end = off;
}


fn
process_functions_all(&mut self)-> Result<(),()>
{
  let  mut fname_ls: Vec<String> = Vec::new();

    for f in &self.function_list
    {
      fname_ls.push(f.name.clone());
    }


    for fi in 0..self.function_list.len()
    {
      let  f = &mut self.function_list[fi];

        if f.resolve_links_all(fi,&self.allocation_list,&fname_ls).is_err()
        {
          return Err(());
        }


      f.assign_allocation_offset();
    }


  Ok(())
}


pub fn
finalize(&mut self)-> Result<(),()>
{
    if self.process_functions_all().is_ok()
    {
      self.assign_allocation_offset(0);

      return Ok(());
    }


  Err(())
}


pub fn
print(&self)
{
    for alo in &self.allocation_list
    {
      alo.print();

      print!("\n");
    }


    for f in &self.function_list
    {
      f.print(self);

      print!("\n");
    }
}


}




