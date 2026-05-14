

use std::rc::Rc;

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::ty::*;
use super::asm::*;
use super::symbol_table::*;
use super::scope::*;
use super::evaluate::*;
use super::evaluate_const::*;




pub struct
LabelID
{
  value: usize,
}


impl
LabelID
{


pub fn
new()-> Self
{
  Self{value: 0}
}


pub fn
make_br_label_holder(&mut self)-> BrLabelHolder
{
  let  blh = BrLabelHolder::new(self.value);

  self.value += 1;

  blh
}


pub fn
make_ctrl_label_holder(&mut self)-> CtrlLabelHolder
{
  let  clh = CtrlLabelHolder::new(self.value);

  self.value += 1;

  clh
}


}




pub struct
BrLabelHolder
{
  base: String,
  number: usize,
  label: String,

}


impl
BrLabelHolder
{


pub fn
new(id: usize)-> Self
{
  let   base = format!("L{}_",id);
  let  label = format!("L{}_1",&base);

  Self{base, number: 1, label}
}


pub fn
get_label(&self)-> &String
{
  &self.label
}


pub fn
make_end_label(&self)-> String
{
  format!("{}_END",&self.base)
}


pub fn
increment(&mut self)
{
  self.number += 1;

  self.label = format!("{}{}",&self.base,self.number);
}


}




pub struct
CtrlLabelHolder
{
     on_break: String,
  on_continue: String,

}


impl
CtrlLabelHolder
{


pub fn
new(id: usize)-> Self
{
  Self{
       on_break: format!("L{}_END",id),
    on_continue: format!("L{}_RESTART",id),
  }
}


}




fn
process_if(ifstmt: &IfStmt, tbl: &SymbolTable, ret_ty_name: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmText)
{
  let  mut blh = lid.make_br_label_holder();

  let  end_label = blh.make_end_label();

  let  txt = evaluate(ifstmt.get_condition(),tbl,Some(scp)).to_text();

    if txt.get_ty().get_name() == "bool"
    {
      output.push_eval_text(txt);

      output.push_brz(blh.get_label());

      process_block(ifstmt.get_block(),tbl,ret_ty_name,lid,clh_opt,scp,output);

      output.push_jmp(&end_label);
    }

  else
    {panic!();}


    for elif in ifstmt.get_elif_stmt_list()
    {
      let  elif_txt = evaluate(elif.get_condition(),tbl,Some(scp)).to_text();

      output.push_label(blh.get_label());

      blh.increment();

        if elif_txt.get_ty().get_name() == "bool"
        {
          output.push_eval_text(elif_txt);

          output.push_brz(blh.get_label());

          process_block(elif.get_block(),tbl,ret_ty_name,lid,clh_opt,scp,output);

          output.push_jmp(&end_label);
        }

      else
        {panic!();}
    }


    if let Some(blk) = ifstmt.get_else_block_opt()
    {
      output.push_label(blh.get_label());

      process_block(blk,tbl,ret_ty_name,lid,clh_opt,scp,output);
    }


  output.push_label(&end_label);
}


fn
process_for(forstmt: &ForStmt, tbl: &SymbolTable, ret_ty_name: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmText)
{
  let  i64_ty = find_ty("i64").unwrap();

  let  clh = lid.make_ctrl_label_holder();

  let  mut new_scp = Scope::new(scp);


  let  mut count_max_off = 0usize;

  let  mut count_max_txt = evaluate(forstmt.get_expr(),tbl,Some(scp)).to_text();

    if count_max_txt.get_ty().get_name() == i64_ty.get_name()
    {
      count_max_off = new_scp.add_var("<FOR_COUNT_MAX>",i64_ty.get_name());

      let  mut var_txt = AsmEvalText::new();

      var_txt.push_local_var(count_max_off,Rc::clone(&i64_ty));

      output.push_assign(var_txt,count_max_txt,"=");
    }

  else
    {panic!();}



  let  count_cur_off = new_scp.add_var(forstmt.get_var_name(),i64_ty.get_name());

  let  mut init_txt = AsmEvalText::new();

  init_txt.push_local_var(count_cur_off,Rc::clone(&i64_ty));
  init_txt.push_opcode(Opcode::Push0);
  init_txt.push_opcode(Opcode::St64);

  output.push_eval_text(init_txt);

  output.push_label(&clh.on_continue);


  let  mut inc_txt = AsmEvalText::new();

  inc_txt.push_local_var(count_cur_off,Rc::clone(&i64_ty));
  inc_txt.push_opcode(Opcode::Dup);
  inc_txt.push_opcode(Opcode::Ld64);
  inc_txt.push_opcode(Opcode::Push1);
  inc_txt.push_opcode(Opcode::Addi);
  inc_txt.push_opcode(Opcode::St64);

  output.push_eval_text(inc_txt);


  let  mut cmp_txt = AsmEvalText::new();

  cmp_txt.push_local_var(count_cur_off,Rc::clone(&i64_ty));
  cmp_txt.push_opcode(Opcode::Ld64);
  cmp_txt.push_local_var(count_max_off,Rc::clone(&i64_ty));
  cmp_txt.push_opcode(Opcode::Ld64);
  cmp_txt.push_opcode(Opcode::Lti);

  output.push_eval_text(cmp_txt);

  output.push_brz(&clh.on_break);

  process_block(forstmt.get_block(),tbl,ret_ty_name,lid,Some(&clh),&new_scp,output);

  output.push_jmp(&clh.on_continue);

  output.push_label(&clh.on_break);
}


