

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#dynamic


type: ["*" | "&"] & .Identifier;


table_element: .Identifier & ":" & expression;

table: "[" & [table_element & {"," & table_element}] & "]";

operand_core: .Identifier | .Number | .Character | .String | table | ("(" & expression & ")");

unary_operator: "!" | "++" | "--" | "-" | "~";

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


access         : "." & .Identifier;
subscript      : "[" & expression & "]";
call           : "(" & [expression & [{"," & expression}]] & ")";
increment      : "++";
decrement      : "++";

postfix_operator: access | subscript | call | increment | decrement;

operand: [{unary_operator}] & operand_core & [{postfix_operator}];

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
  | break
  | continue
  | if
  | while
  | for
  | loop
  | block
  | return
  | declaration
  | print_s
  | print_v
  | assign
  | expression
  ;


break   : 'break;
continue: 'continue;
return  : 'return -> [expression];


else: 'else -> block;
else_if: 'else -> 'if -> expression & block;

if: 'if -> expression & block & [{else_if}] & [else];

block: "{" & [{statement}] & "}";

loop : 'loop -> block;
while: 'while -> expression & block;
for  : 'for -> .Identifier & 'in -> expression & block;


parameter: .Identifier & ":" & type;
parameter_list: "(" & [parameter & [{"," & parameter}]] & ")";

function: 'function -> .Identifier & parameter_list & ["->" & type] & block;
let  : 'let   -> .Identifier & [":" & type] & "=" & expression;
const: 'const -> .Identifier & [":" & type] & "=" & expression;
static: 'static -> .Identifier & [":" & type] & "=" & expression;
print_s: 'print & .String;
print_v: 'print & .Identifier;

declaration: function | let | static | const;




"##;



pub fn
get_dictionary()-> &'static Dictionary
{
  static  mut DIC_OPT: Option<Dictionary> = None;

    unsafe
    {
        if let None = DIC_OPT
        {
            if let Ok(mut tmp_dic) = Dictionary::make_from_string(&DIC_S)
            {
                if tmp_dic.test().is_ok()
                {
                  DIC_OPT = Some(tmp_dic);
                }
            }
        }


        if let Some(dic) = &DIC_OPT
        {
          return dic;
        }
    }

 
  panic!("dynamic dictionary initialize error");
}




