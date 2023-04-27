

use super::{
  TypeNote,
  get_max,
  get_aligned_size,
};

use super::r#struct::{
  Member,
  print_member_list,
};


#[derive(Clone)]
pub struct
Union
{
  member_list: Vec<Member>,

   size: Option<usize>,
  align: Option<usize>,

}


impl
Union
{


pub fn
new()-> Union
{
  Union{ member_list: Vec::new(), size: None, align: None}
}


pub fn
push(&mut self, name: &str, t: TypeNote)
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
  let  mut sz: usize = 0;
  let  mut al: usize = 0;

    for m in &self.member_list
    {
        if let Some(m_sz) = m.type_note.get_size() {
        if let Some(m_al) = m.type_note.get_align(){
          sz = get_max(sz,get_aligned_size(m_sz));
          al = get_max(al,m_al);

          continue;
        }}


      self.size  = None;
      self.align = None;

      return Err(());
    }


  self.size  = Some(sz);
  self.align = Some(al);

  Ok(())
}


pub fn   get_size(&self)-> &Option<usize>{&self.size}
pub fn  get_align(&self)-> &Option<usize>{&self.align}


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
  print!("{{");

    for m in &self.member_list
    {
      m.print();
      println!(",");
    }


  print!("}}");
}


}




