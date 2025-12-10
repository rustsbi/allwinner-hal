/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
///
/// NOTE: `mxstatus` is a custom T-Head register. Do not confuse with `mstatus`.
/// It allows for configuring special eXtensions. See further below for details.
#[cfg_attr(
    any(all(feature = "thead-c906", target_arch = "riscv64"), doc),
    unsafe(link_section = ".text.entry")
)]
#[unsafe(naked)]
pub unsafe extern "C" fn thead_c906_start() -> ! {
    use super::riscv_fpu::init_floating_point;
    use crate::main;
    const STACK_SIZE: usize = 8 * 1024;
    #[unsafe(link_section = ".bss.uninit")]
    static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
    core::arch::naked_asm!(
        // Disable interrupt
        "csrw   mie, zero",
        // Enable T-Head ISA extension
        "li     t1, 1 << 22",
        "csrs   0x7C0, t1",
        // Enable T-Head caches
        "li     t0, 0x70013
        csrw    0x7C2, t0
        li      t0, 0x11ff
        csrw    0x7C1, t0
        li      t0, 0x638000
        csrs    0x7C0, t0
        li      t0, 0x16e30c
        csrw    0x7C5, t0",
        // Invalidate instruction and data cache, branch history table
        // and branch target buffer table
        "li     t1, 0x30013",
        "csrs   0x7C2, t1",
        // Prepare programming language stack
        "la     sp, {stack}
        li      t0, {stack_size}
        add     sp, sp, t0",
        // Clear `.bss` section
        "la     t1, sbss
        la      t2, ebss
    3:  bgeu    t1, t2, 3f
        sd      zero, 0(t1)
        addi    t1, t1, 8
        j       3b
    3:  ",
        // Enable floating point unit
        "call   {init_floating_point}",
        // Start Rust main function
        "call   {main}",
        // Platform halt if main function returns
        "call   {thead_c906_halt}",
        stack      =   sym STACK,
        stack_size = const STACK_SIZE,
        init_floating_point = sym init_floating_point,
        main       =   sym main,
        thead_c906_halt = sym thead_c906_halt,
    )
}

/// Stop a T-Head C906 core.
#[unsafe(naked)]
pub unsafe extern "C" fn thead_c906_halt() -> ! {
    core::arch::naked_asm!(
        "li     x3, 0x20aaa
        csrs    mie, x3
        csrci   mstatus, 0x8
        csrci   0x7C5, 0x4
        .insn i 0x0B, 0, x0, x0, 0x001
        csrci   0x7C1, 0x2
        wfi",
    )
}
