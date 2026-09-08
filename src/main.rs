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
  use crate::language::exec::*;
  use crate::language::machine::*;

    match decl::DeclSet::read(s)
    {
   Ok(mut root)=>
    {
        match root.finalize()
        {
      Ok(())=>
        {
          root.print();

            match root.generate_exec()
            {
          Ok(mut exec)=> 
            {
              println!("");

              exec.print_text();

              println!("");

              let  mut m = Machine::new();

              m.set_verbose();

              m.reset(&mut exec,"main");

              println!("\n  ****");

              println!("machine runs");

              m.keep_run(800,0);

              println!("machine is finished");

              println!("\n  ****");

              exec.print_memory();

              println!("");
            }
          Err(e)=>{e.print();}
            }
        }
      Err(e)=>{e.print();}
        }
    }
  Err(e)=>{e.print();}
    }
}




fn
main()
{
  let  codes =
r#"


fn
main()
{
  var  r = 6;

  return r&0b1111;
}


"#;


  compile_and_run(codes);
}




