

use super::type_info::{
  Parameter,
  TypeKind,
  TypeInfo,
  StorageInfo,

};


use super::expression::{
  Expression,
};


use super::memory::{
  Memory,
};


use super::compile_for_block::{
  compile,
};


use super::symbol::{
  SymbolDirectory,

};


use super::statement::{
  Statement,
  Block,
  For,

};


use super::dictionary::{
  get_dictionary

};


use super::read::{
  read_declaration,

};


use super::evaluator::{
  ExpressionEvaluator,

};




pub enum
Declaration
{
  Fn(FunctionDecl),
  Let(VariableDecl),
  Const(VariableDecl),

}




pub struct
FunctionDecl
{
  pub(crate) name: String,

  pub(crate) parameter_list: Vec<Parameter>,

  pub(crate) return_type_kind: TypeKind,

  pub(crate) block: Block,

}


impl
FunctionDecl
{


pub fn
get_processed_parameter_list(&self, dir: &SymbolDirectory)-> Vec<(String,TypeInfo)>
{
  let  mut ls: Vec<(String,TypeInfo)> = Vec::new();

    for p in &self.parameter_list
    {
        if let Ok(ti) = p.type_kind.make_info(dir)
        {
          ls.push((p.name.clone(),ti));
        }

      else
        {
          panic!();
        }
    }


  ls
}


pub fn
get_return_type_info(&self, dir: &SymbolDirectory)-> TypeInfo
{
    if let Ok(ti) = self.return_type_kind.make_info(dir)
    {
      return ti;
    }

  else
    {
      panic!();
    }
}


pub fn
print(&self)
{
  print!("{}(",&self.name);

    for para in &self.parameter_list
    {
      print!("{},",&para.name);
    }


  print!(")-> ");

  self.return_type_kind.print();

  print!("\n");

  self.block.print();
}


}




pub struct
TypeDecl
{
  pub(crate) name: String,
  pub(crate) type_kind: TypeKind,

}


impl
TypeDecl
{


pub fn
print(&self)
{
  print!("{}: ",&self.name);

  self.type_kind.print();

  print!(" = ");
}


}




pub struct
VariableDecl
{
  pub(crate) name: String,
  pub(crate) type_kind: TypeKind,
  pub(crate) expression: Expression,

}


impl
VariableDecl
{


pub fn
print(&self)
{
  print!("{}: ",&self.name);

  self.type_kind.print();

  print!(" = ");

  self.expression.print();
}


}




pub enum
SymbolSource<'a>
{
  Type(&'a TypeDecl),
  Const(&'a VariableDecl),

}




pub struct
Space
{
  pub(crate)  const_list: Vec<VariableDecl>,
  pub(crate)    let_list: Vec<VariableDecl>,
  pub(crate)     fn_list: Vec<FunctionDecl>,

}


impl
Space
{


pub fn
new()-> Self
{
  Self{
     const_list: Vec::new(),
       let_list: Vec::new(),
        fn_list: Vec::new(),
  }
}


pub fn
read(&mut self, s: &str)
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

  let  dics: Vec<&Dictionary> = vec![];

    if let Ok(dir) = crate::syntax::parse::parse_from_string(s,dic,"declaration",Some(dics))
    {
      let  mut cur = crate::syntax::Cursor::new(&dir);

        while let Some(d_dir) = cur.get_directory()
        {
          let  decl = super::read::read_declaration(d_dir);

            match decl
            {
          Declaration::Fn(f)=>
                {
                  self.fn_list.push(f);
                }
          Declaration::Let(v)=>
                {
                  self.let_list.push(v);
                }
          Declaration::Const(v)=>
                {
                  self.const_list.push(v);
                }
            }


          cur.advance(1);
        }
    }

  else
    {
      println!("Space::read error: parse is failed");
    }
}


pub fn
calculate(e: &Expression, dir: &SymbolDirectory)-> Result<(Vec<u8>,TypeInfo),()>
{
  let  mut ee = ExpressionEvaluator::new();

  ee.reset(e,dir);

  ee.run();

  Ok(ee.get_final_value_and_type_info())
}


pub fn
process_source(src: &SymbolSource, dir: &mut SymbolDirectory)-> Result<(),()>
{
    match src
    {
  SymbolSource::Type(decl)=>
        {
            if let Ok(ti) = decl.type_kind.make_info(dir)
            {
              dir.add_type(&decl.name,ti);

              return Ok(());
            }
        }
  SymbolSource::Const(decl)=>
        {
            if let Ok((b,e_ti)) = Self::calculate(&decl.expression,dir)
            {
                if let Ok(ti) = decl.type_kind.make_info(dir)
                {
                  dir.add_constant(&decl.name,ti,b);
                }


              return Ok(());
            }
        }
  _=>{}
    }


  Err(())
}


pub fn
process_source_list(mut tmp_ls: Vec<SymbolSource>, dir: &mut SymbolDirectory)
{
    if tmp_ls.len() == 0
    {
      return;
    }


  let  mut err_ls: Vec<SymbolSource> = Vec::new();

  let  mut last_len: usize = tmp_ls.len();

    loop
    {
        while let Some(symsrc) = tmp_ls.pop()
        {
            if Self::process_source(&symsrc,dir).is_err()
            {
              err_ls.push(symsrc);
            }
        }


      let  cur_len = err_ls.len();

        if cur_len == 0
        {
          break;
        }

      else
        if cur_len == last_len
        {
          panic!();
        }


      last_len = cur_len;

      std::mem::swap(&mut tmp_ls,&mut err_ls);
    }
}


fn
append_basic_types(dir: &mut SymbolDirectory)
{
  dir.add_type("void",TypeInfo::Void);
  dir.add_type("bool",TypeInfo::Bool);
  dir.add_type(   "i8",TypeInfo::I8);
  dir.add_type(  "i16",TypeInfo::I16);
  dir.add_type(  "i32",TypeInfo::I32);
  dir.add_type(  "i64",TypeInfo::I64);
  dir.add_type("isize",TypeInfo::ISize);
  dir.add_type(   "u8",TypeInfo::U8);
  dir.add_type(  "u16",TypeInfo::U16);
  dir.add_type(  "u32",TypeInfo::U32);
  dir.add_type(  "u64",TypeInfo::U64);
  dir.add_type("usize",TypeInfo::USize);
  dir.add_type(  "f32",TypeInfo::F32);
  dir.add_type(  "f64",TypeInfo::F64);
}


pub fn
compile(&mut self)
{
  let  mut root_dir = SymbolDirectory::new_as_root();

  Self::append_basic_types(&mut root_dir);

  let  mut ls: Vec<SymbolSource> = Vec::new();

    for v in &self.const_list
    {
      ls.push(SymbolSource::Const(v));
    }


  Self::process_source_list(ls,&mut root_dir);

    for fndecl in &self.fn_list
    {
      let      ls = fndecl.get_processed_parameter_list(&root_dir);
      let  ret_ti = fndecl.get_return_type_info(&root_dir);

      root_dir.add_function(&fndecl.name,ls,ret_ti);

//      let  symblk = SymbolBlock::new(&mut root_dir, &fndecl.block);

//      buf.push(symblk);
    }
}


pub fn
print(&self)
{
    for v in &self.let_list
    {
      print!("let  ");

      v.print();

      print!(";\n");
    }


    for v in &self.const_list
    {
      print!("const  ");

      v.print();

      print!(";\n");
    }


    for f in &self.fn_list
    {
      print!("fn  ");

      f.print();
    }
}


}




