//! T-Head E907 microarchitecture support.

/// Initialize the E907 runtime and return to the BootROM when `main` returns.
///
/// # Safety
///
/// This is the naked entry point called by the BootROM.
#[cfg_attr(
    any(all(feature = "thead-e907", target_arch = "riscv32"), doc),
    unsafe(link_section = ".text.entry")
)]
#[unsafe(naked)]
pub unsafe extern "C" fn thead_e907_start() {
    use crate::main;

    const STACK_SIZE: usize = 8 * 1024;

    #[repr(C)]
    #[allow(dead_code)] // Accessed directly by the startup assembly.
    struct RomContext {
        sp: usize,
        ra: usize,
        gp: usize,
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
        // Disable M-mode interrupts before switching away from the BootROM
        // stack, then preserve the fixed state and scratch registers we use.
        "csrrci  a0, mstatus, 0x8
        csrr    a1, mie
        csrw    mie, zero
        addi    sp, sp, -32
        sw      ra, 0(sp)
        sw      gp, 4(sp)
        sw      t0, 8(sp)
        sw      t1, 12(sp)
        sw      t2, 16(sp)
        .option push
        .option norelax
        la      t0, {rom_context}
        .option pop
        addi    t1, sp, 32
        sw      t1, {rom_sp}(t0)
        lw      t1, 0(sp)
        sw      t1, {rom_ra}(t0)
        lw      t1, 4(sp)
        sw      t1, {rom_gp}(t0)
        lw      t1, 8(sp)
        sw      t1, {rom_t0}(t0)
        lw      t1, 12(sp)
        sw      t1, {rom_t1}(t0)
        lw      t1, 16(sp)
        sw      t1, {rom_t2}(t0)
        sw      a0, {rom_mstatus}(t0)
        sw      a1, {rom_mie}(t0)
        csrr    t1, 0x7c0
        sw      t1, {rom_mxstatus}(t0)
        csrr    t1, 0x7c1
        sw      t1, {rom_mhcr}(t0)
        csrr    t1, 0x7c5
        sw      t1, {rom_mhint}(t0)
        addi    sp, sp, 32",
        // Enable the T-Head instruction extension used by the cache teardown.
        "li      t1, 1 << 22
        csrs    0x7c0, t1",
        // Linker relaxation may use gp for small data, so initialize it without
        // consulting the BootROM's gp value.
        ".option push
        .option norelax
        la      gp, __global_pointer$
        .option pop",
        // Prepare the Rust stack.
        "la      sp, {stack}
        li      t0, {stack_size}
        add     sp, sp, t0",
        // `.bss.uninit` contains the context and stack and must stay intact.
        "la      t1, sbss
        la      t2, ebss
    1:  bgeu    t1, t2, 2f
        sw      zero, 0(t1)
        addi    t1, t1, 4
        j       1b
    2:",
        // V821 BootROM hands the E907 over with its FPU state active. Avoid
        // clearing fcsr so that returning to the ROM preserves that state.
        "call   {main}",
        // Restore the BootROM's callee state. a0-a5 are caller-saved and only
        // serve as temporaries during the restore sequence. The raw words are
        // E907 dcache.ciall, sync.is, icache.iall, and sync.is respectively.
        "csrci  mstatus, 0x8
        csrw    mie, zero
        fence   rw, rw
        .word   0x0030000b
        .word   0x01b0000b
        .word   0x0100000b
        .word   0x01b0000b
        fence   rw, rw
        fence.i
        .option push
        .option norelax
        la      a0, {rom_context}
        .option pop
        lw      a1, {rom_gp}(a0)
        lw      a2, {rom_mie}(a0)
        lw      a3, {rom_mstatus}(a0)
        lw      a4, {rom_sp}(a0)
        lw      ra, {rom_ra}(a0)
        lw      t0, {rom_t0}(a0)
        lw      t1, {rom_t1}(a0)
        lw      t2, {rom_t2}(a0)
        lw      a5, {rom_mhint}(a0)
        csrw    0x7c5, a5
        lw      a5, {rom_mhcr}(a0)
        csrw    0x7c1, a5
        lw      a5, {rom_mxstatus}(a0)
        csrw    0x7c0, a5
        mv      gp, a1
        mv      sp, a4
        csrw    mie, a2
        csrw    mstatus, a3
        ret",
        rom_context = sym ROM_CONTEXT,
        rom_sp = const core::mem::offset_of!(RomContext, sp),
        rom_ra = const core::mem::offset_of!(RomContext, ra),
        rom_gp = const core::mem::offset_of!(RomContext, gp),
        rom_t0 = const core::mem::offset_of!(RomContext, t0),
        rom_t1 = const core::mem::offset_of!(RomContext, t1),
        rom_t2 = const core::mem::offset_of!(RomContext, t2),
        rom_mie = const core::mem::offset_of!(RomContext, mie),
        rom_mstatus = const core::mem::offset_of!(RomContext, mstatus),
        rom_mxstatus = const core::mem::offset_of!(RomContext, mxstatus),
        rom_mhcr = const core::mem::offset_of!(RomContext, mhcr),
        rom_mhint = const core::mem::offset_of!(RomContext, mhint),
        stack = sym STACK,
        stack_size = const STACK_SIZE,
        main = sym main,
    )
}

/// Stop a T-Head E907 core.
#[unsafe(naked)]
pub unsafe extern "C" fn thead_e907_halt() -> ! {
    core::arch::naked_asm!(
        "csrw   mie, zero
        csrci   mstatus, 0x8
    1:  wfi
        j       1b",
    )
}
