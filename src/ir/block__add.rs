

use super::block::{
  WordCount,
  VariableLink,
  Operand,
  UnaryOperator,
  BinaryOperator,
  AllocatingOperation,
  NonAllocatingOperation,
  PhiOperand,
  CallInfo,
  BranchInfo,
  BlockLink,
  Terminator,
  Line,
  Block,
};




impl
Block
{


pub fn
add_un(&mut self, dst: &str, o: Operand, u: UnaryOperator)
{
  let  ao = AllocatingOperation::Unary(o,u);

  let  l = Line::AllocatingOperation(VariableLink::new(dst),ao);

  self.line_list.push(l);
}


pub fn
add_exs8(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::ExS8);
}


pub fn
add_exs16(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::ExS16);
}


pub fn
add_exs32(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::ExS32);
}


pub fn
add_exf32(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::ExF32);
}


pub fn
add_stof(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::StoF);
}


pub fn
add_ftoS(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::FtoS);
}


pub fn
add_not(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::Not);
}


pub fn
add_logical_not(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::LogicalNot);
}


pub fn
add_neg(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::Neg);
}


pub fn
add_negf(&mut self, dst: &str, o: Operand)
{
  self.add_un(dst,o,UnaryOperator::NegF);
}




pub fn
add_bin(&mut self, dst: &str, l: Operand, r: Operand, b: BinaryOperator)
{
  let  ao = AllocatingOperation::Binary(l,r,b);

  let  l = Line::AllocatingOperation(VariableLink::new(dst),ao);

  self.line_list.push(l);
}


pub fn
add_addi(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::AddI);
}


pub fn
add_addu(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::AddU);
}


pub fn
add_addf(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::AddF);
}


pub fn
add_subi(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::SubI);
}


pub fn
add_subu(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::SubU);
}


pub fn
add_subf(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::SubF);
}


pub fn
add_muli(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::MulI);
}


pub fn
add_mulu(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::MulU);
}


pub fn
add_mulf(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::MulF);
}


pub fn
add_divi(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::DivI);
}


pub fn
add_divu(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::DivU);
}


pub fn
add_divf(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::DivF);
}


pub fn
add_remi(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::RemI);
}


pub fn
add_remu(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::RemU);
}


pub fn
add_remf(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::RemF);
}


pub fn
add_shl(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::Shl);
}


pub fn
add_shr(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::Shr);
}


pub fn
add_and(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::And);
}


pub fn
add_or(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::Or);
}


pub fn
add_xor(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::Xor);
}


pub fn
add_eq(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::Eq);
}


pub fn
add_neq(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::Neq);
}


pub fn
add_lti(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::LtI);
}


pub fn
add_ltu(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::LtU);
}


pub fn
add_ltf(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::LtF);
}


pub fn
add_lteqi(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::LteqI);
}


pub fn
add_ltequ(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::LteqU);
}


pub fn
add_lteqf(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::LteqF);
}


pub fn
add_gti(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::GtI);
}


pub fn
add_gtu(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::GtU);
}


pub fn
add_gtf(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::GtF);
}


pub fn
add_gteqi(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::GteqI);
}


pub fn
add_gtequ(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::GteqU);
}


pub fn
add_gteqf(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::GteqF);
}


pub fn
add_logical_and(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::LogicalAnd);
}


pub fn
add_logical_or(&mut self, dst: &str, l: Operand, r: Operand)
{
  self.add_bin(dst,l,r,BinaryOperator::LogicalOr);
}


pub fn
add_allocate(&mut self, dst: &str, wc: WordCount)
{
  self.line_list.push(Line::AllocatingOperation(VariableLink::new(dst),AllocatingOperation::Allocate(wc)));
}


pub fn
add_address(&mut self, dst: &str, src: &str)
{
  self.line_list.push(Line::AllocatingOperation(VariableLink::new(dst),AllocatingOperation::Address(VariableLink::new(src))));
}


pub fn
add_phi(&mut self, dst: &str, ops: Vec<PhiOperand>)
{
  self.line_list.push(Line::AllocatingOperation(VariableLink::new(dst),AllocatingOperation::Phi(ops)));
}


pub fn
add_call(&mut self, dst: &str, target: &str, wc: WordCount, args: Vec<Operand>)
{
  let  ci = CallInfo{ target: VariableLink::new(target), return_word_count: wc, argument_list: args};

  self.line_list.push(Line::AllocatingOperation(VariableLink::new(dst),AllocatingOperation::Call(ci)));
}


pub fn
add_copy_word(&mut self, dst: &str, src: &str)
{
  let  dst_vl = VariableLink::new(dst);
  let  src_vl = VariableLink::new(src);

  self.line_list.push(Line::NonAllocatingOperation(NonAllocatingOperation::CopyWord(src_vl,dst_vl)));
}


pub fn
add_copy_string(&mut self, dst: &str, src: &str, sz: usize)
{
  let  dst_vl = VariableLink::new(dst);
  let  src_vl = VariableLink::new(src);

  self.line_list.push(Line::NonAllocatingOperation(NonAllocatingOperation::CopyString(src_vl,dst_vl,sz)));
}


pub fn
add_message(&mut self, s: &str)
{
  self.line_list.push(Line::NonAllocatingOperation(NonAllocatingOperation::Message(String::from(s))));
}


pub fn
add_print(&mut self, s: &str, c: char)
{
  self.line_list.push(Line::NonAllocatingOperation(NonAllocatingOperation::Print(VariableLink::new(s),c)));
}


}





