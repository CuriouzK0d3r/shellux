# Quick Start Guide

Get Shellux syntax highlighting in VS Code in 3 easy steps!

## Installation

### Option 1: Quick Install (Recommended)

```bash
cd shellux-syntax
./install.sh
```

Then reload VS Code (Cmd+R on macOS, Ctrl+R on Windows/Linux).

### Option 2: Manual Install

1. Copy the `shellux-syntax` folder to your VS Code extensions directory:
   ```bash
   # macOS/Linux
   cp -r shellux-syntax ~/.vscode/extensions/
   
   # Windows (PowerShell)
   Copy-Item -Recurse shellux-syntax "$env:USERPROFILE\.vscode\extensions\"
   ```

2. Reload VS Code

### Option 3: Development Mode

1. Open the extension folder in VS Code:
   ```bash
   code shellux-syntax/
   ```

2. Press `F5` to open Extension Development Host

3. Open any `.sx` file to test

## Testing

1. Open the test file:
   ```bash
   code shellux-syntax/test-syntax.sx
   ```

2. Verify that you see proper syntax highlighting:
   - Keywords in blue/purple
   - Strings in orange/red
   - Comments in green/gray
   - Functions in yellow
   - Numbers in light green

## What's Highlighted?

✅ **Keywords**: `fn`, `if`, `else`, `for`, `while`, `is`, `return`, etc.  
✅ **Types**: `int`, `float`, `string`, `bool`, `any`, `error`  
✅ **Built-in Functions**: `print`, `show`, `input`, `read_file`, etc.  
✅ **Comments**: `#` single-line and `/* */` multi-line  
✅ **Strings**: Including interpolation `${expr}` and multi-line `"""`  
✅ **Numbers**: Integers, floats, hex, octal, binary  
✅ **Operators**: All arithmetic, comparison, logical, bitwise operators  
✅ **Commands**: Command substitution with `$(command)`  

## Next Steps

- 📖 Read the [README.md](README.md) for full documentation
- 🛠️ Check [DEVELOPMENT.md](DEVELOPMENT.md) for development guide
- 📦 See [CHANGELOG.md](CHANGELOG.md) for version history
- 🎨 Add custom icons (see [ICONS.md](ICONS.md))

## Troubleshooting

### Extension not working?

1. Check file extension is `.sx` or `.shx`
2. Reload VS Code window
3. Check language mode in bottom-right corner
4. Try opening the test file: `test-syntax.sx`

### Need help?

Open an issue on GitHub or check the documentation files.

---

**Happy coding with Shellux!** 🚀
