# Shellux Zed Extension - Files Overview

## Quick Reference

This document provides a quick overview of all files in the extension.

## Core Extension Files

### `extension.toml` ⭐ REQUIRED
**Format:** TOML  
**Purpose:** Main extension manifest for Zed  
**Status:** ✅ Fixed and working

Defines extension metadata and grammar references. This is the file Zed reads to load the extension.

### `extension.json` 📝 LEGACY
**Format:** JSON  
**Purpose:** Old extension manifest format  
**Status:** ⚠️ Kept for reference only

Not used by current Zed versions. Kept for reference and backward compatibility documentation.

### `install.sh` 🔧
**Format:** Bash script  
**Purpose:** Automated installation  
**Status:** ✅ Working

Copies extension to `~/.config/zed/extensions/shellux/` and provides user feedback.

## Language Configuration

### `languages/shellux/config.toml` ⭐ REQUIRED
**Format:** TOML  
**Purpose:** Language-specific settings for Zed  
**Status:** ✅ Working

Defines:
- File extensions (`.sx`, `.shx`)
- Comment styles
- Bracket pairs
- Grammar name reference

### `languages/shellux/config.json` 📝 LEGACY
**Format:** JSON  
**Purpose:** Old language configuration  
**Status:** ⚠️ Kept for reference only

Legacy format, not used by current Zed.

### `languages/shellux/shellux.json` 📝 REFERENCE
**Format:** JSON (TextMate grammar)  
**Purpose:** TextMate grammar definition  
**Status:** ⚠️ Reference only - not used by Zed

Complete TextMate grammar for Shellux. Zed doesn't support TextMate grammars, so this is kept for:
- Reference when creating tree-sitter grammar
- Use in other editors (VS Code, Sublime, etc.)
- Documentation of Shellux syntax

### `languages/shellux/highlights.scm` 🔜 PLACEHOLDER
**Format:** Tree-sitter queries  
**Purpose:** Syntax highlighting rules  
**Status:** 🔜 Needs tree-sitter grammar first

Will contain tree-sitter queries for highlighting once a proper Shellux tree-sitter grammar exists.

## Documentation Files

### User Documentation

#### `README.md` 📖 PRIMARY DOCS
**For:** End users  
**Contains:**
- Installation instructions (multiple methods)
- Current status and limitations
- Feature list (working and planned)
- Troubleshooting guide
- Configuration examples
- FAQ

Start here if you're installing or using the extension.

#### `QUICKSTART.md` 🚀 QUICK START
**For:** Users who want fast installation  
**Contains:**
- Abbreviated installation steps
- Quick test procedure
- Minimal documentation

For experienced users who just want to get started quickly.

#### `INSTALLATION_TEST.md` 🧪 TESTING
**For:** Users testing the extension  
**Contains:**
- How to verify installation
- Test file creation
- Expected behavior
- What's working vs. not working
- How to check logs

Use this to verify the extension is working correctly.

#### `STATUS.md` 📊 DETAILED STATUS
**For:** Users and contributors wanting full details  
**Contains:**
- Comprehensive status of all features
- Technical details
- Known issues
- What needs to be done
- Timeline and roadmap

Most detailed status information available.

### Developer Documentation

#### `TREE_SITTER_GUIDE.md` 🌳 GRAMMAR DEV GUIDE
**For:** Developers creating tree-sitter grammar  
**Contains:**
- Complete guide to creating tree-sitter grammar
- Prerequisites and setup
- Step-by-step instructions
- Code examples (basic grammar.js)
- Testing procedures
- Integration with Zed
- Resources and examples
- Common issues and solutions

**790 lines** of comprehensive guidance. Start here if you want to create the tree-sitter grammar.

### Project Documentation

#### `CHANGELOG.md` 📝 VERSION HISTORY
**For:** Everyone  
**Contains:**
- Version history
- Changes between versions
- Release notes

Track what's changed over time.

#### `LICENSE` ⚖️ LICENSE
**For:** Legal/licensing info  
**Contains:**
- MIT License text
- Copyright information

