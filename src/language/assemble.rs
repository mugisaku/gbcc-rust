

use super::*;
use super::decl::*;
use super::expr::*;
use super::stmt::*;
use super::asm::*;
use super::symbol_table::*;
use super::scope::*;
use super::evaluate::*;
use super::evaluate_const::*;

use crate::source_file::{
  SourceInfo,
  Error,

};




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
  let   base = format!("L{}",id);
  let  label = format!("{}_1",&base);

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

  self.label = format!("{}_{}",&self.base,self.number);
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
    on_continue: format!("L{}_START",id),
  }
}


}




fn
process_if(ifstmt: &IfStmt, tbl: &SymbolTable, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmText)-> Result<(),Error>
{
  let  mut blh = lid.make_br_label_holder();

  let  end_label = blh.make_end_label();

    for (cond,blk) in ifstmt.get_cond_block_list()
    {
        match evaluate(cond,tbl,Some(scp)).try_to_text()
        {
      Ok(mut txt)=>
        {
          output.push_label(blh.get_label());

          blh.increment();

          txt.push_load();

          output.push_eval_text(txt);

          output.push_brz(blh.get_label());

            match process_block(blk,tbl,lid,clh_opt,scp,output)
            {
          Ok(())=>{output.push_jmp(&end_label);}
          Err(e)=>{return Err(e);}
            }
        }
    Err(e)=>{return Err(e);}
        }
    }


  output.push_label(blh.get_label());

    if let Some(blk) = ifstmt.get_else_block_opt()
    {
        match process_block(blk,tbl,lid,clh_opt,scp,output)
        {
      Ok(())=>{}
      Err(e)=>{return Err(e);}
        }
    }


  output.push_label(&end_label);

  Ok(())
}


fn
process_for(srcinf: &SourceInfo, forstmt: &ForStmt, tbl: &SymbolTable, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmText)-> Result<(),Error>
{
  let  clh = lid.make_ctrl_label_holder();

  let  cmp_label = format!("{}_AT_FIRST",clh.on_continue);

  let  mut new_scp = Scope::new(scp);


  let  mut count_max_off = 0usize;

    match evaluate(forstmt.get_expr(),tbl,Some(scp)).try_to_text()
    {
  Ok(mut count_max_txt)=>
    {
      count_max_off = new_scp.add_var("<FOR_COUNT_MAX>");

      let  mut var_txt = AsmEvalText::new();

      var_txt.push_local_var(count_max_off);

        match output.try_push_assign(srcinf,var_txt,count_max_txt,"=")
        {
      Ok(())=>{}
      Err(e)=>{return Err(e);}
        }
    }
  Err(e)=>{return Err(e);}
    }


  let  count_cur_off = new_scp.add_var(forstmt.get_var_name());

  let  mut init_txt = AsmEvalText::new();

  init_txt.push_local_var(count_cur_off);
  init_txt.push_i64(0);
  init_txt.push_opcode(Opcode::St_i64);

  output.push_eval_text(init_txt);
  output.push_jmp(&cmp_label);


  output.push_label(&clh.on_continue);

  let  mut inc_txt = AsmEvalText::new();

  inc_txt.push_local_var(count_cur_off);
  inc_txt.push_opcode(Opcode::Dup);
  inc_txt.push_opcode(Opcode::Ld_i64);
  inc_txt.push_i64(1);
  inc_txt.push_opcode(Opcode::Add);
  inc_txt.push_opcode(Opcode::St_i64);

  output.push_eval_text(inc_txt);


  output.push_label(&cmp_label);

  let  mut cmp_txt = AsmEvalText::new();

  cmp_txt.push_local_var(count_cur_off);
  cmp_txt.push_opcode(Opcode::Ld_i64);
  cmp_txt.push_local_var(count_max_off);
  cmp_txt.push_opcode(Opcode::Ld_i64);
  cmp_txt.push_opcode(Opcode::Lt);

  output.push_eval_text(cmp_txt);

  output.push_brz(&clh.on_break);

    match process_block(forstmt.get_block(),tbl,lid,Some(&clh),&new_scp,output)
    {
  Ok(())=>
    {
      output.push_jmp(&clh.on_continue);

      output.push_label(&clh.on_break);

      Ok(())
    }
  Err(e)=>{Err(e)}
    }
}


