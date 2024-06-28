

use super::typesystem::{
  TypeInfo,

};

use super::operation::{
  Source,
  Destination,
  Operation,

};

use super::declaration::{
  Storage,
  Declaration,
  Component,
  Space,

};

use super::statement::{
  Scope,
  StorageInfo,

};




#[derive(Clone)]
pub enum
UnaryOperator
{
  Neg,
  Not,
  LogicalNot,
  Deref,

}


impl
UnaryOperator
{


pub fn
compile(&self, dst: Destination, src: Source, ti: TypeInfo, buf: &mut Vec<Operation>)-> Result<TypeInfo,()>
{
    match self
    {
  UnaryOperator::Neg=>
        {
            if ti.is_ulit()
            {
              buf.push(Operation::Neg(dst,src));

              return Ok(TypeInfo::new_ilit());
            }

          else
            if ti.is_signed_integer()
            {
              buf.push(Operation::Neg(dst,src));

              return Ok(ti);
            }

          else
            if ti.is_floating()
            {
              buf.push(Operation::NegF(dst,src));

              return Ok(ti);
            }
        }
  UnaryOperator::Not=>
        {
            if ti.is_word()
            {
              buf.push(Operation::Not(dst,src));

              return Ok(ti);
            }
        }
  UnaryOperator::LogicalNot=>
        {
            if ti.is_bool()
            {
              buf.push(Operation::LogicalNot(dst,src));

              return Ok(ti);
            }
        }
  UnaryOperator::Deref=>
        {
            if let Some(target) = ti.pointer_target()
            {
              return Ok(target.clone());
            }

          else
            if let Some(target) = ti.reference_target()
            {
              return Ok(target.clone());
            }
        }
    }


  print!("UnaryOperator::compile error: ");

  self.print();

  print!(" failed\n");

  Err(())
}


pub fn
print(&self)
{
    match self
    {
  UnaryOperator::Neg=>{print!("-");},
  UnaryOperator::Not=>{print!("~");},
  UnaryOperator::LogicalNot=>{print!("!");},
  UnaryOperator::Deref=>{print!("*");},
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
is_boolean(l_ti: &TypeInfo, r_ti: &TypeInfo)-> bool
{
  l_ti.is_bool() && r_ti.is_bool()
}


fn
check_bitwise_operatable(l_ti: &TypeInfo, r_ti: &TypeInfo)-> Result<TypeInfo,()>
{
    if ((l_ti.is_u8()    || l_ti.is_i8()    || l_ti.is_ulit()) && (r_ti.is_u8()    || r_ti.is_ulit()))
    || ((l_ti.is_u16()   || l_ti.is_i16()   || l_ti.is_ulit()) && (r_ti.is_u16()   || r_ti.is_ulit()))
    || ((l_ti.is_u32()   || l_ti.is_i32()   || l_ti.is_ulit()) && (r_ti.is_u32()   || r_ti.is_ulit()))
    || ((l_ti.is_u64()   || l_ti.is_i64()   || l_ti.is_ulit()) && (r_ti.is_u64()   || r_ti.is_ulit()))
    || ((l_ti.is_usize() || l_ti.is_isize() || l_ti.is_ulit()) && (r_ti.is_usize() || r_ti.is_ulit()))
    {
      Ok(l_ti.clone())
    }

  else
    {
      Err(())
    }
}


fn
is_bitshiftable(l_ti: &TypeInfo, r_ti: &TypeInfo)-> bool
{
     l_ti.is_integer()         
  && r_ti.is_unsigned_integer()
}


fn
check(l_ti: &TypeInfo, r_ti: &TypeInfo)-> Result<TypeInfo,()>
{
       if l_ti.is_u8()    && r_ti.is_u8()   {return Ok(TypeInfo::new_u8()  );}
  else if l_ti.is_u16()   && r_ti.is_u16()  {return Ok(TypeInfo::new_u16() );}
  else if l_ti.is_u32()   && r_ti.is_u32()  {return Ok(TypeInfo::new_u32() );}
  else if l_ti.is_u64()   && r_ti.is_u64()  {return Ok(TypeInfo::new_u64() );}
  else if l_ti.is_usize() && r_ti.is_usize(){return Ok(TypeInfo::new_usize());}
  else if l_ti.is_i8()    && r_ti.is_i8()   {return Ok(TypeInfo::new_i8()  );}
  else if l_ti.is_i16()   && r_ti.is_i16()  {return Ok(TypeInfo::new_i16() );}
  else if l_ti.is_i32()   && r_ti.is_i32()  {return Ok(TypeInfo::new_i32() );}
  else if l_ti.is_i64()   && r_ti.is_i64()  {return Ok(TypeInfo::new_i64() );}
  else if l_ti.is_isize() && r_ti.is_isize(){return Ok(TypeInfo::new_isize());}
  else if l_ti.is_f32()   && r_ti.is_f32()  {return Ok(TypeInfo::new_f32() );}
  else if l_ti.is_f64()   && r_ti.is_f64()  {return Ok(TypeInfo::new_f64() );}
  else if l_ti.is_ulit() && r_ti.is_unsigned_integer(){return Ok(r_ti.clone());}
  else if l_ti.is_unsigned_integer() && r_ti.is_ulit(){return Ok(l_ti.clone());}
  else if l_ti.is_ilit() && r_ti.is_signed_integer(){return Ok(r_ti.clone());}
  else if l_ti.is_signed_integer() && r_ti.is_ilit(){return Ok(l_ti.clone());}
  else if l_ti.is_ulit() && r_ti.is_ilit(){return Ok(r_ti.clone());}
  else if l_ti.is_ilit() && r_ti.is_ulit(){return Ok(l_ti.clone());}
  else if l_ti.is_flit() && r_ti.is_floating(){return Ok(r_ti.clone());}
  else if l_ti.is_floating() && r_ti.is_flit(){return Ok(l_ti.clone());}

  Err(())
}


pub fn
compile(&self, dst: Destination, l_src: Source, l_ti: TypeInfo, r_src: Source, r_ti: TypeInfo, buf: &mut Vec<Operation>)-> Result<TypeInfo,()>
{
    match self
    {
  BinaryOperator::Add=>
        {
            if let Ok(ti) = Self::check(&l_ti,&r_ti)
            {
                if ti.is_unsigned_integer()
                {
                  buf.push(Operation::AddU(dst,l_src,r_src));
                }

              else
                if ti.is_signed_integer()
                {
                  buf.push(Operation::AddI(dst,l_src,r_src));
                }

              else
                if ti.is_floating()
                {
                  buf.push(Operation::AddF(dst,l_src,r_src));
                }


              return Ok(ti);
            }
        },
  BinaryOperator::Sub=>
        {
            if let Ok(ti) = Self::check(&l_ti,&r_ti)
            {
                if ti.is_unsigned_integer()
                {
                  buf.push(Operation::SubU(dst,l_src,r_src));
                }

              else
                if ti.is_signed_integer()
                {
                  buf.push(Operation::SubI(dst,l_src,r_src));
                }

              else
                if ti.is_floating()
                {
                  buf.push(Operation::SubF(dst,l_src,r_src));
                }


              return Ok(ti);
            }
        },
  BinaryOperator::Mul=>
        {
            if let Ok(ti) = Self::check(&l_ti,&r_ti)
            {
                if ti.is_unsigned_integer()
                {
                  buf.push(Operation::MulU(dst,l_src,r_src));
                }

              else
                if ti.is_signed_integer()
                {
                  buf.push(Operation::MulI(dst,l_src,r_src));
                }

              else
                if ti.is_floating()
                {
                  buf.push(Operation::MulF(dst,l_src,r_src));
                }


              return Ok(ti);
            }
        },
  BinaryOperator::Div=>
        {
            if let Ok(ti) = Self::check(&l_ti,&r_ti)
            {
                if ti.is_unsigned_integer()
                {
                  buf.push(Operation::DivU(dst,l_src,r_src));
                }

              else
                if ti.is_signed_integer()
                {
                  buf.push(Operation::DivI(dst,l_src,r_src));
                }

              else
                if ti.is_floating()
                {
                  buf.push(Operation::DivF(dst,l_src,r_src));
                }


              return Ok(ti);
            }
        },
  BinaryOperator::Rem=>
        {
            if let Ok(ti) = Self::check(&l_ti,&r_ti)
            {
                if ti.is_unsigned_integer()
                {
                  buf.push(Operation::RemU(dst,l_src,r_src));
                }

              else
                if ti.is_signed_integer()
                {
                  buf.push(Operation::RemI(dst,l_src,r_src));
                }

              else
                if ti.is_floating()
                {
                  buf.push(Operation::RemF(dst,l_src,r_src));
                }


              return Ok(ti);
            }
        },
  BinaryOperator::Shl=>
        {
            if Self::is_bitshiftable(&l_ti,&r_ti)
            {
              buf.push(Operation::Shl(dst,l_src,r_src));

              return Ok(l_ti);
            }
        }
  BinaryOperator::Shr=>
        {
            if Self::is_bitshiftable(&l_ti,&r_ti)
            {
              buf.push(Operation::Shr(dst,l_src,r_src));

              return Ok(l_ti);
            }
        }
  BinaryOperator::And=>
        {
            if let Ok(ti) = Self::check_bitwise_operatable(&l_ti,&r_ti)
            {
              buf.push(Operation::And(dst,l_src,r_src));

              return Ok(ti);
            }
        }
  BinaryOperator::Or=>
        {
            if let Ok(ti) = Self::check_bitwise_operatable(&l_ti,&r_ti)
            {
              buf.push(Operation::Or(dst,l_src,r_src));

              return Ok(ti);
            }
        }
  BinaryOperator::Xor=>
        {
            if let Ok(ti) = Self::check_bitwise_operatable(&l_ti,&r_ti)
            {
              buf.push(Operation::Xor(dst,l_src,r_src));

              return Ok(ti);
            }
        }
  BinaryOperator::Eq=>
        {
            if let Ok(_) = Self::check(&l_ti,&r_ti)
            {
              buf.push(Operation::Eq(dst,l_src,r_src));

              return Ok(TypeInfo::new_bool());
            }
        }
  BinaryOperator::Neq=>
        {
            if let Ok(_) = Self::check(&l_ti,&r_ti)
            {
              buf.push(Operation::Neq(dst,l_src,r_src));

              return Ok(TypeInfo::new_bool());
            }
        }
  BinaryOperator::Lt=>
        {
            if let Ok(ti) = Self::check(&l_ti,&r_ti)
            {
                if ti.is_unsigned_integer()
                {
                  buf.push(Operation::LtU(dst,l_src,r_src));
                }

              else
                if ti.is_signed_integer()
                {
                  buf.push(Operation::LtI(dst,l_src,r_src));
                }

              else
                if ti.is_floating()
                {
                  buf.push(Operation::LtF(dst,l_src,r_src));
                }


              return Ok(ti);
            }
        },
  BinaryOperator::Lteq=>
        {
            if let Ok(ti) = Self::check(&l_ti,&r_ti)
            {
                if ti.is_unsigned_integer()
                {
                  buf.push(Operation::LteqU(dst,l_src,r_src));
                }

              else
                if ti.is_signed_integer()
                {
                  buf.push(Operation::LteqI(dst,l_src,r_src));
                }

              else
                if ti.is_floating()
                {
                  buf.push(Operation::LteqF(dst,l_src,r_src));
                }


              return Ok(ti);
            }
        },
  BinaryOperator::Gt=>
        {
            if let Ok(ti) = Self::check(&l_ti,&r_ti)
            {
                if ti.is_unsigned_integer()
                {
                  buf.push(Operation::GteqU(dst,l_src,r_src));
                }

              else
                if ti.is_signed_integer()
                {
                  buf.push(Operation::GtI(dst,l_src,r_src));
                }

              else
                if ti.is_floating()
                {
                  buf.push(Operation::GtF(dst,l_src,r_src));
                }


              return Ok(ti);
            }
        },
  BinaryOperator::Gteq=>
        {
            if let Ok(ti) = Self::check(&l_ti,&r_ti)
            {
                if ti.is_unsigned_integer()
                {
                  buf.push(Operation::GteqU(dst,l_src,r_src));
                }

              else
                if ti.is_signed_integer()
                {
                  buf.push(Operation::GteqI(dst,l_src,r_src));
                }

              else
                if ti.is_floating()
                {
                  buf.push(Operation::GteqF(dst,l_src,r_src));
                }


              return Ok(ti);
            }
        },
  BinaryOperator::LogicalAnd=>
        {
            if Self::is_boolean(&l_ti,&r_ti)
            {
              buf.push(Operation::LogicalAnd(dst,l_src,r_src));

              return Ok(r_ti);
            }
        }
  BinaryOperator::LogicalOr=>
        {
            if Self::is_boolean(&l_ti,&r_ti)
            {
              buf.push(Operation::LogicalOr(dst,l_src,r_src));

              return Ok(r_ti);
            }
        }
    }




  print!("BinaryOperator::compile error: ");

  self.print();

  print!(" failed\n");

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




#[derive(Clone,PartialEq)]
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
add(mut self, name: &str)-> Path
{
    if name.len() != 0
    {
      self.identifier_list.push(name.to_string());
    }


  self
}


pub fn
as_strings(&self)-> &Vec<String>
{
  &self.identifier_list
}


pub fn
to_string(&self)-> String
{
  let  mut s = String::new();

    if let Some(first) = self.identifier_list.first()
    {
      s.push_str(first);

        for i in 1..self.identifier_list.len()
        {
          s.push_str("::");
          s.push_str(&self.identifier_list[i]);
        }
    }


  s
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




pub struct
Namer
{
  base: String,
  number: usize,
}


impl
Namer
{


pub fn
new(s: &str)-> Self
{
  Self{
    base: s.to_string(),
    number: 0,
  }
}


pub fn
get(&mut self)-> String
{
  let  n = self.number;

  self.number += 1;

    if n == 0
    {
      self.base.clone()
    }

  else
    {
      format!("{}_TMP{}",&self.base,n)
    }
}


}




#[derive(Clone)]
pub enum
Expression
{
  None,

  Identifier(Path),
  Boolean(bool),
  Integer(u64),
  Floating(f64),
  String(String),

  SubExpression(Box<Expression>),

  Access(Box<Expression>,String),
  Call(Box<Expression>,Vec<Expression>),
  Subscript(Box<Expression>,Box<Expression>),

  Unary(UnaryOperator,Box<Expression>),
  Binary(BinaryOperator,Box<Expression>,Box<Expression>),

}


impl
Expression
{


pub fn  new_id(s: &str)-> Expression{Expression::Identifier(Path::new().add(s))}
pub fn  new_bool(b: bool)-> Expression{Expression::Boolean(b)}
pub fn  new_u64(u: u64)-> Expression{Expression::Integer(u)}
pub fn  new_f64(f: f64)-> Expression{Expression::Floating(f)}

pub fn  new_neg(o: Expression)-> Expression{Expression::Unary(UnaryOperator::Neg,Box::new(o))}
pub fn  new_not(o: Expression)-> Expression{Expression::Unary(UnaryOperator::Not,Box::new(o))}
pub fn  new_not_logical(o: Expression)-> Expression{Expression::Unary(UnaryOperator::LogicalNot,Box::new(o))}
pub fn  new_deref(o: Expression)-> Expression{Expression::Unary(UnaryOperator::Deref,Box::new(o))}

pub fn  new_add(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Add,Box::new(l),Box::new(r))}
pub fn  new_sub(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Sub,Box::new(l),Box::new(r))}
pub fn  new_mul(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Mul,Box::new(l),Box::new(r))}
pub fn  new_div(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Div,Box::new(l),Box::new(r))}
pub fn  new_rem(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Rem,Box::new(l),Box::new(r))}
pub fn  new_shl(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Shl,Box::new(l),Box::new(r))}
pub fn  new_shr(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Shr,Box::new(l),Box::new(r))}
pub fn  new_and(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::And,Box::new(l),Box::new(r))}
pub fn   new_or(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Or ,Box::new(l),Box::new(r))}
pub fn  new_xor(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Xor,Box::new(l),Box::new(r))}

