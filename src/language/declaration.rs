

pub mod read_expression;
pub mod read_statement;
pub mod read_declaration;
pub mod read_type;
pub mod expression_dictionary;
pub mod statement_dictionary;
pub mod typesystem_dictionary;

use std::cell::Cell;

use super::{
  WORD_SIZE,
  get_aligned_size,
};

use super::expression::{
  Path,
  Expression,

};

use super::typesystem::{
  Ty,
  Field,
  Enumerator,

};

use super::operation::{
  Source,
  Destination,
  Operation,

};

use super::statement::{
  Statement,

};




pub enum
Value
{
  U64(u64),
  I64(i64),
  F64(f64),
  String(Vec<u8>),

}


impl
Value
{


pub fn
print(&self)
{
    match self
    {
  Self::U64(v)=>{print!("{}",v)}
  Self::I64(v)=>{print!("{}",v)}
  Self::F64(v)=>{print!("{}",v)}
  Self::String(_)=>{print!("[...]")}
    }
}


}




pub struct
Variable
{
  pub(crate) ty: Ty,

  pub(crate) expression_opt: Option<Expression>,
  pub(crate)      value_opt: Option<Value>,

}


impl
Variable
{


pub fn
print(&self)
{
  self.ty.print();

    if let Some(e) = &self.expression_opt
    {
      print!(" = ");

      e.print();
    }
}


}




pub struct
Signature
{
  pub(crate) return_ty: Ty,
  pub(crate) parameter_list: Vec<Ty>,

}


pub struct
Function
{
  pub(crate) parameter_list: Vec<(String,Ty)>,

  pub(crate) return_ty: Ty,

  pub(crate) signature_opt: Option<Signature>,

  pub(crate) statement_list: Vec<Statement>,

}


impl
Function
{


pub fn
new()-> Function
{
  Function{
    parameter_list: Vec::new(),

    return_ty: Ty::Void,

    signature_opt: None,

    statement_list: Vec::new(),
  }
}


pub fn
set_parameter_list(mut self, ls: Vec<(String,Ty)>)-> Function
{
  self.parameter_list = ls;

  self
}


pub fn
set_statement_list(mut self, ls: Vec<Statement>)-> Function
{
  self.statement_list = ls;

  self
}


pub fn
add_statement(mut self, stmt: Statement)-> Function
{
  self.statement_list.push(stmt);

  self
}


pub fn
set_return_ty(mut self, ty: Ty)-> Function
{
  self.return_ty = ty;

  self
}


}




pub enum
Component
{
  Dummy,

  Fn(Function),

  Var(Variable),
  Static(Variable),
  Const(Variable),

  Space(Space),

  Struct(Vec<Field>),
  Union(Vec<Field>),
  Enum(Vec<Enumerator>),

  Type(Ty),

}




pub struct
Declaration
{
  pub(crate) name: String,

  pub(crate) component: Component,

}


impl
Declaration
{


pub fn
new(name: String, com: Component)-> Self
{
  Self{name: name, component: com}
}


pub fn
print(&self)
{
    match &self.component
    {
  Component::Dummy=>{print!("{}(DUMMY)",&self.name)}
  Component::Space(sp)=>
        {
          print!("space\n{}",&self.name);

          sp.print();
        }
  Component::Fn(f)=>
        {
          print!("fn\n{}(",&self.name);

            for (p_name,p_ty) in &f.parameter_list
            {
              print!("{}: ",&p_name);

              p_ty.print();

              print!(", ");
            }


          print!(")-> ");

          f.return_ty.print();

          print!("\n{{\n");

          super::statement::Statement::print_statement_list(&f.statement_list,1);

          print!("}}");
        },
  Component::Var(s)=>
        {
          print!("var  {}: ",&self.name);

          s.print();
        },
  Component::Static(s)=>
        {
          print!("static  {}: ",&self.name);

          s.print();
        },
  Component::Const(s)=>
        {
          print!("const  {}: ",&self.name);

          s.print();
        },
  Component::Struct(ls)=>
        {
          print!("struct {}{{",&self.name);
          print!("}}");
        },
  Component::Union(ls)=>
        {
          print!("union {}{{",&self.name);
          print!("}}");
        },
  Component::Enum(ls)=>
        {
          print!("enum {}{{",&self.name);
          print!("}}");
        },
  Component::Type(ty)=>
        {
          print!("{}: ",&self.name);

          ty.print();
        },
    }
}


}




pub struct
Symbol
{
  pub(crate) complete_flag: bool,

  pub(crate) prefix_path: Path,

  pub(crate) declaration_ptr: *mut Declaration,

  pub(crate) index: usize,

  pub(crate) offset: usize,

  pub(crate) dependence_list: Vec<usize>,

}




pub struct
Space
{
  pub(crate) declaration_list: Vec<Declaration>,

}