fn
process_block(blk: &Block, tbl: &SymbolTable, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder>, scp: &Scope, output: &mut AsmText)-> Result<(),Error>
{
  let  mut new_scp = Scope::new(scp);

    for stmt in blk.get_stmt_list()
    {
        match process_stmt(stmt,tbl,lid,clh_opt,&mut new_scp,output)
        {
      Ok(())=>{}
      Err(e)=>{return Err(e);}
        }
    }


  Ok(())
}


fn
process_stmt(stmt: &Stmt, tbl: &SymbolTable, lid: &mut LabelID, clh_opt: Option<&CtrlLabelHolder> ,scp: &mut Scope, output: &mut AsmText)-> Result<(),Error>
{
  let  srcinf = stmt.get_source_info();

    match stmt.get_kind()
    {
  StmtKind::Empty=>{Ok(())}
  StmtKind::Block(blk)=>{process_block(blk,tbl,lid,clh_opt,scp,output)}
  StmtKind::Decl(decl)=>
    {
        match decl.get_kind()
        {
      DeclKind::Const(e)=>
        {
            match evaluate_const(e,tbl,Some(scp))
            {
          Ok(i)=>
            {
              scp.add_const_int(decl.get_name(),i);

              Ok(())
            }
          Err(())=>{Err(srcinf.to_error(format!("constの算出に失敗")))}
            }
        }
      DeclKind::Var(e)=>
        {
            match evaluate(e,tbl,Some(scp)).try_to_text()
            {
          Ok(r_txt)=>
            {
              let  off = scp.add_var(decl.get_name());

              let  mut l_txt = AsmEvalText::new();

              l_txt.push_local_var(off);

              output.try_push_assign(srcinf,l_txt,r_txt,"=")
            }
          Err(e)=>{Err(e)}
            }
        }
      _=>{Err(srcinf.to_error(format!("invalid decl")))}
        }
    }
  StmtKind::Expr(e)=>
    {
        match evaluate(e,tbl,Some(scp)).try_to_text()
        {
      Ok(txt)=>
        {
          output.push_eval_text(txt);

          output.push_opcode(Opcode::Pop);

          Ok(())
        }
      Err(e)=>{Err(e)}
        }
    }
  StmtKind::If(i)=>{process_if(i,tbl,lid,clh_opt,scp,output)}
  StmtKind::Loop(blk)=>
    {
      let  clh = lid.make_ctrl_label_holder();

      output.push_label(&clh.on_continue);

        match process_block(blk,tbl,lid,Some(&clh),scp,output)
        {
      Ok(())=>
        {
          output.push_jmp(&clh.on_continue);

          output.push_label(&clh.on_break);

          Ok(())
        }
      Err(e)=>{Err(e)}
        }
    }
  StmtKind::While(e,blk)=>
    {
      let  clh = lid.make_ctrl_label_holder();

      output.push_label(&clh.on_continue);


        match evaluate(e,tbl,Some(scp)).try_to_text()
        {
      Ok(mut txt)=>
        {
          txt.push_load();

          output.push_eval_text(txt);

          output.push_brz(&clh.on_break);

            match process_block(blk,tbl,lid,Some(&clh),scp,output)
            {
          Ok(())=>
            {
              output.push_jmp(&clh.on_continue);

              output.push_label(&clh.on_break);

              Ok(())
            }
          Err(e)=>{Err(e)}
            }
        }
      Err(e)=>{Err(e)}
        }
    }
  StmtKind::For(f)=>{process_for(srcinf,f,tbl,lid,clh_opt,scp,output)}
  StmtKind::Return(e_opt)=>
    {
        if let Some(e) = e_opt
        {
            match evaluate(e,tbl,Some(scp)).try_to_text()
            {
          Ok(mut txt)=>
            {
              txt.push_load();

              output.push_eval_text(txt);
            }
          Err(e)=>{return Err(e);}
            }
        }

      else
        {
          output.push_i64(0);
        }


      output.push_opcode(Opcode::Ret);

      Ok(())
    }
  StmtKind::Assign(l,r,op)=>
    {
        match evaluate(l,tbl,Some(scp)).try_to_text()
        {
      Ok(l_asm)=>
        {
            match evaluate(r,tbl,Some(scp)).try_to_text()
            {
          Ok(r_asm)=>{output.try_push_assign(srcinf,l_asm,r_asm,op)}
          Err(e)=>{Err(e)}
            }
        }
      Err(e)=>{Err(e)}
        }
    }
  StmtKind::Break=>
    {
        match clh_opt
        {
      Some(clh)=>
        {
          output.push_jmp(&clh.on_break);

          Ok(())
        }
      None=>{Err(srcinf.to_error(format!("無効なbreak")))}
        }
    }
  StmtKind::Continue=>
    {
        match clh_opt
        {
      Some(clh)=>
        {
          output.push_jmp(&clh.on_continue);

          Ok(())
        }
      None=>{Err(srcinf.to_error(format!("無効なcontinue")))}
        }
    }
  StmtKind::Halt=>
    {
      output.push_opcode(Opcode::Hlt);

      Ok(())
    }
  StmtKind::Print(e)=>
    {
        match evaluate(e,tbl,Some(scp)).try_to_text()
        {
      Ok(mut txt)=>
        {
          txt.push_load();

          txt.push_opcode(Opcode::Pri);

          output.push_eval_text(txt);

          Ok(())
        }
      Err(e)=>{Err(e)}
        }
    }
    }
}




