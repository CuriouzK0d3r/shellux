# QUICK FIX for Slow Dev Extension Installation

## The Problem
"install dev extension" takes forever or hangs

## The Fix (30 seconds)
```bash
cd shellux-syntax-zed
rm -rf grammars/ target/ *.wasm Cargo.lock
du -sh .  # Should show ~100 KB
```

## Now Install
In Zed: `Cmd+Shift+P` → "install dev extension" → select directory

## Or Use This Instead
```bash
./install.sh
```
Takes 0.019 seconds, installs permanently!

## Why This Works
- Removed 294 MB of build artifacts
- Only 104 KB of config files needed
- Installation now takes <1 second ⚡

## Read More
- README_DEV_EXTENSION.md - Full guide
- FIX_SUMMARY.md - Complete details
