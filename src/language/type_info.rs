

use super::expression::{
  Expression,

};


use super::memory::{
  Memory,

};


const WORD_SIZE: usize = 8;


#[derive(Clone)]
pub struct
Align
{
  value: usize,
}


impl
Default for Align
{


fn
default()-> Self
{
  Self{value: WORD_SIZE}
}


}


impl
Align
{


pub fn
correct(&self, sz: usize)-> usize
{
  if self.value != 0{(sz+(self.value-1))/self.value*self.value} else{sz}
}


}




pub struct
Parameter
{
  pub(crate)      name: String,
  pub(crate) type_kind: TypeKind,

}




pub enum
SymbolKind
{
  Null,
  Type(TypeInfo),
  Variable(TypeInfo,usize),

}




pub struct
SymbolNode
{
  pub(crate) previous_ptr: *const SymbolNode,

  pub(crate) name: String,

  pub(crate) kind: SymbolKind,

}


impl
SymbolNode
{


pub fn
find_any(&self, name: &str)-> Option<&SymbolKind>
{
    if &self.name == name
    {
      return Some(&self.kind);
    }


    if self.previous_ptr != std::ptr::null()
    {
      return unsafe{&*self.previous_ptr}.find_any(name);
    }


  None
}


pub fn
find_type(&self, name: &str)-> Option<&TypeInfo>
{
    if &self.name == name
    {
        if let SymbolKind::Type(ti) = &self.kind
        {
          return Some(ti);
        }
    }


    if self.previous_ptr != std::ptr::null()
    {
      return unsafe{&*self.previous_ptr}.find_type(name);
    }


  None
}


}




#[derive(Clone)]
pub enum
NumberKind
{
    SignedInt(usize),
  UnsignedInt(usize),
  Float(usize),

    SignedSize,
  UnsignedSize,

    IntLiteral,
  FloatLiteral,

}


pub enum
TypeKind
{
  Unknown,

  Void, Bool, Char,
  Number(NumberKind),
  Array(Box<TypeKind>,Expression),
  Tuple(Vec<Parameter>),
  Struct(Vec<Parameter>),
  Union(Vec<Parameter>),
  Enum(String),
  Pointer(Box<TypeKind>),
  Reference(Box<TypeKind>),
  FunctionReference(Vec<Parameter>,Box<TypeKind>),

  External(String),

}


