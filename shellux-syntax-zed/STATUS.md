# Shellux Zed Extension - Current Status

## Overview

This document describes the current state of the Shellux syntax highlighting extension for Zed editor.

**Last Updated:** October 11, 2024  
**Extension Version:** 0.1.0  
**Status:** 🟡 Partially Functional (Basic Support)

## Current State

### ✅ What's Working

1. **File Recognition**
   - `.sx` and `.shx` files are recognized as Shellux
   - Files appear in Zed with the correct language association

2. **Basic Syntax Highlighting**
   - Using Bash tree-sitter grammar as a temporary fallback
   - Comments, strings, and basic keywords are highlighted
   - Better than no highlighting, but not Shellux-specific

3. **Editor Features**
   - Auto-closing brackets, braces, and quotes
   - Comment toggling with `Cmd+/` or `Ctrl+/`
   - Line comments (`#`) and block comments (`/* */`)
   - Basic bracket matching

4. **Installation**
   - `install.sh` script works correctly
   - Extension installs to `~/.config/zed/extensions/shellux`
   - Can be loaded as a dev extension or installed normally

### 🟡 What's Partially Working

1. **Syntax Highlighting**
   - Currently using Bash grammar instead of Shellux-specific grammar
   - Many Shellux-specific constructs are not properly highlighted:
     - `is` keyword for variable assignment
     - `fn` keyword (may appear as variable)
     - Pipeline operator `|>`
     - Shellux-specific built-in functions
     - Type annotations in function parameters
     - Match expressions
     - Try/catch blocks

### ❌ What's Not Working / Missing

1. **Tree-sitter Grammar**
   - **No Shellux-specific tree-sitter grammar exists yet**
   - This is the main blocker for full functionality
   - Zed requires tree-sitter; TextMate grammars are not supported

2. **Advanced Editor Features**
   - No code outline/structure view
   - No syntax-aware code folding
   - No semantic highlighting
   - No intelligent auto-indentation (beyond basic bracket matching)
   - No language-specific snippets
   - No syntax-aware text objects (for Vim mode)

3. **Language Server**
   - No LSP (Language Server Protocol) support
   - No autocomplete
   - No go-to-definition
   - No diagnostics/error checking
   - No hover documentation

## Technical Details

### File Structure

```
shellux-syntax-zed/
├── extension.toml              ✅ Main extension manifest (TOML format)
├── extension.json              ⚠️  Legacy format (kept for reference)
├── languages/
│   └── shellux/
│       ├── config.toml         ✅ Language configuration
│       ├── config.json         ⚠️  Legacy format (kept for reference)
│       ├── shellux.json        📝 TextMate grammar (reference only)
│       └── highlights.scm      🔜 Tree-sitter queries (placeholder)
├── README.md                   ✅ User documentation
├── QUICKSTART.md              ✅ Quick installation guide
├── CHANGELOG.md               ✅ Version history
├── TREE_SITTER_GUIDE.md       ✅ Guide for creating tree-sitter grammar
├── STATUS.md                  ✅ This file
├── LICENSE                     ✅ MIT license
└── install.sh                 ✅ Installation script
```

### Current Configuration

**extension.toml:**
```toml
[grammars.shellux]
repository = "https://github.com/tree-sitter/tree-sitter-bash"
rev = "f8fb3274f72a4a770546e09a4c69c8e6ff1f7ce5"
```

This temporarily uses the Bash grammar as a fallback.

**config.toml:**
```toml
name = "Shellux"
grammar = "shellux"
path_suffixes = ["sx", "shx"]
line_comments = ["# "]
block_comment = ["/*", "*/"]
```

## Why It Doesn't Fully Work

### The Tree-sitter Requirement

Zed is built on tree-sitter and **requires a tree-sitter grammar** for syntax highlighting. Unlike VS Code (which supports TextMate grammars), Zed does not have TextMate support.

**What we have:**
- ✅ TextMate grammar (`shellux.json`) - works in VS Code, Sublime, etc.
- ❌ Tree-sitter grammar - **does not exist yet**

**What we need:**
- A `tree-sitter-shellux` repository with:
  - `grammar.js` - Defines Shellux syntax rules
  - Generated C parser code
  - Test suite
  - Tree-sitter queries for highlighting

### Previous Errors

When trying to install as a dev extension from the source directory, you may have seen:

```
ERROR [extensions_ui] Failed to install dev extension: invalid extension.json for extension shellux-syntax-zed
```

**This error occurred because:**
1. Earlier versions used `extension.json` instead of `extension.toml`
2. Zed requires TOML format for extension manifests
3. The JSON format is a legacy format from older Zed versions

**This has been fixed:** The extension now includes `extension.toml` and installs correctly.

## What Needs to Be Done

### Priority 1: Create Tree-sitter Grammar (Essential)

**Estimated Effort:** 1-2 weeks for a basic grammar

1. Set up `tree-sitter-shellux` repository
2. Write `grammar.js` with Shellux syntax rules
3. Generate and test the parser
4. Create Tree-sitter queries:
   - `highlights.scm` - Syntax highlighting
   - `indents.scm` - Auto-indentation
   - `injections.scm` - Embedded languages
   - `textobjects.scm` - Text objects for Vim mode
