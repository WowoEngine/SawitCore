# Quick Network Test for SawitCore-OS
# This script tests both TAP and User-mode networking

Write-Host "=== SawitCore-OS Network Test ===" -ForegroundColor Cyan
Write-Host ""

# Check if running as Admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if ($isAdmin) {
    Write-Host "[✓] Running as Administrator" -ForegroundColor Green
} else {
    Write-Host "[!] NOT running as Administrator (TAP will not work)" -ForegroundColor Yellow
}

# Check TAP adapter
Write-Host "Checking for TAP adapter..." -ForegroundColor Gray
$tapAdapter = $null
try {
    $tapAdapter = Get-NetAdapter | Where-Object {$_.Name -eq "tap0"} | Select-Object -First 1
} catch {
    Write-Host "Error checking adapters: $_" -ForegroundColor Red
}

if ($tapAdapter) {
    Write-Host "[✓] TAP adapter 'tap0' found" -ForegroundColor Green
    Write-Host "    Status: $($tapAdapter.Status)" -ForegroundColor Gray
    
    # Always try to enable if not Up
    if ($tapAdapter.Status -ne "Up") {
        if ($isAdmin) {
            Write-Host "    Enabling TAP adapter..." -ForegroundColor Yellow
            try {
                Enable-NetAdapter -Name "tap0" -Confirm:$false -ErrorAction Stop
                Start-Sleep -Seconds 2
                $tapAdapter = Get-NetAdapter -Name "tap0" -ErrorAction SilentlyContinue
                Write-Host "    New status: $($tapAdapter.Status)" -ForegroundColor Green
            } catch {
                Write-Host "    Failed to enable: $_" -ForegroundColor Red
            }
        } else {
            Write-Host "    Cannot enable (need Administrator)" -ForegroundColor Red
        }
    } else {
        Write-Host "    TAP adapter is Up" -ForegroundColor Green
    }
} else {
    Write-Host "[X] TAP adapter 'tap0' not found" -ForegroundColor Red
    Write-Host "    Available adapters:" -ForegroundColor Gray
    Get-NetAdapter | Where-Object {$_.InterfaceDescription -like "*TAP*"} | ForEach-Object {
        Write-Host "      - $($_.Name) ($($_.InterfaceDescription))" -ForegroundColor Gray
    }
}

# Check TAP IP configuration
$tapIP = Get-NetIPAddress -InterfaceAlias "tap0" -AddressFamily IPv4 -ErrorAction SilentlyContinue

if ($tapIP) {
    Write-Host "[✓] TAP IP configured: $($tapIP.IPAddress)/$($tapIP.PrefixLength)" -ForegroundColor Green
} else {
    Write-Host "[X] TAP IP not configured" -ForegroundColor Red
}

Write-Host ""
Write-Host "=== Network Mode Selection ===" -ForegroundColor Cyan

# Determine which mode to use
$useTAP = $false
if ($isAdmin -and $tapAdapter -and $tapAdapter.Status -ne "Disconnected" -and $tapIP) {
    $useTAP = $true
    Write-Host "[✓] Using TAP networking (full bidirectional)" -ForegroundColor Green
} else {
    Write-Host "[!] Using User-mode networking (RX limited)" -ForegroundColor Yellow
    if (-not $isAdmin) {
        Write-Host "    Reason: Not running as Administrator" -ForegroundColor Gray
    }
    if (-not $tapAdapter) {
        Write-Host "    Reason: TAP adapter not found" -ForegroundColor Gray
    }
    if ($tapAdapter -and $tapAdapter.Status -eq "Disconnected") {
        Write-Host "    Reason: TAP adapter disconnected" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "Press any key to start QEMU..." -ForegroundColor Cyan
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

# Set QEMU path
$env:Path += ";T:\qemu"

# Launch QEMU
if ($useTAP) {
    Write-Host "Starting with TAP networking..." -ForegroundColor Green
    cargo run -- -netdev tap,id=u1,ifname=tap0 -device virtio-net-pci,netdev=u1 -serial stdio *>&1 | Tee-Object -FilePath qemu.log
} else {
    Write-Host "Starting with User-mode networking..." -ForegroundColor Yellow
    cargo run -- -netdev user,id=u1,hostfwd=tcp::8023-:8023 -device virtio-net-pci,netdev=u1 -serial stdio *>&1 | Tee-Object -FilePath qemu.log
}