impl
TypeKind
{


/*
pub fn
calculate_unary(m: &mut Memory, o: &UnaryOperator, v: &Value, const_list: &Vec<Const>)-> Value
{
    match o
    {
  UnaryOperator::Neg=>{Value::neg(v)},
  UnaryOperator::Not=>{Value::not(v)},
  UnaryOperator::LogicalNot=>{Value::logical_not(v)},
  _=>{Value::Undefined},
    }
}


pub fn
calculate_binary(m: &mut Memory, o: &BinaryOperator, lv: &Value, rv: &Value, const_list: &Vec<Const>)-> Value
{
    match o
    {
  BinaryOperator::Add=>{Value::add(lv,rv)},
  BinaryOperator::Sub=>{Value::sub(lv,rv)},
  BinaryOperator::Mul=>{Value::mul(lv,rv)},
  BinaryOperator::Div=>{Value::div(lv,rv)},
  BinaryOperator::Rem=>{Value::rem(lv,rv)},
  BinaryOperator::Shl=>{Value::shl(lv,rv)},
  BinaryOperator::Shr=>{Value::shr(lv,rv)},
  BinaryOperator::And=>{Value::and(lv,rv)},
  BinaryOperator::Or=>{Value::or(lv,rv)},
  BinaryOperator::Xor=>{Value::xor(lv,rv)},
  BinaryOperator::Eq=>{Value::eq(lv,rv)},
  BinaryOperator::Neq=>{Value::neq(lv,rv)},
  BinaryOperator::Lt=>{Value::lt(lv,rv)},
  BinaryOperator::Lteq=>{Value::lteq(lv,rv)},
  BinaryOperator::Gt=>{Value::gt(lv,rv)},
  BinaryOperator::Gteq=>{Value::gteq(lv,rv)},
  BinaryOperator::LogicalAnd=>{Value::logical_and(lv,rv)},
  BinaryOperator::LogicalOr=>{Value::logical_or(lv,rv)},
    }
}

*/
pub fn
calculate(m: &mut Memory, sp: &mut usize, e: &Expression)-> Result<TypeInfo,()>
{
    match e
    {
  Expression::Identifier(s)=>
        {
               if s ==  "true"{  /*m.push_u(sp,1);*/  return Ok(TypeInfo::Bool);}
          else if s == "false"{  /*m.push_u(sp,0);*/  return Ok(TypeInfo::Bool);}
/*
          else
            if let Some(v) = Self::find_const(const_list,s)
            {
              return Ok(v);
            }
*/
        },
  Expression::Boolean(b)=>{ /* m.push_u(sp,if *b{1} else{0}); */ return Ok(TypeInfo::Bool);},
  Expression::Integer(u)=>{ /* m.push_u(sp,*u);*/  return Ok(TypeInfo::Number(NumberKind::IntLiteral));},
  Expression::Floating(f)=>{/*  m.push_f(sp,*f);*/  return Ok(TypeInfo::Number(NumberKind::FloatLiteral));},
  Expression::SubExpression(sube)=>
        {
          return Self::calculate(m,sp,e);
        },
  Expression::Unary(o,e)=>
        {
            if let Ok(ti) = Self::calculate(m,sp,e)
            {
//              return Ok(Self::calculate_unary(o,&v,const_list));
            }
        },
  Expression::Call(f,args)=>
        {
          panic!();
        },
  Expression::Subscript(target,index)=>
        {
          panic!();
        },
  Expression::Access(target,name)=>
        {
          panic!();
        },
  Expression::Binary(o,l,r)=>
        {
            if let Ok(lti) = Self::calculate(m,sp,l)
            {
                if let Ok(rti) = Self::calculate(m,sp,r)
                {
//                  return Ok(Self::calculate_binary(o,&lv,&rv,const_list));
                }
            }
        },
  _=>{}
    }


  Err(())
}


pub fn
make_ls_info_for_struct(ls: &Vec<Parameter>, nd: &SymbolNode)-> Result<(Vec<StorageInfo>,usize,Align),()>
{
  let  mut si_ls: Vec<StorageInfo> = Vec::new();

  let  mut     index: usize = 0;
  let  mut    offset: usize = 0;
  let  mut max_align: usize = 0;

    for p in ls
    {
        if let Ok(type_info) = p.type_kind.make_info(nd)
        {
          let  sz = type_info.get_size();
          let  al = type_info.get_align();

          offset = al.correct(offset);

          let  si = StorageInfo{
            name: p.name.clone(), 
            index,
            offset,
            type_info
          };


          index  +=  1;
          offset += sz;

          max_align = std::cmp::max(max_align,al.value);

          si_ls.push(si);
        }

      else
        {
          return Err(());
        }
    }


  let  al = Align{value: max_align};
  let  sz = al.correct(offset);

  Ok((si_ls,sz,al))
}


pub fn
make_ls_info_for_union(ls: &Vec<Parameter>, nd: &SymbolNode)-> Result<(Vec<StorageInfo>,usize,Align),()>
{
  let  mut si_ls: Vec<StorageInfo> = Vec::new();

  let  mut           index: usize = 0;
  let  mut        max_size: usize = 0;
  let  mut max_align_value: usize = 0;

    for p in ls
    {
        if let Ok(type_info) = p.type_kind.make_info(nd)
        {
          let  sz = type_info.get_size();
          let  al = type_info.get_align();

          let  si = StorageInfo{
            name: p.name.clone(), 
            index,
            offset: 0,
            type_info
          };


          index +=  1;

          max_size        = std::cmp::max(       max_size,      sz);
          max_align_value = std::cmp::max(max_align_value,al.value);

          si_ls.push(si);
        }

      else
        {
          return Err(());
        }
    }


  let  max_align = Align{value: max_align_value};

  max_size = max_align.correct(max_size);

  Ok((si_ls,max_size,max_align))
}


pub fn
make_fnref_info(ls: &Vec<Parameter>, ret_tk: &TypeKind, nd: &SymbolNode)-> Result<(Vec<StorageInfo>,TypeInfo),()>
{
    if let Ok((si_ls,_,_)) = Self::make_ls_info_for_struct(ls,nd)
    {
        if let Ok(ret_ti) = ret_tk.make_info(nd)
        {
          return Ok((si_ls,ret_ti));
        }
    }


  Err(())
}


pub fn
make_array_info(tk: &TypeKind, e: &Expression, nd: &SymbolNode)-> Result<TypeInfo,()>
{
    if let Ok(ti) = tk.make_info(nd)
    {
      let  mut m = Memory::new();

      let  mut sp: usize = 0;

        if let Ok(e_ti) = Self::calculate(&mut m,&mut sp,e)
        {
            if let TypeInfo::Number(k) = e_ti
            {
                match k
                {
              NumberKind::UnsignedInt(_)
             |NumberKind::UnsignedSize
             |NumberKind::IntLiteral=>
                    {
/*                      let  n = m.pop_u(&mut sp) as usize;

                      return Ok(TypeInfo::Array(Box::new(ti),n));
  */
                  }
              _=>{}
                }
            }
        }
    }


    Err(())
}


pub fn
make_info(&self, nd: &SymbolNode)-> Result<TypeInfo,()>
{
    match self
    {
  TypeKind::Unknown=>{Ok(TypeInfo::Unknown)},

  TypeKind::Void    =>{Ok(TypeInfo::Void)},
  TypeKind::Bool    =>{Ok(TypeInfo::Bool)},
  TypeKind::Char    =>{Ok(TypeInfo::Char)},
  TypeKind::Number(k)=>{Ok(TypeInfo::Number(k.clone()))},
  TypeKind::Array(tk,e)=>
        {
          Self::make_array_info(tk,e,nd)
        },
  TypeKind::Tuple(ls)=>
        {
            if let Ok((ti,sz,al)) = Self::make_ls_info_for_struct(ls,nd)
            {
              return Ok(TypeInfo::Tuple(ti,sz,al));
            }


          Err(())
        },
  TypeKind::Struct(ls)=>
        {
            if let Ok((ti,sz,al)) = Self::make_ls_info_for_struct(ls,nd)
            {
              return Ok(TypeInfo::Struct(ti,sz,al));
            }


          Err(())
        },
  TypeKind::Union(ls)=>
        {
            if let Ok((ti,sz,al)) = Self::make_ls_info_for_union(ls,nd)
            {
              return Ok(TypeInfo::Union(ti,sz,al));
            }


          Err(())
        }
  TypeKind::Enum(name)=>{Ok(TypeInfo::Enum(name.clone()))},
  TypeKind::Pointer(k)=>
        {
            if let Ok(ti) = k.make_info(nd)
            {
              return Ok(TypeInfo::Pointer(Box::new(ti)));
            }


          Err(())
        },
  TypeKind::Reference(k)=>
        {
            if let Ok(ti) = k.make_info(nd)
            {
              return Ok(TypeInfo::Reference(Box::new(ti)));
            }


          Err(())
        },
  TypeKind::FunctionReference(ls,ret_k)=>
        {
            if let Ok((si_ls,ret_ti)) = Self::make_fnref_info(ls,ret_k,nd)
            {
              return Ok(TypeInfo::FunctionReference(si_ls,Box::new(ret_ti)));
            }


          Err(())
        },
  TypeKind::External(name)=>
        {
            if let Some(ti_ref) = nd.find_type(name)
            {
              return Ok(TypeInfo::External(ti_ref as *const TypeInfo));
            }


          Err(())
        },
    }
}


}


