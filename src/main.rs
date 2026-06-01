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
          let  exec = symtbl.generate_exec();

          exec.print_text();

          let  mut mem = exec.generate_memory();

//          symtbl.print();

          println!("");

          let  mut m = Machine::new();

          m.connect_memory(mem.as_mut_ptr(),mem.len());

          m.reset(1024,&exec,"main");

          println!("machine runs");

          m.keep_run();

          println!("machine is finished");

          exec.print_memory(&mem);
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

io INPUT at 0;
io VIDEO at 8;

const HEAP_START = 1024;
const HEAP_SIZE  =  200;

const DATA_START      = (HEAP_START+HEAP_SIZE);
const TEXT_START      = (HEAP_START+HEAP_SIZE)+(1024*1);
const STACK_START     = (HEAP_START+HEAP_SIZE)+(1024*2);
const CALLSTACK_START = (HEAP_START+HEAP_SIZE)+(1024*3);

const     STACK_SIZE  = 1024;
const CALLSTACK_SIZE  = 1024;

const VIDEO_W = 400;

const    UP_KEY = 0b0001;
const  LEFT_KEY = 0b0010;
const RIGHT_KEY = 0b0100;
const  DOWN_KEY = 0b1000;


fn
main()
{
    loop
    {
        if INPUT.byte&   UP_KEY{VIDEO.word -= VIDEO_W;}
        if INPUT.byte& LEFT_KEY{VIDEO.word -= 1;}
        if INPUT.byte&RIGHT_KEY{VIDEO.word += 1;}
        if INPUT.byte& DOWN_KEY{VIDEO.word += VIDEO_W;}

      halt;
    }


  return;
}


"#;


  compile_and_run(codes);
}