5. Test with various Shellux code samples

**See:** `TREE_SITTER_GUIDE.md` for detailed instructions

### Priority 2: Update Extension to Use New Grammar

**Estimated Effort:** 1-2 hours

1. Update `extension.toml`:
   ```toml
   [grammars.shellux]
   repository = "https://github.com/yourusername/tree-sitter-shellux"
   rev = "main"  # or specific commit SHA
   ```

2. Create comprehensive `highlights.scm` with all Shellux tokens
3. Add `indents.scm` for better auto-indentation
4. Test in Zed with real Shellux code

### Priority 3: Language Server (Optional)

**Estimated Effort:** Several weeks

1. Implement Shellux LSP server
2. Add LSP support to extension
3. Provide diagnostics, completion, etc.

This is optional for basic syntax highlighting but would provide IDE-like features.

## How to Help

### If You're a Grammar Developer

1. **Create the tree-sitter grammar** - This is the #1 priority
   - Follow `TREE_SITTER_GUIDE.md`
   - Start with basic features (variables, functions)
   - Gradually add complexity
   - Test frequently

2. **Share progress early** - Don't wait for perfection
   - Even a basic grammar is better than Bash fallback
   - Community can help test and improve

### If You're a User

1. **Use the extension as-is** - Basic highlighting is better than none
2. **Report issues** - Let us know what doesn't work
3. **Share Shellux code samples** - Helps with testing
4. **Spread the word** - More users = more contributors

### If You're Interested in Contributing

Check out:
- `TREE_SITTER_GUIDE.md` - Complete guide to creating the grammar
- `README.md` - Extension documentation
- Official Zed docs: https://zed.dev/docs/extensions
- Tree-sitter docs: https://tree-sitter.github.io/tree-sitter/

## Testing the Current Extension

### Install and Test

```bash
cd shellux-syntax-zed
./install.sh
```

### Create a Test File

```bash
cat > test.shx << 'EOF'
#!/usr/bin/env shellux

# Variables
name is "World"
count := 42

# Function
fn greet(person: string) -> string {
    return "Hello, " + person
}

# Control flow
if count > 10 {
    print("Many items")
}

# Command substitution
files is $(ls -la)
EOF

# Open in Zed (if installed)
zed test.shx
```

### What You'll See

- File opens with language set to "Shellux"
- Basic syntax highlighting (Bash-like)
- Comments highlighted correctly
- Strings highlighted
- Some keywords highlighted (if, return)
- Some Shellux-specific syntax may not be highlighted correctly

## Resources

### Documentation in This Repository

- `README.md` - Complete user guide
- `TREE_SITTER_GUIDE.md` - Guide for creating tree-sitter grammar
- `QUICKSTART.md` - Quick installation instructions
- `CHANGELOG.md` - Version history

### External Resources

- [Zed Extensions Documentation](https://zed.dev/docs/extensions)
- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Creating Tree-sitter Parsers](https://tree-sitter.github.io/tree-sitter/creating-parsers)
- [Zed Discord](https://discord.gg/zed)
- [Tree-sitter Discussions](https://github.com/tree-sitter/tree-sitter/discussions)

## Timeline

### Completed (October 11, 2024)

- ✅ Created extension structure
- ✅ Fixed `extension.json` → `extension.toml`
- ✅ Added proper configuration files
- ✅ Created installation script
- ✅ Wrote comprehensive documentation
- ✅ Added Bash grammar as temporary fallback
- ✅ Extension installs and loads correctly

### Planned

- 🔜 **Week 1-2:** Create basic tree-sitter grammar
- 🔜 **Week 3:** Add comprehensive tree-sitter queries
- 🔜 **Week 4:** Test and refine grammar
- 🔜 **Future:** LSP support (if there's interest)

## Frequently Asked Questions

### Q: Why doesn't the extension work fully?

**A:** Zed requires a tree-sitter grammar for full syntax highlighting. The tree-sitter grammar for Shellux doesn't exist yet and needs to be created.

### Q: Can I use the TextMate grammar?

**A:** No, Zed doesn't support TextMate grammars. Only tree-sitter grammars work with Zed.

### Q: Is this extension useless then?

**A:** No! It provides basic file recognition and Bash-like highlighting, which is better than no highlighting. It's also ready to use the real Shellux grammar as soon as it's created.

### Q: How can I help?

**A:** The biggest help would be creating the tree-sitter grammar. See `TREE_SITTER_GUIDE.md` for a complete guide.

### Q: Will this work in VS Code?

**A:** No, this is specifically for Zed. For VS Code, use the TextMate grammar in `shellux-syntax/` directory (different extension).

### Q: When will full support be available?

**A:** That depends on when someone creates the tree-sitter grammar. With focused effort, a basic grammar could be ready in 1-2 weeks.

## Contact

For questions or contributions:

- Open an issue in the Shellux repository
- Join the Zed Discord: https://discord.gg/zed
- Check Tree-sitter discussions: https://github.com/tree-sitter/tree-sitter/discussions

---

**Status Summary:** Extension is functional for basic use but requires a tree-sitter grammar for full Shellux-specific syntax highlighting. All infrastructure is in place and ready for the grammar once it's created.