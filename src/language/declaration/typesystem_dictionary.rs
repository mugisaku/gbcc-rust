

use crate::syntax::dictionary::Dictionary;


static DIC_S: &'static str =
r##"
#typesystem


function_pointer: 'fn & "(" & [type_list] & ")" & ["->" & type];

tuple: "(" & [type_list] & ")";

primitive:
    'bool
  | 'i8 | 'i16 | 'i32 | 'i64 | 'usize
  | 'u8 | 'u16 | 'u32 | 'u64 | 'isize
  | 'f32 | 'f64
  ;


type: primitive | tuple | function_pointer | .Identifier;

type_list: type & [{"," & type}];


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


   panic!("typesystem dictionary making error");
}




