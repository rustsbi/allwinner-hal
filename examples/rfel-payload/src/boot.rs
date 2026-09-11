// Only referenced as an assembly symbol; a0 carries the parameter pointer.
unsafe extern "C" {
    fn __rfel_payload__main();
}

// Preserve each architecture's cache prefix before Rust executes. Locate the
// parameters relative to the PC and pass them in a0 without changing FEL's ra.
core::arch::global_asm!(
    r#"
.section .text.payload,"ax"
.global _start
.option push
.option norelax
.option norvc
_start:
    lui t1, 0x400
    csrrs zero, 0x7c0, t1
.if {rv64}
    lui t1, 0x30
    addiw t1, t1, 19
    csrrs zero, 0x7c2, t1
.else
    fence.i
.endif
.Lparameters:
    auipc a0, %pcrel_hi(__payload_end)
    addi a0, a0, %pcrel_lo(.Lparameters)
    jal zero, {body}
.option pop
"#,
    rv64 = const cfg!(target_arch = "riscv64") as usize,
    body = sym __rfel_payload__main,
);