fn
process_block(blk: &Block, tbl: &SymbolTable, ret_ty_name: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmText)
{
  let  mut new_scp = Scope::new(scp);

    for stmt in blk.get_stmt_list()
    {
      process_stmt(stmt,tbl,ret_ty_name,lid,clh_opt,&mut new_scp,output);
    }
}


fn
process_stmt(stmt: &Stmt, tbl: &SymbolTable, ret_ty_name: &str, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder> ,scp: &mut Scope, output: &mut AsmText)
{
    match stmt
    {
  Stmt::Empty=>{}
  Stmt::Block(blk)=>{process_block(blk,tbl,ret_ty_name,lid,clh_opt,scp,output);}
  Stmt::Decl(decl)=>
    {
        match decl.get_kind()
        {
      DeclKind::Const(_,e)=>
        {
          let  cres = evaluate_const(e,tbl,Some(scp));

            match cres
            {
          EvalConstResult::Void    =>{}
          EvalConstResult::Bool(b) =>{scp.add_const_bool( decl.get_name(),b);}
          EvalConstResult::Int(i)  =>{scp.add_const_int(  decl.get_name(),i);}
          EvalConstResult::Float(f)=>{scp.add_const_float(decl.get_name(),f);}
          _=>{panic!();}
            }
        }
      DeclKind::Var(_,e)=>
        {
          let  r_txt = evaluate(e,tbl,Some(scp)).to_text();

          let  off = scp.add_var(decl.get_name(),r_txt.get_ty_name());

          let  mut l_txt = AsmEvalText::new();

          l_txt.push_local_var(off,r_txt.get_ty());

          output.push_assign(l_txt,r_txt,"=");
        }
      DeclKind::Static(_,_)=>{}
      _=>{panic!();}
        }
    }
  Stmt::Expr(e)=>
    {
      let  res = evaluate(e,tbl,Some(scp));

        match res
        {
      EvalResult::Value(txt)=>
        {
          output.push_eval_text(txt);

          output.push_opcode(Opcode::Pop);
        }
      _=>{}
        }
    }
  Stmt::If(i)=>{process_if(i,tbl,ret_ty_name,lid,clh_opt,scp,output);}
  Stmt::Loop(blk)=>
    {
      let  clh = lid.make_ctrl_label_holder();

      output.push_label(&clh.on_continue);

      process_block(blk,tbl,ret_ty_name,lid,Some(&clh),scp,output);

      output.push_jmp(&clh.on_continue);

      output.push_label(&clh.on_break);
    }
  Stmt::While(e,blk)=>
    {
      let  clh = lid.make_ctrl_label_holder();

      output.push_label(&clh.on_continue);

      let  txt = evaluate(e,tbl,Some(scp)).to_text();

        if txt.get_ty().get_name() == "bool"
        {
          output.push_eval_text(txt);

          output.push_brz(&clh.on_break);

          process_block(blk,tbl,ret_ty_name,lid,Some(&clh),scp,output);

          output.push_jmp(&clh.on_continue);

          output.push_label(&clh.on_break);
        }

      else
        {panic!();}
    }
  Stmt::For(f)=>{process_for(f,tbl,ret_ty_name,lid,clh_opt,scp,output);}
  Stmt::Return(e_opt)=>
    {
        if let Some(e) = e_opt
        {
          let  mut txt = evaluate(e,tbl,Some(scp)).to_text();

            if txt.get_ty().get_name() == ret_ty_name
            {
                if txt.is_deref()
                {
                  txt.push_load();
                }


              output.push_eval_text(txt);
            }

          else
            {panic!("TYPE OF RETURN VALUE and TYPE OF EVALUATED VALUW are mismatched");}
        }

      else
        {
            if ret_ty_name == "void"
            {
              output.push_opcode(Opcode::Push0);
            }

          else{panic!();}
        }


      output.push_opcode(Opcode::Ret);
    }
  Stmt::Assign(l,r,op)=>
    {
      let  mut l_asm = evaluate(l,tbl,Some(scp)).to_text();
      let  mut r_asm = evaluate(r,tbl,Some(scp)).to_text();

todo!();
      output.push_opcode(Opcode::St64);
    }
  Stmt::Break=>
    {
      output.push_jmp(&clh_opt.unwrap().on_break);
    }
  Stmt::Continue=>
    {
      output.push_jmp(&clh_opt.unwrap().on_continue);
    }
  Stmt::Print(e)=>
    {
      let  mut txt = evaluate(e,tbl,Some(scp)).to_text();

      txt.push_opcode(Opcode::Pri);

      output.push_eval_text(txt);
    }
    }
}