pub fn
assemble(srcinf: &SourceInfo, decl: &FnDecl, tbl: &SymbolTable)-> Result<AsmText,Error>
{
  let  mut text = AsmText::new();
  let   mut lid = LabelID::new();

  let  scp = Scope::new_root(decl,tbl);

    match process_block(decl.get_block(),tbl,&mut lid,None,&scp,&mut text)
    {
  Ok(())=>
    {
      text.set_xs(scp.get_offset_max());
      text.terminate();

      Ok(text)
    }
  Err(e)=>{Err(e)}
    }
}


pub fn
print_bytes(bytes: &Vec<u8>)
{
  let  mut off = 0usize;

    while off < bytes.len()
    {
      print!("[{:0>5}] ",off);

      let  op = Opcode::try_from(bytes[off]).unwrap();

      off += 1;

      op.print();

      print!(" ");

        match op
        {
      Opcode::Push8
     |Opcode::Jmp8
     |Opcode::Brz8
     |Opcode::Brnz8=>
        {
          print!("{}",bytes[off] as i8);

          off += 1;
        }
      Opcode::Push16
     |Opcode::Jmp16
     |Opcode::Brz16
     |Opcode::Brnz16=>
        {
          let  buf: [u8; 2] = [bytes[off  ],
                               bytes[off+1]];

          print!("{}",i16::from_be_bytes(buf));

          off += 2;
        }
      Opcode::Push32
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
      Opcode::Xs8=>
        {
          print!("{}",bytes[off]);

          off += 1;
        }
      Opcode::Xs16=>
        {
          let  buf: [u8; 2] = [bytes[off  ],
                               bytes[off+1]];

          print!("{}",u16::from_be_bytes(buf));

          off += 2;
        }
      Opcode::Xs32=>
        {
          let  buf: [u8; 4] = [bytes[off  ],
                               bytes[off+1],
                               bytes[off+2],
                               bytes[off+3]];

          print!("{}",u32::from_be_bytes(buf));

          off += 4;
        }
      Opcode::Push64=>
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
      _=>{}
        }


      println!("");
    }


  println!("}}");
}




