# Run SawitCore-OS with TAP Networking (Admin Required)
# This script must be run as Administrator to access TAP adapter

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "ERROR: This script must be run as Administrator!" -ForegroundColor Red
    Write-Host "Right-click PowerShell and select 'Run as Administrator', then run this script again." -ForegroundColor Yellow
    pause
    exit 1
}

# Set QEMU path
$env:Path += ";T:\qemu"
if (Test-Path "T:\qemu") {
    Write-Host "Adding T:\qemu to PATH" -ForegroundColor Green
} else {
    Write-Host "WARNING: T:\qemu not found, using system QEMU" -ForegroundColor Yellow
}

# Verify TAP adapter exists
$tapAdapter = Get-NetAdapter | Where-Object {$_.Name -eq "tap0"}
if (-not $tapAdapter) {
    Write-Host "ERROR: TAP adapter 'tap0' not found!" -ForegroundColor Red
    Write-Host "Please create and configure TAP adapter. See docs/TAP_NETWORK_SETUP.md" -ForegroundColor Yellow
    pause
    exit 1
}

if ($tapAdapter.Status -ne "Up") {
    Write-Host "WARNING: TAP adapter 'tap0' is not enabled. Attempting to enable..." -ForegroundColor Yellow
    Enable-NetAdapter -Name "tap0" -Confirm:$false
    Start-Sleep -Seconds 2
}

Write-Host "TAP adapter 'tap0' status: $($tapAdapter.Status)" -ForegroundColor Green

# Run QEMU with TAP networking
Write-Host "Starting SawitCore-OS with TAP networking..." -ForegroundColor Cyan
cargo run -- -netdev tap,id=u1,ifname=tap0 -device virtio-net-pci,netdev=u1 -serial stdio *>&1 | Tee-Object -FilePath qemu.log
