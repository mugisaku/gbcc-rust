

use super::allocation::{
  Allocation,
  AllocationID,
  Source,
  Destination,
  Operand,
};

use super::collection::{
  Collection,
};


use super::allocating_operation::{
  UnaryOperator,
  BinaryOperator,
  AllocatingOperation,
  PhiPair,
  JumpOperand,
  JumpDestination,
  CallInfo,
};


use super::function::{
  Function,
};

use super::memory::{
  WORD_SIZE,
};




pub enum
Line
{
  AllocatingOperation(Destination,usize,AllocatingOperation),

  CopyWord(Destination,Source),
  CopyString(Destination,Source,usize),
  Message(String),
  Print(Source,char),
  Label(String),
  Jump(JumpDestination),
  Branch(Source,JumpDestination,JumpDestination),
  Return(Option<(Source,usize)>),

}


#[allow(dead_code)]
impl
Line
{


pub fn
get_allocation_data(&self)-> Option<(String,usize)>
{
    if let Line::AllocatingOperation(dst,sz,_) = self
    {
      return Some((dst.name.clone(),*sz));
    }


  None
}


pub fn
link_to_function(&mut self, f_name_ls: &Vec<String>)-> Result<(),()>
{
    if let Line::AllocatingOperation(_,_,ao) = self
    {
        if let AllocatingOperation::Call(ci) = ao
        {
            for i in 0..f_name_ls.len()
            {
                if f_name_ls[i] == ci.function_name
                {
                  ci.function_index = i;

                  return Ok(());
                }
            }


          return Err(());
        }
    }


  Ok(())
}


pub fn
link_to_label(&mut self, ls: &Vec<(String,usize)>)-> Result<(),()>
{
    match self
    {
  Line::AllocatingOperation(dst,_,ao)=>
        {
            if let AllocatingOperation::Phi(o_ls,_) = ao
            {
                for o in o_ls
                {
                    if o.from.link(ls).is_err()
                    {
                      return Err(());
                    }
                }
            }


          Ok(())
        },
  Line::Jump(dst)=>
        {
          dst.link(ls)
        },
  Line::Branch(_,on_true,on_false)=>
        {
            if   on_true.link(ls).is_ok()
             && on_false.link(ls).is_ok()
            {
              Ok(())
            }

          else
            {
              Err(())
            }
        },
  _=>{Ok(())}
    }
}


pub fn
link_to_allocation(&mut self, g_alo_ls: &Vec<Allocation>, l_alo_ls: &Vec<Allocation>, para_ls: &Vec<Allocation>)-> Result<(),()>
{
    match self
    {
  Line::AllocatingOperation(dst,_,ao)=>
        {
            if  dst.link(g_alo_ls,l_alo_ls,para_ls).is_ok()
             && ao.link_to_allocation(g_alo_ls,l_alo_ls,para_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              Err(())
            }
        }
  Line::CopyWord(dst,src)=>
        {
            if  dst.link(g_alo_ls,l_alo_ls,para_ls).is_ok()
             && src.link(g_alo_ls,l_alo_ls,para_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              Err(())
            }
        },
  Line::CopyString(dst,src,_)=>
        {
            if  dst.link(g_alo_ls,l_alo_ls,para_ls).is_ok()
             && src.link(g_alo_ls,l_alo_ls,para_ls).is_ok()
            {
              Ok(())
            }

          else
            {
              Err(())
            }
        },
  Line::Print(target,_)=>
        {
          target.link(g_alo_ls,l_alo_ls,para_ls)
        },
  Line::Branch(cond,_,_)=>
        {
          cond.link(g_alo_ls,l_alo_ls,para_ls)
        },
  Line::Return(opt)=>
        {
            if let Some((src,_)) = opt
            {
              return src.link(g_alo_ls,l_alo_ls,para_ls);
            }


          return Ok(());
        },
  _=>{Ok(())}
    }
}


pub fn
print(&self)
{
    match self
    {
  Line::AllocatingOperation(dst,sz,ao)=>
        {
          print!("{} = ({})",&dst.name,*sz);

          ao.print();
        }
  Line::CopyWord(dst,src)=>
        {
          print!("copy_word {} {}",&dst.name,&src.name);
        },
  Line::CopyString(dst,src,sz)=>
        {
          print!("copy_string  {} {} {}",&dst.name,&src.name,*sz);
        },
  Line::Message(s)=>
        {
          print!("message \"{}\"",s);
        },
  Line::Print(target,c)=>
        {
          print!("print {} {}",&target.name,c);
        },
  Line::Label(name)=>
        {
          print!("LABEL {}:",name);
        },
  Line::Jump(dst)=>
        {
          print!("jmp {}",&dst.name);
        },
  Line::Branch(cond,on_true,on_false)=>
        {
          print!("br {} {} {}",&cond.name,&on_true.name,&on_false.name);
        },
  Line::Return(opt)=>
        {
          print!("ret");

            if let Some((src,sz)) = opt
            {
              print!(" {} {}",&src.name,sz);
            }
        },
    }
}




}




pub struct
LineList
{
  pub(crate) content: Vec<Line>

}


impl
LineList
{


pub fn
new()-> LineList
{
  LineList{content: Vec::new()}
}


fn
add_un(&mut self, dst: &str, sz: usize, o: &str, u: UnaryOperator)
{
  let  ao = AllocatingOperation::Unary(Source::new(o),u);

  self.content.push(Line::AllocatingOperation(Destination::new(dst),sz,ao));
}


pub fn         add_exs8(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::ExS8);}
pub fn        add_exs16(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::ExS16);}
pub fn        add_exs32(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::ExS32);}
pub fn        add_exf32(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::ExF32);}
pub fn         add_stof(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::StoF);}
pub fn         add_ftos(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::FtoS);}
pub fn          add_not(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::Not);}
pub fn  add_logical_not(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::LogicalNot);}
pub fn          add_neg(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::Neg);}
pub fn         add_negf(&mut self, dst: &str, o: (&str,)){self.add_un(dst,WORD_SIZE,o.0,UnaryOperator::NegF);}




