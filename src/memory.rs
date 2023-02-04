struct Memory{
    addr_mem : [u8; 4096],
    reg : [u8; 16],
    i : [u8; 2],
    pc : [u8; 2],
    stack : [Option<[u8; 2]>; 16],
    sp : u8,
    delay : u8,
    sound : u8
}