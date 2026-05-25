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
          let  mut mi = MachineInfo::default();

          mi.set_frequency(1000*1000*128)
            .set_memory_size(256)
            .data(1000*32)
            .text(1000*32)
            .heap(1000*32)
            .stack(1000*32)
            .callstack(1000*32)
          ;


          let  exec = symtbl.generate_exec(&mi);

          symtbl.print();

          println!("");

          let  mut m = Machine::new(&mi);

          m.reset(&exec);

          println!("machine runs");

          m.run();

          println!("machine is finished");

          m.print();
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


var
value = 888;

var
value2 = 0;


fn
add(a,b)
{
  return a+b;
}


fn
main()
{
  value2 = (272).deref;

  

  return 0;
}
"#;


  compile_and_run(codes);
}




