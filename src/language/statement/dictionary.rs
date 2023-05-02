

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#statement

statement: ";"
  |  if
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

if      : 'if -> expression::expression & block & [{'else & ['if] & block}];
while   : 'while -> expression::expression & block;
for     : 'for -> block;
break   : 'break;
continue: 'continue;
block   : "{" & [{statement}] & "}";
return  : 'return -> [expression::expression];

parameter: .Identifier & ":" & typesystem::type_note;
parameter_list: "(" & [parameter & [{"," & parameter}]] & ")";

fn    : 'fn -> .Identifier & parameter_list & ["->" & typesystem::type_note] & block;
var   : 'var -> .Identifier & [":" & typesystem::type_note] & ["=" & expression::expression];
static: 'static -> .Identifier & ":" & typesystem::type_note & "=" & expression::expression;
const : 'const  -> .Identifier & ":" & typesystem::type_note & "=" & expression::expression;
struct: 'struct -> .Identifier & member_list;
union : 'union -> .Identifier & member_list;
enum  : 'enum -> .Identifier & enumerator_list;
alias : 'alias -> .Identifier & ":" & typesystem::type_note;

member_list: "{" & [{parameter & [","]}] & "}";

enumerator: .Identifier & ["=" & expression::expression];
enumerator_list: "{" & [{enumerator & [","]}] & "}";

primary_statement: fn | var | static | const | struct | union | enum | alias;


"##;




pub fn
get_dictionary()-> &'static Dictionary
{
  static  mut dic_opt: Option<Dictionary> = None;

    unsafe
    {
        if let None = dic_opt
        {
            if let Ok(mut tmp_dic) = Dictionary::make_from_string(&DIC_S)
            {
                if tmp_dic.test().is_ok()
                {
                  dic_opt = Some(tmp_dic);
                }
            }
        }


        if let Some(dic) = &dic_opt
        {
          return dic;
        }
    }


   panic!("statement dictionary making error");
}




