# Shellux Syntax Highlighting for Zed Editor

A Zed editor extension that provides syntax highlighting for Shellux scripting language (`.sx` and `.shx` files).

## ⚠️ Important Note

**This extension is currently a work in progress.** Zed requires **Tree-sitter grammars** for language support and does not support TextMate grammars. This extension currently uses a Bash tree-sitter grammar as a temporary solution while a proper Shellux tree-sitter grammar is being developed.

### Current Status

- ✅ Basic file recognition (`.sx`, `.shx`)
- ✅ Bash-like syntax highlighting (temporary)
- ✅ Comment support
- ✅ Bracket matching
- ⏳ Full Shellux-specific highlighting (requires tree-sitter grammar)
- ⏳ Advanced language features (requires tree-sitter grammar)

### What's Needed

To fully support Shellux in Zed, we need to create a **Tree-sitter grammar** for Shellux. This involves:

1. Creating a `grammar.js` file defining Shellux syntax rules
2. Generating C code with the Tree-sitter CLI
3. Creating Tree-sitter queries (`highlights.scm`, `injections.scm`, etc.)
4. Testing and refining the grammar

See the [Tree-sitter documentation](https://tree-sitter.github.io/tree-sitter/creating-parsers) for details on creating parsers.

## Installation

### Method 1: Using the Install Script (Recommended)

```bash
cd shellux-syntax-zed
./install.sh
```

### Method 2: Manual Installation

1. **Copy the extension** to Zed's extension directory:

   ```bash
   # Create extensions directory if it doesn't exist
   mkdir -p ~/.config/zed/extensions
   
   # Copy the extension
   cp -r shellux-syntax-zed ~/.config/zed/extensions/shellux
   ```

2. **Restart Zed** or reload extensions:
   - Open Command Palette: `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
   - Type "zed: reload extensions" and press Enter

3. **Open any `.sx` or `.shx` file**

### Method 3: Dev Extension (For Development)

From within Zed:
1. Open Command Palette: `Cmd+Shift+P` / `Ctrl+Shift+P`
2. Type "zed: install dev extension"
3. Select the `shellux-syntax-zed` directory

## Features

### Current Features

- File recognition for `.sx` and `.shx` extensions
- Basic syntax highlighting (using Bash grammar temporarily)
- Comment toggling: `Cmd+/` (macOS) or `Ctrl+/` (Linux/Windows)
- Auto-closing brackets, braces, and quotes
- Line comments: `#`
- Block comments: `/* */`

### Planned Features (Requires Tree-sitter Grammar)

- Full Shellux-specific syntax highlighting:
  - Keywords: `fn`, `if`, `else`, `for`, `while`, `in`, `match`, `try`, `catch`, `return`, `let`, `const`, `is`
  - Types: `int`, `float`, `string`, `bool`, `any`, `error`, `map`, `array`
  - Built-in functions: `print`, `show`, `input`, `exit`, and more
  - String interpolation: `${expression}`
  - Command substitution: `$(command)`
  - Pipeline operator: `|>`
  - All Shellux operators and literals

- Advanced features:
  - Code outline/structure
  - Better auto-indentation
  - Syntax-aware folding
  - Semantic highlighting

## Supported File Extensions

- `.sx` - Shellux script files
- `.shx` - Shellux script files (alternative)

## Editor Features

### Auto-Closing Pairs
- Brackets: `{` `}`, `[` `]`, `(` `)`
- Quotes: `"`, `'`, `` ` ``

### Comments
- Single-line: `#`
- Multi-line: `/* */`
- Toggle comment: `Cmd+/` (macOS) or `Ctrl+/` (Linux/Windows)

## Extension Structure

```
shellux-syntax-zed/
├── extension.toml              # Extension metadata (TOML format required by Zed)
├── extension.json              # Legacy format (not used by current Zed)
├── languages/
│   └── shellux/
│       ├── config.toml         # Language configuration
│       ├── config.json         # Legacy format
│       ├── shellux.json        # TextMate grammar (reference only)
│       └── highlights.scm      # Tree-sitter queries (for future use)
├── README.md                   # This file
├── QUICKSTART.md              # Quick installation guide
├── CHANGELOG.md               # Version history
├── LICENSE                     # MIT license
└── install.sh                 # Installation script
```

## Troubleshooting

### Extension not loading?

1. Check that the extension is in the correct directory:
   ```bash
   ls ~/.config/zed/extensions/shellux
   ```

2. Verify `extension.toml` exists (not just `extension.json`)

3. Check Zed's log for errors:
   - Command Palette → "zed: open log"
   - Look for messages about the shellux extension

4. Restart Zed completely (not just reload extensions)

### Syntax highlighting looks basic?

This is expected. The extension currently uses Bash's tree-sitter grammar as a fallback. To get full Shellux highlighting, a dedicated tree-sitter grammar needs to be created.

### Colors look wrong?

The appearance depends on your Zed theme. Try switching themes:
- Command Palette → "theme selector: toggle"

## Zed Configuration

You can customize Shellux behavior in your Zed settings (`~/.config/zed/settings.json`):

```json
{
  "languages": {
    "Shellux": {
      "tab_size": 4,
      "hard_tabs": false,
      "soft_wrap": "none",
      "format_on_save": false
    }
  }
}
```

## Development

### Creating a Tree-sitter Grammar

To create a proper tree-sitter grammar for Shellux:

1. **Set up a new repository** for the grammar:
   ```bash
   mkdir tree-sitter-shellux
   cd tree-sitter-shellux
   npm init
   npm install --save-dev tree-sitter-cli
   ```

2. **Create `grammar.js`** defining Shellux syntax:
   ```javascript
   module.exports = grammar({
     name: 'shellux',
     rules: {
       source_file: $ => repeat($._statement),
       // Define your language rules here...
     }
   });
   ```

3. **Generate the parser**:
   ```bash
   npx tree-sitter generate
   ```

4. **Test the grammar**:
   ```bash
   npx tree-sitter test
   ```

5. **Update this extension** to use the new grammar in `extension.toml`:
   ```toml
   [grammars.shellux]
   repository = "https://github.com/yourusername/tree-sitter-shellux"
   rev = "main"
   ```

### Testing Changes

1. Edit the configuration files in `languages/shellux/`
2. Reload extensions in Zed:
   - Command Palette → "zed: reload extensions"
3. Test with sample `.sx` files

### Contributing

Contributions are welcome! Priority areas:

1. **Creating a tree-sitter grammar for Shellux** (most important!)
2. Writing comprehensive Tree-sitter queries
3. Improving language configuration
4. Adding code snippets
5. Documentation improvements

## Resources

- [Zed Extension Documentation](https://zed.dev/docs/extensions)
- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Creating Tree-sitter Parsers](https://tree-sitter.github.io/tree-sitter/creating-parsers)
- [Zed Extensions Repository](https://github.com/zed-industries/extensions)

## About Zed

Zed is a high-performance, multiplayer code editor built by the creators of Atom and Tree-sitter. Learn more at [zed.dev](https://zed.dev).

## About Shellux

Shellux is a modern scripting language designed to replace bash scripting with improved syntax, better error handling, and enhanced functionality. Learn more at the [Shellux repository](https://github.com/yourusername/shellux).

## License

MIT License - See LICENSE file for details

## Version History

### 0.1.0 (2024-10-11)

- Initial release with basic file recognition
- Using Bash tree-sitter grammar as temporary solution
- Auto-closing pairs and bracket matching
- Comment toggling support
- Note: Full Shellux highlighting awaits tree-sitter grammar development

---

**Note**: For a fully-featured Shellux extension in Zed, we need to create a tree-sitter grammar. Until then, this extension provides basic support using Bash highlighting as a fallback.

If you're interested in helping create the tree-sitter grammar, please check the Development section above!