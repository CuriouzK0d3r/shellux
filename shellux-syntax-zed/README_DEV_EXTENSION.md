# Installing Shellux Extension as Dev Extension in Zed

## TL;DR - Quick Fix

Before using "install dev extension" in Zed, the directory **MUST be cleaned up** or installation will take forever:

```bash
cd shellux-syntax-zed

# Remove large unnecessary files
rm -rf grammars/
rm -rf target/
rm -f *.wasm
rm -f Cargo.lock

# Verify size (should be ~100 KB)
du -sh .
```

Now "install dev extension" will be **instant** instead of taking forever! ⚡

---

## The Problem

When you select **"install dev extension"** from Zed's command palette, it loads the extension directly from your working directory. If that directory contains large files, Zed may:

1. Try to process/compile them (especially grammar sources)
2. Copy them to its cache
3. Index them

This causes installation to take **forever** (several minutes or hang completely).

### What Makes It Slow

These files/directories cause the slowdown:

| Item | Size | Why It's Bad |
|------|------|--------------|
| `grammars/bash/` | 11 MB | Full tree-sitter source repo - Zed tries to compile it |
| `grammars/shellux/` | Variable | Additional grammar sources |
| `target/` | 281 MB | Rust build artifacts - completely unnecessary |
| `*.wasm` | 492 KB | Pre-compiled extension binary - only needed if you have custom Rust code |
| `Cargo.lock` | 12 KB | Rust dependency lock - not needed for simple language extensions |

**Total bloat: ~294 MB** 🐌

Without these files: **~100 KB** ⚡

---

## Why These Files Aren't Needed

### `grammars/` Directory

**Not needed because:**
- Bash grammar is **built into Zed** already
- The extension just references it via `grammar = "bash"` in `config.toml`
- Zed manages grammars separately from extensions
- Including grammar sources makes Zed try to compile them

**What to do:**
```bash
rm -rf grammars/
```

### `target/` Directory

**Not needed because:**
- Contains Rust build artifacts from development
- Only needed during compilation, never at runtime
- Extensions are configuration-based, not compiled binaries
- Takes up 281 MB for no reason

**What to do:**
```bash
rm -rf target/
```

### `*.wasm` Files

**Not needed because:**
- For simple language extensions, no custom code is required
- Only needed if you implement `language_server_command` or other Rust callbacks
- This extension only provides syntax highlighting (config-based)
- The 492 KB file is just leftover from development

**What to do:**
```bash
rm -f *.wasm *.wasm.backup
```

### `Cargo.lock`

**Not needed because:**
- Rust dependency lock file
- Only needed if building from source
- No Rust source code in this extension
- Just leftover from template

**What to do:**
```bash
rm -f Cargo.lock
```

---

## Installation Instructions

### Step 1: Clean the Directory

```bash
cd shellux-syntax-zed

# Remove all unnecessary files
rm -rf grammars/ target/
rm -f *.wasm Cargo.lock

# Verify the cleanup worked
du -sh .
# Should show ~100-120 KB
```

### Step 2: Install as Dev Extension

1. **Open Zed**
2. **Open Command Palette**: 
   - macOS: `Cmd+Shift+P`
   - Linux/Windows: `Ctrl+Shift+P`
3. **Type**: "zed: install dev extension"
4. **Select**: Navigate to your cleaned `shellux-syntax-zed` directory
5. **Press Enter**

✅ Installation should complete in **under 1 second**!

### Step 3: Verify Installation

1. Create a test file:
   ```bash
   cat > ~/test.sx << 'EOF'
   #!/usr/bin/env shellux
   
   # This is a comment
   name is "World"
   count := 42
   
   # Function definition
   fn greet(person: string) -> string {
       return "Hello, " + person
   }
   
   # Control flow
   if count > 10 {
       show "Many items"
   }
   EOF
   ```

2. Open in Zed:
   ```bash
   zed ~/test.sx
   ```

3. Check bottom-right corner - should display **"Shellux"**

4. Verify syntax highlighting:
   - Comments should be styled/colored
   - Strings should be highlighted
   - Keywords like `if`, `return` should be emphasized
   - Brackets should match

---

## What Gets Loaded

After cleanup, only these files are in the directory:

```
shellux-syntax-zed/              (~100 KB total)
├── extension.toml               # Extension manifest (required)
├── languages/
│   └── shellux/
│       ├── config.toml          # Language config (required)
│       └── highlights.scm       # Syntax highlighting (required)
├── README.md                    # Documentation (optional)
├── LICENSE                      # License (optional)
├── CHANGELOG.md                 # Version history (optional)
├── install.sh                   # Install script (not used by dev ext)
└── *.md                         # Other docs (optional)
```

**Required files (8 KB):**
- `extension.toml`
- `languages/shellux/config.toml`
- `languages/shellux/highlights.scm`

**Optional files (100 KB):**
- Documentation (README, guides, etc.)
- LICENSE, CHANGELOG

---

## Troubleshooting

### Still Taking Forever?

**Check directory size:**
```bash
du -sh shellux-syntax-zed
```

If it's more than 200 KB, find the culprit:
```bash
du -sh shellux-syntax-zed/*
```

Remove any large files/directories.

### Extension Not Loading?

**Check Zed logs:**
1. Open Command Palette: `Cmd+Shift+P` / `Ctrl+Shift+P`
2. Type: "zed: open log"
3. Search for "shellux" or "extension" errors

