

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"


qualified_identifier: .Identifier & [{"::" & .Identifier}];

operand_core: qualified_identifier | .Number | .Character | .String | ("(" & expression & ")");

unary_operator: "!" | "-" | "~";

binary_operator:
    "+"
  | "-"
  | "*"
  | "/"
  | "%"
  | "=="
  | "||" | "|"
  | "&&" | "&"
  | "^"
  | "<<" | "<=" | "<"
  | ">>" | ">=" | ">"
  | "!="
  ;



access: "." & .Identifier;
call: "(" & [{expression & [","]}] & ")";

postfix_op: call | access;

operand: [{unary_operator}] & operand_core & [{postfix_op}];

expression: operand & [{binary_operator & operand}];




assign_operator:
    "="
  | "+="
  | "-="
  | "*="
  | "/="
  | "%="
  | "|=" 
  | "&="
  | "^="
  | "<<="
  | ">>="
  ;


assign: expression & assign_operator & expression;


statement: ";"
  | die
  | halt
  | break
  | continue
  | if
  | while
  | for
  | loop
  | block
  | return
  | declaration
  | print
  | assign
  | expression
  ;


die     : 'die;
halt    : 'halt;
break   : 'break;
continue: 'continue;
return  : 'return -> [expression];
print: 'print & expression;


if_block: 'if -> expression & block;

if: if_block -> [{'else & if_block}] & ['else & block];

block: "{" & [{statement}] & "}";

loop : 'loop -> block;
while: 'while -> expression & block;
for  : 'for -> .Identifier & 'in -> expression & block;


parameter_list: "(" & [{.Identifier & [","]}] & ")";

fn: 'fn -> .Identifier & parameter_list & block;

expression_list: "{" & [{expression & [","]}] & "}";


init_as_word: "=" & expression;
init_as_field: "[" & expression & "]";
init_by_data: "{" & {expression & [","]} & "}";

empty : ";";
static: 'static -> .Identifier & [init_as_word | init_as_field | init_by_data];
var   : 'var    -> .Identifier & [init_as_word | init_as_field | init_by_data];
const : 'const  -> .Identifier & "=" & expression;
enum  : 'enum   -> "{" & {.Identifier & [","]} & "}";

class: 'class -> .Identifier & "{" & [{declaration}] & "}";

declaration: fn
           | static
           | var
           | const
           | enum
           | static
           | class
           | empty;




"##;



pub fn
get_dictionary()-> &'static Dictionary
{
  static  mut DIC_OPT: Option<Dictionary> = None;

    unsafe
    {
        if let None = DIC_OPT
        {
            match Dictionary::make_from_string(&DIC_S)
            {
          Ok(mut tmp_dic)=>
            {
                match tmp_dic.test()
                {
              Ok(())=>{DIC_OPT = Some(tmp_dic);}
              Err(msg)=>{panic!("{}",msg);}
                }
            }
          Err(e)=>{e.print();}
            }
        }


        if let Some(dic) = &DIC_OPT
        {
          return dic;
        }
    }

 
  panic!("dynamic dictionary initialize error");
}




