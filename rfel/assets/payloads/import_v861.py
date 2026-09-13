"""Extract the unchanged MIT-licensed SPI payload from the pinned xfel chips/v881.c."""
import argparse
import hashlib
from pathlib import Path
import re

SOURCE_SHA256 = "213ab05a674c3e1a4d8563b4db9fab9aad1e74253644ad1ab689c454b17168e3"
PAYLOADS = (
    ("chip_spi_init", "spi_v861.bin", "931a4151c7db1ecaf81311591a37d1029a56539113a67abe3abde84528e61c39"),
)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    source = args.source.read_bytes().replace(b"\r\n", b"\n")
    if hashlib.sha256(source).hexdigest() != SOURCE_SHA256:
        parser.error("source does not match xfel commit 7ab3769640be6847e9ca9c5f2df8b11d228d316b")
    source = source.decode("utf-8")
    outputs = []
    for function, name, expected in PAYLOADS:
        body = source.split(function + "(", 1)[1]
        array = re.search(r"static const uint8_t payload\[\] = \{(.*?)\};", body, re.S)
        payload = bytes(int(value, 16) for value in re.findall(r"0x([0-9a-fA-F]{2})\b", array[1]))
        if hashlib.sha256(payload).hexdigest() != expected:
            parser.error(f"unexpected payload hash: {name}")
        outputs.append((Path(__file__).parent / name, payload))
    for path, payload in outputs:
        if args.check:
            if path.read_bytes() != payload:
                parser.error(f"bundled payload differs from upstream: {path.name}")
        else:
            path.write_bytes(payload)
        print(f"{path.name}: {len(payload)} bytes, SHA-256 verified")


if __name__ == "__main__":
    main()
