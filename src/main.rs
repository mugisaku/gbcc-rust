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

          let  mut mem = exec.generate_memory();

//          symtbl.print();

          println!("");

          let  mut m = Machine::new();

          m.connect_memory(mem.as_mut_ptr(),mem.len());

          m.reset(1024,&exec,"main");

          println!("machine runs");

          m.run();

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


var
value = 888;

var
value2 = 0;

var
xorshift_state = 12345678;


const DATA_START = 0;
const TEXT_START = 1024*1;
const HEAP_START = 0;
const HEAP_SIZE  = 0;
const STACK_START = 1024*2;
const STACK_SIZE  = 1024;
const CALLSTACK_START = 1024*3;
const CALLSTACK_SIZE  = 1024;


fn
rand()
{
  xorshift_state ^= xorshift_state<<7;
  xorshift_state ^= xorshift_state>>9;

  return xorshift_state;
}


fn
add(a,b)
{
  return a+b;
}


fn
main()
{
  for _ in 4
  {
    value += 1;
  }
  

  return 0;
}
"#;


  compile_and_run(codes);
}




