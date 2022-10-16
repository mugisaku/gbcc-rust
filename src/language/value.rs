

use std::rc::Rc;
use super::element::Operand;


pub enum
Value
{
  Void,
  Undefined,
  Boolean(bool),
  Character(char),
  Integer(u64),
  SignedInteger(i64),
  Floating(f64),

}


impl
Value
{


pub fn
from(o: &Operand)-> Value
{
    match o
    {
  Operand::Integer(i)=>{return Value::Integer(*i);},
  Operand::Floating(f)=>{return Value::Floating(*f);},
  Operand::Character(c)=>{return Value::Character(*c);},
  Operand::Identifier(s)=>
        {
               if **s ==  "true"{return Value::Boolean(true);}
          else if **s == "false"{return Value::Boolean(false);}
        },
  _=>{},
    }


  return Value::Undefined;
}


pub fn
is_void(&self)-> bool
{
    if let Value::Void = self
    {
      return true;
    }


  false
}


pub fn
is_undefined(&self)-> bool
{
    if let Value::Undefined = self
    {
      return true;
    }


  false
}


pub fn
get_boolean(&self)-> Option<bool>
{
    if let Value::Boolean(b) = self
    {
      return Some(*b);
    }


  None
}


pub fn
get_character(&self)-> Option<char>
{
    if let Value::Character(c) = self
    {
      return Some(*c);
    }


  None
}


pub fn
get_integer(&self)-> Option<u64>
{
    if let Value::Integer(i) = self
    {
      return Some(*i);
    }


  None
}


pub fn
get_signed_integer(&self)-> Option<i64>
{
    if let Value::SignedInteger(i) = self
    {
      return Some(*i);
    }


  None
}


pub fn
get_floating(&self)-> Option<f64>
{
    if let Value::Floating(f) = self
    {
      return Some(*f);
    }


  None
}


pub fn
add(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l+r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l+r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Floating(l+r);
        }
    }


  Value::Undefined
}


pub fn
sub(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l-r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l-r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Floating(l-r);
        }
    }


  Value::Undefined
}


pub fn
mul(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l*r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l*r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Floating(l*r);
        }
    }


  Value::Undefined
}


pub fn
div(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l/r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l/r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Floating(l/r);
        }
    }


  Value::Undefined
}


pub fn
rem(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l%r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l%r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Floating(l%r);
        }
    }


  Value::Undefined
}


pub fn
shl(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l<<r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l<<r);
        }
    }


  Value::Undefined
}


pub fn
shr(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l>>r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l>>r);
        }
    }


  Value::Undefined
}


pub fn
or(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l|r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l|r);
        }
    }


  Value::Undefined
}


pub fn
and(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l&r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l&r);
        }
    }


  Value::Undefined
}


pub fn
xor(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Integer(l^r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::SignedInteger(l^r);
        }
    }


  Value::Undefined
}


pub fn
eq(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Boolean(l == r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::Boolean(l == r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Boolean(l == r);
        }
    }


  Value::Undefined
}


pub fn
neq(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Boolean(l != r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::Boolean(l != r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Boolean(l != r);
        }
    }


  Value::Undefined
}


pub fn
lt(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Boolean(l < r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::Boolean(l < r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Boolean(l < r);
        }
    }


  Value::Undefined
}


pub fn
lteq(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Boolean(l <= r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::Boolean(l <= r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Boolean(l <= r);
        }
    }


  Value::Undefined
}


pub fn
gt(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Boolean(l > r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::Boolean(l > r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Boolean(l > r);
        }
    }


  Value::Undefined
}


pub fn
gteq(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_integer()
    {
        if let Some(r) = rv.get_integer()
        {
          return Value::Boolean(l >= r);
        }
    }

  else
    if let Some(l) = lv.get_signed_integer()
    {
        if let Some(r) = rv.get_signed_integer()
        {
          return Value::Boolean(l >= r);
        }
    }

  else
    if let Some(l) = lv.get_floating()
    {
        if let Some(r) = rv.get_floating()
        {
          return Value::Boolean(l >= r);
        }
    }


  Value::Undefined
}


pub fn
logical_or(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_boolean()
    {
        if let Some(r) = rv.get_boolean()
        {
          return Value::Boolean(l || r);
        }
    }


  Value::Undefined
}


pub fn
logical_and(lv: &Value, rv: &Value)-> Value
{
    if let Some(l) = lv.get_boolean()
    {
        if let Some(r) = rv.get_boolean()
        {
          return Value::Boolean(l && r);
        }
    }


  Value::Undefined
}


pub fn
neg(v: &Value)-> Value
{
    if let Some(i) = v.get_signed_integer()
    {
      return Value::SignedInteger(-i);
    }

  else
    if let Some(f) = v.get_floating()
    {
      return Value::Floating(-f);
    }


  Value::Undefined
}


pub fn
not(v: &Value)-> Value
{
    if let Some(i) = v.get_integer()
    {
      return Value::Integer(!i);
    }

  else
    if let Some(i) = v.get_signed_integer()
    {
      return Value::SignedInteger(!i);
    }


  Value::Undefined
}


pub fn
logical_not(v: &Value)-> Value
{
    if let Some(b) = v.get_boolean()
    {
      return Value::Boolean(!b);
    }


  Value::Undefined
}


pub fn
print(&self)
{
    match self
    {
  Value::Void=>{print!("void");},
  Value::Undefined=>{print!("undefined");},
  Value::Boolean(b)=>{print!("{}",b);},
  Value::Character(c)=>{print!("{}",c);},
  Value::Integer(u)=>{print!("{}",u);},
  Value::SignedInteger(i)=>{print!("{}",i);},
  Value::Floating(f)=>{print!("{}",f);},
    }
}


}




