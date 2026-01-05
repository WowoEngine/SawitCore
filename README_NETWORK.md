# SawitCore-OS Network Setup

## Quick Start

### Prerequisites
- OpenVPN TAP driver installed
- TAP adapter configured (see `docs/TAP_NETWORK_SETUP.md`)

### Run with TAP Networking
```powershell
.\tools\run.ps1
```

### Test Network
```powershell
# In another terminal
python verify_net.py
```

Expected output:
```
Attempting to connect to 127.0.0.1:8023...
Connected!
Sending: Hello SawitCore
Received: SawitRemote> You said: Hello SawitCore
✓ VERIFICATION PASSED!
```

## Network Configuration

- **Guest IP**: `10.0.2.15/24`
- **Host IP**: `10.0.2.1`
- **TCP Server**: Port `8023`
- **Protocol**: TCP Echo/Shell

## Troubleshooting

See `docs/TAP_NETWORK_SETUP.md` for detailed setup and troubleshooting.

## Architecture

```
┌─────────────────┐
│  Host (Windows) │
│   10.0.2.1      │
└────────┬────────┘
         │ TAP Bridge
         │
┌────────┴────────┐
│  SawitCore-OS   │
│   10.0.2.15     │
│                 │
│  ┌───────────┐  │
│  │ TCP:8023  │  │
│  │  (Shell)  │  │
│  └───────────┘  │
└─────────────────┘
```
