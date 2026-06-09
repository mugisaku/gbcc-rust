mod token;
mod source_file;
mod syntax;
mod language;
mod node;
mod object;
mod debug;

use std::env;


fn
compile_and_run(s: &str)
{
  use crate::language::*;
  use crate::language::decl::*;
  use crate::language::symbol_table::*;
  use crate::language::machine::*;

    if let Ok(root) = decl::Decl::read_as_root(s)
    {
        if let Ok(mut symtbl) = SymbolTable::build(root)
        {
          let  mut exec = symtbl.generate_exec(1024*1024*8);

          exec.print_text();
//          symtbl.print();

          println!("");

          let  mut m = Machine::new();

          m.reset(0,256,&mut exec,"main",0);

m.set_verbose();

          println!("machine runs");

          m.keep_run();

          println!("machine is finished");

          exec.print_memory();
        }

      else
        {
          println!("build is failed");
        }
    }
}




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
  let  codes =
r#"

io report;

var  test = 0;

str  test_s u8 = "mumumu";
str  test_a u16 = {0,0,0,0,};
field  test_f 1024;


fn
main()
{
  test = 123;
}


"#;


  compile_and_run(codes);
}