pub fn
assemble(decl: &FnDecl, tbl: &SymbolTable)-> Vec<u8>
{
  let  mut text = AsmText::new();
  let   mut lid = LabelID::new();

  let  scp = Scope::new_root(decl,tbl);

  let  return_ty = add_ty_from_node(decl.get_return_ty_node(),tbl);

  process_block(decl.get_block(),tbl,return_ty.get_name(),&mut lid,None,&scp,&mut text);

  text.set_xs(scp.get_offset_max());

  text.to_bytes()
}


pub fn
print_bytes(bytes: &Vec<u8>)
{
  let  mut off = 0usize;

    while off < bytes.len()
    {
      print!("[{:0>5}] ",off);

      let  op = Opcode::from(bytes[off]);

      off += 1;

      op.print();

      print!(" ");

        match op
        {
      Opcode::Pushi8
     |Opcode::Jmp8
     |Opcode::Brz8
     |Opcode::Brnz8=>
        {
          print!("{}",bytes[off] as i8);

          off += 1;
        }
      Opcode::Pushi16
     |Opcode::Jmp16
     |Opcode::Brz16
     |Opcode::Brnz16=>
        {
          let  buf: [u8; 2] = [bytes[off  ],
                               bytes[off+1]];

          print!("{}",i16::from_be_bytes(buf));

          off += 2;
        }
      Opcode::Pushi32
     |Opcode::Jmp32
     |Opcode::Brz32
     |Opcode::Brnz32=>
        {
              let  buf: [u8; 4] = [bytes[off  ],
                                   bytes[off+1],
                                   bytes[off+2],
                                   bytes[off+3]];

              print!("{}",i32::from_be_bytes(buf));

              off += 4;
        }
      Opcode::Pushu8
     |Opcode::Xs8=>
        {
          print!("{}",bytes[off]);

          off += 1;
        }
      Opcode::Pushu16
     |Opcode::Xs16=>
        {
          let  buf: [u8; 2] = [bytes[off  ],
                               bytes[off+1]];

          print!("{}",u16::from_be_bytes(buf));

          off += 2;
        }
      Opcode::Pushu32
     |Opcode::Xs32=>
        {
          let  buf: [u8; 4] = [bytes[off  ],
                               bytes[off+1],
                               bytes[off+2],
                               bytes[off+3]];

          print!("{}",u32::from_be_bytes(buf));

          off += 4;
        }
      Opcode::Pushu64=>
        {
          let  buf: [u8; 8] = [bytes[off  ],
                               bytes[off+1],
                               bytes[off+2],
                               bytes[off+3],
                               bytes[off+4],
                               bytes[off+5],
                               bytes[off+6],
                               bytes[off+7]];

          let  u = u64::from_be_bytes(buf);

          print!("{}",u);

          off += 8;
        }
      Opcode::Pushf32=>
        {
          let  buf: [u8; 4] = [bytes[off  ],
                               bytes[off+1],
                               bytes[off+2],
                               bytes[off+3]];

          print!("{}",f32::from_be_bytes(buf));

          off += 4;
        }
      _=>{}
        }


      println!("");
    }


  println!("}}");
}




