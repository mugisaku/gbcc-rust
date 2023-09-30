

use super::memory::{
  Memory,
  Word,
  WORD_SIZE,
  get_aligned,
};

use super::allocation::{
  AllocationID,
  Allocation,
  Source,
  Destination,
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
add_allocation(&mut self, name: &str, sz: usize, mem_opt: Option<Memory>)-> AllocationID
{
  let  i = self.allocation_list.len();

  let  id = AllocationID::Global(i);

  let  a = Allocation{
    name: String::from(name),
    size: sz,
    memory_opt: mem_opt,
    user_count: 0,
    offset: 0
  };


  self.allocation_list.push(a);

  id
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
get_allocation_by_id(&self, fi: usize, id: &AllocationID)-> Option<&Allocation>
{
    match id
    {
  AllocationID::Global(i)=>{self.get_allocation(*i)}
  AllocationID::Local(i)=>{self.get_local_allocation(fi,*i)}
  AllocationID::Parameter(i)=> {self.get_parameter(fi,*i)}
  _=>{None}
    }
}


pub fn
get_local_allocation(&self, fi: usize, i: usize)-> Option<&Allocation>
{
    if fi < self.function_list.len()
    {
      return self.function_list[fi].get_allocation(i);
    }


  None
}


pub fn
get_parameter(&self, fi: usize, i: usize)-> Option<&Allocation>
{
    if fi < self.function_list.len()
    {
      return self.function_list[fi].get_parameter(i);
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
find_allocation_index(&self, name: &str)-> Option<usize>
{
    for i in 0..self.allocation_list.len()
    {
        if self.allocation_list[i].name == name
        {
          return Some(i);
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


pub fn
find_function(&self, name: &str)-> Option<(&Function,usize)>
{
    for i in 0..self.function_list.len()
    {
      let  f = &self.function_list[i];

        if f.name == name
        {
          return Some((f,i));
        }
    }


  None
}


pub fn
finalize(&mut self)-> Result<(),()>
{
  self.allocation_area_begin = 0;

  self.allocation_area_end = Allocation::update_offsets(&mut self.allocation_list,self.allocation_area_begin);

  let  mut fname_ls: Vec<String> = Vec::new();

    for f in &self.function_list
    {
      fname_ls.push(f.name.clone());
    }


    for fi in 0..self.function_list.len()
    {
      let  f = &mut self.function_list[fi];

        if f.finalize(fi,&self.allocation_list,&fname_ls).is_err()
        {
          return Err(());
        }
    }


  Ok(())
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
      f.print();

      print!("\n");
    }
}


}




