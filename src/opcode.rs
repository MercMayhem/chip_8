pub enum OpcodeTypes{
    CLS,
    RET,
    JPAddr,
    CALLAddr,
    SEVxByte,
    SNEVxByte,
    SEVxVy,
    LDVxbyte,
    ADDVxbyte,
    LDVxVy,
    ORVxVy,
    ANDVxVy,
    XORVxVy,
    ADDVxVy,
    SUBVxVy,
    SHRVxVy,
    SUBNVxVy,
    SHLVxVy,
    SNEVxVy,
    LDIAddr,
    JPV0Addr,
    RNDVxbyte,
    DRWVxVyNibble,
    SKPVx,
    SKNPVx,
    LDVxDT,
    LDVxK,
    LDDTVx,
    LDSTVx,
    ADDIVx,
    LDFVx,
    LDBVx,
    LDIVx,
    LDVxI
}

pub struct Opcode{
    pub code : u16,
    pub kind : OpcodeTypes
}

impl Opcode {
    pub fn find_kind(opcode : u16) -> OpcodeTypes{
        todo!()
    }
}