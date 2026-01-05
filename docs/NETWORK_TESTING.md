# SawitCore-OS Network Testing Guide

## Quick Test

### Option 1: Auto-detect Network Mode (Recommended)
```powershell
.\tools\test_network.ps1
```

This script will:
- Check if running as Administrator
- Detect TAP adapter availability
- Auto-select best networking mode
- Launch QEMU with appropriate configuration

### Option 2: Force TAP Mode (Requires Admin)
```powershell
# Right-click PowerShell → "Run as Administrator"
.\tools\run_tap_admin.ps1
```

### Option 3: Force User-Mode (No Admin Required)
```powershell
.\tools\run.ps1
```
**Note**: User-mode has RX limitations (connection works but no data received)

---

## Verify Network

### Test TCP Connection
```powershell
# In another terminal
python verify_net.py
```

**Expected with TAP**:
```
✓ RX SUCCESS! Packet Len: 52
[Remote] Hello SawitCore
✓ VERIFICATION PASSED!
```

**Expected with User-mode**:
```
Connected!
Connection failed: timed out
```

### Manual Test with Telnet
```powershell
# With TAP networking
telnet 10.0.2.15 8023

# With User-mode
telnet 127.0.0.1 8023
```

---

## Troubleshooting

### "TAP adapter disconnected"
**Solution**: Run as Administrator, script will auto-enable

### "Access denied" when enabling TAP
**Solution**: Right-click PowerShell → "Run as Administrator"

### "Could not configure '/dev/tap0'"
**Solution**: Verify adapter name is exactly `tap0` in Network Connections

### Still getting "NotReady" with TAP
1. Check TAP adapter status:
   ```powershell
   Get-NetAdapter -Name "tap0"
   ```
   Should show: `Status: Up`

2. Check IP configuration:
   ```powershell
   Get-NetIPAddress -InterfaceAlias "tap0"
   ```
   Should show: `10.0.2.1/24`

3. Check firewall:
   ```powershell
   # Temporarily disable to test
   Set-NetFirewallProfile -Profile Domain,Public,Private -Enabled False
   ```

---

## Network Architecture

### TAP Mode (Bidirectional)
```
Host (10.0.2.1) <---> TAP Bridge <---> Guest (10.0.2.15)
                   Full packet flow
```

### User Mode (Limited)
```
Host <--- NAT/SLIRP ---> Guest
      Only outbound works
```

---

## Current Status

Check `qemu.log` for network activity:
```powershell
# Look for RX packets
Select-String -Path qemu.log -Pattern "RX SUCCESS"

# Look for errors
Select-String -Path qemu.log -Pattern "RX Error"
```

**With TAP**: Should see "RX SUCCESS" messages
**With User-mode**: Will see "RX Error: NotReady"
