

use super::expression::Expression;


#[derive(Clone,Copy)]
pub struct
ExpressionIndex
{
  pub(crate) value: usize,
}


#[derive(Clone,Copy)]
pub struct
StringIndex
{
  pub(crate) value: usize,
}


pub struct
Library
{
  pub(crate) expression_list: Vec<Expression>,
  pub(crate)     string_list: Vec<String>,

}


impl
Library
{


pub fn 
new()-> Library
{
  Library{
    expression_list: Vec::new(),
        string_list: Vec::new(),
  }
}


pub fn
push_expression(&mut self, e: Expression)-> ExpressionIndex
{
  let  i = self.expression_list.len();

  self.expression_list.push(e);

  ExpressionIndex{value: i}
}


pub fn
get_expression(&self, i: ExpressionIndex)-> Option<&Expression>
{
    if i.value < self.expression_list.len()
    {
      return Some(&self.expression_list[i.value]);
    }


  None
}


pub fn
push_string(&mut self, s: String)-> StringIndex
{
  let  i = self.string_list.len();

  self.string_list.push(s);

  StringIndex{value: i}
}


pub fn
get_string(&self, i: StringIndex)-> Option<&String>
{
    if i.value < self.string_list.len()
    {
      return Some(&self.string_list[i.value]);
    }


  None
}




}





