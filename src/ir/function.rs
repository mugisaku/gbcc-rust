

use super::memory::{
  Memory,
};


use super::block::{
  AddressSource,
  OperandLiteral,
  Operand,
  UnaryOperator,
  BinaryOperator,
  AllocatingOperation,
  NonAllocatingOperation,
  Terminator,
  VariableLink,
  BlockLink,
  Block,
  Line,
  WordCount,
};




pub struct
Parameter
{
  pub(crate) name: String,

  pub(crate) word_count: WordCount,

}


#[derive(Clone)]
pub enum
StorageClass
{
  Global,
  Static,
  Argument,
  Local,

}


#[derive()]
pub struct
VariableInfo
{
  pub(crate) name: String,

  pub(crate) storage_class: StorageClass,

  pub(crate) offset: i64,
  pub(crate)   size: u64,

  pub(crate) initial_value: Option<Memory>,

}


impl
VariableInfo
{


pub fn
print(&self)
{
  print!("{}: ({} +{})",&self.name,self.offset,self.size);
}


}




pub struct
Function
{
  pub(crate) name: String,

  pub(crate) parameter_list: Vec<Parameter>,

  pub(crate)   argument_size: u64,
  pub(crate) allocation_size: u64,

  pub(crate) variable_info_list: Vec<VariableInfo>,

  pub(crate) return_word_count: WordCount,

  pub(crate) block_list: Vec<Block>,

}


