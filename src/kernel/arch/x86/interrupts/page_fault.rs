use crate::enum_str;
enum_str! {
    enum PageFaultErrorCode {
        ProtectionViolation = 1 << 0,
        CausedByWrite = 1 << 1,
        UserMode = 1 << 2,
        MalformedTable = 1 << 3,
        InstructionFetch = 1 << 4,
        Unknown = 1 << 5,
    }
}

impl From<u64> for PageFaultErrorCode {
    fn from(code: u64) -> Self {
        match code {
            0x1 =>  PageFaultErrorCode::ProtectionViolation,
            0x2 =>  PageFaultErrorCode::CausedByWrite,
            0x3 =>  PageFaultErrorCode::UserMode,
            0x4 =>  PageFaultErrorCode::MalformedTable,
            0x5 =>  PageFaultErrorCode::InstructionFetch,
            _ =>    PageFaultErrorCode::Unknown
        }
    }
}