

use super::memory::{
  get_aligned,
};

use super::allocation::{
  Allocation,
  AllocationID,
};

use super::line::{
  Line,
};

use super::collection::{
  DestinationTable,
  Collection,
};

use super::executor::{
  Executor,
};




#[derive(Clone)]
pub struct
Function
{
  pub(crate) name: String,

  pub(crate) parameter_list: Vec<Allocation>,

  pub(crate) return_size: usize,

  pub(crate) line_list: Vec<Line>,

  pub(crate) allocation_list: Vec<Allocation>,

  pub(crate) parameter_stack_size: usize,
  pub(crate)     local_stack_size: usize,

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
    parameter_stack_size: 0,
        local_stack_size: 0,
  }
}


pub fn
set_line_list(&mut self, ln_ls: Vec<Line>)
{
  self.line_list = ln_ls;
}


pub fn
add_line_list(&mut self, mut ln_ls: Vec<Line>)
{
  self.line_list.append(&mut ln_ls);
}


pub fn
add_allocation(&mut self, name: &str, sz: usize)
{
  self.allocation_list.push(Allocation::new(name,sz));
}


pub fn
add_parameter(&mut self, name: &str, sz: usize)
{
  self.parameter_list.push(Allocation::new(name,sz));
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
get_parameter(&self, i: usize)-> Option<&Allocation>
{
    if i < self.parameter_list.len()
    {
      return Some(&self.parameter_list[i]);
    }


  None
}


pub fn
find_parameter_index(&self, name: &str)-> Option<usize>
{
    for i in 0..self.parameter_list.len()
    {
        if self.parameter_list[i].name == name
        {
          return Some(i);
        }
    }


  None
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


  self.parameter_stack_size = Allocation::update_offsets_neg(&mut self.parameter_list,0);
  self.local_stack_size     = Allocation::update_offsets(    &mut self.allocation_list,Executor::SYSTEM_RESERVED_STACK_SIZE);
}


pub fn
get_label_info_list(&self)-> Vec<(String,usize)>
{
  let  mut ls: Vec<(String,usize)> = Vec::new();

    for i in 0..self.line_list.len()
    {
        if let Line::Label(name) = &self.line_list[i]
        {
          ls.push((name.clone(),i));
        }
    }


   ls
}


pub fn
finalize(&mut self, fi: usize, g_alo_ls: &Vec<Allocation>, fname_ls: &Vec<String>)-> Result<(),()>
{
  self.build_local_allocation();

  let  lb_ls = self.get_label_info_list();

  let  mut tbl = DestinationTable::new(g_alo_ls);

    for ln in &mut self.line_list
    {
        if ln.link_to_function(fname_ls).is_err()
        {
          println!("function::finalize error");

          return Err(());
        }


        if ln.link_to_label(&lb_ls).is_err()
        {
          println!("function::finalize error");

          return Err(());
        }


      tbl.update_local_list(&self.allocation_list);
      tbl.update_parameter_list(&self.parameter_list);

        if ln.link_to_allocation(&tbl).is_err()
        {
          println!("function::finalize error");

          return Err(());
        }
    }


  Ok(())
}


pub fn
print(&self)
{
  print!("{}(",&self.name);
 
    for p in &self.parameter_list
    {
      print!("{},",&p.name);
    }


  print!(")-> {}",self.return_size);

  print!("\n{{\n");

    for ln in &self.line_list
    {
      ln.print();

      print!("\n");
    }


  print!("\n}}\n");
}




}




impl
core::ops::AddAssign<Line> for Function
{


fn
add_assign(&mut self, ln: Line)
{
  self.line_list.push(ln);
}


}


impl
core::ops::AddAssign<Line> for Vec<Line>
{


fn
add_assign(&mut self, ln: Line)
{
  self.push(ln);
}


}




