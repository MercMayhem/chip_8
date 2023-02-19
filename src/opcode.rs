#[derive(PartialEq, Debug)]
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
    pub kind : Option<OpcodeTypes>
}

impl Opcode {
    pub fn find_kind(opcode : u16) -> Result<OpcodeTypes, String>{
        if opcode == 0x00E0 {
            Ok(OpcodeTypes::CLS)
        }

        else if opcode == 0x00EE {
            Ok(OpcodeTypes::RET)
        }

        else if (opcode & 0xF000) == 0x1000{
            Ok(OpcodeTypes::JPAddr)
        }

        else if (opcode & 0xF000) == 0x2000{
            Ok(OpcodeTypes::CALLAddr)
        }

        else if (opcode & 0xF000) == 0x3000{
            Ok(OpcodeTypes::SEVxByte)
        }

        else if (opcode & 0xF000) == 0x4000{
            Ok(OpcodeTypes::SNEVxByte)
        }

        else if (opcode & 0xF000) == 0x5000{
            Ok(OpcodeTypes::SEVxVy)
        }

        else if (opcode & 0xF000) == 0x6000{
            Ok(OpcodeTypes::LDVxbyte)
        }

        else if (opcode & 0xF000) == 0x7000{
            Ok(OpcodeTypes::ADDVxbyte)
        }

        else if (opcode & 0xF000) == 0x8000{
            if (opcode & 0x000F) == 0x0000{
                Ok(OpcodeTypes::LDVxVy)
            }

            else if (opcode & 0x000F) == 0x0001{
                Ok(OpcodeTypes::ORVxVy)
            }

            else if (opcode & 0x000F) == 0x0002{
                Ok(OpcodeTypes::ANDVxVy)
            }

            else if (opcode & 0x000F) == 0x0003{
                Ok(OpcodeTypes::XORVxVy)
            }

            else if (opcode & 0x000F) == 0x0004{
                Ok(OpcodeTypes::ADDVxVy)
            }

            else if (opcode & 0x000F) == 0x0005{
                Ok(OpcodeTypes::SUBVxVy)
            }

            else if (opcode & 0x000F) == 0x0006{
                Ok(OpcodeTypes::SHRVxVy)
            }

            else if (opcode & 0x000F) == 0x0007{
                Ok(OpcodeTypes::SUBNVxVy)
            }

            else if (opcode & 0x000F) == 0x000E{
                Ok(OpcodeTypes::SHLVxVy)
            }

            else {
                Err("Incorrect Opcode".to_string())
            }
        }

        else if (opcode & 0xF000) == 0x9000{
            Ok(OpcodeTypes::SNEVxVy)
        }

        else if (opcode & 0xF000) == 0xA000{
            Ok(OpcodeTypes::LDIAddr)
        }

        else if (opcode & 0xF000) == 0xB000{
            Ok(OpcodeTypes::JPV0Addr)
        }

        else if (opcode & 0xF000) == 0xC000{
            Ok(OpcodeTypes::RNDVxbyte)
        }

        else if (opcode & 0xF000) == 0xD000{
            Ok(OpcodeTypes::DRWVxVyNibble)
        }

        else if (opcode & 0xF000) == 0xE000{
            if (opcode & 0x00FF) == 0x00A1{
                Ok(OpcodeTypes::SKNPVx)
            }

            else if (opcode & 0x00FF) == 0x009E{
                Ok(OpcodeTypes::SKPVx)
            }

            else{
                Err("Incorrect Opcode".to_string())
            }
        }

        else if (opcode & 0xF000) == 0xF000{
            if (opcode & 0x00FF) == 0x0007{
                Ok(OpcodeTypes::LDVxDT)
            }

            else if (opcode & 0x00FF) == 0x000A{
                Ok(OpcodeTypes::LDVxK)
            }

            else if (opcode & 0x00FF) == 0x0015{
                Ok(OpcodeTypes::LDDTVx)
            }

            else if (opcode & 0x00FF) == 0x0018{
                Ok(OpcodeTypes::LDSTVx)
            }

            else if (opcode & 0x00FF) == 0x001E{
                Ok(OpcodeTypes::ADDIVx)
            }

            else if (opcode & 0x00FF) == 0x0029{
                Ok(OpcodeTypes::LDFVx)
            }

            else if (opcode & 0x00FF) == 0x0033{
                Ok(OpcodeTypes::LDBVx)
            }

            else if (opcode & 0x00FF) == 0x0055{
                Ok(OpcodeTypes::LDIVx)
            }

            else if (opcode & 0x00FF) == 0x0065{
                Ok(OpcodeTypes::LDVxI)
            }

            else{
                Err("Incorrect Opcode".to_string())
            }
        }

        else{
            Err("Incorrect Opcode".to_string())
        }
    }
}


#[cfg(test)]
mod tests{
    use super::{OpcodeTypes, Opcode};

    #[test]
    fn eight_xy0(){
        assert_eq!(OpcodeTypes::LDVxVy, Opcode::find_kind(0x8120).unwrap())
    }
}