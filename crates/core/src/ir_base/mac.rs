/// macro for creating register instances
#[macro_export]
macro_rules! r {
    ("rax") => {
        $crate::ir_base::reg::PhyRegister::RAX
    };
    ("rbx") => {
        $crate::ir_base::reg::PhyRegister::RBX
    };
    ("rcx") => {
        $crate::ir_base::reg::PhyRegister::RCX
    };
    ("rdx") => {
        $crate::ir_base::reg::PhyRegister::RDX
    };
    ("rsi") => {
        $crate::ir_base::reg::PhyRegister::RSI
    };
    ("rdi") => {
        $crate::ir_base::reg::PhyRegister::RDI
    };
    ("rsp") => {
        $crate::ir_base::reg::PhyRegister::RSP
    };
    ("rbp") => {
        $crate::ir_base::reg::PhyRegister::RBP
    };
}
