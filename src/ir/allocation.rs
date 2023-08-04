

use super::collection::Collection;
use super::memory::Memory;

#[derive(Clone)]
pub enum
AllocationLink
{
  Unresolved(String),

     Global(usize),
      Local(usize,usize),
  Parameter(usize,usize),

}


impl
AllocationLink
{


pub fn
new(name: &str)-> AllocationLink
{
  AllocationLink::Unresolved(String::from(name))
}


fn
find_allocation(ls: &Vec<Allocation>, name: &str)-> Option<usize>
{
    for i in 0..ls.len()
    {
        if ls[i].name == name
        {
          return Some(i);
        }
    }


  None
}


pub fn
resolve(&mut self, fi: usize, p_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, g_alo_ls: &Vec<Allocation>)-> Result<(),()>
{
  let  mut new_self_opt: Option<Self> = None;

    if let AllocationLink::Unresolved(name) = self
    {
        if let Some(i) = Self::find_allocation(p_alo_ls,name)
        {
          new_self_opt = Some(AllocationLink::Parameter(fi,i));
        }

      else
        if let Some(i) = Self::find_allocation(l_alo_ls,name)
        {
          new_self_opt = Some(AllocationLink::Local(fi,i));
        }

      else
        if let Some(i) = Self::find_allocation(g_alo_ls,name)
        {
          new_self_opt = Some(AllocationLink::Global(i));
        }

      else
        {
          println!("AllocationLink::resolve error: {} is not found",name);

          return Err(());
        }
    }


    if let Some(new_self) = new_self_opt
    {
      *self = new_self;
    }


  Ok(())
}


pub fn
print(&self, coll: &Collection, verbose: usize)
{
    match self
    {
  AllocationLink::Unresolved(name)=>{print!("{}(UNRESOLVED)",name);},
  AllocationLink::Global(i)=>
        {
            if let Some(alo) = coll.get_allocation(*i)
            {
              alo.print(verbose);
            }
        },
  AllocationLink::Local(fi,i)=>
        {
            if let Some(f) = coll.get_function(*fi)
            {
                if let Some(alo) = f.get_allocation(*i)
                {
                  alo.print(verbose);
                }
            }
        },
  AllocationLink::Parameter(fi,i)=>
        {
            if let Some(f) = coll.get_function(*fi)
            {
                if let Some(alo) = f.get_parameter(*i)
                {
                  alo.print(verbose);
                }
            }
        },
    }
}


}




pub enum
AllocationKind
{
  Global,
  Local,
  Parameter,

}


#[derive()]
pub struct
Allocation
{
  pub(crate) name: String,

  pub(crate) size: usize,

  pub(crate) kind: AllocationKind,

  pub(crate) memory_opt: Option<Memory>,

  pub(crate) user_count: usize,

  pub(crate) offset: usize,

}


impl
Allocation
{


pub fn
new_parameter(name: &str, sz: usize)-> Allocation
{
  Allocation{
    name: String::from(name),
    size: sz,
    kind: AllocationKind::Parameter,
    memory_opt: None,
    user_count: 0,
    offset: 0,
  }
}


pub fn
new_local(name: &str, sz: usize)-> Allocation
{
  Allocation{
    name: String::from(name),
    size: sz,
    kind: AllocationKind::Local,
    memory_opt: None,
    user_count: 0,
    offset: 0,
  }
}


pub fn
new_global(name: &str, sz: usize)-> Allocation
{
  Allocation{
    name: String::from(name),
    size: sz,
    kind: AllocationKind::Global,
    memory_opt: None,
    user_count: 0,
    offset: 0,
  }
}


pub fn
print(&self, verbose: usize)
{
  print!("{}",&self.name);

    if verbose > 0
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




