

pub mod dictionary;
pub mod read_dictionary;
pub mod parse;


use super::source_file::SourceInfo;


pub struct
ParseSyntaxError
{
  source_info_opt: Option<SourceInfo>,

  message: String,

  child_opt: Option<Box<Self>>,

}


impl
ParseSyntaxError
{


pub fn
new(message: String)-> Self
{
  Self{
    source_info_opt: None,
    message,
    child_opt: None,
  }
}


pub fn
new_with_source_info(source_info: SourceInfo, message: String)-> Self
{
  Self{
    source_info_opt: Some(source_info),
    message,
    child_opt: None,
  }
}


pub fn
join(mut self, child: Self)-> Self
{
  self.child_opt = Some(Box::new(child));

  self
}


pub fn
print(&self)
{
    if let Some(info) = &self.source_info_opt
    {
      info.print();

      println!("");
    }


  println!("{}",self.message);

    if let Some(child) = &self.child_opt
    {
      child.print();
    }
}


}




