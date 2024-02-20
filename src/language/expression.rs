

use super::typesystem::{
  TypeItem,
  TypeInfo,

};

use super::declaration::{
  Declaration,
  DeclarationLink,

};

use super::value::{
  Value,
  ValueData,

};




#[derive(Clone)]
pub struct
StringKeeper
{
  pub(crate) string: String,
  pub(crate) offset: usize,

}


impl
StringKeeper
{


pub fn
new(s: &str)-> StringKeeper
{
  StringKeeper{
    string: s.to_string(),
    offset: 0,
  }
}


}




#[derive(Clone)]
pub struct
ExpressionKeeper
{
  pub(crate) expression: Box<Expression>,
  pub(crate) preevaluated_value_opt: Option<Box<Value>>,

}


impl
ExpressionKeeper
{


pub fn
new(e: Expression)-> ExpressionKeeper
{
  ExpressionKeeper{
    expression: Box::new(e),
    preevaluated_value_opt: None,
  }
}


pub fn
get_value_mut(&mut self, decln: &DeclarationLink)-> Option<&Value>
{
    if let None = &self.preevaluated_value_opt
    {
        if let Ok(v) = self.expression.try_get_value_mut(decln)
        {
          self.preevaluated_value_opt = Some(Box::new(v));
        }
    }


    if let Some(v) = &self.preevaluated_value_opt
    {
      return Some(v);
    }


  None
}


}




#[derive(Clone)]
pub enum
PostfixOperator
{
  Access(String),
  Subscript(ExpressionKeeper),
  Call(Vec<ExpressionKeeper>),
  Increment,
  Decrement,

}


impl
PostfixOperator
{


pub fn
evaluate(&self, decln: &DeclarationLink, e: &mut ExpressionKeeper)-> Result<Value,()>
{
    if let Some(v) = e.get_value_mut(decln)
    {
    }


  Err(())
}


pub fn
print(&self)
{
    match self
    {
  PostfixOperator::Access(s)=>{print!(".{}",s);},
  PostfixOperator::Subscript(ek)=>
        {
          print!("[");
          ek.expression.print();
          print!("]");
        },
  PostfixOperator::Call(args)=>
        {
          print!("(");

            for ek in args
            {
              ek.expression.print();

              print!(", ");
            }


          print!(")");
        },
  PostfixOperator::Increment=>{print!("++");},
  PostfixOperator::Decrement=>{print!("--");},
    }
}


}




#[derive(Clone)]
pub enum
PrefixOperator
{
  Neg,
  Not,
  Address,
  Dereference,
  LogicalNot,
  Increment,
  Decrement,

}


impl
PrefixOperator
{


pub fn
evaluate(&self, decln: &DeclarationLink, e: &mut ExpressionKeeper)-> Result<Value,()>
{
    if let Some(v) = e.get_value_mut(decln)
    {
    }


  Err(())
}


pub fn
print(&self)
{
    match self
    {
  PrefixOperator::Neg=>{print!("-");},
  PrefixOperator::Not=>{print!("~");},
  PrefixOperator::Address=>{print!("&");},
  PrefixOperator::Dereference=>{print!("*");},
  PrefixOperator::LogicalNot=>{print!("!");},
  PrefixOperator::Increment=>{print!("++");},
  PrefixOperator::Decrement=>{print!("--");},
    }
}


}




#[derive(Clone)]
pub enum
BinaryOperator
{
  Add,
  Sub,
  Mul,
  Div,
  Rem,
  Shl,
  Shr,
  And,
  Or,
  Xor,
  Eq,
  Neq,
  Lt,
  Lteq,
  Gt,
  Gteq,
  LogicalOr,
  LogicalAnd,

}


