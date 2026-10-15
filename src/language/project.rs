

use crate::node::*;

use crate::source_file::{
  SourceInfo,
  Message,

};

use super::machine::{
  CORE_NUMBER,
   STACK_SIZE,

};


use crate::syntax::*;
use super::*;
use super::expr::*;
use super::stmt::*;
use super::scope::*;
use super::assemble::assemble;
use super::asm::Opcode;
use super::tplg_sort::*;
use super::evaluate::*;
use super::decl::*;
use super::exec::*;




pub struct
StringSet
{
  set: Vec<String>,

  fail_records: Vec<(SourceInfo,String)>,

}


impl
StringSet
{


pub fn
new()-> Self
{
  Self{set: Vec::new(), fail_records: Vec::new()}
}


pub fn
insert(&mut self, new_s: &str)
{
    for s in &self.set
    {
        if s == new_s
        {
          return;
        }
    }


  self.set.push(new_s.to_string());
}


pub fn
record_fail(&mut self, srcinf: &SourceInfo, s: &str)
{
  self.fail_records.push((srcinf.clone(),s.to_string()));
}


}




pub enum
StaticObjectKind
{
     String(String),
  U16String(String),
  U32String(String),

  Var(VarDecl),

}


pub struct
StaticSet
{
  set: Vec<(SourceInfo,String,StaticObjectKind)>,

}


impl
StaticSet
{


pub fn
new()-> Self
{
  Self{set: Vec::new()}
}


pub fn
insert_string(&mut self, srcinf: &SourceInfo, new_s: &str)-> String
{
    for (_,name,k) in &self.set
    {
        if let StaticObjectKind::String(s) = k
        {
            if s == new_s
            {
              return name.clone();
            }
        }
    }


  let  name = format!(".STATIC{}",self.set.len());

  let  k = StaticObjectKind::String(new_s.to_string());

  self.set.push((srcinf.clone(),name.clone(),k));

  name
}


pub fn
insert_u16string(&mut self, srcinf: &SourceInfo, new_s: &str)-> String
{
    for (_,name,k) in &self.set
    {
        if let StaticObjectKind::U16String(s) = k
        {
            if s == new_s
            {
              return name.clone();
            }
        }
    }


  let  name = format!(".STATIC{}",self.set.len());

  let  k = StaticObjectKind::U16String(new_s.to_string());

  self.set.push((srcinf.clone(),name.clone(),k));

  name
}


pub fn
insert_u32string(&mut self, srcinf: &SourceInfo, new_s: &str)-> String
{
    for (_,name,k) in &self.set
    {
        if let StaticObjectKind::U32String(s) = k
        {
            if s == new_s
            {
              return name.clone();
            }
        }
    }


  let  name = format!(".STATIC{}",self.set.len());

  let  k = StaticObjectKind::U32String(new_s.to_string());

  self.set.push((srcinf.clone(),name.clone(),k));

  name
}


pub fn
insert_var(&mut self, srcinf: &SourceInfo, v: VarDecl)-> String
{
  let  name = format!(".STATIC{}",self.set.len());

  let  k = StaticObjectKind::Var(v);

  self.set.push((srcinf.clone(),name.clone(),k));

  name
}


}




pub struct
Project
{
  decls: Vec<Decl>,

  size: usize,

}


