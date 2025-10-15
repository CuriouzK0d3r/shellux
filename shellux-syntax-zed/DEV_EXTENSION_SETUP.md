# Installing Shellux Extension as Dev Extension in Zed

## The Problem with Dev Extensions

When you use **"install dev extension"** in Zed, it loads the extension directly from your working directory. This means it will see ALL files, including:

- ❌ `grammars/bash/` (11 MB of source code)
- ❌ `target/` (281 MB of build artifacts)
- ❌ Development documentation files

If Zed sees grammar source directories, it may try to **compile them**, which takes forever!

## Quick Fix - Clean Your Directory First

Before using "install dev extension", clean up unnecessary files:

```bash
cd shellux-syntax-zed

# Remove the large directories that slow things down
rm -rf grammars/bash
rm -rf grammars/shellux
rm -rf target/

# Optional: remove large dev docs you don't need at runtime
rm -f TREE_SITTER_GUIDE.md
rm -f FILES_OVERVIEW.md
rm -f STATUS.md
rm -f OPTIMIZATION.md
rm -f INSTALLATION_FIX.md
rm -f DEV_EXTENSION_SETUP.md
```

**After cleanup, your directory should be ~20-30 KB** instead of 294 MB.

## Install as Dev Extension

Now it will be fast:

1. **Open Zed**
2. **Open Command Palette**: `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. **Type**: "zed: install dev extension"
4. **Select the cleaned `shellux-syntax-zed` directory**
5. ✅ **Installation should be instant!**

## Why This Works

The extension only needs these files at runtime:

```
shellux-syntax-zed/
├── extension.toml           # Extension manifest
├── languages/
│   └── shellux/
│       ├── config.toml      # Language configuration
│       └── highlights.scm   # Syntax highlighting
├── README.md                # Optional documentation
├── LICENSE                  # Optional
└── CHANGELOG.md             # Optional
```

**Key Points:**

- ✅ **Bash grammar is built into Zed** - No need to include it
- ✅ **Grammar sources never needed** - Zed manages grammars separately
- ✅ **Build artifacts never needed** - Extensions are config-based, not compiled
- ✅ **Dev docs never needed** - They're for developers, not runtime

## Verify It Works

After installation:

1. Create a test file:
   ```bash
   cat > test.sx << 'EOF'
   #!/usr/bin/env shellux
   
   # Variables
   name is "World"
   count := 42
   
   # Function
   fn greet(person: string) -> string {
       return "Hello, " + person
   }
   EOF
   ```

2. Open in Zed:
   ```bash
   zed test.sx
   ```

3. Check bottom-right corner - should show **"Shellux"**

4. Verify syntax highlighting works (comments, strings, keywords)

## Troubleshooting

### Still Slow?

Check what's in your directory:
```bash
du -sh shellux-syntax-zed
# Should be ~30K or less

du -sh shellux-syntax-zed/*
# Look for large directories
```

If you see large sizes, remove those directories.

### Extension Not Loading?

1. Check Zed logs:
   - Open Command Palette: `Cmd+Shift+P`
   - Type: "zed: open log"
   - Look for errors mentioning "shellux"

2. Common issues:
   - Missing `extension.toml` (required)
   - Missing `languages/shellux/config.toml` (required)
   - Syntax error in TOML files

3. Validate TOML syntax:
   ```bash
   # Check extension.toml
   cat extension.toml
   
   # Check language config
   cat languages/shellux/config.toml
   ```

### Syntax Highlighting Not Working?

The extension uses **Bash grammar as a fallback** until a proper Shellux tree-sitter grammar is created. This means:

- ✅ Basic highlighting works
- ⚠️  Some Shellux-specific syntax may not be perfectly highlighted
- ⚠️  `is` keyword might not be special
- ⚠️  `fn` might not be highlighted as keyword

This is expected! See `STATUS.md` for details.

## Alternative: Use install.sh

Instead of "install dev extension", you can use the optimized install script:

```bash
cd shellux-syntax-zed
./install.sh
```

This automatically copies only essential files to `~/.config/zed/extensions/shellux/` and is **instant** (<20ms).

Then just restart Zed or reload extensions.

## For Development

If you're actively developing the extension:

### Option 1: Keep Source Separate (Recommended)

Keep development files in a separate directory:

```
~/development/
└── shellux-syntax-zed-source/    # Full source with docs, grammars, etc.
    └── ...

~/.config/zed/extensions/
└── shellux/                       # Minimal runtime files only
    └── ...
```

Use `install.sh` to copy changes to Zed's extension directory.

### Option 2: Use Symlinks (Advanced)

Create a minimal directory and symlink it:

```bash
# Create minimal structure
mkdir -p ~/shellux-zed-minimal/languages
cd ~/shellux-zed-minimal

# Symlink essential files
ln -s ~/shellux-syntax-zed/extension.toml .
ln -s ~/shellux-syntax-zed/languages/shellux languages/
ln -s ~/shellux-syntax-zed/README.md .

# Install this minimal version as dev extension
```

## Summary

**DO NOT** use "install dev extension" on the full source directory with:
- ❌ `grammars/` directories (11+ MB)
- ❌ `target/` directory (281 MB)
- ❌ Large documentation files

**DO** either:
- ✅ Clean the directory first (remove grammars/, target/)
- ✅ Use `install.sh` script instead (copies only essentials)
- ✅ Create a minimal directory with only runtime files

**Result:** Installation in <1 second instead of taking forever! 🚀

---

**See also:**
- `OPTIMIZATION.md` - Technical details on why this matters
- `INSTALLATION_FIX.md` - Complete summary of optimization work
- `QUICKSTART.md` - Quick installation guide
- `README.md` - Full documentation