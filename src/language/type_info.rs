

use super::expression::{
  Expression,

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

  Definition(String),

}


impl
TypeKind
{


pub fn
calculate(e: &Expression)-> Result<usize,()>
{
  Ok(0)
}


pub fn
make_ls_info_for_struct(ls: &Vec<Parameter>)-> Result<(Vec<StorageInfo>,usize,Align),()>
{
  let  mut si_ls: Vec<StorageInfo> = Vec::new();

  let  mut     index: usize = 0;
  let  mut    offset: usize = 0;
  let  mut max_align: usize = 0;

    for p in ls
    {
        if let Ok(type_info) = p.type_kind.make_info()
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
make_ls_info_for_union(ls: &Vec<Parameter>)-> Result<(Vec<StorageInfo>,usize,Align),()>
{
  let  mut si_ls: Vec<StorageInfo> = Vec::new();

  let  mut           index: usize = 0;
  let  mut        max_size: usize = 0;
  let  mut max_align_value: usize = 0;

    for p in ls
    {
        if let Ok(type_info) = p.type_kind.make_info()
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
make_fnref_info(ls: &Vec<Parameter>, ret_tk: &TypeKind)-> Result<(Vec<StorageInfo>,TypeInfo),()>
{
    if let Ok((si_ls,_,_)) = Self::make_ls_info_for_struct(ls)
    {
        if let Ok(ret_ti) = ret_tk.make_info()
        {
          return Ok((si_ls,ret_ti));
        }
    }


  Err(())
}


pub fn
make_info(&self)-> Result<TypeInfo,()>
{
    match self
    {
  TypeKind::Unknown=>{Ok(TypeInfo::Unknown)},

  TypeKind::Void    =>{Ok(TypeInfo::Void)},
  TypeKind::Bool    =>{Ok(TypeInfo::Bool)},
  TypeKind::Char    =>{Ok(TypeInfo::Char)},
  TypeKind::Number(k)=>{Ok(TypeInfo::Number(k.clone()))},
  TypeKind::Array(k,e)=>
        {
            if let Ok(ti) = k.make_info()
            {
                if let Ok(n) = Self::calculate(e)
                {
                  return Ok(TypeInfo::Array(Box::new(ti),n));
                }
            }


          Err(())
        },
  TypeKind::Tuple(ls)=>
        {
            if let Ok((ti,sz,al)) = Self::make_ls_info_for_struct(ls)
            {
              return Ok(TypeInfo::Tuple(ti,sz,al));
            }


          Err(())
        },
  TypeKind::Struct(ls)=>
        {
            if let Ok((ti,sz,al)) = Self::make_ls_info_for_struct(ls)
            {
              return Ok(TypeInfo::Struct(ti,sz,al));
            }


          Err(())
        },
  TypeKind::Union(ls)=>
        {
            if let Ok((ti,sz,al)) = Self::make_ls_info_for_union(ls)
            {
              return Ok(TypeInfo::Union(ti,sz,al));
            }


          Err(())
        }
  TypeKind::Enum(name)=>{Ok(TypeInfo::Enum(name.clone()))},
  TypeKind::Pointer(k)=>
        {
            if let Ok(ti) = k.make_info()
            {
              return Ok(TypeInfo::Pointer(Box::new(ti)));
            }


          Err(())
        },
  TypeKind::Reference(k)=>
        {
            if let Ok(ti) = k.make_info()
            {
              return Ok(TypeInfo::Reference(Box::new(ti)));
            }


          Err(())
        },
  TypeKind::FunctionReference(ls,ret_k)=>
        {
            if let Ok((si_ls,ret_ti)) = Self::make_fnref_info(ls,ret_k)
            {
              return Ok(TypeInfo::FunctionReference(si_ls,Box::new(ret_ti)));
            }


          Err(())
        },
  TypeKind::Definition(name)=>{Err(())},
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