impl
Function
{


pub fn
new(name: &str, ret_wc: WordCount)-> Function
{
  let mut  f = Function{ name: String::from(name), parameter_list: Vec::new(), argument_size: 0, allocation_size: 0, variable_info_list: Vec::new(), return_word_count: ret_wc, block_list: Vec::new()};

//  f.add_block("start");

  f
}




pub fn
find_variable_info_in<'a,'b>(var_ls: &'a Vec<VariableInfo>, name: &'b str)-> Option<&'a VariableInfo>
{
    for vi in var_ls
    {
        if vi.name == name
        {
          return Some(vi);
        }
    }


  None
}


pub fn
find_variable_info(&self, name: &str)-> Option<&VariableInfo>
{
  Self::find_variable_info_in(&self.variable_info_list,name)
}


pub fn
find_block(&self, name: &str)-> Option<&Block>
{
    for blk in &self.block_list
    {
        if blk.name == name
        {
          return Some(blk);
        }
    }


  None
}


pub fn
find_block_index(&self, name: &str)-> Option<usize>
{
  let mut  i: usize = 0;

    for blk in &self.block_list
    {
        if blk.name == name
        {
          return Some(i);
        }


      i += 1;
    }


  None
}




pub fn
add_parameter(&mut self, name: &str, wc: WordCount)-> Result<(),()>
{
    if let None = self.find_variable_info(name)
    {
      let  sz = wc.get_size() as u64;

        if sz != 0
        {
          self.parameter_list.push(Parameter{ name: String::from(name), word_count: wc});

          self.argument_size += sz;

          self.variable_info_list.push(VariableInfo{ name: String::from(name), storage_class: StorageClass::Argument, offset: -(self.argument_size as i64), size: sz, initial_value: None});

          return Ok(());
        }
    }


   Err(())
}


pub fn
add_local_variable(&mut self, name: &str, wc: WordCount)-> Result<AddressSource,()>
{
    if let None = self.find_variable_info(name)
    {
      let  sz = wc.get_size() as i64;

        if sz != 0
        {
          self.variable_info_list.push(VariableInfo{ name: String::from(name), storage_class: StorageClass::Local, offset: self.allocation_size as i64, size: sz as u64, initial_value: None});

          let  addr_src = AddressSource::LocalOffset(self.allocation_size as i64);

          self.allocation_size += sz as u64;

          return Ok(addr_src);
        }
    }


   Err(())
}


pub fn
add_block(&mut self, mut blk: Block)-> Result<(),()>
{
    if let None = self.find_block(&blk.name)
    {
        for l in &mut blk.line_list
        {
            if let Line::AllocatingOperation(vl,ao) = l
            {
              let  wc = ao.get_word_count();

                if let Ok(addr_src) = self.add_local_variable(&vl.name,wc)
                {
                  vl.address_source = Some(addr_src);
                }

              else
                {
                  return Err(());
                }
            }
        }


      self.block_list.push(blk);

      return Ok(());
    }


   Err(())
}




pub fn
fix_operand(o: &mut Operand, lvar_ls: &Vec<VariableInfo>, gvar_ls: &Vec<VariableInfo>)
{
    if let OperandLiteral::Identifier(s) = &o.literal
    {
      let  name = s.clone();

        if let Some(vi) = Self::find_variable_info_in(lvar_ls,&name)
        {
            match &vi.storage_class
            {
          StorageClass::Local=>   {o.address_source = Some(AddressSource::LocalOffset(vi.offset));}
          StorageClass::Argument=>{o.address_source = Some(AddressSource::ArgumentOffset(vi.offset));}
          _=>{}
            }
        }

      else
        if let Some(vi) = Self::find_variable_info_in(gvar_ls,&name)
        {
          o.address_source = Some(AddressSource::GlobalOffset(vi.offset));
        }
    }
}


pub fn
fix_variable_address_source(vl: &mut VariableLink, lvar_ls: &Vec<VariableInfo>, gvar_ls: &Vec<VariableInfo>)
{
    if let Some(vi) = Self::find_variable_info_in(lvar_ls,&vl.name)
    {
      vl.address_source = Some(AddressSource::LocalOffset(vi.offset));
    }

  else
    if let Some(vi) = Self::find_variable_info_in(gvar_ls,&vl.name)
    {
      vl.address_source = Some(AddressSource::GlobalOffset(vi.offset));
    }
}


pub fn
fix_block_index(ln: &mut BlockLink, name_ls: &Vec<String>)
{
    for i in 0..name_ls.len()
    {
        if name_ls[i] == ln.name
        {
          ln.index = Some(i as u64);

          return;
        }
    }
}




pub fn
fix_allocating_operation(ao: &mut AllocatingOperation, lvar_ls: &Vec<VariableInfo>, gvar_ls: &Vec<VariableInfo>, name_ls: &Vec<String>)
{
    match ao
    {
  AllocatingOperation::Unary(o,_)=>
        {
          Self::fix_operand(o,lvar_ls,gvar_ls);
        },
  AllocatingOperation::Binary(l,r,_)=>
        {
          Self::fix_operand(l,lvar_ls,gvar_ls);
          Self::fix_operand(r,lvar_ls,gvar_ls);
        },
  AllocatingOperation::Allocate(wc)=>
        {
        },
  AllocatingOperation::Copy(o)=>
        {
          Self::fix_operand(o,lvar_ls,gvar_ls);
        },
  AllocatingOperation::Load(vl,_)=>
        {
          Self::fix_variable_address_source(vl,lvar_ls,gvar_ls);
        },
  AllocatingOperation::Address(vl)=>
        {
          Self::fix_variable_address_source(vl,lvar_ls,gvar_ls);
        },
  AllocatingOperation::Phi(ops)=>
        {
            for po in ops
            {
              Self::fix_block_index(&mut po.from,name_ls);
              Self::fix_operand(&mut po.value,lvar_ls,gvar_ls);
            }
        },
  AllocatingOperation::Call(ci)=>
        {
          Self::fix_variable_address_source(&mut ci.target,lvar_ls,gvar_ls);

            for a in &mut ci.argument_list
            {
              Self::fix_operand(a,lvar_ls,gvar_ls);
            }
        },
    }
}


pub fn
fix_non_allocating_operation(nao: &mut NonAllocatingOperation, lvar_ls: &Vec<VariableInfo>, gvar_ls: &Vec<VariableInfo>)
{
    match nao
    {
  NonAllocatingOperation::Store(dst,src,sz)=>
        {
          Self::fix_variable_address_source(src,lvar_ls,gvar_ls);
          Self::fix_variable_address_source(dst,lvar_ls,gvar_ls);
        },
    }
}


pub fn
fix_terminator(tm: &mut Terminator, lvar_ls: &Vec<VariableInfo>, gvar_ls: &Vec<VariableInfo>, name_ls: &Vec<String>)
{
    match tm
    {
  Terminator::Jump(bl)=>
        {
          Self::fix_block_index(bl,name_ls);
        },
  Terminator::Branch(bi)=>
        {
          Self::fix_variable_address_source(&mut bi.condition,lvar_ls,gvar_ls);
          Self::fix_block_index(&mut bi.on_true ,name_ls);
          Self::fix_block_index(&mut bi.on_false,name_ls);
        },
  Terminator::Return(o_opt)=>
        {
            if let Some(o) = o_opt
            {
              Self::fix_operand(o,lvar_ls,gvar_ls);
            }
        },
    }
}


pub fn
fix(&mut self, gvar_ls: &Vec<VariableInfo>)
{
  let mut  name_ls: Vec<String> = Vec::new();

    for blk in &self.block_list
    {
      name_ls.push(blk.name.clone());
    }


    for blk in &mut self.block_list
    {
        for l in &mut blk.line_list
        {
            match l
            {
          Line::AllocatingOperation(vl,ao)=>
                {
                  Self::fix_variable_address_source(vl,&self.variable_info_list,gvar_ls);

                  Self::fix_allocating_operation(ao,&self.variable_info_list,gvar_ls,&name_ls);
                }
          Line::NonAllocatingOperation(nao)=>
                {
                  Self::fix_non_allocating_operation(nao,&self.variable_info_list,gvar_ls);
                }
            }
        }


        if let Some(tm) = &mut blk.terminator
        {
          Self::fix_terminator(tm,&self.variable_info_list,gvar_ls,&name_ls);
        }
    }
}


pub fn
print(&self)
{
  print!("fn\n{}(",&self.name);

    for p in &self.parameter_list
    {
      print!(" {}: ({} bytes),",&p.name,p.word_count.get_size());
    }

  print!(")->");

  self.return_word_count.print();

  print!("\nargument size: {} bytes",self.argument_size);
  print!("\nallocation size: {} bytes",self.allocation_size);

  print!("\n{{\n");

    for blk in &self.block_list
    {
      blk.print();
    }


  print!("\n}}\n");
}


}





