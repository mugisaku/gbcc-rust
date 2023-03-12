mod tokenizer;
mod token;
mod source_file;
mod syntax;
mod language;
mod ir;
mod debug;


static dic_s: &'static str =
r#"
top_element =
  function_definition |
    struct_definition |
      enum_definition |
  variable_definition;

statement =
                 ";" |
        if_statement |
     while_statement |
     break_statement |
  continue_statement |
     block_statement |
     return_statement;

if_statement       = "if" & expression & block_statement & [{"else" & ["if"] & block_statement}];
while_statement    = "while" & block_statement;
break_statement    = "break";
continue_statement = "continue";
block_statement    = "{" & [{statement | (variable_declaration & ["=" & expression]) | expression}] &"}";
return_statement   = "return" & [expression];


type_expression = IDENTIFIER;

parameter = IDENTIFIER & ":" & type_expression;

function_signature = "(" & [parameter & {"," & parameter}] & ")" & ["->" & type_expression];

function_definition = "fn" & IDENTIFIER & function_signature & block_statement;

variable_declaration = "var" & parameter;

struct_definition = "struct" & IDENTIFIER & "{" & [{}] & "}";

enum_definition = "enum" & IDENTIFIER & "{" & [{}] &  "}";


operand = IDENTIFIER | INTEGER_LITERAL | FLOATING_LITERAL | LETTER_LITERAL | CHARACTER_LITERAL | STRING_LITERAL | ("(" & expression & ")");

unary_operator = "!" | "++" | "--" | "-" | "~" | "*" | "&";

binary_operator =
  "+=" | "+" |
  "-=" | "-" |
  "*=" | "*" |
  "/=" | "/" |
  "%=" | "%" |
  "==" | "!=" | "=" |
  "||" | "|=" | "|" |
  "&&" | "&=" | "&" |
  "^=" | "^" |
  "<<=" |"<<" | "<=" | "<" |
  ">>=" |">>" | ">=" | ">" ;

access    = "." & IDENTIFIER;
subscript = "[" & expression & "]";
call      = "(" & [expression & [{"," & expression}]] & ")";

primary_operation = access | subscript | call;

unary_operation = [{unary_operator}] & operand & [{primary_operation}];

expression = unary_operation & [{binary_operator & unary_operation}];

"#;


static txt_s: &'static str =
r#"

fn
test()-> slen
{
  return 1+2*3
}


"#;


fn
main()
{
  use crate::ir::block::WordCount;
  use crate::ir::block::Operand;
  use crate::ir::block::Block;
  use crate::ir::function::VariableInfo;
  use crate::ir::function::Function;
  use crate::ir::executor::Library;
  use crate::ir::executor::Executor;

  let  mut f = Function::new("test",WordCount::one());
  let  mut blk = Block::new("start");
  let  mut lib = Library::new();
  let  mut exe = Executor::new(65536);

f.add_parameter("a",WordCount::one());
f.add_parameter("b",WordCount::one());

blk.add_addi("c",Operand::from("a"),Operand::from(9999));
blk.add_ret(Some(Operand::from("c")));

f.add_block(blk);
f.fix(lib.get_variable_info_list());

f.print();

lib.add_function(f);

  if exe.reset(&lib).is_ok()
  {
      if exe.prepare_first_call(&lib,"test").is_ok()
      {
        exe.push_argument(&lib,Operand::from(2));
        exe.push_argument(&lib,Operand::from(1));

          if exe.raise_call(&lib).is_ok()
          {
            exe.run(&lib,None);

              if let Some(m) = exe.get_return_value()
              {
                println!("{} is returned",m.get_u64(0));
              }

            else
              {
                println!("no value is returned");
              }
          }
      }
  }



/*
  let  dic_f = source_file::SourceFile::from(dic_s);
  let  txt_f = source_file::SourceFile::from(txt_s);

  let  dic = syntax::dictionary::Dictionary::from(&dic_f);

//  dic.print();

    if let Ok(toks) = tokenizer::tokenize(&txt_f)
    {
    }
*/
}