impl
BinaryOperator
{


fn
for_i(l: &Value, r: &Value)-> Option<i64>
{
    if (l.type_item.is_i8()    == r.type_item.is_i8())
    || (l.type_item.is_i16()   == r.type_item.is_i16())
    || (l.type_item.is_i32()   == r.type_item.is_i32())
    || (l.type_item.is_i64()   == r.type_item.is_i64())
    || (l.type_item.is_isize() == r.type_item.is_isize())
    {
        if let ValueData::I64(ri) = r.data
        {
          return Some(ri);
        }
    }


  None
}


fn
for_u(l: &Value, r: &Value)-> Option<u64>
{
    if (l.type_item.is_u8()    == r.type_item.is_u8())
    || (l.type_item.is_u16()   == r.type_item.is_u16())
    || (l.type_item.is_u32()   == r.type_item.is_u32())
    || (l.type_item.is_u64()   == r.type_item.is_u64())
    || (l.type_item.is_usize() == r.type_item.is_usize())
    {
        if let ValueData::U64(ri) = r.data
        {
          return Some(ri);
        }
    }


  None
}


fn
for_f(l: &Value, r: &Value)-> Option<f64>
{
    if (l.type_item.is_f32()   == r.type_item.is_f32())
    || (l.type_item.is_f64()   == r.type_item.is_f64())
    {
        if let ValueData::F64(ri) = r.data
        {
          return Some(ri);
        }
    }


  None
}


fn
for_shu(l: &Value, r: &Value)-> Option<u64>
{
    if (l.type_item.is_u8()
    ||  l.type_item.is_u16()
    ||  l.type_item.is_u32()
    ||  l.type_item.is_u64()
    ||  l.type_item.is_usize())
    && r.type_item.is_usize()
    {
        if let ValueData::U64(ri) = r.data
        {
          return Some(ri);
        }
    }


  None
}


fn
for_shi(l: &Value, r: &Value)-> Option<u64>
{
    if (l.type_item.is_i8()
    ||  l.type_item.is_i16()
    ||  l.type_item.is_i32()
    ||  l.type_item.is_i64()
    ||  l.type_item.is_isize())
    && r.type_item.is_usize()
    {
        if let ValueData::U64(ri) = r.data
        {
          return Some(ri);
        }
    }


  None
}


fn
for_log(l: &Value, r: &Value)-> Option<bool>
{
    if l.type_item.is_bool() && r.type_item.is_bool()
    {
        if let ValueData::U64(ri) = r.data
        {
          return Some(ri != 0);
        }
    }


  None
}


pub fn
evaluate(&self, decln: &DeclarationLink, le: &mut ExpressionKeeper, re: &mut ExpressionKeeper)-> Result<Value,()>
{
    if let Some(lv) = le.get_value_mut(decln)
    {
        if let Some(rv) = re.get_value_mut(decln)
        {
            match self
            {
          BinaryOperator::Add=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.addu(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.addi(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.addf(r));}
                }
          BinaryOperator::Sub=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.subu(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.subi(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.subf(r));}
                }
          BinaryOperator::Mul=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.mulu(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.muli(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.mulf(r));}
                }
          BinaryOperator::Div=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.divu(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.divi(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.divf(r));}
                }
          BinaryOperator::Rem=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.remu(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.remi(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.remf(r));}
                }
          BinaryOperator::Shl=>
                {
                       if let Some(r) = Self::for_shu(&lv,&rv){return Ok(lv.shlu(r));}
                  else if let Some(r) = Self::for_shi(&lv,&rv){return Ok(lv.shli(r));}
                }
          BinaryOperator::Shr=>
                {
                       if let Some(r) = Self::for_shu(&lv,&rv){return Ok(lv.shru(r));}
                  else if let Some(r) = Self::for_shi(&lv,&rv){return Ok(lv.shri(r));}
                }
          BinaryOperator::And=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.andu(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.andi(r));}
                }
          BinaryOperator::Or=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.oru(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.ori(r));}
                }
          BinaryOperator::Xor=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.xoru(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.xori(r));}
                }
          BinaryOperator::Eq=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.equ(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.eqi(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.eqf(r));}
                }
          BinaryOperator::Neq=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.nequ(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.neqi(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.neqf(r));}
                }
          BinaryOperator::Lt=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.ltu(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.lti(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.ltf(r));}
                }
          BinaryOperator::Lteq=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.ltequ(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.lteqi(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.lteqf(r));}
                }
          BinaryOperator::Gt=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.gtu(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.gti(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.gtf(r));}
                }
          BinaryOperator::Gteq=>
                {
                       if let Some(r) = Self::for_u(&lv,&rv){return Ok(lv.gtequ(r));}
                  else if let Some(r) = Self::for_i(&lv,&rv){return Ok(lv.gteqi(r));}
                  else if let Some(r) = Self::for_f(&lv,&rv){return Ok(lv.gteqf(r));}
                }
          BinaryOperator::LogicalAnd=>{if let Some(r) = Self::for_log(&lv,&rv){return Ok(lv.log_and(r));}}
          BinaryOperator::LogicalOr =>{if let Some(r) = Self::for_log(&lv,&rv){return Ok(lv.log_or( r));}}
          _=>{}
            }
        }
    }


  Err(())
}


