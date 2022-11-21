



pub const    nop: u8 =  0;
pub const   ldu8: u8 =  1;
pub const  ldu16: u8 =  2;
pub const  ldu32: u8 =  3;
pub const   lds8: u8 =  4;
pub const  lds16: u8 =  5;
pub const  lds32: u8 =  6;
pub const  ldf32: u8 =  7;
pub const   ld64: u8 =  8;
pub const   stu8: u8 =  9;
pub const  stu16: u8 = 10;
pub const  stu32: u8 = 11;
pub const   sts8: u8 = 12;
pub const  sts16: u8 = 13;
pub const  sts32: u8 = 14;
pub const  stf32: u8 = 15;
pub const   st64: u8 = 16;

pub const  adds: u8 = 20;
pub const  subs: u8 = 21;
pub const  muls: u8 = 22;
pub const  divs: u8 = 23;
pub const  rems: u8 = 24;
pub const  addu: u8 = 25;
pub const  subu: u8 = 26;
pub const  mulu: u8 = 27;
pub const  divu: u8 = 28;
pub const  remu: u8 = 29;
pub const  addf: u8 = 30;
pub const  subf: u8 = 31;
pub const  mulf: u8 = 32;
pub const  divf: u8 = 33;
pub const  remf: u8 = 34;

pub const   land: u8 = 40;
pub const    lor: u8 = 41;
pub const   lnot: u8 = 42;
pub const    neg: u8 = 43;
pub const   negf: u8 = 44;

pub const   shl: u8 = 50;
pub const   shr: u8 = 51;
pub const   and: u8 = 52;
pub const    or: u8 = 53;
pub const   xor: u8 = 54;
pub const    eq: u8 = 55;
pub const   neq: u8 = 56;
pub const   not: u8 = 57;

pub const    lts: u8 = 60;
pub const  lteqs: u8 = 61;
pub const    gts: u8 = 62;
pub const  gteqs: u8 = 63;
pub const    ltu: u8 = 64;
pub const  ltequ: u8 = 65;
pub const    gtu: u8 = 66;
pub const  gtequ: u8 = 67;
pub const    ltf: u8 = 68;
pub const  lteqf: u8 = 69;
pub const    gtf: u8 = 70;
pub const  gteqf: u8 = 71;

pub const     psh0: u8 = 80;
pub const     psh1: u8 = 81;
pub const     psh2: u8 = 82;
pub const     psh4: u8 = 83;
pub const     psh8: u8 = 84;
pub const    pshfw: u8 = 85;
pub const    pshu8: u8 = 86;
pub const   pshu16: u8 = 87;
pub const   pshu32: u8 = 88;
pub const    pshs8: u8 = 90;
pub const   pshs16: u8 = 91;
pub const   pshs32: u8 = 92;
pub const   pshf32: u8 = 93;
pub const   pshb64: u8 = 94;

pub const  xsp: u8 = 100;
pub const  ssp: u8 = 101;
pub const  maa: u8 = 102;
pub const  swp: u8 = 103;

pub const    jmp: u8 = 110;
pub const     br: u8 = 111;
pub const    cal: u8 = 112;
pub const    ret: u8 = 113;
pub const    hlt: u8 = 114;
pub const   retd: u8 = 115;
pub const   reti: u8 = 116;
pub const   prnu64: u8 = 117;
pub const   putlog0: u8 = 118;
pub const   putlog1: u8 = 119;


pub fn
get_name(op: u8)-> &'static str
{
    match op
    {
    nop=>{return "nop";},
   ldu8=>{return "ldu8";},
  ldu16=>{return "ldu16";},
  ldu32=>{return "ldu32";},
   lds8=>{return "lds8";},
  lds16=>{return "lds16";},
  lds32=>{return "lds32";},
  ldf32=>{return "ldf32";},
   ld64=>{return "ld64";},
   stu8=>{return "stu8";},
  stu16=>{return "stu16";},
  stu32=>{return "stu32";},
   sts8=>{return "sts8";},
  sts16=>{return "sts16";},
  sts32=>{return "sts32";},
  stf32=>{return "stf32";},
   st64=>{return "st64";},

  adds=>{return "adds";},
  subs=>{return "subs";},
  muls=>{return "muls";},
  divs=>{return "divs";},
  rems=>{return "rems";},
  addu=>{return "addu";},
  subu=>{return "subu";},
  mulu=>{return "mulu";},
  divu=>{return "divu";},
  remu=>{return "remu";},
  addf=>{return "addf";},
  subf=>{return "subf";},
  mulf=>{return "mulf";},
  divf=>{return "divf";},
  remf=>{return "remf";},

   land=>{return "land";},
    lor=>{return "lor";},
   lnot=>{return "lnot";},
    neg=>{return "neg";},
   negf=>{return "negf";},

   shl=>{return "shl";},
   shr=>{return "shr";},
   and=>{return "and";},
    or=>{return "or";},
   xor=>{return "xor";},
    eq=>{return "eq";},
   neq=>{return "neq";},
   not=>{return "not";},

    lts=>{return "lts";},
  lteqs=>{return "lteqs";},
    gts=>{return "gts";},
  gteqs=>{return "gteqs";},
    ltu=>{return "ltu";},
  ltequ=>{return "ltequ";},
    gtu=>{return "gtu";},
  gtequ=>{return "gtequ";},
    ltf=>{return "ltf";},
  lteqf=>{return "lteqf";},
    gtf=>{return "gtf";},
  gteqf=>{return "gteqf";},

     psh0=>{return "psh0";},
     psh1=>{return "psh1";},
     psh2=>{return "psh2";},
     psh4=>{return "psh4";},
     psh8=>{return "psh8";},
     pshfw=>{return "pshfw";},
    pshu8=>{return "pshu8";},
   pshu16=>{return "pshu16";},
   pshu32=>{return "pshu32";},
    pshs8=>{return "pshs8";},
   pshs16=>{return "pshs16";},
   pshs32=>{return "pshs32";},
   pshf32=>{return "pshf32";},
   pshb64=>{return "pshb64";},

  xsp=>{return "xsp";},
  ssp=>{return "ssp";},
  maa=>{return "maa";},
  swp=>{return "swp";},

   jmp=>{return "jmp";},
    br=>{return "br";},
   cal=>{return "cal";},
   ret=>{return "ret";},
   hlt=>{return "hlt";},
   retd=>{return "retd";},
   reti=>{return "reti";},
   prnu64=>{return "prnu64";},
   putlog0=>{return "putlog0";},
   putlog1=>{return "putlog1";},


  _=>{return "NOP"},
    }
}