pub fn    new_eq(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Eq  ,Box::new(l),Box::new(r))}
pub fn   new_neq(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Neq ,Box::new(l),Box::new(r))}
pub fn    new_lt(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Lt  ,Box::new(l),Box::new(r))}
pub fn  new_lteq(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Lteq,Box::new(l),Box::new(r))}
pub fn    new_gt(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Gt  ,Box::new(l),Box::new(r))}
pub fn  new_gteq(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::Gteq,Box::new(l),Box::new(r))}

pub fn  new_and_logical(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::LogicalAnd,Box::new(l),Box::new(r))}
pub fn   new_or_logical(l: Expression, r: Expression)-> Expression{Expression::Binary(BinaryOperator::LogicalOr ,Box::new(l),Box::new(r))}

pub fn   new_accs(s: Expression, name: &str)-> Expression{Expression::Access(Box::new(s),name.to_string())}
pub fn  new_subsc(s: Expression, i: Expression)-> Expression{Expression::Subscript(Box::new(s),Box::new(i))}
pub fn    new_cal(f: Expression, args: Vec<Expression>)-> Expression{Expression::Call(Box::new(f),args)}


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
compile(&self, dst_name: &str, scope: &Scope, buf: &mut Vec<Operation>)-> Result<TypeInfo,()>
{
  let  mut namer = Namer::new(dst_name);

  let  dst = Destination{name: namer.get()};

  self.compile_main(dst,scope,buf,&mut namer)
}


