#!/bin/sh

# Set paths
BASE_DIR=$(dirname "$(readlink -f "$0")")

# Parse command-line flags
while [ "$#" -gt 0 ]; do
    case "$1" in
        -p|--path)
            BASE_DIR="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
        ;;
    esac
done

. "$BASE_DIR/genesis now --output $BASE_DIR/build/genesis.rdata"
. "$BASE_DIR/rhex-craft build --scope "" --author 