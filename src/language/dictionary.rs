

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#dynamic


type: ["*" | "&"] & .Identifier;


table_element: .Identifier & ":" & expression;

table: "[" & [{table_element & [","]}] & "]";

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


self_access    : "."  & .Identifier;
type_access    : "::" & .Identifier;
subscript      : "[" & expression & "]";
call           : "(" & [{expression & [","]}] & ")";
increment      : "++";
decrement      : "++";

postfix_op: self_access | type_access | subscript | call | increment | decrement;

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


break   : 'break;
continue: 'continue;
return  : 'return -> [expression];


else: 'else -> block;
else_if: 'else & 'if -> expression & block;

if: 'if -> expression & block & [{else_if}] & [else];

block: "{" & [{statement}] & "}";

loop : 'loop -> block;
while: 'while -> expression & block;
for  : 'for -> .Identifier & 'in -> expression & block;


parameter: .Identifier & ":" & type;
parameter_list: "(" & [{parameter & [","]}] & ")";

fn: 'fn -> .Identifier & parameter_list & ["->" & type] & block;
var  : 'var   -> .Identifier & "=" & expression;
const: 'const -> .Identifier & "=" & expression;
static: 'static -> .Identifier & "=" & expression;
print: 'print & expression;


field_list: "{" & [{parameter & [","]}] & "}";

enumerator: .Identifier;
enumerator_list: "{" & [{enumerator & [","]}] & "}";

struct: 'struct -> .Identifier & field_list;
union:  'union  -> .Identifier & field_list;
enum:   'enum   -> .Identifier & enumerator_list;

declaration: fn
           | var
           | static
           | const
           | struct
           | union
           | enum
           | ";";




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

              else
                {panic!();}
            }
        }


        if let Some(dic) = &DIC_OPT
        {
          return dic;
        }
    }

 
  panic!("dynamic dictionary initialize error");
}




