pub mod nr;

/// Syscall type
pub type SCT = isize;

#[inline(always)]
pub unsafe fn syscall0(n: SCT) -> SCT {
    let mut ret : SCT;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: SCT, a1: SCT) -> SCT {
    let mut ret : SCT;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: SCT, a1: SCT, a2: SCT) -> SCT {
    let mut ret : SCT;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: SCT, a1: SCT, a2: SCT, a3: SCT) -> SCT {
    let mut ret : SCT;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT) -> SCT {
    let mut ret : SCT;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT) -> SCT {
    let mut ret : SCT;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall6(n: SCT, a1: SCT, a2: SCT, a3: SCT, a4: SCT, a5: SCT,
                       a6: SCT) -> SCT {
    let mut ret : SCT;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5), "{r9}"(a6)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}