fn
compile_sub(&self, src: &Source, scope: &Scope, buf: &mut Vec<Operation>, namer: &mut Namer)-> Result<TypeInfo,()>
{
  let  dst = Destination{name: src.name.clone()};

  self.compile_main(dst,scope,buf,namer)
}


fn
compile_main(&self, dst: Destination, scope: &Scope, buf: &mut Vec<Operation>, namer: &mut Namer)-> Result<TypeInfo,()>
{
    match self
    {
  Expression::None=>{Err(())},
  Expression::Identifier(p)=>
        {
          let  s = p.to_string();

                 if s ==  "true"{   buf.push(Operation::ImmU(dst,1));  return Ok(TypeInfo::new_bool())}
            else if s == "false"{   buf.push(Operation::ImmU(dst,0));  return Ok(TypeInfo::new_bool())}
            else
              {
                Err(())
              }
        },
  Expression::Boolean(b)=>  {  buf.push(Operation::ImmU(dst,if *b{1}else{0}));  Ok(TypeInfo::new_bool())},
  Expression::Integer(u)=>  {  buf.push(Operation::ImmU(dst,*u));  Ok(TypeInfo::new_ulit())},
  Expression::Floating(f)=> {  buf.push(Operation::ImmF(dst,*f));  Ok(TypeInfo::new_flit())},
  Expression::String(s)=>   {  /*buf.push(Operation::LoadS(dst,s.clone()));*/  Ok(TypeInfo::new_str_lit())},
  Expression::SubExpression(e)=>
        {
          e.compile_main(dst,scope,buf,namer)
        },
  Expression::Unary(o,e)=>
        {
          let  src = Source{name: namer.get()};

            if let Ok(ti) = e.compile_sub(&src,scope,buf,namer)
            {
                if let Ok(new_ti) = o.compile(dst,src,ti,buf)
                {
                  return Ok(new_ti);
                }
            }


          Err(())
        },
  Expression::Call(f,args)=>
        {
          let  src = Source{name: namer.get()};

            if let Ok(ti) = f.compile_sub(&src,scope,buf,namer)
            {
              let  mut src_ls: Vec<Source> = Vec::new();

                for i in 0..args.len()
                {
                  let  arg_src = Source{name: namer.get()};

                    if let Ok(a_ti) = args[i].compile_sub(&arg_src,scope,buf,namer)
                    {
                      src_ls.push(arg_src);
                    }

                  else
                    {
                      return Err(());
                    }
                }


              buf.push(Operation::CallNonVoid(dst,0,src,src_ls));
            }


          Err(())
        },
  Expression::Subscript(target,index)=>
        {
          let  src1 = Source{name: namer.get()};
          let  src2 = Source{name: namer.get()};

            if let Ok(t_ti) = target.compile_sub(&src1,scope,buf,namer)
            {
                if let Ok(i_ti) = index.compile_sub(&src2,scope,buf,namer)
                {
                  buf.push(Operation::Subscript(dst,src1,src2));
                }
            }


          return Err(());
        },
  Expression::Access(target,name)=>
        {
          let  src = Source{name: namer.get()};

            if let Ok(ti) = target.compile_sub(&src,scope,buf,namer)
            {
              buf.push(Operation::Access(dst,src,0));

              Ok(ti)
            }

          else
            {
              Err(())
            }
        },
  Expression::Binary(o,l,r)=>
        {
          let  l_src = Source{name: namer.get()};
          let  r_src = Source{name: namer.get()};

            if let Ok(l_ti) = l.compile_sub(&l_src,scope,buf,namer)
            {
                if let Ok(r_ti) = r.compile_sub(&r_src,scope,buf,namer)
                {
                    if let Ok(ti) = o.compile(dst,l_src,l_ti,r_src,r_ti,buf)
                    {
                      return Ok(ti);
                    }
                }
            }


          return Err(());
        },
    }
}


pub fn
is_valid(&self)-> bool
{
  if let Expression::None = self{false} else{true}
}


pub fn
is_empty(&self)-> bool
{
  if let Expression::None = self{true} else{false}
}


pub fn
print(&self)
{
    match self
    {
  Expression::None=>{},
  Expression::Identifier(p)=>{p.print();},
  Expression::Boolean(b)=>{print!("{}",b);},
  Expression::Integer(u)=>{print!("{}",u);},
  Expression::Floating(f)=>{print!("{}",f);},
  Expression::String(s)=>{print!("\"{}\"",s);},
  Expression::SubExpression(e)=>
        {
          print!("(");
          e.print();
          print!(")");
        },
  Expression::Unary(o,e)=>
        {
          o.print();
          e.print();
        },
  Expression::Call(f,args)=>
        {
          f.print();

          print!("(");

            for e in args
            {
              e.print();

              print!(", ");
            }


          print!(")");
        },
  Expression::Subscript(target,index)=>
        {
          target.print();

          print!("[");

          index.print();

          print!("]");
        },
  Expression::Access(target,name)=>
        {
          target.print();

          print!(".{}",name);
        },
  Expression::Binary(o,l,r)=>
        {
          l.print();
          o.print();
          r.print();
        },
    }
}


}




