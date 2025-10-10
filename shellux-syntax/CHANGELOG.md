# Changelog

All notable changes to the "shellux-syntax" extension will be documented in this file.

## [0.1.0] - 2025-10-10

### Added
- Initial release of Shellux syntax highlighting extension
- Comprehensive syntax highlighting for Shellux language constructs
- Support for `.sx` and `.shx` file extensions
- Keyword highlighting: `fn`, `if`, `else`, `for`, `while`, `in`, `match`, `try`, `catch`, `return`, `let`, `const`, `is`
- Type highlighting: `int`, `float`, `string`, `bool`, `any`, `error`, `map`, `array`
- Built-in function highlighting: `print`, `show`, `input`, `exit`, file operations, and more
- String interpolation support with `${}` syntax
- Command substitution support with `$()` syntax
- Comment support: single-line (`#`) and multi-line (`/* */`)
- Operator highlighting: arithmetic, comparison, logical, bitwise, pipeline
- Number literals: integers, floats, hex, octal, binary
- Auto-closing pairs for brackets, braces, parentheses, and quotes
- Bracket matching and proper indentation rules
- Code folding support with region markers
- Language configuration for better editing experience

### Features
- Shebang line support (`#!/usr/bin/env shellux`)
- Multi-line string support with triple quotes (`"""`)
- Escape character highlighting in strings
- Function declaration and call highlighting
- Return type annotation support
- Variable and constant highlighting
