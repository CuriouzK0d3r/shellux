# Testing the Shellux Zed Extension

## Quick Test

After installing, verify the extension works:

### 1. Check Installation

```bash
ls -la ~/.config/zed/extensions/shellux/
```

You should see:
- `extension.toml` (not just extension.json)
- `languages/` directory
- Other documentation files

### 2. Create Test File

```bash
cat > /tmp/test.shx << 'SHELLUX'
#!/usr/bin/env shellux

# Test Shellux syntax highlighting

# Variables
name is "Alice"
age := 30
const MAX_VALUE = 100

# Function
fn calculate(x: int, y: int) -> int {
    result is x + y
    return result
}

# Control flow
if age >= 18 {
    print("Adult")
} else {
    print("Minor")
}

# Loops
for i in range(0, 10) {
    print(i)
}

# Command execution
files is $(ls -la)
show files

# String interpolation
message is "Hello, ${name}!"
print(message)

# Pipeline
result is "HELLO" |> lower() |> trim()

# Try-catch
try {
    risky_operation()
} catch error {
    print("Error: ${error}")
}
SHELLUX
```

### 3. Open in Zed

```bash
# If you have Zed CLI installed
zed /tmp/test.shx

# Or open Zed and File → Open → /tmp/test.shx
```

### 4. What to Expect

✅ **Working:**
- File is recognized as "Shellux" (check bottom-right of Zed)
- Comments are colored/italicized
- Strings are colored
- Some keywords highlighted (if, else, return)
- Bracket matching works
- `Cmd+/` or `Ctrl+/` toggles comments

⚠️ **Partially Working:**
- Syntax highlighting is basic (Bash-like)
- Not all Shellux-specific keywords highlighted correctly
- `is` keyword might not be special
- `fn` might not be highlighted
- Pipeline operator `|>` might not be special

❌ **Not Working:**
- Full Shellux-specific highlighting
- Code outline/structure
- Advanced editor features

This is expected! See STATUS.md for details.

### 5. Check Zed Logs (if issues)

```bash
# Open Zed, then:
# Cmd+Shift+P → "zed: open log"
# Or check directly:
tail -f ~/Library/Logs/Zed/Zed.log | grep -i shellux
```

## Uninstalling

```bash
rm -rf ~/.config/zed/extensions/shellux
# Then restart Zed
```

## Reinstalling

```bash
cd shellux-syntax-zed
./install.sh
# Restart Zed or reload extensions
```
