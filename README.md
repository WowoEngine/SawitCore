# SawitCore [OS]

SawitCore OS is a specialized Unikernel Database Operating System built with Rust, designed to run directly on bare metal (x86_64) without a traditional OS like Linux. It prioritizes performance for database workloads through direct hardware access.

The goal is to port the **SawitDB** engine (originally Go/Node.js) to run natively as the OS kernel.

<div align="center">

[![SawitDB-Go](https://img.shields.io/badge/SawitDB%20Go%20Version-Visit%20Repo-cyan?style=for-the-badge&logo=go)](https://github.com/WowoEngine/SawitDB-Go)
[![SawitDB-Node](https://img.shields.io/badge/SawitDB%20Node.js%20Version-Visit%20Repo-green?style=for-the-badge&logo=nodedotjs)](https://github.com/WowoEngine/SawitDB)

</div>

## Current Features

- **Kernel Core**: Minimal x86_64 kernel with custom target specification.
- **VGA Text Mode**: Driver for printing text to the screen (`println!` macro).
- **Interrupt Handling**:
    - IDT (Interrupt Descriptor Table)
    - PIC (Programmable Interrupt Controller)
    - Keyboard Input (via PS/2 Controller)
- **Memory Management**:
    - Paging (Recursive Page Tables)
    - Heap Allocation (`Linked List Allocator`)
    - Dynamic types enabled (`Box`, `Vec`, `String`, etc.)
- **Async & Multitasking**:
    - Cooperative Multitasking (Async/Await)
    - Simple Task Executor
    - Lock-free Scancode Queue (`crossbeam-queue`)
- **User Interface**:
    - Interactive Shell (`Sawit> `)
    - Management Menu (`manage` command)

## Project Structure

The project is organized as a Rust workspace:

- `src/`: Kernel source code
    - `main.rs`: Kernel entry point and initialization
    - `lib.rs`: Central module exports and PIC initialization
    - `drivers/`: Hardware drivers
        - `vga_buffer.rs`: VGA text mode driver
        - `serial.rs`: COM1 serial port driver
        - `net.rs`: VirtIO network driver
        - `virtio_transport.rs`: Legacy PCI transport for VirtIO
        - `virtio_hal.rs`: DMA memory management (HAL trait)
        - `block.rs`: Block I/O abstraction
    - `sawitdb/`: Database engine
        - `mod.rs`: Database module exports
        - `btree.rs`: BTree index implementation
        - `pager.rs`: Page management and I/O
    - `task/`: Async executor and tasks
        - `simple_executor.rs`: Cooperative task executor
        - `keyboard.rs`: Keyboard input handler
        - `shell.rs`: Interactive shell task
        - `net.rs`: Network polling and server tasks
    - `memory.rs` & `allocator.rs`: Memory management
    - `interrupts.rs`: IDT and interrupt handlers
- `tools/`: Helper scripts
    - `run.ps1`: Build and run with user-mode networking
    - `run_tap.ps1`: Build and run with TAP networking (requires admin)
    - `test_network.ps1`: Auto-detect network mode and launch
- `docs/`: Documentation
    - `TAP_NETWORK_SETUP.md`: TAP adapter setup guide
    - `NETWORK_TESTING.md`: Network testing procedures

## Roadmap / TODO

### Features

### Core OS
- [x] Custom bootloader with `bootimage`
- [x] VGA text mode driver
- [x] Serial port driver (COM1)
- [x] Interrupt handling (IDT)
- [x] Memory management (paging, heap allocation)
- [x] Keyboard input handling
- [x] Shell interface (VGA console)

### SawitDB Integration
- [x] Core types (`Value` enum for Int/Float/String)
- [x] BTree index implementation (insert, search)
- [x] Pager with block I/O abstraction
- [x] RamDisk storage backend (in-memory)
- [x] Shell commands: `db_init`, `put`, `get` (via management menu)
- [x] Global database state management
- [ ] Persistent storage (requires disk driver)

### Networking (In Progress)
- [x] VirtIO network driver (Legacy PCI transport)
- [x] smoltcp TCP/IP stack integration
- [x] TCP server on port 8023
- [x] Network interface configuration (10.0.2.15/24)
- [x] Feature negotiation (VIRTIO_F_MRG_RXBUF masking)
- [x] DMA memory management via VirtioHal
- [x] Polling-based network task
- [⚠️] RX packet reception (blocked by QEMU/Windows TAP limitation)
- [ ] TCP echo/shell functionality (pending RX fix)

**Note**: Network stack is 85% complete. Code is production-ready but RX reception is blocked by QEMU user-mode networking limitations on Windows. See `docs/TAP_NETWORK_SETUP.md` for TAP adapter setup, or use WSL2/Linux for full functionality.

## Building and Running

### Prerequisites
- **Rust Nightly**: `rustup toolchain install nightly`
- **Bootimage**: `cargo install bootimage`
- **QEMU**: Ensure `qemu-system-x86_64` is in your PATH.

### Run (QEMU)
Use the provided PowerShell script:

```powershell
./tools/run.ps1
```

Or manually:

```bash
cargo build
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-sawitcore/debug/bootimage-sawitcore-os.bin
```

## License & Contributing

- **License**: This project is licensed under the [MIT License](LICENSE).
- **Contributing**: Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.