pub fn
add_bin(&mut self, dst: &str, sz: usize, o: (&str,&str), b: BinaryOperator)
{
  let  ao = AllocatingOperation::Binary(Source::new(o.0),Source::new(o.1),b);

  self.content.push(Line::AllocatingOperation(Destination::new(dst),sz,ao));
}


pub fn  add_addi(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::AddI)}
pub fn  add_addu(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::AddU)}
pub fn  add_addf(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::AddF)}
pub fn  add_subi(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::SubI)}
pub fn  add_subu(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::SubU)}
pub fn  add_subf(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::SubF)}
pub fn  add_muli(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::MulI)}
pub fn  add_mulu(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::MulU)}
pub fn  add_mulf(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::MulF)}
pub fn  add_divi(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::DivI)}
pub fn  add_divu(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::DivU)}
pub fn  add_divf(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::DivF)}
pub fn  add_remi(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::RemI)}
pub fn  add_remu(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::RemU)}
pub fn  add_remf(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::RemF)}
pub fn  add_shl(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::Shl)}
pub fn  add_shr(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::Shr)}
pub fn  add_and(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::And)}
pub fn   add_or(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::Or)}
pub fn  add_xor(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::Xor)}


pub fn     add_eq(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::Eq)}
pub fn    add_neq(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::Neq)}
pub fn    add_lti(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::LtI)}
pub fn    add_ltu(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::LtU)}
pub fn    add_ltf(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::LtF)}
pub fn  add_lteqi(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::LteqI)}
pub fn  add_ltequ(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::LteqU)}
pub fn  add_lteqf(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::LteqF)}
pub fn    add_gti(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::GtI)}
pub fn    add_gtu(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::GtU)}
pub fn    add_gtf(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::GtF)}
pub fn  add_gteqi(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::GteqI)}
pub fn  add_gtequ(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::GteqU)}
pub fn  add_gteqf(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::GteqF)}

pub fn  add_logical_and(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::LogicalAnd)}
pub fn   add_logical_or(&mut self, dst: &str, o: (&str,&str)){self.add_bin(dst,WORD_SIZE,o,BinaryOperator::LogicalOr)}


pub fn
add_alo(&mut self, dst: &str, sz: usize)
{
  self.content.push(Line::AllocatingOperation(Destination::new(dst),sz,AllocatingOperation::Allocate));
}


pub fn
add_movu64(&mut self, dst: &str, u: u64)
{
  self.content.push(Line::AllocatingOperation(Destination::new(dst),WORD_SIZE,AllocatingOperation::MoveU64(u)));
}


pub fn
add_movf64(&mut self, dst: &str, f: f64)
{
  self.content.push(Line::AllocatingOperation(Destination::new(dst),WORD_SIZE,AllocatingOperation::MoveF64(f)));
}


pub fn
add_addr(&mut self, dst: &str, src: (&str,))
{
  self.content.push(Line::AllocatingOperation(Destination::new(dst),WORD_SIZE,AllocatingOperation::Address(Source::new(src.0))));
}


pub fn
add_phi(&mut self, dst: &str, sz: usize, ops: Vec<PhiPair>, defau: &str)
{
  let  ao = AllocatingOperation::Phi(ops,Source::new(defau));

  self.content.push(Line::AllocatingOperation(Destination::new(dst),sz,ao));
}


pub fn
add_cal(&mut self, dst: &str, ret_sz: usize, target: (&str,Vec<Source>))
{
  let  ci = CallInfo{function_name: String::from(target.0), function_index: 0, argument_list: target.1};

  self.content.push(Line::AllocatingOperation(Destination::new(dst),ret_sz,AllocatingOperation::Call(ci)));
}


pub fn
add_cpyw(&mut self, dst: &str, src: (&str,))
{
  let  dst_al = Destination::new(dst);
  let  src_al =      Source::new(src.0);

  self.content.push(Line::CopyWord(dst_al,src_al));
}


pub fn
add_cpys(&mut self, dst: &str, src: (&str,usize))
{
  let  dst_al = Destination::new(dst);
  let  src_al =      Source::new(src.0);

  self.content.push(Line::CopyString(dst_al,src_al,src.1));
}


pub fn
add_msg(&mut self, s: &str)
{
  self.content.push(Line::Message(String::from(s)));
}


pub fn
add_pr(&mut self, s: &str, c: char)
{
  self.content.push(Line::Print(Source::new(s),c));
}


pub fn
add_lb(&mut self, name: &str)
{
  self.content.push(Line::Label(String::from(name)));
}


pub fn
add_jmp(&mut self, name: &str)
{
  self.content.push(Line::Jump(JumpDestination::new(name)));
}


pub fn
add_br(&mut self, cond: &str, dst: (&str,&str))
{
  self.content.push(Line::Branch(Source::new(cond),JumpDestination::new(dst.0),JumpDestination::new(dst.1)));
}


pub fn
add_ret(&mut self)
{
  self.content.push(Line::Return(None));
}


pub fn
add_retval(&mut self, src: &str, sz: usize)
{
  let  opt: Option<(Source,usize)> = Some((Source::new(src),sz));

  self.content.push(Line::Return(opt));
}




}




