

use super::{
  TypeNote,
  get_aligned_size,
  get_max,
};


#[derive(Clone)]
pub struct
Member
{
  pub(crate) name: String,

  pub(crate) type_note: TypeNote,

  pub(crate) offset: Option<usize>,

}


impl
Member
{


pub fn
print(&self)
{
    if self.name.len() != 0
    {
      print!("{}: ",&self.name);
    }


  self.type_note.print();

  print!("(off: ");

    if let Some(off) = self.offset
    {
      print!("{}",off);
    }


  print!(")");
}


}


pub fn
print_member_list(ls: &Vec<Member>)
{
    for m in ls
    {
      m.print();

      println!(",");
    }
}




#[derive(Clone)]
pub struct
Struct
{
  pub(crate) member_list: Vec<Member>,

  pub(crate)  size: Option<usize>,
  pub(crate) align: Option<usize>,

}


impl
Struct
{


pub fn
new()-> Struct
{
  Struct{member_list: Vec::new(), size: None, align: None}
}


pub fn
push(&mut self, m: Member)
{
  self.member_list.push(m);
}


pub fn
add(&mut self, name: &str, t: TypeNote)
{
  self.member_list.push(Member{ name: String::from(name), type_note: t, offset: None});
}


pub fn
merge(&mut self, ls: Vec<Member>)
{
    for m in ls
    {
      self.member_list.push(m);
    }
}


pub fn
fix(&mut self)-> Result<(),()>
{
  let  mut off: usize = 0;
  let  mut  al: usize = 0;

    for m in &mut self.member_list
    {
      m.offset = Some(off);

        if let Some(m_sz) = m.type_note.get_size() {
        if let Some(m_al) = m.type_note.get_align(){
          off = get_aligned_size(off+m_sz);
           al = get_max(al,m_al);

          continue;
        }}


      self.size  = None;
      self.align = None;

      return Err(());
    }


  self.size  = Some(off);
  self.align = Some(al);

  Ok(())
}


pub fn   get_size(&self)-> &Option<usize>{&self.size}
pub fn  get_align(&self)-> &Option<usize>{&self.align}

pub fn  get_member_list(&self)-> &Vec<Member>{&self.member_list}


pub fn
find(&self, name: &str)-> Option<&Member>
{
    for m in &self.member_list
    {
        if m.name == name
        {
          return Some(&m);
        }
    }


  None
}


pub fn
get(&self, i: usize)-> Option<&Member>
{
    if i < self.member_list.len()
    {
      return Some(&self.member_list[i]);
    }


  None
}


pub fn
print_id(&self, buf: &mut String)
{
    for m in &self.member_list
    {
//      m.type_info.print_id(buf);
    }
}


pub fn
print(&self)
{
  print!("struct{{");

  print_member_list(&self.member_list);

  print!("}}");
}


}





