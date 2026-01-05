# Simple TAP Network Launcher
# Run this as Administrator

$ErrorActionPreference = "Stop"

# Check Admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "ERROR: Must run as Administrator!" -ForegroundColor Red
    Write-Host "Right-click PowerShell → 'Run as Administrator'" -ForegroundColor Yellow
    pause
    exit 1
}

Write-Host "=== TAP Network Setup ===" -ForegroundColor Cyan

# Find TAP adapter
$tapAdapter = Get-NetAdapter | Where-Object {$_.InterfaceDescription -like "*TAP*"} | Select-Object -First 1

if (-not $tapAdapter) {
    Write-Host "ERROR: No TAP adapter found!" -ForegroundColor Red
    Write-Host "Install OpenVPN TAP driver first." -ForegroundColor Yellow
    pause
    exit 1
}

Write-Host "Found TAP adapter: $($tapAdapter.Name)" -ForegroundColor Green
Write-Host "Status: $($tapAdapter.Status)" -ForegroundColor Gray

# Rename if needed
if ($tapAdapter.Name -ne "tap0") {
    Write-Host "Renaming adapter to 'tap0'..." -ForegroundColor Yellow
    Rename-NetAdapter -Name $tapAdapter.Name -NewName "tap0"
    $tapAdapter = Get-NetAdapter -Name "tap0"
}

# Enable if needed
if ($tapAdapter.Status -ne "Up") {
    Write-Host "Enabling TAP adapter..." -ForegroundColor Yellow
    Enable-NetAdapter -Name "tap0" -Confirm:$false
    Start-Sleep -Seconds 2
    $tapAdapter = Get-NetAdapter -Name "tap0"
}

Write-Host "TAP adapter ready: $($tapAdapter.Status)" -ForegroundColor Green

# Set QEMU path
$env:Path += ";T:\qemu"

# Launch with TAP
Write-Host ""
Write-Host "Starting QEMU with TAP networking..." -ForegroundColor Cyan
Write-Host "Guest IP: 10.0.2.15" -ForegroundColor Gray
Write-Host "Host IP: 10.0.2.1" -ForegroundColor Gray
Write-Host ""

cargo run -- -netdev tap,id=u1,ifname=tap0 -device virtio-net-pci,netdev=u1 -serial stdio *>&1 | Tee-Object -FilePath qemu.log
