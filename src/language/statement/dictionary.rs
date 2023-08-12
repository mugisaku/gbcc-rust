

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#statement

statement: ";"
  |  if
  | loop
  | while
  | for
  | break
  | continue
  | block
  | return
  | fn
  | var
  | static
  | const
  | struct
  | union
  | enum
  | alias
  | expression::expression
  ;

else    : 'else & block;
else_if : 'else & 'if & conditional_block;
if      : 'if -> conditional_block & [{else_if}] & [else];
loop    : 'loop -> block;
while   : 'while -> conditional_block;
for     : 'for -> block;
break   : 'break;
continue: 'continue;
block   : "{" & [{statement}] & "}";
return  : 'return -> [expression::expression];

conditional_block: expression::expression & block;

parameter: .Identifier & ":" & typesystem::type;
parameter_list: "(" & [parameter & [{"," & parameter}]] & ")";

fn    : 'fn -> .Identifier & parameter_list & ["->" & typesystem::type] & block;
var   : 'var -> .Identifier & [":" & typesystem::type] & ["=" & expression::expression];
static: 'static -> .Identifier & ":" & typesystem::type & "=" & expression::expression;
const : 'const  -> .Identifier & ":" & typesystem::type & "=" & expression::expression;
struct: 'struct -> .Identifier & member_list;
union : 'union -> .Identifier & member_list;
enum  : 'enum -> .Identifier & enumerator_list;
alias : 'alias -> .Identifier & ":" & typesystem::type;

member_list: "{" & [{parameter & [","]}] & "}";

enumerator: .Identifier & ["=" & expression::expression];
enumerator_list: "{" & [{enumerator & [","]}] & "}";

primary_statement: fn | var | static | const | struct | union | enum | alias;


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


   panic!("statement dictionary making error");
}




