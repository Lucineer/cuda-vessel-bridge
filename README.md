# cuda-vessel-bridge

Hardware Abstraction Layer — sensors/actuators to agent runtime with safety (Rust)

Part of the Cocapn communications layer — inter-agent messaging and data exchange.

## What It Does

### Key Types

- `SensorReading` — core data structure
- `ActuatorCommand` — core data structure
- `SafetyCheck` — core data structure
- `EquipmentProfile` — core data structure
- `SafetyMonitor` — core data structure
- `Bridge` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-vessel-bridge.git
cd cuda-vessel-bridge

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_vessel_bridge::*;

// See src/lib.rs for full API
// 13 unit tests included
```

### Available Implementations

- `SensorType` — see source for methods
- `ActuatorType` — see source for methods
- `SafetyCheck` — see source for methods
- `EquipmentProfile` — see source for methods
- `SafetyMonitor` — see source for methods
- `Bridge` — see source for methods

## Testing

```bash
cargo test
```

13 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: comms
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates

- [cuda-communication](https://github.com/Lucineer/cuda-communication)
- [cuda-tuple-space](https://github.com/Lucineer/cuda-tuple-space)

## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
