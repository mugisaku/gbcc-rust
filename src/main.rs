mod token;
mod source_file;
mod syntax;
mod language;
mod node;
mod debug;

use std::env;


fn
compile(s: &str)
{
  use crate::language::*;
  use crate::language::decl::*;
  use crate::language::symbol_table::*;
  use crate::language::machine::*;
  use crate::language::ty::*;

    if let Ok(root) = decl::Decl::read_as_root(s)
    {
      let  mut tytbl = TyTable::new();

        if let Ok(mut symtbl) = SymbolTable::build(root,&mut tytbl)
        {
          let  mut mi = MachineInfo::default();

          mi.set_memory_size(256)
            .data(1000*32)
            .text(1000*32)
            .heap(1000*32)
            .stack(1000*32)
            .callstack(1000*32)
          ;


          let  exec = symtbl.generate_exec(&mut tytbl,&mi);

          symtbl.print();

          println!("");

          let  mut m = Machine::new(&mi);

          m.reset(&exec);

          println!("machine runs");

          m.run();

          println!("machine is finished");
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

const  a = 8;
const  b = a+4;
const  c = b+8;

var  test: St = St::default;

struct St{
a: i64,
b: u16,
c: i8,
d: f32

}

union Un{
a: i16,
b: u8,
c: f32

}


enum En
{
  Apple, Grape, Peach
}


fn
add(a: i64, b: i64)-> i64
{
  return a+b;
}


fn
main()-> i64
{
  return add(7,add(40,322));
}
"#;


  compile(codes);
}




