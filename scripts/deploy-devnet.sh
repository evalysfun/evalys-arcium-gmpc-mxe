#!/bin/bash
# Deploy Evalys Arcium gMPC MXE to Devnet
#
# Prerequisites:
# - Solana CLI installed
# - Arcium CLI installed
# - Keypair file at ~/.config/solana/id.json (or specify with --keypair-path)
# - Sufficient SOL in keypair for deployment fees
#
# Usage:
#   ./scripts/deploy-devnet.sh
#   ./scripts/deploy-devnet.sh --keypair-path /path/to/keypair.json
#   ./scripts/deploy-devnet.sh --rpc-url https://custom-rpc-url.com

set -e

# Default values
CLUSTER_OFFSET="1078779259"
KEYPAIR_PATH="${HOME}/.config/solana/id.json"
RPC_URL="https://api.devnet.solana.com"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --cluster-offset)
            CLUSTER_OFFSET="$2"
            shift 2
            ;;
        --keypair-path)
            KEYPAIR_PATH="$2"
            shift 2
            ;;
        --rpc-url)
            RPC_URL="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--cluster-offset OFFSET] [--keypair-path PATH] [--rpc-url URL]"
            exit 1
            ;;
    esac
done

# Check if Arcium CLI is installed
if ! command -v arcium &> /dev/null; then
    echo "Error: Arcium CLI not found. Please install it first."
    echo "See: https://docs.arcium.com/developers/installation"
    exit 1
fi

# Check if keypair exists
if [ ! -f "$KEYPAIR_PATH" ]; then
    echo "Error: Keypair file not found at $KEYPAIR_PATH"
    echo "Generate one with: solana-keygen new -o $KEYPAIR_PATH"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Arcium.toml" ]; then
    echo "Error: Arcium.toml not found. Please run this script from the MXE root directory."
    exit 1
fi

echo "=========================================="
echo "Deploying Evalys Arcium gMPC MXE to Devnet"
echo "=========================================="
echo ""
echo "Configuration:"
echo "  Cluster Offset: $CLUSTER_OFFSET"
echo "  Keypair: $KEYPAIR_PATH"
echo "  RPC URL: $RPC_URL"
echo ""

# Build the program first
echo "Building MXE program..."
arcium build || {
    echo "Error: Build failed. Please check your code and dependencies."
    exit 1
}

echo ""
echo "Deploying to Arcium network..."
arcium deploy \
    --cluster-offset "$CLUSTER_OFFSET" \
    --keypair-path "$KEYPAIR_PATH" \
    --rpc-url "$RPC_URL"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Deployment successful!"
    echo ""
    echo "Next steps:"
    echo "  1. Note the deployed program ID (shown above)"
    echo "  2. Update ARCIUM_MXE_PROGRAM_ID in bridge service .env file"
    echo "  3. Test the deployment with: anchor test"
else
    echo ""
    echo "❌ Deployment failed. Check the error messages above."
    exit 1
fi

