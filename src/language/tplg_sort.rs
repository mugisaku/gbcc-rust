



pub struct
TplgNode
{
  name: String,

  deps_child_list: Vec<String>,

  parent_count: usize,

}


impl
TplgNode
{


pub fn
new(name: String, deps_child_list: Vec<String>, parent_count: usize)-> Self
{
  Self{
    name,
    deps_child_list,
    parent_count,
  }
}


}




fn
step(nodes: &mut Vec<TplgNode>, remains: &mut Vec<TplgNode>)-> Vec<TplgNode>
{
  let  mut buf = Vec::<TplgNode>::new();

    while let Some(nd) = nodes.pop()
    {
        if nd.parent_count == 0
        {
          buf.push(nd);
        }

      else
        {
          remains.push(nd);
        }
    }


    for removed_nd in &buf
    {
        for name in &removed_nd.deps_child_list
        {
            for remained_nd in remains.iter_mut()
            {
                if &remained_nd.name == name
                {
                  remained_nd.parent_count -= 1;

                  break;
                }
            }
        }
    }


  buf
}


pub fn
tplg_sort(mut nodes: Vec<TplgNode>)-> Result<Vec<String>,()>
{
  let  mut remains = Vec::<TplgNode>::new();
  let  mut  output = Vec::<String>::new();

//  println!("トポロジカルソートを開始");

    while nodes.len() != 0
    {
      let  mut res = step(&mut nodes,&mut remains);

        if res.is_empty()
        {
          println!("循環参照を検出したので中断");

          return Err(());
        }


        for nd in res
        {
//          println!("{}をプッシュ",nd.key.to_number());

          output.push(nd.name);
        }


      std::mem::swap(&mut nodes,&mut remains);
    }


//  println!("トポロジカルソートを完了");

  Ok(output)
}