pub enum
TypeInfo
{
  Unknown,

  Void, Bool, Char,
  Number(NumberKind),
  Array(Box<TypeInfo>,usize),
  Pointer(Box<TypeInfo>),
  Reference(Box<TypeInfo>),
  Tuple(Vec<StorageInfo>,usize,Align),
  Struct(Vec<StorageInfo>,usize,Align),
  Union(Vec<StorageInfo>,usize,Align),
  Enum(String),
  FunctionReference(Vec<StorageInfo>,Box<TypeInfo>),
  External(*const TypeInfo),

}


impl
TypeInfo
{


pub fn
get_size(&self)-> usize
{
    match self
    {
  TypeInfo::Unknown=>{0},

  TypeInfo::Void=>{0},
  TypeInfo::Bool=>{1},
  TypeInfo::Char=>{1},
  TypeInfo::Number(k)=>
        {
            match k
            {
          NumberKind::SignedInt(sz)  =>{*sz}
          NumberKind::UnsignedInt(sz)=>{*sz}
          NumberKind::Float(sz)      =>{*sz}
          NumberKind::SignedSize  =>{WORD_SIZE}
          NumberKind::UnsignedSize=>{WORD_SIZE}
          NumberKind::IntLiteral  =>{0}
          NumberKind::FloatLiteral=>{0}
            }
        },
  TypeInfo::Array(ti,n)=>{ti.get_size()* *n},
  TypeInfo::Tuple(_,sz,_)=>{*sz},
  TypeInfo::Struct(_,sz,_)=>{*sz},
  TypeInfo::Union(_,sz,_)=>{*sz},
  TypeInfo::Enum(_)=>{WORD_SIZE},
  TypeInfo::Pointer(_)=>{WORD_SIZE},
  TypeInfo::Reference(_)=>{WORD_SIZE},
  TypeInfo::FunctionReference(_,_)=>{WORD_SIZE},
  TypeInfo::External(ptr)=>{unsafe{&**ptr}.get_size()},
    }
}


pub fn
get_align(&self)-> Align
{
    match self
    {
  TypeInfo::Unknown=>{Align{value: 0}},

  TypeInfo::Void=>{Align{value: 0}},
  TypeInfo::Bool=>{Align{value: 1}},
  TypeInfo::Char=>{Align{value: 1}},
  TypeInfo::Number(k)=>
        {
            match k
            {
          NumberKind::SignedInt(sz)  =>{Align{value: *sz}}
          NumberKind::UnsignedInt(sz)=>{Align{value: *sz}}
          NumberKind::Float(sz)      =>{Align{value: *sz}}
          NumberKind::SignedSize  =>{Align::default()}
          NumberKind::UnsignedSize=>{Align::default()}
          NumberKind::IntLiteral  =>{Align{value: 0}}
          NumberKind::FloatLiteral=>{Align{value: 0}}
            }
        },
  TypeInfo::Array(ti,_)=>{ti.get_align()},
  TypeInfo::Tuple(_,_,al)=>{al.clone()},
  TypeInfo::Struct(_,_,al)=>{al.clone()},
  TypeInfo::Union(_,_,al)=>{al.clone()},
  TypeInfo::Enum(_)=>{Align::default()},
  TypeInfo::Pointer(_)=>{Align::default()},
  TypeInfo::Reference(_)=>{Align::default()},
  TypeInfo::FunctionReference(_,_)=>{Align::default()},
  TypeInfo::External(ptr)=>{unsafe{&**ptr}.get_align()},
    }
}


fn
print_ls_id_to_string(ls: &Vec<StorageInfo>, s: &mut String)
{
    for si in ls
    {
      si.type_info.print_id_to_string(s);
    }
}


pub fn
print_id_to_string(&self, s: &mut String)
{
    match self
    {
  TypeInfo::Unknown=>{s.push('_');},

  TypeInfo::Void=>{s.push('v');},
  TypeInfo::Bool=>{s.push('b');},
  TypeInfo::Char=>{s.push('c');},
  TypeInfo::Number(k)=>
        {
            match k
            {
          NumberKind::SignedInt(sz)  =>{  s.push('i');  let  t = format!("{}",8* *sz);  s.push_str(&t);}
          NumberKind::UnsignedInt(sz)=>{  s.push('u');  let  t = format!("{}",8* *sz);  s.push_str(&t);}
          NumberKind::SignedSize  =>{s.push_str("isz")}
          NumberKind::UnsignedSize=>{s.push_str("usz")}
          NumberKind::Float(sz)      =>{  s.push('f');  let  t = format!("{}",8* *sz);  s.push_str(&t);}
          NumberKind::IntLiteral  =>{s.push_str("il");}
          NumberKind::FloatLiteral=>{s.push_str("fl");}
            }
        },
  TypeInfo::Array(ti,n)=>
        {
          s.push_str("arr");

          ti.print_id_to_string(s);

          let  n_id = format!("{}",*n);

          s.push_str(&n_id);
        },
  TypeInfo::Tuple(ls,_,_)=>
        {
          s.push_str("tp");

          Self::print_ls_id_to_string(ls,s);
        },
  TypeInfo::Struct(ls,_,_)=>
        {
          s.push_str("st");

          Self::print_ls_id_to_string(ls,s);
        },
  TypeInfo::Union(ls,_,_)=>
        {
          s.push_str("un");

          Self::print_ls_id_to_string(ls,s);
        },
  TypeInfo::Enum(name)=>{s.push_str(name);},
  TypeInfo::Pointer(ti)=>
        {
          s.push_str("ptr");

          ti.print_id_to_string(s);
        },
  TypeInfo::Reference(ti)=>
        {
          s.push_str("ref");

          ti.print_id_to_string(s);
        },
  TypeInfo::FunctionReference(ls,ret_ti)=>
        {
          s.push_str("fn");

            for si in ls
            {
              si.type_info.print_id_to_string(s);
            }


          s.push_str("->");

          ret_ti.print_id_to_string(s);
        },
  TypeInfo::External(ptr)=>{unsafe{&**ptr}.print_id_to_string(s);},
    }
}


}




pub struct
StorageInfo
{
  pub(crate)   name: String,
  pub(crate)  index: usize,
  pub(crate) offset: usize,
  pub(crate) type_info: TypeInfo,

}


impl
StorageInfo
{


pub fn
new()-> Self
{
  Self{
      name: String::new(),
     index: 0,
    offset: 0,
    type_info: TypeInfo::Unknown,
  }
}


}




