

pub mod read_expression;
pub mod dictionary;

use super::library::{
  ExpressionIndex,
  StringIndex,
  Library
};

use super::typesystem::{
  Type,
  TypeInfo,
};

#[derive(Clone)]
pub enum
OperandCore
{
  Identifier(String),
  Integer(u64),
  Floating(f64),
  Character(char),
  String(StringIndex),
  Expression(ExpressionIndex),

}


impl
OperandCore
{


pub fn
get_type(&self, lib: &Library)-> Result<Type,()>
{
    match self
    {
  OperandCore::Identifier(s)=>
        {
               if s ==  "true"{return Ok(Type::Bool);}
          else if s == "false"{return Ok(Type::Bool);}
        },
  OperandCore::Integer(_)  =>{return Ok(Type::U64);},
  OperandCore::Floating(_) =>{return Ok(Type::F64);},
  OperandCore::Character(_)=>{return Ok(Type::Char);},
  OperandCore::String(_)   =>{return Ok(Type::Reference(Box::new(Type::Char)));},
  OperandCore::Expression(i)=>
        {
            if let Some(e) = lib.get_expression(*i)
            {
              return e.get_type(lib);
            }
        },
    }


  Err(())
}


pub fn
print(&self, lib: &Library)
{
    match self
    {
  OperandCore::Identifier(s)=>{print!("{}",s);},
  OperandCore::Integer(u)=>{print!("{}",u);},
  OperandCore::Floating(f)=>{print!("{}",f);},
  OperandCore::Character(c)=>{print!("{}",c);},
  OperandCore::String(i)=>
        {
            if let Some(s) = lib.get_string(*i)
            {
              print!("\"{}\"",s);
            }
        },
  OperandCore::Expression(i)=>
        {
            if let Some(e) = lib.get_expression(*i)
            {
              print!("(");
              e.print(lib);
              print!(")");
            }
        },
    }
}


}




#[derive(Clone)]
pub enum
PostfixOperator
{
  Access(String),
  Subscript(ExpressionIndex),
  Call(Vec<ExpressionIndex>),
  NameResolution(String),
  Increment,
  Decrement,

}


