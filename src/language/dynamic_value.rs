

use super::dynamic_machine::{
  Operation,

};




#[derive(Clone)]
pub struct
Element
{
  pub(crate) name: String,
  pub(crate) heap_reference: usize,

}


impl
Element
{


pub fn
new(name: &str, r: usize)-> Self
{
  Self{
    name: name.to_string(),
    heap_reference: r,
  }
}


}




#[derive(Clone)]
pub enum
Value
{
  Null,
  Undefined,
  Boolean(bool),
  Integer(i64),
  Floating(f64),
  String(String),

  HeapReference(usize),
  StackReference(usize),
  ProgramReference(usize),

  Mutable(Box<Value>),

  Table(Vec<Element>),

  BasePointer(usize),
  ProgramPointer(*const Vec<Operation>),
  ProgramCounter(usize),
  ArgumentCounter(usize),

}


impl
Value
{


pub fn
to_bool(&self)-> bool
{
    match self
    {
  Value::Boolean(b)=>{return *b;}
  Value::Integer(i)=>{return *i != 0;}
  Value::Floating(f)=>{return *f != 0.0;}
  Value::Mutable(v)=>{return v.to_bool();}
  _=>{}
    }


  false
}


pub fn
to_int(&self)-> i64
{
    match self
    {
  Value::Boolean(b)=>{return if *b{1} else{0};}
  Value::Integer(i)=>{return *i;}
  Value::Floating(f)=>{return *f as i64;}
  Value::Mutable(v)=>{return v.to_int();}
  _=>{}
    }


  0
}


pub fn
to_float(&self)-> f64
{
    match self
    {
  Value::Integer(i)=>{return *i as f64;}
  Value::Floating(f)=>{return *f;}
  Value::Mutable(v)=>{return v.to_float();}
  _=>{}
    }


  0.0
}


pub fn
to_string(&self)-> String
{
    match self
    {
  Value::Boolean(b)=>{return if *b{"true".to_string()} else{"false".to_string()};}
  Value::Integer(i)=>{return format!("{}",*i);}
  Value::Floating(f)=>{return format!("{}",*f);}
  Value::String(s)=>{return s.clone();}
  Value::Mutable(v)=>{return v.to_string();}
  _=>{}
    }


  "".to_string()
}


pub fn
to_program_reference(&self)-> usize
{
    if let Value::ProgramReference(i) = self
    {
      return *i;
    }


  0
}


pub fn
add(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li+ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Floating(lf+rf)
    }

  else
    if let Value::String(ls) = lv
    {
      let  rs = rv.to_string();

      Value::String(format!("{}{}",ls,&rs))
    }

  else
    {
      Value::Undefined
    }
}


pub fn
sub(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li-ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Floating(lf-rf)
    }

  else
    {
      Value::Undefined
    }
}


pub fn
mul(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li*ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Floating(lf*rf)
    }

  else
    {
      Value::Undefined
    }
}


pub fn
div(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li/ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Floating(lf/rf)
    }

  else
    {
      Value::Undefined
    }
}


pub fn
rem(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li%ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Floating(lf%rf)
    }

  else
    {
      Value::Undefined
    }
}


pub fn
shl(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li<<ri)
    }

  else
    {
      Value::Undefined
    }
}


pub fn
shr(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li>>ri)
    }

  else
    {
      Value::Undefined
    }
}


pub fn
and(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li&ri)
    }

  else
    {
      Value::Undefined
    }
}


pub fn
or(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li|ri)
    }

  else
    {
      Value::Undefined
    }
}


pub fn
xor(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Integer(li^ri)
    }

  else
    {
      Value::Undefined
    }
}


pub fn
neg(v: &Value)-> Value
{
  let  i = v.to_int();

  Value::Integer(-i)
}


pub fn
not(v: &Value)-> Value
{
  let  i = v.to_int();

  Value::Integer(!i)
}


pub fn
logical_not(v: &Value)-> Value
{
  let  b = v.to_bool();

  Value::Boolean(!b)
}


pub fn
logical_and(lv: &Value, rv: &Value)-> Value
{
  let  l = lv.to_bool();
  let  r = rv.to_bool();

  Value::Boolean(l && r)
}


pub fn
logical_or(lv: &Value, rv: &Value)-> Value
{
  let  l = lv.to_bool();
  let  r = rv.to_bool();

  Value::Boolean(l || r)
}


pub fn
eq(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Boolean(li == ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Boolean(lf == rf)
    }

  else
    if let Value::String(ls) = lv
    {
      let  rs = rv.to_string();

      Value::Boolean(ls == &rs)
    }

  else
    if let Value::ProgramReference(lf) = *lv
    {
      let  rf = rv.to_program_reference();

      Value::Boolean(&lf == &rf)
    }

  else
    {
      Value::Boolean(false)
    }
}


pub fn
neq(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Boolean(li != ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Boolean(lf != rf)
    }

  else
    if let Value::String(ls) = lv
    {
      let  rs = rv.to_string();

      Value::Boolean(ls != &rs)
    }

  else
    if let Value::ProgramReference(lf) = *lv
    {
      let  rf = rv.to_program_reference();

      Value::Boolean(&lf != &rf)
    }

  else
    {
      Value::Boolean(false)
    }
}


pub fn
lt(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Boolean(li < ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Boolean(lf < rf)
    }

  else
    {
      Value::Boolean(false)
    }
}


pub fn
lteq(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Boolean(li <= ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Boolean(lf <= rf)
    }

  else
    {
      Value::Boolean(false)
    }
}


pub fn
gt(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = *lv
    {
      let  ri = rv.to_int();

      Value::Boolean(li > ri)
    }

  else
    if let Value::Floating(lf) = *lv
    {
      let  rf = rv.to_float();

      Value::Boolean(lf > rf)
    }

  else
    {
      Value::Boolean(false)
    }
}


pub fn
gteq(lv: &Value, rv: &Value)-> Value
{
    if let Value::Integer(li) = lv
    {
      let  ri = rv.to_int();

      Value::Boolean(*li >= ri)
    }

  else
    if let Value::Floating(lf) = lv
    {
      let  rf = rv.to_float();

      Value::Boolean(*lf >= rf)
    }

  else
    {
      Value::Boolean(false)
    }
}


}




