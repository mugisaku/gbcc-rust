

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#dynamic


element: .Identifier & ":" & expression;

table: "{" & [element & {"," & element}] & "}";

operand_core: .Identifier | .Integer | .Floating | .Character | .String | table | ("(" & expression & ")");

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


expression_or_assign: expression & [assign_operator & expression];


statement: ";"
  | break
  | continue
  | if
  | while
  | for
  | loop
  | statement_list
  | return
  | let
  | const
  | print_s
  | print_v
  | expression_or_assign
  ;


break   : 'break;
continue: 'continue;
return  : 'return -> [expression];


if: 'if -> expression & statement_list & [{'else & 'if & expression & statement_list}] & ['else & statement_list];

statement_list: "{" & [{statement}] & "}";

loop : 'loop -> statement_list;
while: 'while -> expression & statement_list;
for  : 'for -> .Identifier & 'in -> expression & statement_list;


parameter_list: "(" & [.Identifier & [{"," & .Identifier}]] & ")";

fn   : 'fn    -> .Identifier & parameter_list & statement_list;
let  : 'let   -> .Identifier & [":" & expression];
const: 'const -> .Identifier & ":" & expression;
print_s: 'print & .String;
print_v: 'print & .Identifier;

declaration: fn | let | const;




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




