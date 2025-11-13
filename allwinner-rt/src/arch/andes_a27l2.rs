/// Jump over head data to executable code.
///
/// TODO Andes start code.
#[cfg(feature = "v821")]
#[unsafe(naked)]
#[unsafe(link_section = ".text.entry")]
pub unsafe extern "C" fn start() -> ! {
    const STACK_SIZE: usize = 8 * 1024;
    #[unsafe(link_section = ".bss.uninit")]
    static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
    core::arch::naked_asm!(
        // Disable interrupt
        "csrw   mie, zero",
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
        // TODO Enable floating point unit
        // Start Rust main function
        "call   {main}",
        // Platform halt if main function returns
    "3: wfi
        j       3b",
        stack      =   sym STACK,
        stack_size = const STACK_SIZE,
        main       =   sym crate::main,
    )
}
