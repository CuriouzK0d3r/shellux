# Shellux Syntax Highlighting for Zed Editor

A Zed editor extension that provides comprehensive syntax highlighting for Shellux scripting language (`.sx` and `.shx` files).

## Features

- **Full syntax highlighting** for Shellux language constructs
- **Smart auto-closing** brackets, braces, and quotes
- **Comment toggling** with keyboard shortcuts
- **Bracket matching** and proper indentation
- **TextMate grammar** support for rich highlighting
- **Tree-sitter queries** for enhanced code understanding

### Highlighted Elements

- Keywords: `fn`, `if`, `else`, `for`, `while`, `in`, `match`, `try`, `catch`, `return`, `let`, `const`, `is`
- Types: `int`, `float`, `string`, `bool`, `any`, `error`, `map`, `array`
- Constants: `true`, `false`, `nil`, `null`
- Built-in functions: `print`, `show`, `input`, `exit`, file operations, and 30+ more
- String interpolation: `${expression}`
- Command substitution: `$(command)`
- Comments: Single-line (`#`) and multi-line (`/* */`)
- All operators: arithmetic, comparison, logical, bitwise, pipeline
- Numbers: integers, floats, hex, octal, binary literals

## Installation

### Method 1: Install from Extension Directory (Recommended)

1. **Clone or copy the extension** to Zed's extension directory:

   ```bash
   # Create extensions directory if it doesn't exist
   mkdir -p ~/.config/zed/extensions
   
   # Copy the extension
   cp -r shellux-syntax-zed ~/.config/zed/extensions/shellux
   ```

2. **Restart Zed** or reload the extensions:
   - Open Command Palette: `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
   - Type "zed: reload extensions" and press Enter

3. **Open any `.sx` or `.shx` file** to see syntax highlighting in action!

### Method 2: Using the Install Script

```bash
cd shellux-syntax-zed
./install.sh
```

### Method 3: Manual Symlink

```bash
# Create a symbolic link
ln -s "$(pwd)/shellux-syntax-zed" ~/.config/zed/extensions/shellux

# Restart Zed
```

## Usage

Simply open any file with `.sx` or `.shx` extension in Zed, and syntax highlighting will be applied automatically.

### Example Shellux Code

```shellux
#!/usr/bin/env shellux

# Variables and types
name is "Alice"
age is 30
is_admin is true

# Functions
fn greet(person: string) -> string {
    return "Hello, " + person + "!"
}

# Control flow
if age >= 18 {
    print("You are an adult!")
}

# Command execution
files is $(ls -la)
show files.stdout

# String interpolation
message is "Hello, ${name}! Age: ${age}"
print(message)
```

## Supported File Extensions

- `.sx` - Shellux script files
- `.shx` - Shellux script files (alternative)

## Editor Features

### Auto-Closing Pairs
- Brackets: `{` `}`, `[` `]`, `(` `)`
- Quotes: `"`, `'`
- String interpolation: `${` `}`

### Comments
- Single-line: `#`
- Multi-line: `/* */`
- Toggle comment: `Cmd+/` (macOS) or `Ctrl+/` (Linux/Windows)

### Bracket Matching
- Automatic matching and highlighting of paired brackets
- Smart newline insertion within braces

### Indentation
- Auto-indentation based on context
- Smart indent/dedent with bracket pairs

## Extension Structure

```
shellux-syntax-zed/
├── extension.json              # Extension metadata
├── languages/
│   └── shellux/
│       ├── config.json         # Language configuration
│       ├── shellux.json        # TextMate grammar
│       └── highlights.scm      # Tree-sitter queries
├── README.md                   # This file
├── QUICKSTART.md              # Quick installation guide
├── LICENSE                     # MIT license
└── install.sh                 # Installation script
```

## Troubleshooting

### Extension not loading?

1. Check that the extension is in the correct directory:
   ```bash
   ls ~/.config/zed/extensions/shellux
   ```

2. Verify the extension structure is correct

3. Restart Zed completely

4. Check Zed's log for errors:
   - Open Command Palette
   - Type "zed: open log"

### Syntax highlighting not working?

1. Verify the file extension is `.sx` or `.shx`
2. Check the language mode in the bottom-right of Zed
3. Try manually setting the language:
   - Command Palette → "editor: select language" → "Shellux"

### Colors look wrong?

The appearance depends on your Zed theme. Different themes will render the syntax highlighting colors differently. Try switching themes to find one that works well with Shellux:
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

### Testing Changes

1. Edit the grammar files in `languages/shellux/`
2. Reload extensions in Zed:
   - Command Palette → "zed: reload extensions"
3. Test with sample `.sx` files

### Grammar Files

- **`shellux.json`**: TextMate grammar with pattern matching rules
- **`highlights.scm`**: Tree-sitter queries for semantic highlighting
- **`config.json`**: Language configuration (brackets, comments, etc.)

### Contributing

Contributions are welcome! If you find issues or want to improve the highlighting:

1. Fork the repository
2. Make your changes
3. Test thoroughly with various Shellux files
4. Submit a pull request

## About Zed

Zed is a high-performance, multiplayer code editor built by the creators of Atom and Tree-sitter. Learn more at [zed.dev](https://zed.dev).

## About Shellux

Shellux is a modern scripting language designed to replace bash scripting with improved syntax, better error handling, and enhanced functionality. Learn more at the [Shellux repository](https://github.com/yourusername/shellux).

## License

MIT License - See LICENSE file for details

## Version History

### 0.1.0 (2025-10-10)

- Initial release
- Complete syntax highlighting support
- TextMate grammar
- Tree-sitter query support
- Auto-closing pairs
- Comment toggling
- Bracket matching

---

**Happy coding with Shellux in Zed!** ⚡
