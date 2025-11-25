# Arcium CLI Installation Guide

## ⚠️ Windows Support

**Arcium CLI is not currently supported on Windows.** The official installation script will fail with:
```
✗ unsupported OS: MINGW64_NT-10.0-26100
```

**Recommended Solution**: Use **Windows Subsystem for Linux (WSL2)** with Ubuntu for the best experience.

See [WSL2 Installation](#wsl2-installation-for-windows-users) section below.

## Prerequisites

Before installing Arcium CLI, ensure you have:

- **Rust** (latest stable version)
- **Solana CLI 2.3.0** (install from [here](https://docs.solana.com/cli/install-solana-cli-tools), then run `solana-keygen new`)
- **Yarn** (install from [here](https://yarnpkg.com/getting-started/install))
- **Anchor 0.32.1** (install from [here](https://www.anchor-lang.com/docs/installation))
- **Docker & Docker Compose** (install from [here](https://docs.docker.com/engine/install/))

## Installing Arcium CLI

### Recommended Method: Official Install Script (Mac/Linux/WSL2)

The easiest way to install Arcium CLI is using their official installation script:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://install.arcium.com/ | bash
```

This script will:
- ✅ Check for all required dependencies
- ✅ Install Linux build dependencies automatically (if needed)
- ✅ Download and install `arcup` (Arcium version manager)
- ✅ Install the latest Arcium CLI
- ✅ Install the Arx Node (core node software for encrypted computations)

After installation, verify:

```bash
arcium --version
```

### Manual Installation (If Script Fails)

If the script fails, you can install manually:

1. **Install `arcup`** (Arcium version manager):

   Choose your platform and run:
   
   ```bash
   # For x86_64 Linux
   TARGET=x86_64_linux && curl "https://bin.arcium.com/download/arcup_${TARGET}_0.4.0" -o ~/.cargo/bin/arcup && chmod +x ~/.cargo/bin/arcup
   
   # For aarch64 Linux (ARM)
   TARGET=aarch64_linux && curl "https://bin.arcium.com/download/arcup_${TARGET}_0.4.0" -o ~/.cargo/bin/arcup && chmod +x ~/.cargo/bin/arcup
   
   # For x86_64 macOS (Intel)
   TARGET=x86_64_macos && curl "https://bin.arcium.com/download/arcup_${TARGET}_0.4.0" -o ~/.cargo/bin/arcup && chmod +x ~/.cargo/bin/arcup
   
   # For aarch64 macOS (Apple Silicon)
   TARGET=aarch64_macos && curl "https://bin.arcium.com/download/arcup_${TARGET}_0.4.0" -o ~/.cargo/bin/arcup && chmod +x ~/.cargo/bin/arcup
   ```

2. **Install Arcium CLI using `arcup`**:

   ```bash
   arcup install
   ```

3. **Verify installation**:

   ```bash
   arcium --version
   ```

### Using Anchor (Fallback - Limited Functionality)

Since Arcium extends Anchor, you can use Anchor CLI for basic operations:

```bash
# Build with Anchor (works for basic builds)
anchor build

# Test with Anchor
anchor test
```

**Note**: For full Arcium functionality (deployment, encrypted instructions), you'll need the Arcium CLI.

## Installing Prerequisites

### Install Rust

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or on Windows, download from: https://rustup.rs/
```

### Install Solana CLI

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Verify installation
solana --version
```

### Install Anchor

```bash
# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

# Install Anchor version (e.g., 0.29.0)
avm install 0.29.0
avm use 0.29.0

# Verify installation
anchor --version
```

## Verifying Installation

After installation, verify all tools are available:

```bash
# Check Rust
rustc --version

# Check Solana
solana --version

# Check Anchor
anchor --version

# Check Arcium
arcium --version
```

## WSL2 Installation for Windows Users

Since Arcium CLI doesn't support Windows natively, use WSL2:

### Step 1: Install WSL2

1. **Enable WSL2** in PowerShell (as Administrator):
   ```powershell
   wsl --install
   ```

2. **Restart your computer** when prompted

3. **Install Ubuntu** from Microsoft Store (if not installed automatically)

### Step 2: Set Up Ubuntu in WSL2

1. **Open Ubuntu** from Start Menu

2. **Create a user account** (username and password) when prompted

3. **Update Ubuntu**:
   ```bash
   sudo apt-get update && sudo apt-get upgrade
   ```

### Step 3: Install Prerequisites in WSL2

```bash
# Install build dependencies
sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
solana-keygen new

# Install Yarn
npm install -g yarn

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.32.1
avm use 0.32.1

# Install Docker (if needed)
# Follow: https://docs.docker.com/desktop/wsl/
```

### Step 4: Install Arcium CLI in WSL2

```bash
# Run the official install script
curl --proto '=https' --tlsv1.2 -sSfL https://install.arcium.com/ | bash

# Verify
arcium --version
```

### Step 5: Access Your Project in WSL2

```bash
# Navigate to your project (Windows files are accessible at /mnt/c/)
cd /mnt/c/Users/kamal/Documents/Kahmah/doings/apis/evalys/evalys-arcium-gmpc-mxe

# Or clone the repo fresh in WSL2
git clone <your-repo-url>
cd evalys-arcium-gmpc-mxe
```

## Troubleshooting

### Windows: "unsupported OS" Error

**Solution**: Use WSL2. See [WSL2 Installation](#wsl2-installation-for-windows-users) section above.

### Arcium CLI Not Found

If `arcium` command is not found after installation:

1. **Check PATH**: Ensure `~/.cargo/bin` is in your PATH
   ```bash
   # Linux/Mac/WSL2
   export PATH="$HOME/.cargo/bin:$PATH"
   
   # Add to ~/.bashrc or ~/.zshrc to make permanent
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

2. **Restart Terminal**: Close and reopen your terminal after installation

3. **Verify Cargo Installation**: 
   ```bash
   cargo --version
   ```

4. **Check if `arcup` is installed**:
   ```bash
   which arcup
   # Should show: /home/user/.cargo/bin/arcup
   ```

### Build Errors

If you encounter build errors:

1. **Install Linux build dependencies** (Ubuntu/Debian):
   ```bash
   sudo apt-get update && sudo apt-get upgrade && sudo apt-get install -y pkg-config build-essential libudev-dev libssl-dev
   ```

2. **Update Rust**:
   ```bash
   rustup update
   ```

3. **Update Cargo**:
   ```bash
   cargo update
   ```

4. **Check Dependencies**: Ensure all Rust dependencies are installed

## Next Steps

After installing Arcium CLI:

1. **Build the MXE**:
   ```bash
   cd evalys-arcium-gmpc-mxe
   arcium build
   ```

2. **Run Tests**:
   ```bash
   anchor test
   ```

3. **Deploy**:
   ```bash
   ./scripts/deploy-devnet.sh
   ```

## Resources

- [Arcium Developer Documentation](https://docs.arcium.com/developers)
- [Arcium Installation Guide](https://docs.arcium.com/developers/installation)
- [Arcium GitHub](https://github.com/orgs/arcium-hq/)
- [Arcium Discord](https://discord.com/invite/arcium)

## Quick Install (All-in-One)

For a complete setup on a fresh system:

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# 3. Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.29.0
avm use 0.29.0

# 4. Install Arcium CLI
cargo install arcium-cli

# 5. Verify all installations
rustc --version
solana --version
anchor --version
arcium --version
```

**Note**: Windows users must use WSL2. See [WSL2 Installation](#wsl2-installation-for-windows-users) section above.

