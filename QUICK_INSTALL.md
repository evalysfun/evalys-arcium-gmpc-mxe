# Quick Install Guide

## ⚠️ Windows Users

**Arcium CLI does not support Windows.** You must use **WSL2 (Windows Subsystem for Linux)**.

See [INSTALLATION.md](INSTALLATION.md#wsl2-installation-for-windows-users) for WSL2 setup instructions.

## Step-by-Step Installation (Mac/Linux/WSL2)

### 1. Install Prerequisites

**Install Rust** (if not installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Install Solana CLI 2.3.0**:
```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
solana-keygen new
```

**Install Yarn**:
```bash
npm install -g yarn
```

**Install Anchor 0.32.1** (Required - Arcium extends Anchor):
```bash
# Install Anchor Version Manager (AVM)
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

# Install Anchor version 0.32.1 (Arcium requirement)
avm install 0.32.1

# Use Anchor 0.32.1
avm use 0.32.1

# Verify installation
anchor --version
```

**Install Docker & Docker Compose** (if not installed):
- Follow: https://docs.docker.com/engine/install/

### 2. Install Arcium CLI

**Recommended: Official Install Script**

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://install.arcium.com/ | bash
```

This will automatically:
- Check all dependencies
- Install `arcup` (Arcium version manager)
- Install Arcium CLI
- Install Arx Node

**Verify installation**:

```bash
arcium --version
```

**Manual Installation** (if script fails):

```bash
# Install arcup for your platform
TARGET=x86_64_linux && curl "https://bin.arcium.com/download/arcup_${TARGET}_0.4.0" -o ~/.cargo/bin/arcup && chmod +x ~/.cargo/bin/arcup

# Install Arcium CLI
arcup install

# Verify
arcium --version
```

### 3. Verify Installation

```bash
# Check all tools
rustc --version      # Should show Rust version
cargo --version      # Should show Cargo version
solana --version     # Should show Solana CLI version
yarn --version       # Should show Yarn version
anchor --version     # Should show Anchor 0.32.1
arcium --version     # Should show Arcium CLI version
docker --version     # Should show Docker version
```

### 4. Build the MXE

Once installed, you can build:

```bash
# Using Arcium CLI (recommended)
arcium build

# Or using Anchor (basic build - limited functionality)
anchor build
```

## Quick Install (All-in-One Script)

For a complete setup on a fresh Linux/Mac/WSL2 system:

```bash
# 1. Install build dependencies (Ubuntu/Debian)
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev libssl-dev

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
solana-keygen new

# 4. Install Yarn
npm install -g yarn

# 5. Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.32.1
avm use 0.32.1

# 6. Install Arcium CLI (official method)
curl --proto '=https' --tlsv1.2 -sSfL https://install.arcium.com/ | bash

# 7. Verify all installations
rustc --version
solana --version
yarn --version
anchor --version
arcium --version
```

## Troubleshooting

**If `cargo install` fails**:
- Make sure Rust is up to date: `rustup update`
- Check your internet connection
- Try with `--force` flag: `cargo install --force ...`

**If Anchor installation fails**:
- Make sure you have the latest Rust: `rustup update stable`
- Check Anchor GitHub for latest installation method

**If Arcium CLI installation fails**:
- Make sure you're on Linux/Mac/WSL2 (Windows is not supported)
- Check that all prerequisites are installed
- Try manual installation method (see INSTALLATION.md)
- Check Arcium's official channels: https://docs.arcium.com/developers/installation

**If you're on Windows**:
- You must use WSL2. See INSTALLATION.md for WSL2 setup instructions
- Arcium CLI does not work natively on Windows

