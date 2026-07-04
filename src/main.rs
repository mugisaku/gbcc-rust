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
          let  mut tmp = Vec::<u8>::new();

          tmp.resize(4*384*384,0);

          symtbl.add_img("image",384,384,tmp);

//          symtbl.print();

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


fn
main()
{
  loop{
    if sys.spawn(test,900,800) == 0{halt;}
  }
}
"#;


  compile_and_run(codes);
}