impl
Project
{


pub const fn
new()-> Self
{
  Self{
    decls: Vec::new(),
    size: 0,

  }
}


pub fn
clear(&mut self)
{
  self.decls.clear();
}


pub fn
add_source(&mut self, name: &str, s: &str)-> Result<(),Message>
{
  use crate::syntax::dictionary::Dictionary;

  let  dic = super::dictionary::get_dictionary();

  let  nd = crate::syntax::parse::parse_from_string(name,s,dic,"declaration")?;

  let  mut cur = nd.cursor();

    while let Some(decl_nd) = cur.select_node("declaration")
    {
      let  decl = read_decl(decl_nd)?;

      self.insert(decl);

      cur.advance(1);
    }


  Ok(())
}


pub fn
find(&self, name: &str)-> Option<&Decl>
{
    for decl in &self.decls
    {
        if decl.get_name() == name
        {
          return Some(decl);
        }
    }


  None
}


pub fn
find_mut(&mut self, name: &str)-> Option<&mut Decl>
{
    for decl in &mut self.decls
    {
        if decl.get_name() == name
        {
          return Some(decl);
        }
    }


  None
}


pub fn
find_const(&self, name: &str)-> Option<i64>
{
    if let Some(decl) = self.find(name)
    {
        if let DeclKind::Const(_,v) = decl.get_kind()
        {
          return Some(*v);
        }
    }


  None
}


pub fn
add_const(&mut self, name: &str, v: i64)
{
  self.insert(Decl::new_const(name,v));
}


pub fn
insert(&mut self, mut decl: Decl)
{
    if let DeclKind::Enum(ls) = decl.get_kind()
    {
         for (i,s) in ls.iter().enumerate()
         {
           let  const_decl = Decl::new_const(s,i as i64);

           self.insert(const_decl);
         }
    }

  else
    {
      self.decls.push(decl);
    }
}


pub fn
collect_identifier(&self, ss: &mut StringSet)
{
    for decl in &self.decls
    {
      decl.collect_identifier(self,ss);
    }
}


fn
collect_static(&mut self, ss: &mut StaticSet)
{
    for decl in &mut self.decls
    {
      decl.collect_static(ss);
    }
}


fn
collect_as_tplg_nodes(&mut self, buf: &mut Vec<TplgNode>)
{
    for decl in &mut self.decls
    {
      let  value = decl as *mut Decl as usize;

      let  nd = TplgNode::new(decl.get_name(),
                              value,
                              decl.get_deps_child_names(),
                              decl.get_deps_parent_names().len());

      buf.push(nd);
    }
}


fn
process_deps_relationship(&mut self)-> Result<(),Message>
{
    for i in 0..self.decls.len()
    {
      let  mut ss = StringSet::new();

      self.decls[i].collect_identifier(self,&mut ss);

        if ss.fail_records.len() != 0
        {
          let  mut msg_s = String::new();

            for ((srcinf,s)) in ss.fail_records
            {
              msg_s.push_str(&format!("{} {} not found\n",&srcinf.to_string(),&s));
            }


          return Err(Message::from(msg_s));
        }


        for s in ss.set
        {
          let  parent_name = s;
          let   child_name = self.decls[i].get_name().clone();

            if let Some(parent) = self.find_mut(&parent_name)
            {
              parent.add_deps_child_name(child_name);

              self.decls[i].add_deps_parent_name(parent_name);
            }

          else
            {panic!();}
        }
    }


  Ok(())
}


fn
get_const_or(&mut self, s: &str, defval: usize)-> usize
{
    if let Some(v) = self.find_const(s)
    {
      v as usize
    }

  else
    {
      self.add_const(s,defval as i64);

      defval
    }
}


fn
push_to_buf<'a>(buf: &mut Vec<(&'a str,Vec<&'a SourceInfo>)>, decl: &'a Decl)
{
  let  decl_name = decl.get_name();

    if decl_name.is_empty()
    {
      return;
    }


    for (name,ls) in buf.iter_mut()
    {
        if name == decl_name
        {
          ls.push(decl.get_source_info());

          return;
        }
    }


  buf.push((decl_name,vec![decl.get_source_info()]));
}


pub fn
check_name(&self)-> Result<(),Message>
{
  let  mut msg_buf = Vec::<Message>::new();

  let  mut buf = Vec::<(&str,Vec<&SourceInfo>)>::new();

    for decl in &self.decls
    {
      Self::push_to_buf(&mut buf,decl);
    }


    for (name,ls) in buf
    {
        if ls.len() > 1
        {
          let  mut s = format!("{}が複数定義されている\n",name);

            for si in ls
            {
              let  msg_s = si.to_message().to_string();

              s.push_str("  ");

              s.push_str(&msg_s);

              s.push('\n');
            }


          msg_buf.push(Message::from(s));
        }
    }


    if msg_buf.len() != 0
    {
      let  mut final_msg = Message::new();

        for msg in msg_buf
        {
          final_msg = final_msg+msg;
        }


      return Err(final_msg);
    }


  Ok(())
}


