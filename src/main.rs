mod token;
mod source_file;
mod syntax;
mod language;
mod node;
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
  use crate::language::*;
  use crate::language::decl::*;
  use crate::language::symbol_table::*;
  use crate::language::machine::*;

    if let Ok(root) = decl::Decl::read_as_root(
r#"

const  a = 8;
const  b = a+4;
const  c = b+8;



fn
test()
{
  for x in 8
  {
    print x;
  }
}
"#)
{
  let  mut tbl = SymbolTable::from(root);

    if let Ok(img) = tbl.build()
    {
      tbl.print();

      let  mut m = Machine::new();

      println!("machine is reset");

      m.reset(&img);

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




