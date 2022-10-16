

#[macro_export]
macro_rules!
report
{
    ()=>
    {
      println!("reported on {}",line!());
    }
}




