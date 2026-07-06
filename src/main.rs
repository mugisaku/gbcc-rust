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

    match decl::Decl::read_as_root(s)
    {
   Ok(root)=>
    {
        match SymbolTable::build(root)
        {
       Ok(mut symtbl)=>
        {
//symtbl.print();

            match symtbl.generate_exec()
            {
          Ok(mut exec)=> 
            {
              println!("");
exec.print_text();
              println!("");

              let  mut m = Machine::new();


m.set_verbose();
              m.reset(1024,&mut exec,"main");

              println!("machine runs");

              m.keep_run();

              println!("machine is finished");
exec.print_memory();
              println!("");
            }
          Err(e)=>{e.print();}
            }
        }
      Err(e)=>
        {
          println!("build is failed");

          e.print();
        }
        }
    }
  Err(e)=>
    {
      e.print();

      println!("");
    }
    }
}




fn
main()
{
  let  codes =
r#"
fn
test(a,b)
{
  halt;

  return a-b;
}

enum{
  Apple, Grape, Orange

}



fn
main()
{
  sys.spawn(test,123,9);
  sys.input();
}
"#;


  compile_and_run(codes);
}




