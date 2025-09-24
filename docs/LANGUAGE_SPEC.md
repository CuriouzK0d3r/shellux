# Shellux Language Specification

## Overview
Shellux is a modern scripting language designed to replace bash scripting with improved syntax, better error handling, and enhanced functionality while maintaining the power and flexibility needed for system administration and automation tasks.

## Design Goals

### Improvements over Bash
1. **Clear and consistent syntax** - No cryptic variable expansions like `${var%%pattern}`
2. **Proper error handling** - Built-in error types and structured exception handling
3. **Type safety** - Optional static typing with runtime type checking
4. **Modern string handling** - Unicode support and intuitive string operations
5. **Better data structures** - Native arrays, maps, and objects
6. **Structured output** - Built-in JSON/YAML support
7. **Improved process management** - Better pipeline handling and process control
8. **Cross-platform compatibility** - Works consistently across Unix-like systems

## Syntax Overview

### Variables and Types
```shellux
# Variable declaration (inferred typing)
name := "John"
age := 30
is_admin := true

# Explicit typing
count: int = 0
items: []string = ["apple", "banana", "cherry"]
config: map[string]any = {"debug": true, "port": 8080}

# Constants
const PI: float = 3.14159
const MAX_RETRIES: int = 3
```

### Functions
```shellux
# Function definition
fn greet(name: string) -> string {
    return "Hello, " + name + "!"
}

# Function with multiple return values
fn divide(a: float, b: float) -> (float, error) {
    if b == 0 {
        return 0.0, error("division by zero")
    }
    return a / b, nil
}

# Using functions
message := greet("Alice")
result, err := divide(10.0, 2.0)
if err != nil {
    print("Error:", err)
    exit(1)
}
```

### Control Flow
```shellux
# If statements
if age >= 18 {
    print("Adult")
} else if age >= 13 {
    print("Teenager")
} else {
    print("Child")
}

# For loops
for i in range(0, 10) {
    print(i)
}

for item in items {
    print("Item:", item)
}

for key, value in config {
    print(key + ":", value)
}

# While loops
count := 0
while count < 5 {
    print("Count:", count)
    count += 1
}
```

### Command Execution and Pipelines
```shellux
# Simple command execution
result := $(ls -la)
print(result.stdout)

# Pipeline with error handling
result := $(cat file.txt | grep "pattern" | sort)
if result.exit_code != 0 {
    print("Command failed:", result.stderr)
}

# Modern pipeline syntax
files := ls("/tmp")
    |> filter(fn(f) f.is_file())
    |> map(fn(f) f.name)
    |> sort()

# Process substitution
diff $(cat file1.txt) $(cat file2.txt)
```

### Error Handling
```shellux
# Try-catch blocks
try {
    content := read_file("config.json")
    config := parse_json(content)
} catch FileNotFound as e {
    print("Config file not found:", e.path)
    config := default_config()
} catch JsonParseError as e {
    print("Invalid JSON:", e.message)
    exit(1)
}

# Result type pattern
result := try_operation()
match result {
    Ok(value) => print("Success:", value),
    Err(error) => print("Failed:", error)
}
```

### String Operations
```shellux
# String interpolation
name := "World"
message := "Hello, ${name}!"

# Multi-line strings
sql := """
    SELECT * FROM users
    WHERE age > 18
    AND active = true
"""

# String methods
text := "  Hello World  "
cleaned := text.trim().lower()
words := text.split(" ")
```

### File and Path Operations
```shellux
# Path manipulation
home := env("HOME")
config_path := join_path(home, ".config", "myapp", "config.json")

# File operations
if exists(config_path) {
    content := read_file(config_path)
} else {
    write_file(config_path, default_config_json())
}

# Directory operations
for file in walk_dir("/tmp", recursive: true) {
    if file.extension == ".log" && file.age > duration("7d") {
        rm(file.path)
    }
}
```

## Built-in Functions

### Core Functions
- `print(args...)` - Print values to stdout
- `println(args...)` - Print values with newline
- `input(prompt: string) -> string` - Read input from stdin
- `exit(code: int)` - Exit program with code
- `env(name: string) -> string` - Get environment variable
- `set_env(name: string, value: string)` - Set environment variable

### String Functions
- `len(s: string) -> int` - String length
- `contains(s: string, substr: string) -> bool` - Check substring
- `starts_with(s: string, prefix: string) -> bool` - Check prefix
- `ends_with(s: string, suffix: string) -> bool` - Check suffix
- `split(s: string, delimiter: string) -> []string` - Split string
- `join(parts: []string, delimiter: string) -> string` - Join strings

### File System Functions
- `read_file(path: string) -> string` - Read file contents
- `write_file(path: string, content: string)` - Write file contents
- `exists(path: string) -> bool` - Check if path exists
- `is_file(path: string) -> bool` - Check if path is file
- `is_dir(path: string) -> bool` - Check if path is directory
- `mkdir(path: string, mode: int)` - Create directory
- `rm(path: string)` - Remove file or directory
- `mv(src: string, dst: string)` - Move/rename file
- `cp(src: string, dst: string)` - Copy file

### Process Functions
- `$(command: string) -> CommandResult` - Execute command
- `spawn(command: string, args: []string) -> Process` - Spawn process
- `kill(pid: int, signal: string)` - Send signal to process
- `wait(process: Process) -> int` - Wait for process completion

## File Extension
Shellux scripts use the `.sx` file extension.

## Shebang
```shellux
#!/usr/bin/env shellux
```

## Comments
```shellux
# Single line comment

/*
Multi-line
comment
*/
```

This specification provides a foundation for a modern, powerful scripting language that addresses the common pain points of bash scripting while maintaining the essential capabilities needed for system automation.