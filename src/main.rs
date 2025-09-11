mod token;
mod source_file;
mod syntax;
mod language;
mod asm;
mod debug;

use std::env;


fn
open_and_print_tokens()
{
  let  args: Vec<String> = env::args().collect();

    for i in 1..args.len()
    {
      let  arg = &args[i];

        if let Ok(src) = crate::source_file::SourceFile::from_file(&arg)
        {
          println!("{} is opened",&arg);

            if let Ok(toks) = crate::token::tokenize::tokenize(&src)
            {
              crate::token::print_token_string(&toks);

              println!("\n--");

              crate::token::restore_token_string(&toks);
            }

          else
            {
              println!("tokenize is failed");

              return;
            }
        }
    }
}


fn
main()
{
  let  mut src = Vec::<asm::instruction::Instruction>::new();
  let  mut m = asm::machine::Machine::new();

  use asm::instruction::*;

    if let Ok(ximg) = crate::asm::execution_image::ExecutionImage::try_from(
r#"

data s "";
data x 4 9 7 4 5;
space z 255;


routine
start a b c | tmp cnt
{
  pushglo x;
  dup;
  dup;
  ldu8;
  pri;
  pop;
  push1;
  addu;
  ldu8;
  pri;
  pop;
  push2;
  addu;
  ldi8;
  pri;
  pop;
  hlt;
loop:
  pushglo z;
  ld64;
  push 3;
  ltu;
  brz;
  pushdst end;
  pushglo z;
  pushglo add;
  prcal;
  pushglo z;
  ld64;
  push1;
  cal;
  st64;
  jmp;
  pushdst loop;
end:
  pushglo z;
  ld64;
  hlt;
}


routine
add a b |
{
  pusharg a;
  ld64;
  pusharg b;
  ld64;
  addu;
  ret;
}

"#)
    {
//      print_binary(&ximg);

      m.reset();
      m.install(&ximg);

      m.run(Some(8));
    }


//  open_and_print_tokens();
}




