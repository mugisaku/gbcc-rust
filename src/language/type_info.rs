

use super::expression::{
  Expression,

};


use super::literal::{
  Literal,

};


use super::constant::{
  Constant,

};


use super::symbol::{
  SymbolDirectory,

};


use super::memory::{
  Memory,

};


use super::evaluator::{
  ExpressionEvaluator,

};


pub const WORD_SIZE: usize = 8;


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


impl
Parameter
{


pub fn
print(&self)
{
  print!("{}: ",&self.name);

  self.type_kind.print();
}


}




pub enum
TypeKind
{
  Unknown,

  Void, Bool,
  I8, I16, I32, I64, ISize,
  U8, U16, U32, U64, USize,
  F32, F64,
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


pub fn
make_ls_info_for_struct(ls: &Vec<Parameter>, dir: &SymbolDirectory)-> Result<(Vec<StorageInfo>,usize,Align),()>
{
  let  mut si_ls: Vec<StorageInfo> = Vec::new();

  let  mut     index: usize = 0;
  let  mut    offset: usize = 0;
  let  mut max_align: usize = 0;

    for p in ls
    {
        if let Ok(type_info) = p.type_kind.make_info(dir)
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
make_ls_info_for_union(ls: &Vec<Parameter>, dir: &SymbolDirectory)-> Result<(Vec<StorageInfo>,usize,Align),()>
{
  let  mut si_ls: Vec<StorageInfo> = Vec::new();

  let  mut           index: usize = 0;
  let  mut        max_size: usize = 0;
  let  mut max_align_value: usize = 0;

    for p in ls
    {
        if let Ok(type_info) = p.type_kind.make_info(dir)
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
make_fnref_info(ls: &Vec<Parameter>, ret_tk: &TypeKind, dir: &SymbolDirectory)-> Result<(Vec<StorageInfo>,TypeInfo),()>
{
    if let Ok((si_ls,_,_)) = Self::make_ls_info_for_struct(ls,dir)
    {
        if let Ok(ret_ti) = ret_tk.make_info(dir)
        {
          return Ok((si_ls,ret_ti));
        }
    }


  Err(())
}


pub fn
make_array_info(tk: &TypeKind, e: &Expression, dir: &SymbolDirectory)-> Result<TypeInfo,()>
{
    if let Ok(ti) = tk.make_info(dir)
    {
      let  mut ee = ExpressionEvaluator::new();

      ee.reset(e,dir);

      ee.run();

        if let TypeInfo::USize = &ee.final_value_type_info
        {
          let  n = ee.get_final_value_as_usize();

          return Ok(TypeInfo::Array(Box::new(ti),n));
        }
    }


  Err(())
}


pub fn
make_info(&self, dir: &SymbolDirectory)-> Result<TypeInfo,()>
{
    match self
    {
  TypeKind::Unknown=>{Ok(TypeInfo::Unknown)},

  TypeKind::Void    =>{Ok(TypeInfo::Void)},
  TypeKind::Bool    =>{Ok(TypeInfo::Bool)},
  TypeKind::I8=>{Ok(TypeInfo::I8)},
  TypeKind::I16=>{Ok(TypeInfo::I16)},
  TypeKind::I32=>{Ok(TypeInfo::I32)},
  TypeKind::I64=>{Ok(TypeInfo::I64)},
  TypeKind::ISize=>{Ok(TypeInfo::ISize)},
  TypeKind::U8=>{Ok(TypeInfo::U8)},
  TypeKind::U16=>{Ok(TypeInfo::U16)},
  TypeKind::U32=>{Ok(TypeInfo::U32)},
  TypeKind::U64=>{Ok(TypeInfo::U64)},
  TypeKind::USize=>{Ok(TypeInfo::USize)},
  TypeKind::F32=>{Ok(TypeInfo::F32)},
  TypeKind::F64=>{Ok(TypeInfo::F64)},
  TypeKind::Array(tk,e)=>
        {
          Self::make_array_info(tk,e,dir)
        },
  TypeKind::Tuple(ls)=>
        {
            if let Ok((ti,sz,al)) = Self::make_ls_info_for_struct(ls,dir)
            {
              return Ok(TypeInfo::Tuple(ti,sz,al));
            }


          Err(())
        },
  TypeKind::Struct(ls)=>
        {
            if let Ok((ti,sz,al)) = Self::make_ls_info_for_struct(ls,dir)
            {
              return Ok(TypeInfo::Struct(ti,sz,al));
            }


          Err(())
        },
  TypeKind::Union(ls)=>
        {
            if let Ok((ti,sz,al)) = Self::make_ls_info_for_union(ls,dir)
            {
              return Ok(TypeInfo::Union(ti,sz,al));
            }


          Err(())
        }
  TypeKind::Enum(name)=>{Ok(TypeInfo::Enum(name.clone()))},
  TypeKind::Pointer(k)=>
        {
            if let Ok(ti) = k.make_info(dir)
            {
              return Ok(TypeInfo::Pointer(Box::new(ti)));
            }


          Err(())
        },
  TypeKind::Reference(k)=>
        {
            if let Ok(ti) = k.make_info(dir)
            {
              return Ok(TypeInfo::Reference(Box::new(ti)));
            }


          Err(())
        },
  TypeKind::FunctionReference(ls,ret_k)=>
        {
            if let Ok((si_ls,ret_ti)) = Self::make_fnref_info(ls,ret_k,dir)
            {
              return Ok(TypeInfo::FunctionReference(si_ls,Box::new(ret_ti)));
            }


          Err(())
        },
  TypeKind::External(name)=>
        {
            if let Some(ti_ref) = dir.find_type(name)
            {
              return Ok(TypeInfo::External(ti_ref as *const TypeInfo));
            }


          Err(())
        },
    }
}


pub fn
print(&self)
{
    match self
    {
  TypeKind::Unknown=>{print!("unknown");},

  TypeKind::Void    =>{print!("void");},
  TypeKind::Bool    =>{print!("bool");},
  TypeKind::I8=>{print!("i8");},
  TypeKind::I16=>{print!("i16");},
  TypeKind::I32=>{print!("i32");},
  TypeKind::I64=>{print!("i64");},
  TypeKind::ISize=>{print!("isize");},
  TypeKind::U8=>{print!("u8");},
  TypeKind::U16=>{print!("u16");},
  TypeKind::U32=>{print!("u32");},
  TypeKind::U64=>{print!("u64");},
  TypeKind::USize=>{print!("usize");},
  TypeKind::F32=>{print!("f32");},
  TypeKind::F64=>{print!("f64");},
  TypeKind::Array(tk,e)=>
        {
          print!("[");

          e.print();

          print!("]");

          tk.print();
        },
  TypeKind::Tuple(ls)=>
        {
          print!("(");

            for p in ls
            {
              p.print();

              print!(", ");
            }

          print!(")-> ");
        },
  TypeKind::Struct(ls)=>
        {
          print!("struct{{");

            for p in ls
            {
              p.print();

              print!(", ");
            }

          print!("}}");
        },
  TypeKind::Union(ls)=>
        {
          print!("union{{");

            for p in ls
            {
              p.print();

              print!(", ");
            }

          print!("}}");
        }
  TypeKind::Enum(name)=>{print!("enum {}",name);},
  TypeKind::Pointer(k)=>
        {
          print!("*");

          k.print();
        },
  TypeKind::Reference(k)=>
        {
          print!("&");

          k.print();
        },
  TypeKind::FunctionReference(ls,ret_k)=>
        {
          print!("(");

            for p in ls
            {
              p.type_kind.print();

              print!(", ");
            }

          print!(")-> ");

          ret_k.print();
        },
  TypeKind::External(name)=>
        {
          print!("{}",name);
        },
    }
}


}


#[derive(Clone)]
pub enum
TypeInfo
{
  Unknown,

  Void, Bool,
  I8, I16, I32, I64, ISize,
  U8, U16, U32, U64, USize,
  F32, F64,
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
is_unknown(&self)-> bool
{
  if let TypeInfo::Unknown = self{true} else{false}
}


pub fn
is_bool(&self)-> bool
{
  if let TypeInfo::Bool = self{true} else{false}
}


pub fn
is_comparable(&self)-> bool
{
    match self
    {
  TypeInfo::I8 |TypeInfo::I16|TypeInfo::I32|TypeInfo::I64|TypeInfo::ISize
 |TypeInfo::U8 |TypeInfo::U16|TypeInfo::U32|TypeInfo::U64|TypeInfo::USize
 |TypeInfo::F32|TypeInfo::F64
 |TypeInfo::Pointer(_)=>{true}
  _=>{false}
    }
}


pub fn
is_number(&self)-> bool
{
    match self
    {
  TypeInfo::I8 |TypeInfo::I16|TypeInfo::I32|TypeInfo::I64|TypeInfo::ISize
 |TypeInfo::U8 |TypeInfo::U16|TypeInfo::U32|TypeInfo::U64|TypeInfo::USize
 |TypeInfo::F32|TypeInfo::F64=>{true}
  _=>{false}
    }
}


pub fn
is_int(&self)-> bool
{
    match self
    {
  TypeInfo::I8
 |TypeInfo::I16
 |TypeInfo::I32
 |TypeInfo::I64
 |TypeInfo::ISize=>{true}
  _=>{false}
    }
}


pub fn
is_uint(&self)-> bool
{
    match self
    {
  TypeInfo::U8
 |TypeInfo::U16
 |TypeInfo::U32
 |TypeInfo::U64
 |TypeInfo::USize=>{true}
  _=>{false}
    }
}


pub fn
is_float(&self)-> bool
{
    match self
    {
  TypeInfo::F32
 |TypeInfo::F64=>{true}
  _=>{false}
    }
}


pub fn
get_size(&self)-> usize
{
    match self
    {
  TypeInfo::Unknown=>{0},

  TypeInfo::Void=>{0},
  TypeInfo::Bool=>{1},
  TypeInfo::I8=>{1},
  TypeInfo::I16=>{2},
  TypeInfo::I32=>{4},
  TypeInfo::I64=>{8},
  TypeInfo::ISize=>{8},
  TypeInfo::U8=>{1},
  TypeInfo::U16=>{2},
  TypeInfo::U32=>{4},
  TypeInfo::U64=>{8},
  TypeInfo::USize=>{8},
  TypeInfo::F32=>{4},
  TypeInfo::F64=>{8},
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
  TypeInfo::I8=>{Align{value: 1}},
  TypeInfo::I16=>{Align{value: 2}},
  TypeInfo::I32=>{Align{value: 4}},
  TypeInfo::I64=>{Align{value: 8}},
  TypeInfo::ISize=>{Align{value: 8}},
  TypeInfo::U8=>{Align{value: 1}},
  TypeInfo::U16=>{Align{value: 2}},
  TypeInfo::U32=>{Align{value: 4}},
  TypeInfo::U64=>{Align{value: 8}},
  TypeInfo::USize=>{Align{value: 8}},
  TypeInfo::F32=>{Align{value: 4}},
  TypeInfo::F64=>{Align{value: 8}},
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
  TypeInfo::I8=>{s.push_str("i8");},
  TypeInfo::I16=>{s.push_str("i16");},
  TypeInfo::I32=>{s.push_str("i32");},
  TypeInfo::I64=>{s.push_str("i64");},
  TypeInfo::ISize=>{s.push_str("isize");},
  TypeInfo::U8=>{s.push_str("u8");},
  TypeInfo::U16=>{s.push_str("u16");},
  TypeInfo::U32=>{s.push_str("u32");},
  TypeInfo::U64=>{s.push_str("u64");},
  TypeInfo::USize=>{s.push_str("usize");},
  TypeInfo::F32=>{s.push_str("f32");},
  TypeInfo::F64=>{s.push_str("f64");},
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


pub fn
get_id(&self)-> String
{
  let  mut s = String::new();

  self.print_id_to_string(&mut s);

  s
}


pub fn
print(&self)
{
    match self
    {
  TypeInfo::Unknown=>{print!("unknown");},

  TypeInfo::Void=>{print!("void");},
  TypeInfo::Bool=>{print!("bool");},
  TypeInfo::I8=>{print!("i8");},
  TypeInfo::I16=>{print!("i16");},
  TypeInfo::I32=>{print!("i32");},
  TypeInfo::I64=>{print!("i64");},
  TypeInfo::ISize=>{print!("isize");},
  TypeInfo::U8=>{print!("u8");},
  TypeInfo::U16=>{print!("u16");},
  TypeInfo::U32=>{print!("u32");},
  TypeInfo::U64=>{print!("u64");},
  TypeInfo::USize=>{print!("usize");},
  TypeInfo::F32=>{print!("f32");},
  TypeInfo::F64=>{print!("f64");},
  TypeInfo::Array(ti,n)=>
        {
        },
  TypeInfo::Tuple(ls,_,_)=>
        {
        },
  TypeInfo::Struct(ls,_,_)=>
        {
        },
  TypeInfo::Union(ls,_,_)=>
        {
        },
  TypeInfo::Enum(name)=>{print!("{}",name);},
  TypeInfo::Pointer(ti)=>
        {
          print!("");
        },
  TypeInfo::Reference(ti)=>
        {
          print!("");
        },
  TypeInfo::FunctionReference(ls,ret_ti)=>
        {
          print!("");

            for si in ls
            {
            }


          print!("");
        },
  TypeInfo::External(ptr)=>{unsafe{&**ptr}.print();},
    }
}


}




#[derive(Clone)]
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




#[derive(Clone)]
pub enum
ValueKind
{
  Literal(Literal),
  Constant(Constant),
  Variable,
  Dereference,
  Moved,

}


#[derive(Clone)]
pub struct
ValueInfo
{
  pub(crate)      kind: ValueKind,
  pub(crate) type_info: TypeInfo,

}


impl
ValueInfo
{


pub fn      new_literal(l: Literal)-> Self{Self{kind: ValueKind::Literal(l), type_info: TypeInfo::Void}}
pub fn     new_constant(c: Constant)-> Self{Self{kind: ValueKind::Constant(c), type_info: TypeInfo::Void}}
pub fn     new_variable(type_info: TypeInfo)-> Self{Self{kind: ValueKind::Variable   , type_info}}
pub fn  new_dereference(type_info: TypeInfo)-> Self{Self{kind: ValueKind::Dereference, type_info}}
pub fn        new_moved(type_info: TypeInfo)-> Self{Self{kind: ValueKind::Moved      , type_info}}

pub fn      is_literal(&self)-> bool{if let ValueKind::Literal(_)  = &self.kind{true} else{false}}
pub fn     is_constant(&self)-> bool{if let ValueKind::Constant(_) = &self.kind{true} else{false}}
pub fn     is_variable(&self)-> bool{if let ValueKind::Variable    = &self.kind{true} else{false}}
pub fn  is_dereference(&self)-> bool{if let ValueKind::Dereference = &self.kind{true} else{false}}
pub fn        is_moved(&self)-> bool{if let ValueKind::Moved       = &self.kind{true} else{false}}


}






