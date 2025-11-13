#[cfg(riscvf)]
pub unsafe extern "C" fn init_floating_point() {
    unsafe {
        core::arch::asm! {
            "li     t0, 0x4000
            li     t1, 0x2000
            csrrc  x0, mstatus, t0
            csrrs  x0, mstatus, t1
            fscsr  x0",
        }
    }
}

#[cfg(not(riscvf))]
pub unsafe extern "C" fn init_floating_point() {}
