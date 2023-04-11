

use super::value::Value;

use super::expression::{
   PrefixOperator,
  PostfixOperator,
  BinaryOperator,
  Operand,
  Expression,
};




pub fn
operate_prefix_constant(v: &Value, p: &PrefixOperator)-> Value
{
    match p
    {
  PrefixOperator::Neg=>
        {
            match v
            {
          Value::I8(i)=>{Value::I8(-i)}
          Value::I16(i)=>{Value::I16(-i)}
          Value::I32(i)=>{Value::I32(-i)}
          Value::I64(i)=>{Value::I64(-i)}
          Value::F32(f)=>{Value::F32(-f)}
          Value::F64(f)=>{Value::F64(-f)}
          _=>{Value::Undefined}
            }
        },
  PrefixOperator::Not=>
        {
            match v
            {
          Value::I8(i)=>{Value::I8(!i)}
          Value::I16(i)=>{Value::I16(!i)}
          Value::I32(i)=>{Value::I32(!i)}
          Value::I64(i)=>{Value::I64(!i)}
          Value::U8(u)=>{Value::U8(!u)}
          Value::U16(u)=>{Value::U16(!u)}
          Value::U32(u)=>{Value::U32(!u)}
          Value::U64(u)=>{Value::U64(!u)}
          _=>{Value::Undefined}
            }
        },
  PrefixOperator::LogicalNot=>
        {
            match v
            {
          Value::Bool(b)=>{Value::Bool(!b)}
          _=>{Value::Undefined}
            }
        },
/*
  PrefixOperator::Address=>{},
  PrefixOperator::Dereference=>{},
  PrefixOperator::PrefixIncrement=>{},
  PrefixOperator::PrefixDecrement=>{},
*/
  _=>{Value::Undefined},
    }
}


pub fn
operate_postfix_constant(v: &Value, p: &PostfixOperator)-> Value
{
    match p
    {
  PostfixOperator::Access(s)=>
        {
                 if *s == "as_i8" {if let Some(i) = v.to_i8(){return Value::I8(i);}}
            else if *s == "as_i16"{if let Some(i) = v.to_i16(){return Value::I16(i);}}
            else if *s == "as_i32"{if let Some(i) = v.to_i32(){return Value::I32(i);}}
            else if *s == "as_i64"{if let Some(i) = v.to_i64(){return Value::I64(i);}}
            else if *s == "as_u8" {if let Some(u) = v.to_u8(){return Value::U8(u);}}
            else if *s == "as_u16"{if let Some(u) = v.to_u16(){return Value::U16(u);}}
            else if *s == "as_u32"{if let Some(u) = v.to_u32(){return Value::U32(u);}}
            else if *s == "as_u64"{if let Some(u) = v.to_u64(){return Value::U64(u);}}
            else if *s == "as_f32"{if let Some(f) = v.to_f32(){return Value::F32(f);}}
            else if *s == "as_f64"{if let Some(f) = v.to_f64(){return Value::F64(f);}}
            else if *s == "as_bool"{if let Some(b) = v.to_bool(){return Value::Bool(b);}}


          return Value::Undefined;
        },
/*
  PostfixOperator::Subscript(o)=>{},
  PostfixOperator::Call(args)=>{},
*/
  _=>{Value::Undefined}
    }
}




pub fn
get_i64(l: &Value, r: &Value)-> Option<(i64,i64)>
{
    if let Some(li) = l.get_i64()
    {
        if let Some(ri) = r.get_i64()
        {
          return Some((li,ri));
        }
    }


  None
}


pub fn
get_u64(l: &Value, r: &Value)-> Option<(u64,u64)>
{
    if let Some(lu) = l.get_u64()
    {
        if let Some(ru) = r.get_u64()
        {
          return Some((lu,ru));
        }
    }


  None
}


pub fn
get_f64(l: &Value, r: &Value)-> Option<(f64,f64)>
{
    if let Some(lf) = l.get_f64()
    {
        if let Some(rf) = r.get_f64()
        {
          return Some((lf,rf));
        }
    }


  None
}


pub fn
get_bool(l: &Value, r: &Value)-> Option<(bool,bool)>
{
    if let Value::Bool(lb) = l
    {
        if let Value::Bool(rb) = r
        {
          return Some((*lb,*rb));
        }
    }


  None
}


pub fn
operate_binary_constant(l: &Value, r: &Value, b: &BinaryOperator)-> Value
{
    if l.get_id() != r.get_id()
    {
      println!("");

      return Value::Undefined;
    }


    match b
    {
  BinaryOperator::Add=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo+ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo+ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::F64(lo+ro);}
        },
  BinaryOperator::Sub=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo-ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo-ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::F64(lo-ro);}
        },
  BinaryOperator::Mul=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo*ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo*ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::F64(lo*ro);}
        },
  BinaryOperator::Div=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo/ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo/ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::F64(lo/ro);}
        },
  BinaryOperator::Rem=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo%ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo%ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::F64(lo%ro);}
        },
  BinaryOperator::Shl=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo<<ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo<<ro);}
        },
  BinaryOperator::Shr=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo>>ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo>>ro);}
        },
  BinaryOperator::And=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo&ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo&ro);}
        },
  BinaryOperator::Or=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo|ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo|ro);}
        },
  BinaryOperator::Xor=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::I64(lo^ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::U64(lo^ro);}
        },
  BinaryOperator::Eq=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::Bool(lo == ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::Bool(lo == ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::Bool(lo == ro);}
        },
  BinaryOperator::Neq=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::Bool(lo != ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::Bool(lo != ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::Bool(lo != ro);}
        },
  BinaryOperator::Lt=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::Bool(lo < ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::Bool(lo < ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::Bool(lo < ro);}
        },
  BinaryOperator::Lteq=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::Bool(lo <= ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::Bool(lo <= ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::Bool(lo <= ro);}
        },
  BinaryOperator::Gt=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::Bool(lo > ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::Bool(lo > ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::Bool(lo > ro);}
        },
  BinaryOperator::Gteq=>
        {
               if let Some((lo,ro)) = get_i64(l,r){return Value::Bool(lo >= ro);}
          else if let Some((lo,ro)) = get_u64(l,r){return Value::Bool(lo >= ro);}
          else if let Some((lo,ro)) = get_f64(l,r){return Value::Bool(lo >= ro);}
        },
  BinaryOperator::LogicalAnd=>
        {
            if let Some((lo,ro)) = get_bool(l,r){return Value::Bool(lo && ro);}
        },
  BinaryOperator::LogicalOr=>
        {
            if let Some((lo,ro)) = get_bool(l,r){return Value::Bool(lo || ro);}
        },
  BinaryOperator::Assign=>{},
  BinaryOperator::AddAssign=>{},
  BinaryOperator::SubAssign=>{},
  BinaryOperator::MulAssign=>{},
  BinaryOperator::DivAssign=>{},
  BinaryOperator::RemAssign=>{},
  BinaryOperator::ShlAssign=>{},
  BinaryOperator::ShrAssign=>{},
  BinaryOperator::AndAssign=>{},
  BinaryOperator::OrAssign=>{},
  BinaryOperator::XorAssign=>{},
    }


  Value::Undefined
}




