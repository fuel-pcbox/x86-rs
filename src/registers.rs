pub struct Flags {
    pub carry: bool,
    pub parity: bool,
    pub adjust: bool,
    pub zero: bool,
    pub sign: bool,
    pub trap: bool,
    pub intr: bool,
    pub direction: bool,
    pub overflow: bool,
    pub iopl: u8, //Actually only 2 bits.
    pub nested_task: bool,
    pub resume: bool,
    pub vm8086: bool,
    pub ac: bool,
    pub vif: bool,
    pub vip: bool,
    pub id: bool,
}

impl Into<u64> for Flags {
    fn into(self) -> u64 {
        ((self.carry as u64) << 0) |
        ((self.parity as u64) << 2) |
        ((self.adjust as u64) << 4) |
        ((self.zero as u64) << 6) |
        ((self.sign as u64) << 7) |
        ((self.trap as u64) << 8) |
        ((self.intr as u64) << 9) |
        ((self.direction as u64) << 10) |
        ((self.overflow as u64) << 11) |
        (((self.iopl & 3) as u64) << 12) |
        ((self.nested_task as u64) << 14) |
        ((self.resume as u64) << 16) |
        ((self.vm8086 as u64) << 17) |
        ((self.ac as u64) << 18) |
        ((self.vif as u64) << 19) |
        ((self.vip as u64) << 20) |
        ((self.id as u64) << 21) | 0x2u64
    }
}

impl From<u64> for Flags {
    fn from(val: u64) -> Flags {
        Flags {
            carry: (val & (1 << 0)) == (1 << 0),
            parity: (val & (1 << 2)) == (1 << 2),
            adjust: (val & (1 << 4)) == (1 << 4),
            zero: (val & (1 << 6)) == (1 << 6),
            sign: (val & (1 << 7)) == (1 << 7),
            trap: (val & (1 << 8)) == (1 << 8),
            intr: (val & (1 << 9)) == (1 << 9),
            direction: (val & (1 << 10)) == (1 << 10),
            overflow: (val & (1 << 11)) == (1 << 11),
            iopl: ((val >> 12) & 3) as u8,
            nested_task: (val & (1 << 14)) == (1 << 14),
            resume: (val & (1 << 16)) == (1 << 16),
            vm8086: (val & (1 << 17)) == (1 << 17),
            ac: (val & (1 << 18)) == (1 << 18),
            vif: (val & (1 << 19)) == (1 << 19),
            vip: (val & (1 << 20)) == (1 << 20),
            id: (val & (1 << 21)) == (1 << 21),
        }
    }
}

pub struct SegmentRegister {
    pub base: u64,
    pub limit: u32,
    pub access: u16,
    pub selector: u16,
}

pub struct FPUMMXRegister {
    pub mantissa: u64,
    pub exp: u16, //only 15 bits
    pub sign: bool,
}

pub struct Registers {
    pub gprs: [u64; 16],
    pub rflags: Flags,
    pub rip: u64,
    pub segs: [SegmentRegister; 6],
    pub gdtr: SegmentRegister,
    pub ldtr: SegmentRegister,
    pub idtr: SegmentRegister,
    pub tr: SegmentRegister,

    pub fprs: [FPUMMXRegister; 8],
    pub top: usize,
    pub fpu_cw: u16,
    pub fpu_sw: u16,
    pub fpu_tw: u16,
    pub fpu_opcode: u16,
    pub fpu_cs: u16,
    pub fpu_ds: u16,
    pub fpu_ip: u64,
    pub fpu_dp: u64,
}

enum Reg8 {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
}

enum Reg16 {
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

enum Reg32 {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

enum SegReg {
    ES,
    CS,
    SS,
    DS,
    FS,
    GS,
}