pub fn
print(&self)
{
    match self
    {
  BinaryOperator::Add=>{print!("+");},
  BinaryOperator::Sub=>{print!("-");},
  BinaryOperator::Mul=>{print!("*");},
  BinaryOperator::Div=>{print!("/");},
  BinaryOperator::Rem=>{print!("%");},
  BinaryOperator::Shl=>{print!("<<");},
  BinaryOperator::Shr=>{print!(">>");},
  BinaryOperator::And=>{print!("&");},
  BinaryOperator::Or=>{print!("|");},
  BinaryOperator::Xor=>{print!("^");},
  BinaryOperator::Eq=>{print!("==");},
  BinaryOperator::Neq=>{print!("!=");},
  BinaryOperator::Lt=>{print!("<");},
  BinaryOperator::Lteq=>{print!("<=");},
  BinaryOperator::Gt=>{print!(">");},
  BinaryOperator::Gteq=>{print!(">=");},
  BinaryOperator::LogicalAnd=>{print!("&&");},
  BinaryOperator::LogicalOr=>{print!("||");},
    }
}


}




#[derive(Clone)]
pub struct
Path
{
  pub(crate) identifier_list: Vec<String>,
}


impl
Path
{


pub fn
new()-> Path
{
  Path{identifier_list: Vec::new()}
}


pub fn
print(&self)
{
    if let Some(first) = self.identifier_list.first()
    {
      print!("{}",first);

        for i in 1..self.identifier_list.len()
        {
          print!("::{}",&self.identifier_list[i]);
        }
    }
}


}




#[derive(Clone)]
pub enum
Expression
{
  Identifier(Path),
  Integer(u64),
  Floating(f64),
  Character(char),
  String(StringKeeper),

  SubExpression(ExpressionKeeper),

  PostfixOperation(PostfixOperator,ExpressionKeeper),
   PrefixOperation(PrefixOperator,ExpressionKeeper),

  BinaryOperation(BinaryOperator,ExpressionKeeper,ExpressionKeeper),

}


impl
Expression
{


pub fn
try_from(s: &str)-> Result<Expression,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::declaration::expression_dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"expression_with_assign",Some(dics))
    {
      let  cur = crate::syntax::Cursor::new(&dir);

        if let Some(e_dir) = cur.get_directory()
        {
//          e_dir.print(0);

          return crate::language::declaration::read_expression::read_expression(&e_dir);
        }
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
try_get_value_mut(&mut self, decln: &DeclarationLink)-> Result<Value,()>
{
    match self
    {
  Expression::Identifier(path)=>
        {
          Err(())
        },
  Expression::Integer(u)=>{Ok(Value::new_u64(*u))},
  Expression::Floating(f)=>{Ok(Value::new_f64(*f))},
  Expression::Character(c)=>{Ok(Value::new_char(*c))},
  Expression::String(sk)=>
        {
Err(())
//          Ok(Value::new_string(&sk.string))
        },
  Expression::SubExpression(ek)=>
        {
            if let Some(v) = ek.get_value_mut(decln)
            {
              Ok(v.clone())
            }

          else
            {
              Err(())
            }
        },
  Expression::PrefixOperation(o,e)=>
        {
          o.evaluate(decln,e)
        },
  Expression::PostfixOperation(o,e)=>
        {
          o.evaluate(decln,e)
        },
  Expression::BinaryOperation(o,l,r)=>
        {
          o.evaluate(decln,l,r)
        },
    }
}


pub fn
print(&self)
{
    match self
    {
  Expression::Identifier(path)=>{path.print();},
  Expression::Integer(u)=>{print!("{}",u);},
  Expression::Floating(f)=>{print!("{}",f);},
  Expression::Character(c)=>{print!("{}",c);},
  Expression::String(sk)=>
        {
          print!("\"{}\"",&sk.string);
        },
  Expression::SubExpression(ek)=>
        {
          print!("(");
          ek.expression.print();
          print!(")");
        },
  Expression::PrefixOperation(o,e)=>
        {
          o.print();
          e.expression.print();
        },
  Expression::PostfixOperation(o,e)=>
        {
          e.expression.print();
          o.print();
        },
  Expression::BinaryOperation(o,l,r)=>
        {
          l.expression.print();
          o.print();
          r.expression.print();
        },
    }
}


}




