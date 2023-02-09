pub struct Memory{
    pub addr_mem : [u8; 4096],
    pub reg : [u8; 16],
    pub i : u16,
    pub pc : u16,
    pub stack : [Option<u16>; 16],
    pub sp : u8,
    pub delay : u8,
    pub sound : u8
}