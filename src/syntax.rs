

pub mod dictionary;
pub mod read_dictionary;
pub mod parse;


use super::token::TokenInfo;


pub struct
ParseSyntaxError
{
  token_info_opt: Option<TokenInfo>,

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
    token_info_opt: None,
    message,
    child_opt: None,
  }
}


pub fn
new_with_token_info(token_info: TokenInfo, message: String)-> Self
{
  Self{
    token_info_opt: Some(token_info),
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
    if let Some(info) = &self.token_info_opt
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