pub fn
compile(&mut self)-> Result<(),Message>
{
  let  mut ss = StaticSet::new();

  self.collect_static(&mut ss);

    for (srcinf,name,k) in ss.set
    {
        match k
        {
      StaticObjectKind::String(s)=>
        {
          let  decl = Decl::new_string(srcinf,name,s);

          self.insert(decl);
        }
      StaticObjectKind::U16String(s)=>
        {
          let  decl = Decl::new_u16string(srcinf,name,&s);

          self.insert(decl);
        }
      StaticObjectKind::U32String(s)=>
        {
          let  decl = Decl::new_u32string(srcinf,name,&s);

          self.insert(decl);
        }
      StaticObjectKind::Var(v)=>
        {
          let  decl = Decl::new_static(srcinf,name,v);

          self.insert(decl);
        }
        }
    }


  let  _ = self.check_name()?;

  let  _ = self.process_deps_relationship()?;

  let  mut tplg_nodes = Vec::<TplgNode>::new();

  self.collect_as_tplg_nodes(&mut tplg_nodes);

  let  sorted_values = tplg_sort(tplg_nodes)?;

  let  mut off = 256usize;

    for v in sorted_values
    {
      let  decl = unsafe{&mut *(v as *mut Decl)};

      let  sz = decl.build_const_data(self)?;

      off = get_word_aligned(off);

      decl.set_offset(off);

      off += sz;
    }


  self.size = get_word_aligned(off);

  Ok(())
}


fn
write_to_exec(&self, exec: &mut Exec, pos: &mut usize)-> Result<(),Message>
{
    for decl in &self.decls
    {
        match decl.get_kind()
        {
      DeclKind::Fn(fd)=>
        {
          let   ptr_sym = Symbol::new_static(decl.get_name(),decl.get_offset() as isize,1,TyKind::I64);
          let  text_sym = Symbol::new_text(decl.get_name(),*pos as isize);

          exec.add_symbol( ptr_sym);
          exec.add_symbol(text_sym);

            match assemble(decl.get_source_info(),fd,self)
            {
          Ok(mut text)=>
            {
              text.finalize();

              let  bytes = text.to_bytes();

                if ((*pos)+bytes.len()) > Exec::MEMORY_SIZE
                {
                  return Err(Message::from("プログラムおよびデータが、容量を超えている"));
                }


              exec.put_bytes(*pos,&bytes);

              exec.add_text((decl.get_name().clone(),*pos,text));

              let  pos_bytes = pos.to_ne_bytes();

              exec.put_bytes(decl.get_offset(),&pos_bytes);

              *pos += bytes.len();
            }
          Err(msg)=>{return Err(msg+format!("関数{}のアセンブルに失敗",decl.get_name()));}
            }
        }
      DeclKind::Const(_,v)=>
        {
          exec.add_symbol(Symbol::new_const_int(decl.get_name(),*v));
        }
      DeclKind::Static(inf)=>
        {
          exec.put_bytes(decl.get_offset(),inf.get_content());

          let  sym = Symbol::new_static(decl.get_name(),decl.get_offset() as isize,inf.get_length(),inf.get_ty_kind().clone());

          exec.add_symbol(sym);
        }
      DeclKind::Var(_)=>
        {
          panic!();
        }
      DeclKind::String(s)=>
        {
          exec.put_bytes(decl.get_offset(),s.as_bytes());

          let  sym = Symbol::new_static(decl.get_name(),decl.get_offset() as isize,s.len(),TyKind::U8);

          exec.add_symbol(sym);
        }
      DeclKind::U16String(s)=>
        {
          exec.put_u16s(decl.get_offset(),s);

          let  sym = Symbol::new_static(decl.get_name(),decl.get_offset() as isize,2*s.len(),TyKind::U16);

          exec.add_symbol(sym);
        }
      DeclKind::U32String(s)=>
        {
          exec.put_u32s(decl.get_offset(),s);

          let  sym = Symbol::new_static(decl.get_name(),decl.get_offset() as isize,4*s.len(),TyKind::U32);

          exec.add_symbol(sym);
        }
      _=>{}
        }
    }


  Ok(())
}


pub fn
generate_exec(&mut self)-> Result<Exec,Message>
{
  let  mut exec = Exec::new_with_memory();

  let   stack_start = self.size;

  let  stack_size = self.get_const_or("STACK_SIZE",STACK_SIZE*CORE_NUMBER);

  let  text_start = get_word_aligned(stack_start+stack_size);


  self.add_const( "STACK_START", stack_start as i64);

  let  mut pos = text_start;

  let  _ = self.write_to_exec(&mut exec,&mut pos)?;


  exec.add_symbol(Symbol::new_const_int("HEAP_START",pos as i64));


  Ok(exec)
}




pub fn
print(&self)
{
    for decl in &self.decls
    {
      decl.print();

      println!("");
    }
}


}




