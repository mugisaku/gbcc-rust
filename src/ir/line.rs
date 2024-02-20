

use super::allocation::{
  Allocation,
  AllocationID,
  Source,
  Destination,
  Operand,
};

use super::collection::{
  DestinationTable,
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




#[derive(Clone)]
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
link_to_allocation(&mut self, tbl: &DestinationTable)-> Result<(),()>
{
    match self
    {
  Line::AllocatingOperation(dst,_,ao)=>
        {
            if  dst.link(tbl).is_ok()
             && ao.link_to_allocation(tbl).is_ok()
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
            if  dst.link(tbl).is_ok()
             && src.link(tbl).is_ok()
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
            if  dst.link(tbl).is_ok()
             && src.link(tbl).is_ok()
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
          target.link(tbl)
        },
  Line::Branch(cond,_,_)=>
        {
          cond.link(tbl)
        },
  Line::Return(opt)=>
        {
            if let Some((src,_)) = opt
            {
              return src.link(tbl);
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




fn
new_un(dst: &str, sz: usize, o: &str, u: UnaryOperator)-> Line
{
  let  ao = AllocatingOperation::Unary(Source::new(o),u);

  Line::AllocatingOperation(Destination::new(dst),sz,ao)
}


pub fn         new_exs8(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::ExS8)}
pub fn        new_exs16(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::ExS16)}
pub fn        new_exs32(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::ExS32)}
pub fn        new_exf32(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::ExF32)}
pub fn         new_stof(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::StoF)}
pub fn         new_ftos(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::FtoS)}
pub fn          new_not(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::Not)}
pub fn  new_logical_not(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::LogicalNot)}
pub fn          new_neg(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::Neg)}
pub fn         new_negf(dst: &str, o: (&str,))-> Line{Self::new_un(dst,WORD_SIZE,o.0,UnaryOperator::NegF)}




pub fn
new_bin(dst: &str, sz: usize, o: (&str,&str), b: BinaryOperator)-> Line
{
  let  ao = AllocatingOperation::Binary(Source::new(o.0),Source::new(o.1),b);

  Line::AllocatingOperation(Destination::new(dst),sz,ao)
}


pub fn  new_addi(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::AddI)}
pub fn  new_addu(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::AddU)}
pub fn  new_addf(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::AddF)}
pub fn  new_subi(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::SubI)}
pub fn  new_subu(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::SubU)}
pub fn  new_subf(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::SubF)}
pub fn  new_muli(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::MulI)}
pub fn  new_mulu(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::MulU)}
pub fn  new_mulf(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::MulF)}
pub fn  new_divi(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::DivI)}
pub fn  new_divu(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::DivU)}
pub fn  new_divf(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::DivF)}
pub fn  new_remi(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::RemI)}
pub fn  new_remu(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::RemU)}
pub fn  new_remf(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::RemF)}
pub fn  new_shl(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::Shl)}
pub fn  new_shr(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::Shr)}
pub fn  new_and(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::And)}
pub fn   new_or(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::Or)}
pub fn  new_xor(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::Xor)}


pub fn     new_eq(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::Eq)}
pub fn    new_neq(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::Neq)}
pub fn    new_lti(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::LtI)}
pub fn    new_ltu(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::LtU)}
pub fn    new_ltf(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::LtF)}
pub fn  new_lteqi(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::LteqI)}
pub fn  new_ltequ(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::LteqU)}
pub fn  new_lteqf(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::LteqF)}
pub fn    new_gti(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::GtI)}
pub fn    new_gtu(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::GtU)}
pub fn    new_gtf(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::GtF)}
pub fn  new_gteqi(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::GteqI)}
pub fn  new_gtequ(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::GteqU)}
pub fn  new_gteqf(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::GteqF)}

pub fn  new_logical_and(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::LogicalAnd)}
pub fn   new_logical_or(dst: &str, o: (&str,&str))-> Line{Self::new_bin(dst,WORD_SIZE,o,BinaryOperator::LogicalOr)}


pub fn
new_alo(dst: &str, sz: usize)-> Line
{
  Line::AllocatingOperation(Destination::new(dst),sz,AllocatingOperation::Allocate)
}


pub fn
new_movu64(dst: &str, u: u64)-> Line
{
  Line::AllocatingOperation(Destination::new(dst),WORD_SIZE,AllocatingOperation::MoveU64(u))
}


pub fn
new_movf64(dst: &str, f: f64)-> Line
{
  Line::AllocatingOperation(Destination::new(dst),WORD_SIZE,AllocatingOperation::MoveF64(f))
}


pub fn
new_addr(dst: &str, src: (&str,))-> Line
{
  Line::AllocatingOperation(Destination::new(dst),WORD_SIZE,AllocatingOperation::Address(Source::new(src.0)))
}


pub fn
new_phi(dst: &str, sz: usize, ops: Vec<PhiPair>, defau: &str)-> Line
{
  let  ao = AllocatingOperation::Phi(ops,Source::new(defau));

  Line::AllocatingOperation(Destination::new(dst),sz,ao)
}


pub fn
new_cal(dst: &str, ret_sz: usize, target: (&str,Vec<Source>))-> Line
{
  let  ci = CallInfo{function_name: String::from(target.0), function_index: 0, argument_list: target.1};

  Line::AllocatingOperation(Destination::new(dst),ret_sz,AllocatingOperation::Call(ci))
}


pub fn
new_cpyw(dst: &str, src: (&str,))-> Line
{
  let  dst_al = Destination::new(dst);
  let  src_al =      Source::new(src.0);

  Line::CopyWord(dst_al,src_al)
}


pub fn
new_cpys(dst: &str, src: (&str,usize))-> Line
{
  let  dst_al = Destination::new(dst);
  let  src_al =      Source::new(src.0);

  Line::CopyString(dst_al,src_al,src.1)
}


pub fn
new_msg(s: &str)-> Line
{
  Line::Message(String::from(s))
}


pub fn
new_pr(s: &str, c: char)-> Line
{
  Line::Print(Source::new(s),c)
}


pub fn
new_lb(name: &str)-> Line
{
  Line::Label(String::from(name))
}


pub fn
new_jmp(name: &str)-> Line
{
  Line::Jump(JumpDestination::new(name))
}


pub fn
new_br(cond: &str, dst: (&str,&str))-> Line
{
  Line::Branch(Source::new(cond),JumpDestination::new(dst.0),JumpDestination::new(dst.1))
}


pub fn
new_ret()-> Line
{
  Line::Return(None)
}


pub fn
new_retval(src: &str, sz: usize)-> Line
{
  let  opt: Option<(Source,usize)> = Some((Source::new(src),sz));

  Line::Return(opt)
}




}




