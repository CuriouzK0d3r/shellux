# Changelog

All notable changes to the Shellux extension for Zed will be documented in this file.

## [0.1.0] - 2025-10-10

### Added
- Initial release of Shellux syntax highlighting extension for Zed
- Complete TextMate grammar support
- Tree-sitter highlighting queries
- Support for `.sx` and `.shx` file extensions
- Comprehensive keyword highlighting:
  - Control flow: `fn`, `if`, `else`, `for`, `while`, `in`, `match`, `try`, `catch`, `return`
  - Declarations: `let`, `const`
  - Operators: `is`, `as`, `and`, `or`, `not`
- Type highlighting: `int`, `float`, `string`, `bool`, `any`, `error`, `map`, `array`
- Built-in function highlighting: 30+ functions including:
  - I/O: `print`, `println`, `show`, `input`, `exit`
  - Strings: `len`, `contains`, `split`, `join`, `trim`, `lower`, `upper`
  - Files: `read_file`, `write_file`, `exists`, `is_file`, `is_dir`, `mkdir`, `rm`, `mv`, `cp`
  - Process: `env`, `set_env`, `spawn`, `kill`, `wait`
  - Utilities: `parse_json`, `join_path`, `duration`, `filter`, `map`, `sort`, `range`
- String features:
  - String interpolation with `${}` syntax
  - Multi-line strings with triple quotes
  - Escape sequence highlighting
- Command substitution with `$()` syntax
- Comment support:
  - Single-line comments with `#`
  - Multi-line comments with `/* */`
  - Shebang line support
- Operator highlighting:
  - Arithmetic: `+`, `-`, `*`, `/`, `%`, `**`
  - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
  - Logical: `&&`, `||`, `!`
  - Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`
  - Assignment: `=`, `:=`, `+=`, `-=`, `*=`, `/=`
  - Pipeline: `|>`
  - Arrows: `->`, `=>`
- Number literal support:
  - Integer literals
  - Float literals with scientific notation
  - Hexadecimal: `0xFF`
  - Octal: `0o77`
  - Binary: `0b1010`
- Editor features:
  - Auto-closing brackets, braces, and parentheses
  - Auto-closing quotes
  - Bracket matching
  - Comment toggling
  - Smart indentation
  - Code folding (implicit through brackets)
- Language configuration:
  - Line and block comment definitions
  - Bracket pairs with newline behavior
  - Auto-close behavior configuration
  - Word character definitions

### Features
- Function declaration highlighting with parameter and return type support
- Variable and constant name highlighting
- Type annotation support
- Constants detection (uppercase identifiers)
- Error highlighting for invalid syntax

### Documentation
- Comprehensive README with installation instructions
- Quick start guide
- Troubleshooting section
- Zed-specific configuration examples
- Installation script for easy setup

## Future Enhancements

Planned for future releases:
- [ ] Full Tree-sitter parser implementation
- [ ] Semantic highlighting
- [ ] Code folding queries
- [ ] Indentation queries
- [ ] Injection queries for embedded languages
- [ ] Outline/symbols support
- [ ] Go-to-definition support (requires Tree-sitter parser)
- [ ] Auto-completion suggestions
- [ ] Linting integration
- [ ] Format on save support
