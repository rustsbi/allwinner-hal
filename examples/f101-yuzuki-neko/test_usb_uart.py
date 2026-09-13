"""Exercise the F101 CDC console; requires pyserial (python -m pip install pyserial)."""

import argparse
import json
from pathlib import Path
import re
import subprocess
import time

import serial
from serial.tools.list_ports import comports


def open_console():
    deadline = time.monotonic() + 10
    while time.monotonic() < deadline:
        ports = [p for p in comports() if (p.vid, p.pid) == (0x1F3A, 0xF101)]
        if len(ports) > 1:
            raise RuntimeError("More than one F101 CDC console is connected")
        if ports:
            try:
                return serial.Serial(ports[0].device, 115200, timeout=0.1, write_timeout=3)
            except serial.SerialException:
                pass  # Windows can publish the port before it is ready to open.
        time.sleep(0.1)
    raise RuntimeError("F101 CDC console did not appear")


def receive(port, expected):
    data = bytearray()
    deadline = time.monotonic() + 5
    while time.monotonic() < deadline and len(data) < len(expected):
        data.extend(port.read(len(expected) - len(data)))
    if bytes(data) != expected:
        raise AssertionError(f"expected {expected!r}, received {bytes(data)!r}")


def start_session(port, greeting):
    # Closing a serial handle does not always deassert DTR on Windows.
    port.dtr = False
    time.sleep(0.1)
    port.reset_input_buffer()
    port.dtr = True
    receive(port, greeting)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--device", required=True, help="F101 selector printed by rfel version")
    parser.add_argument("--rounds", type=int, default=100, help="64-command stress batches")
    parser.add_argument("--report", type=Path, help="Optional JSON result file")
    args = parser.parse_args()
    if args.rounds < 1:
        parser.error("--rounds must be positive")

    results = []
    greeting = b"Welcome to Allwinner-HAL f101-yuzuki-neko example!\r\n> "
    hello = b"hello\r\nhello world\r\n> "

    with open_console() as port:
        # Start a fresh terminal session even if a terminal was open previously.
        start_session(port, greeting)
        results.append({"name": "greeting", "port": port.port})
        print(f"PASS greeting on {port.port}")

        def check(name, data, expected):
            port.write(data)
            receive(port, expected)
            results.append({"name": name, "tx_bytes": len(data), "rx_bytes": len(expected)})
            print(f"PASS {name}")

        check("CRLF", b"hello\r\n", hello)
        check("help", b"help\n", b"help\r\nCommands:\r\n  help   show this help\r\n  hello  print hello world\r\n  exit   return to FEL\r\n> ")
        check("backspace", b"hellx\x08o\r", b"hellx\x08 \x08o\r\nhello world\r\n> ")
        check("whitespace", b"  hello  \n", b"  hello  \r\nhello world\r\n> ")
        check("line overflow", b"x" * 33 + b"\n", b"x" * 32 + b"\x07\r\nunknown command; try help\r\n> ")
        for size in (1, 63, 64, 65, 127, 128, 129, 511, 512, 513):
            count, remainder = divmod(size, 6)
            check(f"{size}-byte write", b"hello\n" * count + b"\n" * remainder,
                  hello * count + b"\r\n> " * remainder)
        for _ in range(args.rounds):
            port.write(b"hello\n" * 64)
            receive(port, hello * 64)
        results.append({"name": "stress", "commands": args.rounds * 64})
        print(f"PASS stress: {args.rounds * 64} commands")

    time.sleep(0.1)
    with open_console() as port:
        start_session(port, greeting)
        port.write(b"hello\n")
        receive(port, hello)
        results.append({"name": "reopen"})
        print("PASS reopen")
        port.write(b"exit\n")
        receive(port, b"exit\r\nBye!\r\n")

    # The selector is mandatory: the host must never fall back to another board.
    deadline = time.monotonic() + 10
    while time.monotonic() < deadline:
        try:
            probe = subprocess.run(["rfel", "--device", args.device, "read32", "0x22000"],
                                   capture_output=True, text=True, timeout=3)
        except subprocess.TimeoutExpired:
            continue
        # Some rfel command errors are printed despite a zero process exit code.
        if probe.returncode == 0 and re.search(r"(?m)^0x[0-9a-fA-F]{8}\s*$", probe.stdout):
            results.append({"name": "exit to FEL", "selector": args.device})
            print("PASS exit to FEL on the selected board")
            break
        time.sleep(0.2)
    else:
        raise RuntimeError("The selected F101 did not return to FEL")

    if args.report:
        args.report.write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
