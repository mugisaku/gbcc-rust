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
  use crate::language::project::*;
  use crate::language::exec::*;
  use crate::language::machine::*;

  let  mut pj = Project::new();

    match pj.add_source("",s)
    {
   Ok(())=>
    {
        match pj.compile()
        {
      Ok(())=>
        {
          pj.print();

          println!("");

            match pj.generate_exec()
            {
          Ok(mut exec)=> 
            {
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


static S: &'static str = include_str!("../gamebaby_font14.txt");


fn
main()
{
  let  a = include_str!("../gamebaby_font14.txt");
  let  b = include_str!("../gamebaby_font8x12.txt");
  let  c = 
r#"

static
tmp[4]: i64{
  [2]: 8,123,

};


static
s = u16"nu".ptr;


fn
main()
{
  return s.len;
}


"#;


  let  codes = format!("{}{}{}",a,b,c);

  compile_and_run(&codes);
}




