

pub mod read_expression;
pub mod dictionary;

use super::library::{
  ExpressionIndex,
  StringIndex,
  TypeIndex,
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
get_type_index(&self, lib: &Library)-> Result<TypeIndex,()>
{
    match self
    {
  OperandCore::Identifier(s)=>
        {
               if s ==  "true"{return Ok(Library::get_embedded_type_index(Type::Bool));}
          else if s == "false"{return Ok(Library::get_embedded_type_index(Type::Bool));}
        },
  OperandCore::Integer(u)=>{return Ok(Library::get_embedded_type_index(Type::U64));},
  OperandCore::Floating(f)=>{return Ok(Library::get_embedded_type_index(Type::F64));},
  OperandCore::Character(c)=>{return Ok(Library::get_embedded_type_index(Type::Char));},
  OperandCore::String(_)=>
        {
        },
  OperandCore::Expression(i)=>
        {
            if let Some(e) = lib.get_expression(*i)
            {
              return e.get_type_index(lib);
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
get_type_index(&self, tx: TypeIndex, lib: &Library)-> Result<TypeIndex,()>
{
    if let Some(t) = lib.get_type(tx)
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
                if let Type::Array(el_ti,_) = t
                {
                  return Ok(*el_ti);
                }
            },
      PostfixOperator::Call(_)=>
            {
                if let Type::FunctionPointer(_) = t
                {
                }
            },
      PostfixOperator::NameResolution(_)=>
            {
            },
      PostfixOperator::Increment=>{},
      PostfixOperator::Decrement=>{},
        }
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
get_type_index(&self, ti: TypeIndex, lib: &Library)-> Result<TypeIndex,()>
{
    if let Some(t) = lib.get_type(ti)
    {
        match self
        {
      PrefixOperator::Neg=>
            {
                match t
                {
              Type::I8=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I16=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I32=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I64=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::ISize=>{Ok(Library::get_embedded_type_index(Type::ISize))}
              Type::F32=>{Ok(Library::get_embedded_type_index(Type::F64))}
              Type::F64=>{Ok(Library::get_embedded_type_index(Type::F64))}
              _=>{Err(())}
                }
            },
      PrefixOperator::Not=>
            {
                match t
                {
              Type::I8=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I16=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I32=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I64=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::ISize=>{Ok(Library::get_embedded_type_index(Type::ISize))}
              Type::U8=>{Ok(Library::get_embedded_type_index(Type::U64))}
              Type::U16=>{Ok(Library::get_embedded_type_index(Type::U64))}
              Type::U32=>{Ok(Library::get_embedded_type_index(Type::U64))}
              Type::U64=>{Ok(Library::get_embedded_type_index(Type::U64))}
              Type::USize=>{Ok(Library::get_embedded_type_index(Type::USize))}
              _=>{Err(())}
                }
            },
      PrefixOperator::Address=>
             {
                match t
                {
              Type::Reference(_)=>{Ok(Library::get_embedded_type_index(Type::U64))}
              _=>{Err(())}
                }
            },
      PrefixOperator::Dereference=>
             {
                match t
                {
              Type::Pointer(_)=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::Reference(_)=>{Ok(Library::get_embedded_type_index(Type::I64))}
              _=>{Err(())}
                }
            },
      PrefixOperator::LogicalNot=>
             {
                match t
                {
              Type::Bool=>{Ok(Library::get_embedded_type_index(Type::Bool))}
              _=>{Err(())}
                }
            },
      PrefixOperator::Increment=>
             {
                match t
                {
              Type::I8=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I16=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I32=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I64=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::ISize=>{Ok(Library::get_embedded_type_index(Type::ISize))}
              _=>{Err(())}
                }
            },
      PrefixOperator::Decrement=>
            {
                match t
                {
              Type::I8=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I16=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I32=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::I64=>{Ok(Library::get_embedded_type_index(Type::I64))}
              Type::ISize=>{Ok(Library::get_embedded_type_index(Type::ISize))}
              _=>{Err(())}
                }
            },
        }
    }

  else
    {
      Err(())
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


pub fn
get_type_index(&self, lti: TypeIndex, rti: TypeIndex, lib: &Library)-> Result<TypeIndex,()>
{
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
get_type_index(&self, lib: &Library)-> Result<TypeIndex,()>
{
    if let Ok(mut lti) = self.core.get_type_index(lib)
    {
        for o in &self.postfix_operator_list
        {
            if let Ok(ti) = o.get_type_index(lti,lib)
            {
              lti = ti;
            }

          else
            {
              return Err(());
            }
        }


        for o in &self.prefix_operator_list
        {
            if let Ok(ti) = o.get_type_index(lti,lib)
            {
              lti = ti;
            }

          else
            {
              return Err(());
            }
        }


      return Ok(lti);
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
get_type_index(&self, lib: &Library)-> Result<TypeIndex,()>
{
    if let Ok(mut lti) = self.operand.get_type_index(lib)
    {
        for tail in &self.tail_list
        {
            if let Ok(rti) = tail.operand.get_type_index(lib)
            {
                if let Ok(new_ti) = tail.operator.get_type_index(lti,rti,lib)
                {
                  lti = new_ti;
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
          return Ok(Library::get_embedded_type_index(Type::Void));
        }


      return Ok(lti);
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