Standard MIT license for open source distribution.

### Summary Files (In Parent Directory)

#### `../ZED_EXTENSION_SUMMARY.md` 📋 FIX SUMMARY
**For:** Understanding what was fixed  
**Contains:**
- What was broken
- How it was fixed
- Current limitations
- Next steps

High-level summary of the fix.

## File Dependency Tree

```
extension.toml (REQUIRED)
├── References: grammars.shellux
│   └── Currently: tree-sitter-bash (temporary)
│   └── Future: tree-sitter-shellux (needs creation)
│
└── References: languages/shellux/config.toml

languages/shellux/config.toml (REQUIRED)
├── Defines: grammar = "shellux"
├── Defines: path_suffixes
└── Defines: comments, brackets

languages/shellux/highlights.scm (FUTURE)
└── Requires: tree-sitter-shellux grammar
```

## Reading Order

### For Users Installing the Extension

1. `README.md` - Start here for full overview
2. `QUICKSTART.md` or `install.sh` - Install the extension
3. `INSTALLATION_TEST.md` - Verify it's working
4. `STATUS.md` - Understand current limitations

### For Developers Creating Grammar

1. `STATUS.md` - Understand what's needed
2. `TREE_SITTER_GUIDE.md` - Complete development guide
3. `languages/shellux/shellux.json` - Reference for Shellux syntax
4. External docs (tree-sitter.github.io)

### For Troubleshooting

1. `INSTALLATION_TEST.md` - Test procedures
2. `README.md` - Troubleshooting section
3. `STATUS.md` - Known issues
4. Zed logs (`~/Library/Logs/Zed/Zed.log`)

## File Sizes (Approximate)

```
extension.toml              0.4 KB   (Small config)
extension.json              0.5 KB   (Small config)
config.toml                 0.2 KB   (Small config)
config.json                 0.5 KB   (Small config)
shellux.json                9.0 KB   (Large TextMate grammar)
highlights.scm              2.6 KB   (Tree-sitter queries)
install.sh                  1.9 KB   (Install script)
README.md                   8.0 KB   (Main documentation)
QUICKSTART.md               1.5 KB   (Quick guide)
STATUS.md                   8.5 KB   (Detailed status)
TREE_SITTER_GUIDE.md       21.0 KB   (Comprehensive guide)
INSTALLATION_TEST.md        2.5 KB   (Testing guide)
CHANGELOG.md                3.0 KB   (Version history)
FILES_OVERVIEW.md           8.0 KB   (This file)
```

## Quick Cheat Sheet

| Want to...                        | Read this file              |
|-----------------------------------|-----------------------------|
| Install the extension             | `QUICKSTART.md` or `README.md` |
| Understand limitations            | `STATUS.md`                 |
| Test if it's working              | `INSTALLATION_TEST.md`      |
| Create tree-sitter grammar        | `TREE_SITTER_GUIDE.md`      |
| Fix installation issues           | `README.md` (Troubleshooting) |
| See what's changed                | `CHANGELOG.md`              |
| Understand the fix                | `../ZED_EXTENSION_SUMMARY.md` |
| Reference Shellux syntax          | `languages/shellux/shellux.json` |
| Configure the extension           | `README.md` (Configuration) |

## Essential vs. Optional Files

### Essential for Zed
- ✅ `extension.toml` - Required
- ✅ `languages/shellux/config.toml` - Required
- 🔜 Tree-sitter grammar repository - Needed for full functionality
- 🔜 `languages/shellux/highlights.scm` - Needed for highlighting

### Optional/Reference
- 📝 `extension.json` - Legacy reference
- 📝 `config.json` - Legacy reference
- 📝 `shellux.json` - TextMate reference
- 📖 All `.md` files - Documentation

### Utilities
- 🔧 `install.sh` - Makes installation easier
- ⚖️ `LICENSE` - Required for distribution

---

**Summary:** The extension has proper structure and comprehensive documentation. The only missing piece is the tree-sitter grammar, which is clearly documented in `TREE_SITTER_GUIDE.md`.
