$qemuPath = "T:\qemu"
if (Test-Path $qemuPath) {
    Write-Host "Adding $qemuPath to PATH" -ForegroundColor Green
    $env:PATH = "$qemuPath;$env:PATH"
}

# Run via cargo run (which uses bootimage runner -> qemu)
cargo run

