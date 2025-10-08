# Shellux

A modern scripting language designed to replace bash scripting with improved syntax, better error handling, and enhanced functionality.

## Features

- **Clear and consistent syntax** - No cryptic variable expansions
- **Proper error handling** - Built-in error types and structured exception handling
- **Type safety** - Optional static typing with runtime type checking
- **Modern string handling** - Unicode support and intuitive string operations
- **Better data structures** - Native arrays, maps, and objects
- **Structured output** - Built-in JSON/YAML support
- **Improved process management** - Better pipeline handling and process control
- **Cross-platform compatibility** - Works consistently across Unix-like systems

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/shellux.git
cd shellux

# Build and install
cargo build --release
cargo install --path .
```

### Hello World

Create a file `hello.sx`:

```shellux
#!/usr/bin/env shellux

name := "World"
print("Hello, ${name}!")
```

Run it:

```bash
chmod +x hello.sx
./hello.sx
```

Or run directly:

```bash
shellux hello.sx
```

## Language Overview

### Variables and Types

```shellux
# Variable declaration (inferred typing)
name is "John"
age is 30
is_admin is true

# Explicit typing
count: int = 0
items: []string = ["apple", "banana", "cherry"]
config: map[string]any = {"debug": true, "port": 8080}
```

### Functions

```shellux
fn greet(name: string) -> string {
    return "Hello, " + name + "!"
}

message is greet("Alice")
print(message)
```

### Output and Display

```shellux
# Using print with parentheses
x is 42
print(x)

# Using show without parentheses (simpler syntax)
show x

# Both are equivalent
print("Hello, World!")
show "Hello, World!"

# Multiple arguments work with both
print("Value:", x)
show "Value:" x
```

### Command Execution

```shellux
# Execute commands with command substitution
result is $(ls -la)
print(result)

# Direct command execution
pwd
ls
date
```

### Control Flow

```shellux
# If statements
if score >= 90 {
    print("Grade: A")
} else {
    if score >= 80 {
        print("Grade: B")
    } else {
        print("Grade: C")
    }
}

# Functions with conditional logic
fn max(a: int, b: int) -> int {
    if a > b {
        return a
    } else {
        return b
    }
}
```

## Documentation

- [Language Specification](docs/LANGUAGE_SPEC.md)
- [Built-in Functions Reference](docs/BUILTINS.md)
- [Examples](examples/)

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running

```bash
cargo run -- script.sx
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.# shellux
