# Running External Programs in Shellux

This guide explains how to execute external programs and system commands in Shellux.

## Methods for Running External Commands

### 1. Command Substitution `$(...)`

Use `$(command)` syntax to execute commands and capture their output:

```shellux
$(ls -la)           # Execute ls -la and return output as string
$(pwd)              # Get current directory
$(echo "hello")     # Execute echo command
$(date)             # Get current date/time
$(cat file.txt)     # Read file contents
```

**Example:**
```shellux
let files = $(ls)
echo "Files in directory: " + files
```

### 2. Direct Command Execution (Zero Arguments)

For commands without arguments, you can call them directly as identifiers:

```shellux
ls                  # List directory contents
pwd                 # Print working directory (built-in)
date                # Show current date
whoami              # Show current user
```

### 3. Function Call Syntax

Use parentheses to call external commands with arguments:

```shellux
ls("-la")           # List with long format
echo("hello world") # Echo a message
cat("file.txt")     # Read a file
grep("pattern", "file.txt")  # Search in file
```

### 4. The `run()` Built-in Function

Use the `run()` function for complex command execution with shell-style arguments:

```shellux
run("ls", "-la")                    # Basic usage
run("find", ". -name '*.rs'")      # Find files with complex arguments
run("curl", "-s https://api.example.com")  # HTTP requests
run("git", "status")               # Git commands
```

**Advantages of `run()`:**
- Handles complex argument strings with spaces
- Splits arguments automatically by whitespace
- Better error handling and output control

## Built-in Shell Commands

Shellux provides several built-in commands that work like traditional shell built-ins:

### File Operations
```shellux
pwd                 # Print working directory
cd("/path/to/dir")  # Change directory
cd()                # Go to home directory
ls                  # List current directory
```

### I/O Operations
```shellux
echo("message")     # Print message to stdout
print("value")      # Print value (built-in function)
input("prompt: ")   # Read user input
```

### File System
```shellux
read_file("file.txt")        # Read file contents
write_file("file.txt", data) # Write data to file
```

## Examples

### Basic File Operations
```shellux
# List files in current directory
ls

# Change to a directory and list files
cd("/tmp")
ls("-la")

# Get current directory
let current_dir = $(pwd)
echo("Currently in: " + current_dir)
```

### Working with Command Output
```shellux
# Capture command output
let date_str = $(date)
echo("Today is: " + date_str)

# Chain commands
let file_count = $(ls | wc -l)
echo("Number of files: " + file_count)
```

### Complex Command Execution
```shellux
# Using run() for complex commands
run("find", ". -name '*.txt' -type f")
run("grep", "-r 'TODO' src/")
run("tar", "-czf backup.tar.gz .")

# Process management
run("ps", "aux")
run("kill", "-9 1234")
```

### Error Handling
```shellux
# Commands that fail will show error messages
try {
    run("nonexistent_command")
} catch (error) {
    echo("Command failed: " + error)
}
```

## Command Resolution Order

When you type a command, Shellux resolves it in this order:

1. **Built-in functions** (`echo`, `pwd`, `cd`, etc.)
2. **Environment variables** (user-defined variables)
3. **External commands** (programs in your PATH)

## Tips and Best Practices

### 1. Use Appropriate Method
- Use `$(...)` when you need the command output as a string
- Use direct calls (`ls`) for simple commands without arguments
- Use function syntax (`ls("-la")`) for commands with arguments
- Use `run()` for complex commands with multiple arguments

### 2. Error Handling
```shellux
# Check if command exists before running
if $(which git) != "" {
    run("git", "status")
} else {
    echo("Git is not installed")
}
```

### 3. Working Directory
```shellux
# Save current directory before changing
let original_dir = $(pwd)
cd("/some/path")
run("make", "build")
cd(original_dir)
```

### 4. Environment Variables
```shellux
# Access environment variables
let home = $(echo $HOME)
let user = $(whoami)
```

## Common Use Cases

### System Administration
```shellux
# Check system info
run("uname", "-a")
run("df", "-h")
run("free", "-m")

# Process management
run("ps", "aux | grep nginx")
run("systemctl", "status nginx")
```

### Development Workflow
```shellux
# Git operations
run("git", "add .")
run("git", "commit -m 'Update'")
run("git", "push origin main")

# Build and test
run("cargo", "build")
run("cargo", "test")
run("npm", "install")
run("npm", "run build")
```

### File Processing
```shellux
# Text processing
run("grep", "-n 'pattern' *.txt")
run("sed", "-i 's/old/new/g' file.txt")
run("awk", "'{print $1}' data.csv")

# File operations
run("cp", "-r src/ backup/")
run("find", ". -name '*.log' -delete")
```

## Limitations

1. **Interactive commands**: Commands that require interactive input may not work properly
2. **Shell features**: Advanced shell features like pipes (`|`), redirections (`>`), and job control are not fully supported yet
3. **Argument parsing**: Complex quoting and escaping in arguments may not work as expected

## Future Enhancements

Planned features include:
- Pipeline support (`command1 | command2`)
- Output redirection (`command > file`)
- Background job execution (`command &`)
- Shell-style argument parsing with proper quoting
- Interactive command support