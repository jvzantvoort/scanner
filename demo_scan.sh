#!/bin/bash
echo "=== Network Scanner Demo ==="
echo ""
echo "1. Testing help:"
./target/release/scanner --help | head -15
echo ""
echo "2. Scanning single host with table output:"
./target/release/scanner --target 192.168.1.1 --ports 80,443,8080 --timeout 500
echo ""
echo "3. Testing parser with port ranges:"
./target/release/scanner --target 192.168.1.1 --ports 1-3,80,443 --timeout 300 --format csv
