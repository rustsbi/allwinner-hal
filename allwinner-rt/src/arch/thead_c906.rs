/// Initialize the runtime and return to the BootROM when `main` returns.
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
pub unsafe extern "C" fn thead_c906_start() {
    use super::riscv_fpu::init_floating_point;
    use crate::main;
    const STACK_SIZE: usize = 8 * 1024;

    #[repr(C)]
    #[allow(dead_code)] // Accessed directly by the startup assembly.
    struct RomContext {
        sp: usize,
        ra: usize,
        t0: usize,
        t1: usize,
        t2: usize,
        mie: usize,
        mstatus: usize,
        mxstatus: usize,
        mhcr: usize,
        mhint: usize,
    }

    #[repr(align(16))]
    #[allow(dead_code)] // Accessed directly by the startup assembly.
    struct RuntimeStack([u8; STACK_SIZE]);

    #[unsafe(link_section = ".bss.uninit")]
    static mut ROM_CONTEXT: core::mem::MaybeUninit<RomContext> = core::mem::MaybeUninit::uninit();

    #[unsafe(link_section = ".bss.uninit")]
    static mut STACK: core::mem::MaybeUninit<RuntimeStack> = core::mem::MaybeUninit::uninit();

    core::arch::naked_asm!(
        // Use a temporary ROM stack frame while locating the private runtime
        // context. No scratch register has to carry its address across main.
        "addi   sp, sp, -32
        sd      ra, 0(sp)
        sd      t0, 8(sp)
        sd      t1, 16(sp)
        sd      t2, 24(sp)
        la      t0, {rom_context}
        addi    t1, sp, 32
        sd      t1, {rom_sp}(t0)
        ld      t1, 0(sp)
        sd      t1, {rom_ra}(t0)
        ld      t1, 8(sp)
        sd      t1, {rom_t0}(t0)
        ld      t1, 16(sp)
        sd      t1, {rom_t1}(t0)
        ld      t1, 24(sp)
        sd      t1, {rom_t2}(t0)
        csrr    t1, mie
        sd      t1, {rom_mie}(t0)
        csrrci  t1, mstatus, 0x8
        sd      t1, {rom_mstatus}(t0)
        csrr    t1, 0x7C0
        sd      t1, {rom_mxstatus}(t0)
        csrr    t1, 0x7C1
        sd      t1, {rom_mhcr}(t0)
        csrr    t1, 0x7C5
        sd      t1, {rom_mhint}(t0)
        addi    sp, sp, 32
        csrw    mie, zero",
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
        // Make all runtime writes visible before restoring the BootROM cache
        // policy. These encodings are C906 dcache.ciall, sync.s, icache.iall,
        // and sync.s respectively.
        "fence rw, rw
        .word   0x0030000b
        .word   0x01b0000b
        .word   0x0100000b
        .word   0x01b0000b
        fence.i
        la      a0, {rom_context}
        ld      t0, {rom_mhint}(a0)
        csrw    0x7C5, t0
        ld      t0, {rom_mhcr}(a0)
        csrw    0x7C1, t0
        ld      a1, {rom_mxstatus}(a0)
        ld      a2, {rom_mie}(a0)
        ld      a3, {rom_mstatus}(a0)
        ld      ra, {rom_ra}(a0)
        ld      t0, {rom_t0}(a0)
        ld      t1, {rom_t1}(a0)
        ld      t2, {rom_t2}(a0)
        ld      sp, {rom_sp}(a0)
        csrw    0x7C0, a1
        csrw    mie, a2
        csrw    mstatus, a3
        ret",
        rom_context = sym ROM_CONTEXT,
        rom_sp       = const core::mem::offset_of!(RomContext, sp),
        rom_ra       = const core::mem::offset_of!(RomContext, ra),
        rom_t0       = const core::mem::offset_of!(RomContext, t0),
        rom_t1       = const core::mem::offset_of!(RomContext, t1),
        rom_t2       = const core::mem::offset_of!(RomContext, t2),
        rom_mie      = const core::mem::offset_of!(RomContext, mie),
        rom_mstatus  = const core::mem::offset_of!(RomContext, mstatus),
        rom_mxstatus = const core::mem::offset_of!(RomContext, mxstatus),
        rom_mhcr     = const core::mem::offset_of!(RomContext, mhcr),
        rom_mhint    = const core::mem::offset_of!(RomContext, mhint),
        stack        = sym STACK,
        stack_size   = const STACK_SIZE,
        init_floating_point = sym init_floating_point,
        main         = sym main,
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
