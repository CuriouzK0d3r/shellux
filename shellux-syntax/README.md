# Shellux Syntax Highlighting

A Visual Studio Code extension that provides syntax highlighting for Shellux scripting language (`.sx` and `.shx` files).

## Features

- **Comprehensive syntax highlighting** for Shellux language constructs:
  - Keywords: `fn`, `if`, `else`, `for`, `while`, `in`, `match`, `try`, `catch`, `return`, `let`, `const`, `is`
  - Built-in types: `int`, `float`, `string`, `bool`, `any`, `error`, `map`, `array`
  - Constants: `true`, `false`, `nil`, `null`
  - String interpolation with `${}` syntax
  - Command substitution with `$()` syntax
  - Built-in functions: `print`, `println`, `show`, `input`, `exit`, and more
  - Comments: single-line (`#`) and multi-line (`/* */`)
  - Operators: arithmetic, comparison, logical, bitwise, and pipeline operators
  - Numbers: integers, floats, hex, octal, and binary literals

- **Language features**:
  - Auto-closing brackets, braces, and parentheses
  - Auto-closing quotes for strings
  - Comment toggling with `Cmd+/` (macOS) or `Ctrl+/` (Windows/Linux)
  - Bracket matching and indentation rules
  - Code folding support

## Installation

### From Source (Manual Installation)

1. Copy the `shellux-syntax` folder to your VS Code extensions directory:
   - **macOS/Linux**: `~/.vscode/extensions/`
   - **Windows**: `%USERPROFILE%\.vscode\extensions\`

2. Reload VS Code or restart the editor

3. Open any `.sx` or `.shx` file to see syntax highlighting in action

### From VSIX (Recommended for distribution)

1. Package the extension:
   ```bash
   cd shellux-syntax
   npm install -g vsce
   vsce package
   ```

2. Install the generated `.vsix` file:
   - Open VS Code
   - Go to Extensions view (`Cmd+Shift+X` or `Ctrl+Shift+X`)
   - Click the `...` menu at the top
   - Select "Install from VSIX..."
   - Choose the generated `.vsix` file

## Usage

Simply open any file with `.sx` or `.shx` extension, and syntax highlighting will be applied automatically.

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
    print("Adult")
} else {
    print("Minor")
}

# Command execution
result is $(ls -la)
print(result.stdout)

# String interpolation
message is "Hello, ${name}! You are ${age} years old."
show message
```

## Supported File Extensions

- `.sx` - Shellux script files
- `.shx` - Shellux script files (alternative extension)

## Language Features

### Keywords
- **Control flow**: `if`, `else`, `for`, `while`, `in`, `match`, `try`, `catch`, `return`, `break`, `continue`
- **Declarations**: `fn`, `let`, `const`
- **Operators**: `is`, `and`, `or`, `not`, `as`

### Built-in Functions
- **I/O**: `print`, `println`, `show`, `input`
- **String operations**: `len`, `contains`, `starts_with`, `ends_with`, `split`, `join`, `trim`, `lower`, `upper`
- **File operations**: `read_file`, `write_file`, `exists`, `is_file`, `is_dir`, `mkdir`, `rm`, `mv`, `cp`
- **Process control**: `exit`, `env`, `set_env`, `spawn`, `kill`, `wait`
- **Utilities**: `parse_json`, `join_path`, `duration`, `filter`, `map`, `sort`

### Operators
- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `**`
- **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Logical**: `&&`, `||`, `!`
- **Bitwise**: `&`, `|`, `^`, `~`, `<<`, `>>`
- **Assignment**: `=`, `:=`, `+=`, `-=`, `*=`, `/=`
- **Pipeline**: `|>`
- **Arrow**: `->`, `=>`

## Contributing

Contributions are welcome! If you find any issues or want to improve the syntax highlighting, please:

1. Fork the repository
2. Make your changes
3. Submit a pull request

## License

MIT License - See LICENSE file for details

## About Shellux

Shellux is a modern scripting language designed to replace bash scripting with improved syntax, better error handling, and enhanced functionality. Learn more at the [Shellux repository](https://github.com/yourusername/shellux).

## Release Notes

### 0.1.0

- Initial release
- Comprehensive syntax highlighting for Shellux language
- Support for `.sx` and `.shx` file extensions
- Auto-closing brackets, braces, and quotes
- Comment toggling support
- Code folding and indentation rules
