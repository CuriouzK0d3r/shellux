# Quick Start - Shellux for Zed

Get Shellux syntax highlighting in Zed in under 30 seconds! ⚡

## Installation

### One-Line Install

```bash
mkdir -p ~/.config/zed/extensions && cp -r shellux-syntax-zed ~/.config/zed/extensions/shellux
```

Then restart Zed.

### Using the Install Script (Recommended - Takes < 1 second!)

```bash
cd shellux-syntax-zed
chmod +x install.sh
./install.sh
```

**Installation is now optimized** - copies only ~15KB of essential files (down from 294MB!)

## Test It

1. **Create a test file:**
   ```bash
   echo '#!/usr/bin/env shellux
   name is "World"
   print("Hello, " + name + "!")' > test.sx
   ```

2. **Open in Zed:**
   ```bash
   zed test.sx
   ```

3. **See the magic!** ✨
   - Keywords highlighted
   - Strings in color
   - Comments styled
   - Proper syntax coloring

## What's Highlighted?

| Element | Example |
|---------|---------|
| Keywords | `fn`, `if`, `else`, `for`, `while`, `is` |
| Types | `int`, `string`, `bool`, `float` |
| Functions | `print()`, `show`, `read_file()` |
| Comments | `# comment` or `/* block */` |
| Strings | `"text"` or `"""multi-line"""` |
| Interpolation | `"Hello ${name}"` |
| Commands | `$(ls -la)` |
| Numbers | `42`, `3.14`, `0xFF` |

## Editor Features

✅ **Auto-closing** - Brackets, braces, quotes  
✅ **Comment toggle** - `Cmd+/` or `Ctrl+/`  
✅ **Bracket matching** - Paired highlights  
✅ **Smart indent** - Context-aware formatting  

## File Extensions

- `.sx` - Shellux scripts
- `.shx` - Alternative extension

## Troubleshooting

**Not working?**
1. Check: `ls ~/.config/zed/extensions/shellux`
2. Restart Zed completely
3. Try: Command Palette → "zed: reload extensions"

**Wrong colors?**
- Different Zed themes render colors differently
- Try: Command Palette → "theme selector: toggle"

## Next Steps

📖 Read [README.md](README.md) for full documentation  
🎨 Customize in `~/.config/zed/settings.json`  
🐛 Report issues on GitHub  

---

**Ready to code!** 🚀
