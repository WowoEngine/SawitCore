$qemuPath = "T:\qemu"
if (Test-Path $qemuPath) {
    Write-Host "Adding $qemuPath to PATH" -ForegroundColor Green
    $env:PATH = "$qemuPath;$env:PATH"
}

# Run via cargo run (which uses bootimage runner -> qemu)
# Run via cargo run with network flags
cargo run -- -netdev user,id=u1,hostfwd=tcp::8023-:8023 -device virtio-net-pci,netdev=u1 -serial stdio > qemu.log 2>&1
