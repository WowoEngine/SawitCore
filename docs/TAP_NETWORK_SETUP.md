# TAP Network Setup for SawitCore-OS

## Problem

QEMU `-netdev user` (SLIRP) has limitations:
- Only supports **outbound** connections (guest → host)
- **Inbound packets** (host → guest) are not delivered to RX queue
- Result: TCP connections succeed but timeout (no data received)

## Solution: TAP Adapter

Use TAP (network bridge) for full bidirectional packet flow.

---

## Windows Setup

### Step 1: Install OpenVPN TAP Driver

**Option A: Install OpenVPN** (Recommended)
1. Download OpenVPN from: https://openvpn.net/community-downloads/
2. Run installer
3. During installation, ensure **TAP-Windows** component is selected
4. Complete installation

**Option B: Standalone TAP Driver**
1. Download TAP-Windows from: https://build.openvpn.net/downloads/releases/
2. Install `tap-windows-*.exe`

### Step 2: Create TAP Adapter

1. Open **Control Panel** → **Network and Sharing Center**
2. Click **Change adapter settings**
3. Look for adapter named **"TAP-Windows Adapter V9"** or similar
4. Right-click → **Rename** to `tap0` (for easy reference)

### Step 3: Configure TAP Adapter

1. Right-click `tap0` → **Properties**
2. Select **Internet Protocol Version 4 (TCP/IPv4)** → **Properties**
3. Set static IP:
   - **IP Address**: `10.0.2.1`
   - **Subnet Mask**: `255.255.255.0`
   - **Default Gateway**: (leave blank)
4. Click **OK**

### Step 4: Enable Adapter

1. Right-click `tap0` → **Enable** (if disabled)
2. Verify adapter is active (should show "Network" or "Unidentified network")

### Step 5: Run as Administrator

**IMPORTANT**: QEMU requires Administrator privileges to access TAP adapter on Windows.

**Option A: Use Admin Launcher Script** (Recommended)
```powershell
# Right-click PowerShell → "Run as Administrator"
.\tools\run_tap_admin.ps1
```

**Option B: Elevate Current Session**
```powershell
# Right-click PowerShell → "Run as Administrator", then:
.\tools\run.ps1
```

---

## Update QEMU Configuration

### Edit `tools/run.ps1`

Replace the `-netdev user` line with TAP configuration:

**Before:**
```powershell
cargo run -- -netdev user,id=u1,hostfwd=tcp::8023-:8023 -device virtio-net-pci,netdev=u1 -serial stdio
```

**After:**
```powershell
cargo run -- -netdev tap,id=u1,ifname=tap0 -device virtio-net-pci,netdev=u1 -serial stdio
```

**Note**: Remove `hostfwd` - not needed with TAP (direct bridge)

---

## Verify Setup

### 1. Check TAP Adapter
```powershell
ipconfig /all | Select-String "tap0" -Context 5,5
```

Should show:
```
Ethernet adapter tap0:
   IPv4 Address. . . . . . . . . . . : 10.0.2.1
   Subnet Mask . . . . . . . . . . . : 255.255.255.0
```

### 2. Run SawitCore-OS
```powershell
.\tools\run.ps1
```

### 3. Test Network
```powershell
python verify_net.py
```

Expected: **TCP echo succeeds** (no timeout!)

---

## Troubleshooting

### Error: "Could not configure '/dev/tap0'"
- **Cause**: TAP adapter not found
- **Fix**: Verify adapter name is exactly `tap0` in Network Connections

### Error: "Access denied"
- **Cause**: Need admin privileges
- **Fix**: Run PowerShell as Administrator

### Error: "Network cable unplugged"
- **Cause**: TAP adapter disabled
- **Fix**: Enable adapter in Network Connections

### Packets still not received
- **Check**: Firewall blocking TAP adapter
- **Fix**: Add firewall exception for `tap0`

---

## Alternative: User-Mode with Workarounds

If TAP setup fails, you can use user-mode with limitations:

```powershell
# For outbound-only testing (ping from guest)
cargo run -- -netdev user,id=u1 -device virtio-net-pci,netdev=u1 -serial stdio
```

**Limitation**: Guest can ping host, but host cannot reach guest services.

---

## Network Configuration in SawitCore-OS

Current IP configuration (in `src/drivers/net.rs`):
```rust
let ip_addr = IpCidr::new(IpAddress::v4(10, 0, 2, 15), 24);
```

**With TAP**:
- Guest IP: `10.0.2.15`
- Host IP: `10.0.2.1`
- Gateway: `10.0.2.1` (host acts as gateway)

**Testing**:
- From host: `telnet 10.0.2.15 8023`
- From guest: Ping `10.0.2.1` (future feature)

---

## References

- OpenVPN TAP: https://openvpn.net/
- QEMU Networking: https://wiki.qemu.org/Documentation/Networking
- TAP-Windows: https://github.com/OpenVPN/tap-windows6