**Common errors:**
- Missing `extension.toml` → Must be present
- Missing `languages/shellux/config.toml` → Must be present
- Syntax error in TOML files → Validate syntax
- Permission issues → Check file permissions

**Validate TOML files:**
```bash
# Check for syntax errors
cat extension.toml
cat languages/shellux/config.toml
```

### Syntax Highlighting Not Working?

**This is expected (for now):**
- Extension uses **Bash grammar as fallback**
- Not all Shellux-specific syntax is highlighted correctly
- Keywords like `is`, `fn`, `|>` may not be specially styled
- This is a temporary solution until tree-sitter-shellux is created

**What should work:**
- ✅ File recognition (.sx, .shx → "Shellux")
- ✅ Comments highlighted
- ✅ Strings highlighted  
- ✅ Basic keywords (if, else, return)
- ✅ Bracket matching
- ✅ Comment toggling (`Cmd+/`)

**What won't work (yet):**
- ⏳ Shellux-specific keywords (is, fn)
- ⏳ Pipeline operator (|>)
- ⏳ Advanced syntax features
- ⏳ LSP features (autocomplete, go-to-definition)

See `STATUS.md` for full details.

### Need to Update Extension?

After making changes:

**Option A: Reload extensions**
1. Open Command Palette
2. Type: "zed: reload extensions"
3. Press Enter

**Option B: Restart Zed**
- Just quit and reopen Zed

---

## Alternative: Use install.sh Script

Instead of "install dev extension", you can use the optimized install script:

```bash
cd shellux-syntax-zed
./install.sh
```

**Advantages:**
- Automatically copies only essential files
- Installs to `~/.config/zed/extensions/shellux/`
- Takes <20 milliseconds
- No need to clean directory first
- Production-ready installation

**After running:**
- Restart Zed or reload extensions
- Extension is permanently installed

---

## For Active Development

If you're actively modifying the extension:

### Best Practice: Separate Directories

Keep two directories:

```
~/development/shellux-syntax-zed/     # Full source (with docs, dev files)
└── ...

~/.config/zed/extensions/shellux/     # Runtime (only essentials)
└── ...
```

**Workflow:**
1. Edit files in `~/development/shellux-syntax-zed/`
2. Run `./install.sh` to copy changes
3. Reload extensions in Zed
4. Test changes

This is faster than using "install dev extension" repeatedly.

### Alternative: Minimal Dev Directory

Create a minimal directory specifically for dev extension:

```bash
# Create minimal structure
mkdir -p ~/shellux-zed-dev/languages

# Copy/symlink only essentials
cd ~/shellux-zed-dev
cp ~/shellux-syntax-zed/extension.toml .
cp -r ~/shellux-syntax-zed/languages/shellux languages/

# Use this for "install dev extension"
```

Update files as needed, then reload extensions.

---

## Performance Comparison

| Method | Size | Install Time | Notes |
|--------|------|--------------|-------|
| Dev ext (uncleaned) | 294 MB | 30+ seconds or hangs | ❌ Don't do this |
| Dev ext (cleaned) | 100 KB | <1 second | ✅ Fast! |
| install.sh script | 28 KB | 0.019 seconds | ✅ Fastest! |

**Recommendation:** Use `install.sh` for normal use, cleaned dev extension only for active development.

---

## Understanding the Architecture

### How Zed Extensions Work

Zed extensions are **configuration-based**, not compiled:

```
Extension provides:
├── Manifest (extension.toml)          → What grammar to use, which languages
├── Language config (config.toml)      → File extensions, comment syntax, etc.
└── Tree-sitter queries (*.scm)        → Syntax highlighting rules

Zed provides:
├── Grammar management                 → Downloads/compiles/caches grammars
├── Editor integration                 → Uses configs and queries
└── Language server support            → Manages LSPs
```

**Key insight:** Extensions don't ship grammars, they **reference** them!

### Why Bash Grammar Works

The extension uses:
```toml
# languages/shellux/config.toml
grammar = "bash"
```

This tells Zed: "Use the bash tree-sitter grammar for this language."

Since bash is built into Zed, it works immediately. No need to:
- ❌ Include bash grammar source
- ❌ Compile bash grammar
- ❌ Ship bash .wasm files

Zed handles all of that internally.

### When Shellux Grammar is Ready

Once tree-sitter-shellux exists, the extension will change to:

```toml
# extension.toml
[grammars.shellux]
repository = "https://github.com/username/tree-sitter-shellux"
rev = "main"
```

```toml
# languages/shellux/config.toml
grammar = "shellux"  # Changed from "bash"
```

Zed will then:
1. Download tree-sitter-shellux automatically
2. Compile it to WebAssembly
3. Cache it in `~/.local/share/zed/grammars/`
4. Use it for syntax highlighting

**Still no need to ship grammar source with the extension!**

---

## Summary

### The Fix

✅ Remove `grammars/`, `target/`, `*.wasm`, `Cargo.lock` before using "install dev extension"

### The Benefit

⚡ Installation goes from **30+ seconds** (or hanging) to **<1 second**

### The Reason

🎯 Zed doesn't need grammar sources or build artifacts - just configuration files

### The Result

🚀 Fast, clean, professional dev extension experience!

---

**See Also:**
- `OPTIMIZATION.md` - Technical details on optimization
- `INSTALLATION_FIX.md` - Complete summary of improvements
- `STATUS.md` - Current functionality and limitations
- `QUICKSTART.md` - Quick installation guide
- `README.md` - Full documentation

**Last Updated:** 2024  
**Extension Version:** 0.1.0