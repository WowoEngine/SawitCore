$qemuPath = "T:\qemu"
if (Test-Path $qemuPath) {
    Write-Host "Adding $qemuPath to PATH" -ForegroundColor Green
    $env:PATH = "$qemuPath;$env:PATH"
}

# Run via cargo run (which uses bootimage runner -> qemu)
# Network Options:
# Option 1: User-mode (SLIRP) - Simple but RX limited
# cargo run -- -netdev user,id=u1,hostfwd=tcp::8023-:8023 -device virtio-net-pci,netdev=u1 -serial stdio *>&1 | Tee-Object -FilePath qemu.log

# Option 2: TAP adapter - Full bidirectional (Recommended, requires TAP setup)
# See docs/TAP_NETWORK_SETUP.md for installation instructions
cargo run -- -netdev tap,id=u1,ifname=tap0 -device virtio-net-pci,netdev=u1 -serial stdio *>&1 | Tee-Object -FilePath qemu.log

# Fallback: User-mode (uncomment if TAP not available)
# cargo run -- -netdev user,id=u1,hostfwd=tcp::8023-:8023 -device virtio-net-pci,netdev=u1 -serial stdio *>&1 | Tee-Object -FilePath qemu.log
