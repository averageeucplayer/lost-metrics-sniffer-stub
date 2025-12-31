![rustc](https://img.shields.io/badge/rustc-1.92.0-blue.svg)
[![codecov](https://codecov.io/gh/averageeucplayer/lost-metrics-sniffer-stub/graph/badge.svg?token=HHRGYYUNM2)](https://codecov.io/gh/averageeucplayer/lost-metrics-sniffer-stub)
![CI](https://github.com/averageeucplayer/lost-metrics-sniffer-stub/actions/workflows/ci.yml/badge.svg)

# 🚧 Lost Metrics Sniffer Stub  

This repository contains Rust typings extracted from a `meter-core-rs` for public use. The goal is to provide useful type definitions that can be leveraged in other projects without exposing private code.

## 📦 Installation & Setup

### 1️⃣ **Clone the Repository**

```sh
git clone https://github.com/averageeucplayer/lost-metrics-sniffer-stub.git
```

### 2️⃣ Add to Cargo.toml

```toml
[dependencies]
lost-metrics-sniffer-stub = { git = "https://github.com/averageeucplayer/lost-metrics-sniffer-stub", features = ["serde"], branch = "main" }
lost-metrics-sniffer-stub = { git = "https://github.com/averageeucplayer/lost-metrics-sniffer-stub", features = ["bincode"], branch = "main" }
```

### 3️⃣ Configure in startup

```rust
use meter_core::*;

let capture = CustomPacketCapture::new();
let damage_encryption_handler = Custom::DamageEncryptionHandler::new();

set_packet_capture_impl(capture);
damage_encryption_handler_impl(damage_encryption_handler);
```