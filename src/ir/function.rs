

use super::memory::{
  get_aligned,
};

use super::allocation::{
  Allocation,
  AllocationKind,
  AllocationLink,
};

use super::line::{
  Line,
};

use super::block::{
  Terminator,
  Block,
};

use super::collection::{
  Collection,
};




pub enum
FunctionLink
{
  Unresolved(String),
    Resolved(usize),
}


impl
FunctionLink
{


pub fn
new(name: &str)-> FunctionLink
{
  FunctionLink::Unresolved(String::from(name))
}


pub fn
resolve(&mut self, f_ls: &Vec<Function>)-> Result<(),()>
{
    if let FunctionLink::Unresolved(this_name) = self
    {
        for i in 0..f_ls.len()
        {
            if f_ls[i].name == this_name.as_str()
            {
              *self = FunctionLink::Resolved(i);

              return Ok(());
            }
        }
    }


  Err(())
}


pub fn
print(&self, coll: &Collection)
{
    match self
    {
  FunctionLink::Unresolved(name)=>{print!("{}",name);},
  FunctionLink::Resolved(i)=>
        {
            if let Some(f) = coll.get_function(*i)
            {
              print!("{}",&f.name);
            }
        },
    }
}


}




pub struct
Function
{
  pub(crate) name: String,

  pub(crate) parameter_list: Vec<Allocation>,

  pub(crate) return_size: usize,

  pub(crate) block_list: Vec<Block>,

  pub(crate) allocation_list: Vec<Allocation>,

}


impl
Function
{


pub fn
new(name: &str, sz: usize)-> Function
{
  Function{
    name: String::from(name),
    return_size: sz,
    parameter_list: Vec::new(),
    block_list: Vec::new(),
    allocation_list: Vec::new(),
  }
}


pub fn
add_block(&mut self, blk: Block)
{
  self.block_list.push(blk);
}


pub fn
add_allocation(&mut self, name: &str, sz: usize)
{
  self.allocation_list.push(Allocation::new_local(name,sz));
}


pub fn
add_parameter(&mut self, name: &str, sz: usize)
{
  let  i = self.parameter_list.len();

  self.parameter_list.push(Allocation::new_parameter(i,name,sz));
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
get_parameter(&self, i: usize)-> Option<&Allocation>
{
    if i < self.parameter_list.len()
    {
      return Some(&self.parameter_list[i]);
    }


  None
}


pub fn
get_allocation_size(&self)-> usize
{
    if let Some(alo) = self.allocation_list.last()
    {
      return get_aligned(alo.offset+alo.size)
    }


  0
}


pub fn
find_block(&self, name: &str)-> Option<&Block>
{
    for blk in &self.block_list
    {
        if blk.name == name
        {
          return Some(blk);
        }
    }


  None
}


fn
resolve_block_links_all(&mut self)-> Result<(),()>
{
  let  mut ls: Vec<String> = Vec::new();

    for blk in &mut self.block_list
    {
      ls.push(blk.name.clone());
    }


    for blk in &mut self.block_list
    {
        if blk.resolve_block_link(&ls).is_err()
        {
          return Err(());
        }
    }


  Ok(())
}


fn
build_local_allocation(&mut self)
{
  let  mut ls: Vec<(String,usize)> = Vec::new();

    for blk in &self.block_list
    {
        for ln in &blk.line_list
        {
            if let Some((name,size)) = ln.get_allocation_data()
            {
              ls.push((name,size));
            }
        }
    }


  self.allocation_list.clear();

    for e in ls
    {
      self.add_allocation(&e.0,e.1);
    }
}


fn
resolve_other_links_all(&mut self, fi: usize, g_alo_ls: &Vec<Allocation>, fname_ls: &Vec<String>)-> Result<(),()>
{
    for blk in &mut self.block_list
    {
        if blk.resolve(fi,&self.parameter_list,&self.allocation_list,g_alo_ls,fname_ls).is_err()
        {
          println!("function::resolve_other_links_all error");

          return Err(());
        }
    }


  Ok(())
}


pub fn
assign_allocation_offset(&mut self)
{
  let  mut off: usize = 0;

    for alo in &mut self.parameter_list
    {
      off += get_aligned(alo.size);

      alo.offset = off;
    }


  off = 128;

    for alo in &mut self.allocation_list
    {
//        if alo.user_count != 0
        {
          alo.offset = off                            ;
                       off = get_aligned(off+alo.size);
        }
    }
}


pub fn
resolve_links_all(&mut self, fi: usize, g_alo_ls: &Vec<Allocation>, fname_ls: &Vec<String>)-> Result<(),()>
{
  self.build_local_allocation();

    if self.resolve_block_links_all().is_ok()
    && self.resolve_other_links_all(fi,g_alo_ls,fname_ls).is_ok()
    {
      return Ok(());
    }


  Err(())
}


pub fn
print(&self, coll: &Collection)
{
  print!("fn\n{}(",&self.name);

    for p in &self.parameter_list
    {
      p.print();

      print!(",");
    }


  print!(")-> {}",self.return_size);

  print!("\n{{\n");

    for blk in &self.block_list
    {
      blk.print(coll);
    }


  print!("\n}}\n");
}


}





