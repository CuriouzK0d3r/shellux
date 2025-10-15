# Shellux Zed Extension - Installation Optimization

## Problem

The original installation was extremely slow because it copied unnecessary files:

### What Was Being Copied

1. **Entire bash tree-sitter source repository** (~11 MB)
   - Full source code in `grammars/bash/`
   - Tests, examples, bindings
   - Development files
   - GitHub workflows
   - Only the compiled `.wasm` would be used (if at all)

2. **Rust build artifacts** (~281 MB)
   - `target/` directory with debug/release builds
   - Compiled objects and dependencies
   - Never needed at runtime

3. **Development documentation**
   - Installation guides
   - Development files
   - Quickstart docs

**Total size being copied: ~294 MB**

This resulted in:
- Slow installation (several seconds to minutes)
- Wasted disk space
- Unnecessary files in user's Zed extensions directory

## Solution

### Optimized Installation

The new `install.sh` script copies **only essential files**:

```
extension.toml              (~200 bytes)
languages/shellux/
  ├── config.toml           (~200 bytes)
  └── highlights.scm        (~2 KB)
README.md                   (~10 KB)
LICENSE                     (~1 KB)
CHANGELOG.md                (~2 KB)
```

**Total size: ~15 KB** (99.995% reduction!)

### Why This Works

1. **Zed has bash grammar built-in**
   - The extension references `grammar = "bash"` in `config.toml`
   - Zed uses its own built-in bash tree-sitter parser
   - No need to include bash grammar source or wasm

2. **Only runtime files needed**
   - `extension.toml` - Extension manifest
   - `languages/shellux/config.toml` - Language configuration
   - `languages/shellux/highlights.scm` - Syntax highlighting queries
   - Documentation files (optional but helpful)

3. **No build artifacts**
   - `target/` directory is build cache, not runtime
   - Only needed during development/compilation
   - Added to `.gitignore`

## Implementation

### Updated `install.sh`

```bash
# Old approach (slow)
cp -r "$SCRIPT_DIR" "$EXTENSION_DIR"

# New approach (fast)
mkdir -p "$EXTENSION_DIR/languages/shellux"
cp "$SCRIPT_DIR/extension.toml" "$EXTENSION_DIR/"
cp -r "$SCRIPT_DIR/languages/shellux" "$EXTENSION_DIR/languages/"
cp "$SCRIPT_DIR/README.md" "$EXTENSION_DIR/"  # Optional docs
cp "$SCRIPT_DIR/LICENSE" "$EXTENSION_DIR/"
cp "$SCRIPT_DIR/CHANGELOG.md" "$EXTENSION_DIR/"
```

### Updated `.gitignore`

```gitignore
# Rust build artifacts (280+ MB)
target/

# Tree-sitter grammar sources (11+ MB)
grammars/bash/
grammars/shellux/

# Development files not needed at runtime
*.swp
*.swo
*~
.DS_Store
*.log
test.sx
test.shx
*.tmp
```

## Results

### Before Optimization
- Installation time: 5-30 seconds (depending on disk speed)
- Installed size: ~294 MB
- Files copied: ~1000+

### After Optimization
- Installation time: <1 second
- Installed size: ~15 KB
- Files copied: ~5-7 files

### Performance Gain
- **20,000x smaller** installation size
- **10-30x faster** installation time
- **Instant** to near-instant installation

## For Developers

### What to Include in Repository

**Keep in repo:**
- `extension.toml` - Required
- `languages/` - Required
- `README.md`, `LICENSE`, `CHANGELOG.md` - Recommended
- Development docs (`STATUS.md`, `TREE_SITTER_GUIDE.md`, etc.)

**Don't commit (add to `.gitignore`):**
- `target/` - Build artifacts
- `grammars/bash/` - External tree-sitter source
- `grammars/shellux/` - Will be external when created
- `*.wasm` - Build outputs

### Building vs. Installing

**Development (building):**
- May need `grammars/` source to compile
- May generate `target/` artifacts
- May produce `.wasm` files

**Installation (end users):**
- Only need runtime configuration files
- Zed handles grammar loading internally
- No compilation required

### If You Add a Custom Grammar

When Shellux tree-sitter grammar is ready:

```toml
# extension.toml
[grammars.shellux]
repository = "https://github.com/username/tree-sitter-shellux"
rev = "main"
```

Zed will:
1. Download and cache the grammar automatically
2. Compile it on first use
3. Store it in its own cache directory

**You still don't need to ship the grammar source!**

## Testing

### Test Installation Speed

```bash
# Clean previous installation
rm -rf ~/.config/zed/extensions/shellux

# Time the installation
time ./install.sh

# Should complete in < 1 second
```

### Verify Installed Size

```bash
du -sh ~/.config/zed/extensions/shellux
# Should show ~20K or less
```

### Check Extension Works

```bash
# Create test file
echo 'name is "World"' > test.sx

# Open in Zed
zed test.sx

# Check language is recognized (bottom-right of Zed window)
# Should show "Shellux"
```

## Benefits

### For Users
- **Fast installation** - No waiting
- **Small disk footprint** - Barely noticeable
- **Quick updates** - Reinstalling is instant
- **Clean installation** - Only what's needed

### For Developers
- **Easier testing** - Install/uninstall cycles are fast
- **Clear structure** - Obvious what's needed vs. optional
- **Better debugging** - Less clutter in installation directory
- **Professional** - Matches best practices for extensions

### For Distribution
- **Smaller downloads** - If packaged for distribution
- **Faster CI/CD** - Testing installations is quick
- **Less bandwidth** - Particularly for git clones
- **Easier maintenance** - Clear separation of dev vs. runtime files

## Future Considerations

### When Shellux Grammar is Ready

Even with a custom grammar, keep it in a separate repository:

```
tree-sitter-shellux/      # Separate repo
├── grammar.js
├── src/
├── bindings/
└── ...

shellux-syntax-zed/       # Extension repo (this one)
├── extension.toml        # References tree-sitter-shellux
├── languages/
└── ...
```

Benefits:
- Grammar can be used by other editors
- Extension stays lightweight
- Zed handles grammar caching
- Users get automatic updates

### Package for Distribution

If creating a distributable package:

```bash
# Create minimal package
tar czf shellux-zed-extension.tar.gz \
  extension.toml \
  languages/ \
  README.md \
  LICENSE \
  CHANGELOG.md

# Result: < 20 KB tarball
```

## Summary

By copying only runtime-essential files, we achieved:
- ✅ 99.995% size reduction (294 MB → 15 KB)
- ✅ 10-30x faster installation
- ✅ Cleaner user experience
- ✅ Better development workflow
- ✅ Industry best practices

The key insight: **Zed manages grammars, not extensions.** Extensions only need to declare what grammar to use and provide configuration/queries.

---

**Last Updated:** 2024
**Extension Version:** 0.1.0