impl
Space
{


pub fn 
new()-> Self
{
  let  mut sp = Self{
    declaration_list: Vec::new(),
  };


  sp
}


pub fn 
add_new_dummy(&mut self, name: &str)
{
  self.declaration_list.push(Declaration::new(name.to_string(),Component::Dummy))
}


pub fn 
add_new_space(&mut self, name: &str)-> &mut Space
{
  let  decl = Declaration::new(name.to_string(),Component::Space(Space::new()));

  self.declaration_list.push(decl);

    if let Component::Space(sp) = &mut self.declaration_list.last_mut().unwrap().component
    {
      return sp;
    }


  panic!();
}


pub fn 
get_space_mut(&mut self, name: &str)-> &mut Space
{
    for decl in &mut self.declaration_list
    {
        if decl.name == name
        {
            if let Component::Space(sp) = &mut decl.component
            {
              return sp;
            }


          break;
        }
    }


  panic!();
}


pub fn
find_declaration_by_name(&self, name: &str)-> Option<&Declaration>
{
    for decl in &self.declaration_list
    {
        if decl.name == name
        {
          return Some(decl);
        }
    }


  None
}




pub fn
append_from_str(&mut self, s: &str)
{
  use crate::syntax::dictionary::Dictionary;

  let       dic = self::statement_dictionary::get_dictionary();
  let  expr_dic = self::expression_dictionary::get_dictionary();
  let    ty_dic = self::typesystem_dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![expr_dic,ty_dic];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"primary_statement",Some(dics))
    {
      let  mut cur = crate::syntax::Cursor::new(&dir);

        while let Some(decl_d) = cur.get_directory()
        {
            if let Ok(di) = crate::language::declaration::read_declaration::read_declaration(decl_d)
            {
              self.declaration_list.push(di);

              cur.advance(1);
            }

          else
            {
            }
        }


      return;
    }


  println!("make_from_string error: parse is failed");
}


fn
make_symbol_table_internal(&mut self, prefix: &mut Path, table: &mut Vec<Symbol>)
{
    for decl in &mut self.declaration_list
    {
        if let Component::Space(sp) = &mut decl.component
        {
          prefix.push(&decl.name);

          sp.make_symbol_table_internal(prefix,table);

          let  _ = prefix.pop();
        }

      else
        {
          let  index = table.len();

          let  sym = Symbol{
            complete_flag: false,
            prefix_path: prefix.clone(),
            declaration_ptr: decl as *mut Declaration,
            index,
            offset: 0,
            dependence_list: Vec::new(),
          };

          table.push(sym);
        }
    }
}


fn
process_function(table: &Vec<Symbol>, f: &mut Function, mod_flag_ref: &mut bool, comp_flag_ref: &mut bool)
{
    if let None = &mut f.signature_opt
    {
    }


  panic!();
}


fn
process_variable(table: &Vec<Symbol>, var: &mut Variable, mod_flag_ref: &mut bool, comp_flag_ref: &mut bool)
{
}


fn
process_symbol(table: &Vec<Symbol>, sym: &mut Symbol)-> bool
{
  let  mut mod_flag = false;
  let  comp_flag_ref = &mut sym.complete_flag;

  let  decl = unsafe{&mut *sym.declaration_ptr};

    match &mut decl.component
    {
  Component::Fn(f)=>
        {
          Self::process_function(table,f,&mut mod_flag,comp_flag_ref);
        },
  Component::Var(v)=>
        {
          Self::process_variable(table,v,&mut mod_flag,comp_flag_ref);
        },
  Component::Static(v)=>
        {
          Self::process_variable(table,v,&mut mod_flag,comp_flag_ref);
        },
  Component::Const(v)=>
        {
          Self::process_variable(table,v,&mut mod_flag,comp_flag_ref);
        },
  Component::Struct(ls)=>
        {
        },
  Component::Union(ls)=>
        {
        },
  Component::Enum(ls)=>
        {
        },
  Component::Type(ty)=>
        {
        },
  _=>{}
    }


  mod_flag
}


fn
process_symbol_table(table: &mut Vec<Symbol>)-> Option<()>
{
  let  mut flag: usize = 0;

  let  mut tmp_sym = Symbol{
    complete_flag: false,
    prefix_path: Path::new(),
    declaration_ptr: std::ptr::null_mut(),
    index: 0,
    offset: 0,
    dependence_list: Vec::new(),
  };


    for i in 0..table.len()
    {
        if !table[i].complete_flag
        {
          std::mem::swap(&mut tmp_sym,&mut table[i]);

            if Self::process_symbol(table,&mut tmp_sym)
            {
              flag |= 1;
            }


          std::mem::swap(&mut tmp_sym,&mut table[i]);
        }
    }


  if flag != 0{Some(())} else{None}
}


pub fn
check_dependency(table: &mut Vec<Symbol>)
{
}


pub fn
make_symbol_table(&mut self)-> Vec<Symbol>
{
  let  mut prefix = Path::new();
  let  mut table: Vec<Symbol> = Vec::new();

  self.make_symbol_table_internal(&mut prefix,&mut table);
  Self::check_dependency(&mut table);

//    while let Some(()) = Self::process_symbol_table(&mut table)
    {
    }


  table
}


pub fn
print(&self)
{
  print!("\n{{\n");

    for decl in &self.declaration_list
    {
      decl.print();

      print!("\n\n");
    }


  print!("}}\n\n");
}


}




