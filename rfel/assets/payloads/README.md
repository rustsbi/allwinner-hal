# FEL helper payloads

The V821 payloads are byte-for-byte extractions from
[`chips/v821.c`](https://github.com/xboot/xfel/blob/v1.3.3/chips/v821.c) in
xfel v1.3.3:

| File | Size | SHA-256 |
| --- | ---: | --- |
| `read32_v821.bin` | 44 | `0cfc30483a755676e39f47375934922a041166c1ee88651bb126dacb9bb8e2dd` |
| `write32_v821.bin` | 44 | `3bcd29372d7be2f75b566c6b6931428f46c5412355f18889c1d7ce1496082daa` |
| `ddr_v821.bin` | 14976 | `4dcc369f8c4d1449101e52986cf03b5b0fafbc778900df75bee991d91da912a2` |
| `spi_v821.bin` | 1206 | `19e1e0c6a3d08b65f0059d748a38619ab7f8a1c7f14e85538f250915c49189b0` |

The V821 BootROM copy helper is encoded as `COPY_V821` in
`src/chips/payload.rs`. Unlike xfel's single-word helper, it copies a block into
SRAM and finishes with `dcache.ciall` plus `sync.is`; without those operations,
USB/FEL can observe stale cache lines during a full BootROM dump.
