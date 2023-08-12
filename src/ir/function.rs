

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

  pub(crate) line_list: Vec<Line>,

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
    line_list: Vec::new(),
    allocation_list: Vec::new(),
  }
}


pub fn
add_line(&mut self, ln: Line)
{
  self.line_list.push(ln);
}


pub fn
add_allocation(&mut self, name: &str, sz: usize)
{
  self.allocation_list.push(Allocation::new_local(name,sz));
}


pub fn
add_parameter(&mut self, name: &str, sz: usize)
{
  self.parameter_list.push(Allocation::new_parameter(name,sz));
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
get_block_name(&self, i: usize)-> Option<&String>
{
    if i < self.line_list.len()
    {
        if let Line::BlockOpen(name) = &self.line_list[i]
        {
          return Some(name);
        }
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


fn
build_local_allocation(&mut self)
{
  let  mut ls: Vec<(String,usize)> = Vec::new();

    for ln in &self.line_list
    {
        if let Some((name,size)) = ln.get_allocation_data()
        {
          ls.push((name,size));
        }
    }


  self.allocation_list.clear();

    for e in ls
    {
      self.add_allocation(&e.0,e.1);
    }
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
resolve(&mut self, fi: usize, g_alo_ls: &Vec<Allocation>, fname_ls: &Vec<String>)-> Result<(),()>
{
  self.build_local_allocation();

  let  mut blkop_ls: Vec<(String,usize)> = Vec::new();

    for i in 0..self.line_list.len()
    {
        if let Line::BlockOpen(name) = &self.line_list[i]
        {
          blkop_ls.push((name.clone(),i));
        }
    }


    for ln in &mut self.line_list
    {
        if ln.resolve(fi,&blkop_ls,&self.parameter_list,&self.allocation_list,g_alo_ls,fname_ls).is_err()
        {
          println!("function::resolve_other_links_all error");

          return Err(());
        }
    }


  Ok(())
}


pub fn
print(&self, coll: &Collection)
{
  print!("fn\n{}(",&self.name);

    for p in &self.parameter_list
    {
      p.print(1);

      print!(",");
    }


  print!(")-> {}",self.return_size);

  print!("\n{{\n");

    for ln in &self.line_list
    {
      ln.print(coll,self);

      print!("\n");
    }


  print!("\n}}\n");
}


}