impl
PostfixOperator
{


pub fn
get_type(&self, ty: &Type, lib: &Library)-> Result<Type,()>
{
    match self
    {
  PostfixOperator::Access(s)=>
        {
/*
            if let Ok(ti) = TypeInfo::make(tx,lib)
            {
                if let Some(f) = ti.get_field(s)
                {
                  return Ok(f.type_index);
                }
            }
*/
        },
  PostfixOperator::Subscript(_)=>
        {
        },
  PostfixOperator::Call(_)=>
        {
        },
  PostfixOperator::NameResolution(_)=>
        {
        },
  PostfixOperator::Increment=>{},
  PostfixOperator::Decrement=>{},
    }


  Err(())
}


pub fn
print(&self, lib: &Library)
{
    match self
    {
  PostfixOperator::Access(s)=>{print!(".{}",s);},
  PostfixOperator::Subscript(i)=>
        {
            if let Some(e) = lib.get_expression(*i)
            {
              print!("[");
              e.print(lib);
              print!("]");
            }
        },
  PostfixOperator::Call(args)=>
        {
          print!("(");

            for i in args
            {
                if let Some(e) = lib.get_expression(*i)
                {
                  e.print(lib);
                }


              print!(", ");
            }

          print!(")");
        },
  PostfixOperator::NameResolution(s)=>
        {
          print!("::{}",s);
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
get_type(&self, ty: &Type, lib: &Library)-> Result<Type,()>
{
    match self
    {
  PrefixOperator::Neg=>
        {
            if ty.is_signed_integer() || ty.is_floating()
            {
              Ok(ty.clone())
            }

          else
            {
              Err(())
            }
        },
  PrefixOperator::Not=>
        {
            if ty.is_integer()
            {
              Ok(ty.clone())
            }

          else
            {
              Err(())
            }
        },
  PrefixOperator::Address=>
         {
            match ty
            {
          Type::Reference(_)=>{Ok(Type::U64)}
          _=>{Err(())}
            }
        },
  PrefixOperator::Dereference=>
         {
            match ty
            {
          Type::Pointer(_)=>{Ok(Type::I64)}
          Type::Reference(_)=>{Ok(Type::I64)}
          _=>{Err(())}
            }
        },
  PrefixOperator::LogicalNot=>
         {
            match ty
            {
          Type::Bool=>{Ok(Type::Bool)}
          _=>{Err(())}
            }
        },
  PrefixOperator::Increment=>
         {
            match ty
            {
          Type::I8=>{Ok(Type::I64)}
          Type::I16=>{Ok(Type::I64)}
          Type::I32=>{Ok(Type::I64)}
          Type::I64=>{Ok(Type::I64)}
          Type::ISize=>{Ok(Type::ISize)}
          _=>{Err(())}
            }
        },
  PrefixOperator::Decrement=>
        {
            match ty
            {
          Type::I8=>{Ok(Type::I64)}
          Type::I16=>{Ok(Type::I64)}
          Type::I32=>{Ok(Type::I64)}
          Type::I64=>{Ok(Type::I64)}
          Type::ISize=>{Ok(Type::ISize)}
          _=>{Err(())}
            }
        },
    }
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
for_arithmetic(lt: &Type, rt: &Type, lib: &Library)-> Result<Type,()>
{
    if  (lt.is_i8()    == rt.is_i8())
     || (lt.is_i16()   == rt.is_i16())
     || (lt.is_i32()   == rt.is_i32())
     || (lt.is_i64()   == rt.is_i64())
     || (lt.is_isize() == rt.is_isize())
     || (lt.is_u8()    == rt.is_u8())
     || (lt.is_u16()   == rt.is_u16())
     || (lt.is_u32()   == rt.is_u32())
     || (lt.is_u64()   == rt.is_u64())
     || (lt.is_usize() == rt.is_usize())
     || (lt.is_f32()   == rt.is_f32())
     || (lt.is_f64()   == rt.is_f64())
    {
      return Ok(lt.clone());
    }


  Err(())
}


fn
for_comparison(lt: &Type, rt: &Type, lib: &Library)-> Result<Type,()>
{
    if  (lt.is_char()  == rt.is_char())
     || (lt.is_i8()    == rt.is_i8())
     || (lt.is_i16()   == rt.is_i16())
     || (lt.is_i32()   == rt.is_i32())
     || (lt.is_i64()   == rt.is_i64())
     || (lt.is_isize() == rt.is_isize())
     || (lt.is_u8()    == rt.is_u8())
     || (lt.is_u16()   == rt.is_u16())
     || (lt.is_u32()   == rt.is_u32())
     || (lt.is_u64()   == rt.is_u64())
     || (lt.is_usize() == rt.is_usize())
     || (lt.is_f32()   == rt.is_f32())
     || (lt.is_f64()   == rt.is_f64())
    {
      return Ok(lt.clone());
    }


  Err(())
}


fn
for_bitshift(lt: &Type, rt: &Type, lib: &Library)-> Result<Type,()>
{
    if lt.is_integer() && rt.is_unsigned_integer()
    {
      return Ok(lt.clone());
    }


  Err(())
}


fn
for_bitwise(lt: &Type, rt: &Type, lib: &Library)-> Result<Type,()>
{
    if lt.is_integer() && rt.is_unsigned_integer()
    {
      return Ok(lt.clone());
    }


  Err(())
}


fn
for_logical(lt: &Type, rt: &Type, lib: &Library)-> Result<Type,()>
{
    if lt.is_bool() && rt.is_bool()
    {
      return Ok(Type::Bool);
    }


  Err(())
}


pub fn
get_type(&self, lt: &Type, rt: &Type, lib: &Library)-> Result<Type,()>
{
    match self
    {
  BinaryOperator::Add=>{Self::for_arithmetic(lt,rt,lib)},
  BinaryOperator::Sub=>{Self::for_arithmetic(lt,rt,lib)},
  BinaryOperator::Mul=>{Self::for_arithmetic(lt,rt,lib)},
  BinaryOperator::Div=>{Self::for_arithmetic(lt,rt,lib)},
  BinaryOperator::Rem=>{Self::for_arithmetic(lt,rt,lib)},
  BinaryOperator::Shl=>{Self::for_bitshift(lt,rt,lib)},
  BinaryOperator::Shr=>{Self::for_bitshift(lt,rt,lib)},
  BinaryOperator::And=>{Self::for_bitwise(lt,rt,lib)},
  BinaryOperator::Or =>{Self::for_bitwise(lt,rt,lib)},
  BinaryOperator::Xor=>{Self::for_bitwise(lt,rt,lib)},
  BinaryOperator::Eq  =>{Self::for_comparison(lt,rt,lib)},
  BinaryOperator::Neq =>{Self::for_comparison(lt,rt,lib)},
  BinaryOperator::Lt  =>{Self::for_comparison(lt,rt,lib)},
  BinaryOperator::Lteq=>{Self::for_comparison(lt,rt,lib)},
  BinaryOperator::Gt  =>{Self::for_comparison(lt,rt,lib)},
  BinaryOperator::Gteq=>{Self::for_comparison(lt,rt,lib)},
  BinaryOperator::LogicalAnd=>{Self::for_logical(lt,rt,lib)},
  BinaryOperator::LogicalOr =>{Self::for_logical(lt,rt,lib)},
    }
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
pub enum
AssignOperator
{
  Nop,
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

}


impl
AssignOperator
{


pub fn
print(&self)
{
    match self
    {
  AssignOperator::Nop=>{print!("=");},
  AssignOperator::Add=>{print!("+=");},
  AssignOperator::Sub=>{print!("-=");},
  AssignOperator::Mul=>{print!("*=");},
  AssignOperator::Div=>{print!("/=");},
  AssignOperator::Rem=>{print!("%=");},
  AssignOperator::Shl=>{print!("<<=");},
  AssignOperator::Shr=>{print!(">>=");},
  AssignOperator::And=>{print!("&=");},
  AssignOperator::Or=>{print!("|=");},
  AssignOperator::Xor=>{print!("^=");},
    }
}


}




#[derive(Clone)]
pub enum
Operator
{
  Prefix(PrefixOperator),
  Postfix(PostfixOperator),
  Binary(BinaryOperator),

}


impl
Operator
{


pub fn
get_priority(&self)-> usize
{
    match self
    {
  Operator::Postfix(o)=>{return 3;},
  Operator::Prefix(o)=>  {return 2;},
  Operator::Binary(o)=> {return 1;},
    }
}


pub fn
print(&self, lib: &Library)
{
    match self
    {
  Operator::Postfix(o)=>{o.print(lib);},
  Operator::Prefix(o)=>{o.print();},
  Operator::Binary(o)=>{o.print();},
    }
}


}




#[derive(Clone)]
pub struct
Operand
{
  pub(crate) prefix_operator_list: Vec<PrefixOperator>,

  pub(crate) core: OperandCore,

  pub(crate) postfix_operator_list: Vec<PostfixOperator>,

}


impl
Operand
{


pub fn
get_type(&self, lib: &Library)-> Result<Type,()>
{
    if let Ok(mut lt) = self.core.get_type(lib)
    {
        for o in &self.postfix_operator_list
        {
            if let Ok(t) = o.get_type(&lt,lib)
            {
              lt = t;
            }

          else
            {
              return Err(());
            }
        }


        for o in &self.prefix_operator_list
        {
            if let Ok(t) = o.get_type(&lt,lib)
            {
              lt = t;
            }

          else
            {
              return Err(());
            }
        }


      return Ok(lt);
    }


  Err(())
}


pub fn
print(&self, lib: &Library)
{
    for o in &self.prefix_operator_list
    {
      o.print();
    }


  self.core.print(lib);

    for o in &self.postfix_operator_list
    {
      o.print(lib);
    }
}


}




#[derive(Clone)]
pub struct
ExpressionTail
{
  pub(crate) operator: BinaryOperator,

  pub(crate) operand: Operand,

}


#[derive(Clone)]
pub struct
Expression
{
  pub(crate) operand: Operand,

  pub(crate) tail_list: Vec<ExpressionTail>,

  pub(crate) assign_part_opt: Option<(AssignOperator,ExpressionIndex)>,

}


impl
Expression
{


pub fn
make_from_string(s: &str, lib: &mut Library)-> Result<Expression,()>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = self::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"expression_with_assign",Some(dics))
    {
      let  cur = crate::syntax::Cursor::new(&dir);

        if let Some(e_dir) = cur.get_directory()
        {
//          e_dir.print(0);

          return self::read_expression::read_expression(&e_dir,lib);
        }
    }


  println!("make_from_string error: parse is failed");

  Err(())
}


pub fn
get_type(&self, lib: &Library)-> Result<Type,()>
{
    if let Ok(mut lt) = self.operand.get_type(lib)
    {
        for tail in &self.tail_list
        {
            if let Ok(rt) = tail.operand.get_type(lib)
            {
                if let Ok(new_t) = tail.operator.get_type(&lt,&rt,lib)
                {
                  lt = new_t;
                }

              else
                {
                  return Err(());
                }
            }

          else
            {
              return Err(());
            }
        }


        if let Some(_) = &self.assign_part_opt
        {
          return Ok(Type::Void);
        }


      return Ok(lt);
    }


  Err(())
}


pub fn
print(&self, lib: &Library)
{
  self.operand.print(lib);

    for t in &self.tail_list
    {
      t.operator.print();
      t.operand.print(lib);
    }


    if let Some((a,ei)) = &self.assign_part_opt
    {
      a.print();

        if let Some(e) = lib.get_expression(*ei)
        {
          e.print(lib);
        }
    }
}


}




