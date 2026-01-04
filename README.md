# SawitCore OS

SawitCore OS is a specialized Unikernel Database Operating System built with Rust, designed to run directly on bare metal (x86_64) without a traditional OS like Linux. It prioritizes performance for database workloads through direct hardware access (VirtIO).

## Project Structure

The project is organized as a Rust workspace:

- `kernel/`: The core OS kernel (bootable binary).
    - `src/memory/`: Paging and Heap allocation.
    - `src/drivers/`: Hardware drivers (VirtIO Net, Block).
    - `src/storage/`: Database engine (Ported from SawitDB).
- `tools/`: Helper scripts for building and running via QEMU.
- `x86_64-sawitcore.json`: Custom target specification for the unikernel.

## Toolchain Requirements

To build and run this project, you need the **Rust Nightly** toolchain and QEMU.

### 1. Install Rust Nightly & Components
SawitCore requires nightly features for bare-metal development.

```bash
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview --toolchain nightly
```

### 2. Install Bootimage
The `bootimage` tool compiles the kernel and bootloader into a bootable disk image.

```bash
cargo install bootimage
```

### 3. Install QEMU
Ensure `qemu-system-x86_64` is in your PATH.

## Building and Running

### Build
```bash
cargo build --target x86_64-sawitcore.json
```

### Run (QEMU)
Use the provided PowerShell script:

```powershell
./tools/run.ps1
```

Or manually:
```bash
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-sawitcore/debug/bootimage-sawitcore.bin
```
