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

- `src/`: Kernel source code.
    - `main.rs`: Kernel entry point and initialization.
    - `lib.rs`: Central module exports.
    - `drivers/`: Hardware drivers (VGA).
    - `memory.rs` & `allocator.rs`: Memory subsystem.
    - `interrupts.rs`: Hardware interrupt handlers.
    - `task/`: Async executor and tasks (Shell, Keyboard).
- `tools/`: Helper scripts.
    - `run.ps1`: Script to build and run in QEMU.

## Roadmap / TODO

### OS Infrastructure
- [ ] **Disk Driver**: Implement VirtIO Block driver for persistent storage.
- [ ] **Filesystem**: Simple filesystem to manage database pages.
- [ ] **Networking**: VirtIO Net driver for database connections.

### SawitDB Port
- [ ] **Pager**: Buffer pool manager for reading/writing pages.
- [ ] **BTree Index**: Core indexing structure.
- [ ] **Query Parser**: SQL-like query parser.
- [ ] **Executor**: Query execution engine.

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
