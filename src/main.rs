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
      Ok(())=>{root.print();}
      Err(e)=>
        {
          e.print();

          return;
        }
        }


/*
        match Class::build(root)
        {
       Ok(mut cla)=>
        {
//symtbl.print();

            match cla.generate_exec()
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
*/
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

class Test{

enum{
  Apple, Grape, Orange

}

}


const  c1 = 24;
const  c2 = c1+60;

fn
main()
{
  sys.spawn(test,123,9);
  sys.input();
}
"#;


  compile_and_run(codes);
}




