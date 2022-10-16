



pub enum
TypeInfo
{
  Null,
  Void,
  Boolean(usize),
  Character(usize),
  SignedInteger(usize),
  Integer(usize),
  Floating(usize),
  Derived(Box<DerivedTypeInfo>),

}


pub enum
DerivedTypeInfo
{
  Array(TypeInfo,usize),
  Pointer(TypeInfo),
  Reference(TypeInfo),
  Struct(StructInfo),
  Union(UnionInfo),
  Enum(EnumInfo),
  Function(FunctionInfo),

}


pub struct
Member
{
       name: String,
  type_info: TypeInfo,

  offset: usize,

}


pub struct
StructInfo
{
  member_list: Vec<Member>,

  size: usize,

}


pub struct
UnionInfo
{
  member_list: Vec<Member>,

  size: usize,

}


pub struct
EnumInfo
{
  member_list: Vec<Member>,

  size: usize,

}


pub struct
FunctionInfo
{
  type_info: TypeInfo,

  parameter_list: StructInfo,

}




