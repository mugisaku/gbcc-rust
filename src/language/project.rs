

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
use super::font14::*;
use super::font8::*;
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




pub struct
StaticSet
{
  set: Vec<(SourceInfo,String,StorageInfo)>,

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
    for (_,name,inf) in &self.set
    {
        if inf.get_content() == new_s.as_bytes()
        {
          return name.clone();
        }
    }


  let  inf = StorageInfo::from_string(new_s);

  let  n = self.set.len();

  let  name = format!(".STATIC{}",n);

  self.set.push((srcinf.clone(),name.clone(),inf));

  name
}


pub fn
insert_storage(&mut self, srcinf: &SourceInfo, inf: StorageInfo)-> String
{
  let  n = self.set.len();

  let  name = format!(".STATIC{}",n);

  self.set.push((srcinf.clone(),name.clone(),inf));

  name
}


}




pub struct
Project
{
  decls: Vec<Decl>,

}


impl
Project
{


pub const fn
new()-> Self
{
  Self{
    decls: Vec::new(),

  }
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

      let  _ = self.insert(decl)?;

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
insert(&mut self, mut decl: Decl)-> Result<(),Message>
{
    if let DeclKind::Undef = decl.get_kind()
    {
      Ok(())
    }

  else
    if let DeclKind::Enum(ls) = decl.get_kind()
    {
         for (i,s) in ls.iter().enumerate()
         {
           let  const_decl = Decl::new_const(s,i as i64);

           let  _ = self.insert(const_decl)?;
         }


      Ok(())
    }

  else
   if self.find(decl.get_name()).is_some()
    {
      Err(decl.get_source_info().to_message()+format!("{}という名前は既に存在している",decl.get_name()))
    }

  else
    {
      self.decls.push(decl);

      Ok(())
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
          let  mut msg = String::new();

            for ((srcinf,s)) in ss.fail_records
            {
              msg.push_str(&format!("{} {} not found\n",&srcinf.to_string(),&s));
            }


          return Err(Message::new(msg));
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
process_data_offset(&mut self, start: usize)-> usize
{
  let  mut pos = get_word_aligned(start);

    for decl in &mut self.decls
    {
      let  mut sz = 0usize;

        match decl.get_kind()
        {
      DeclKind::Static(inf)=>
        {
          sz = inf.get_size();
        }
      DeclKind::Var(k)=>
        {
          panic!();
        }
      DeclKind::Fn(_)=>
        {
          sz = WORD_SIZE;
        }
      _=>{}
        }


      decl.set_offset(pos)     ;
                      pos += sz;

      pos = get_word_aligned(pos);
    }


  get_word_aligned(pos)
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


pub fn
compile(&mut self)-> Result<(),Message>
{
  let  mut ss = StaticSet::new();

  self.collect_static(&mut ss);

    for (srcinf,name,si) in ss.set
    {
      let  decl = Decl::new_static(srcinf,name,si);

      let  _ = self.insert(decl)?;
    }


  let  _ = self.process_deps_relationship()?;

  let  mut tplg_nodes = Vec::<TplgNode>::new();

  self.collect_as_tplg_nodes(&mut tplg_nodes);

  let  sorted_values = tplg_sort(tplg_nodes)?;

    for v in sorted_values
    {
      let  decl = unsafe{&mut *(v as *mut Decl)};

      let  _ = decl.build_const_data(self)?;
    }


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
      _=>{}
        }
    }


  Ok(())
}


pub fn
generate_exec(&mut self)-> Result<Exec,Message>
{
  let  mut exec = Exec::new_with_memory();

  let   font8_start = self.process_data_offset(256);
  let  combi8_start = get_word_aligned( font8_start+(   8*0x10000));
  let  font14_start = get_word_aligned(combi8_start+(2* 3*0x10000));
  let   stack_start = get_word_aligned(font14_start+(2*14*0x10000));

  let  stack_size = self.get_const_or("STACK_SIZE",STACK_SIZE*CORE_NUMBER);

  let  text_start = get_word_aligned(stack_start+stack_size);


  self.add_const( "FONT8_START", font8_start as i64);
  self.add_const("COMBI8_START",combi8_start as i64);
  self.add_const("FONT14_START",font14_start as i64);
  self.add_const( "STACK_START", stack_start as i64);

  let  mut pos = text_start;

  let  _ = self.write_to_exec(&mut exec,&mut pos)?;


  exec.add_symbol(Symbol::new_const_int("HEAP_START",pos as i64));

  Self::install_font8( exec.get_memory_slice_mut(font8_start ));
  Self::install_combi8(exec.get_memory_slice_mut(combi8_start));
  Self::install_font14(exec.get_memory_slice_mut(font14_start));


  Ok(exec)
}




pub fn
add_ex_img(&mut self, name: &str, w: u32, h: u32, data: &Vec<u8>)
{
  let  mut new_data = Vec::<u8>::new();

    for b in w.to_ne_bytes(){new_data.push(b);}
    for b in h.to_ne_bytes(){new_data.push(b);}

  let  mut iter = data.iter();

    while let Some(r_ref) = iter.next()
    {
      let  r = *r_ref as u32;
      let  g = *iter.next().unwrap() as u32;
      let  b = *iter.next().unwrap() as u32;
      let  _ = *iter.next().unwrap() as u32;

      let  pix = (r<<24)
                |(g<<16)
                |(b<< 8);

        for b in pix.to_ne_bytes()
        {
          new_data.push(b);
        }
    }


  let  inf = StorageInfo::from_data(new_data,TyKind::U32);

  let  decl = Decl::new_static(SourceInfo::new(),name.to_string(),inf);

  self.insert(decl);
}




fn
install_font8(dst: &mut [u8])
{
  let  mut  iter = FONT8.iter();

    while let Some(unicode) = iter.next()
    {
      let  base = (8*((*unicode) as usize));

        for i in 0..8
        {
          let  bits = (*iter.next().unwrap()) as u8;

          dst[base+i] = bits;
        }
    }
}


fn
install_combi8(dst: &mut [u8])
{
  let  mut  iter = COMBI8.iter();

    while let Some(unicode) = iter.next()
    {
      let  base = (2*((*unicode) as usize));

      let  upper = (*iter.next().unwrap()) as u16;
      let  lower = (*iter.next().unwrap()) as u16;

      let  u_bytes = upper.to_ne_bytes();
      let  l_bytes = lower.to_ne_bytes();

      dst[base  ] = u_bytes[0];
      dst[base+1] = u_bytes[1];
      dst[base+2] = l_bytes[0];
      dst[base+3] = l_bytes[1];
    }
}


fn
install_font14(dst: &mut [u8])
{
  let  mut  iter = FONT14.iter();

    while let Some(unicode) = iter.next()
    {
      const  FULLWIDTH_FIRST: usize = 0xFF01;
      const  FULLWIDTH_LAST: usize  = 0xFF5E;

      let  u = *unicode as usize;

      let  base = 2*14*u;

      let  is_fullwidth_ascii = (u >= FULLWIDTH_FIRST) && (u <= FULLWIDTH_LAST);

        for i in 0..14
        {
          let  bytes = iter.next().unwrap().to_ne_bytes();

          dst[base+(2*i)  ] = bytes[0];
          dst[base+(2*i)+1] = bytes[1];

            if is_fullwidth_ascii
            {
              let  ascii_base = 2*14*(('!' as usize)+u-FULLWIDTH_FIRST);

              dst[ascii_base+(2*i)  ] = bytes[0];
              dst[ascii_base+(2*i)+1] = bytes[1];
            }
        }
    }
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




