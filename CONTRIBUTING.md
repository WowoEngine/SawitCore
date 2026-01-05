# Contributing to SawitCore OS

First off, thanks for taking the time to contribute! 🎉

SawitCore is an open-source project, and we love to receive contributions from our community — you! There are many ways to contribute, from writing tutorials or blog posts, improving the documentation, submitting bug reports and feature requests, or writing code which can be incorporated into SawitCore itself.

## How to Contribute

### 1. Fork & Clone
Fork the repository on GitHub and clone it to your local machine:

```bash
git clone https://github.com/WowoEngine/SawitCore.git
cd SawitCore
```

### 2. Prerequisites
Ensure you have the required toolchain:
- **Rust Nightly**: `rustup toolchain install nightly`
- **Bootimage**: `cargo install bootimage`
- **QEMU**: Installed and available in PATH.

### 3. Create a Branch
Create a new branch for your feature or fix:

```bash
git checkout -b feature/amazing-feature
# or
git checkout -b fix/annoying-bug
```

### 4. Make Changes
- Write your code.
- Ensure it compiles: `cargo build`
- Run it in QEMU to verify: `cargo run` (or `./tools/run.ps1`)

### 5. Commit & Push
Commit your changes with a clear and descriptive message:

```bash
git commit -m "feat: implement virtio disk driver"
git push origin feature/amazing-feature
```

### 6. Pull Request
Open a Pull Request on GitHub. Please describe your changes in detail and reference any related issues.

## Style Guide
- Use `rustfmt` to format your code.
- Follow standard Rust idioms.
- Keep the kernel minimal and focused on database workloads.

## Code of Conduct
Please note that this project is released with a [Contributor Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms. (Note: We strictly prohibit "Anggaran Bocor" behavior here 😉).

## Need Help?
If you have questions, feel free to open an issue or contact the maintainers.

Happy Coding! 🦀
