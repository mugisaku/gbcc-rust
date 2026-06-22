

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"


operand_core: .Identifier | .Number | .Character | .String | ("(" & expression & ")");

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

initialize: "=" & expression;

str: 'str
  -> .Identifier
  & ('i8 | 'i16 | 'i32 | 'i64 | 'u8 | 'u16 | 'u32)
  & "=" & (.String | expression_list);


field: 'field -> .Identifier & expression;
io   : 'io    -> .Identifier;
var  : 'var   -> .Identifier & initialize;
const: 'const -> .Identifier & initialize;

declaration: fn
           | io
           | var
           | const
           | str
           | field
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




