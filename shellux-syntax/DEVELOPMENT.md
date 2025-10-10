# Development and Testing Guide

This guide covers how to develop, test, and package the Shellux syntax highlighting extension.

## Quick Start

### Testing the Extension Locally

1. **Method 1: Copy to Extensions Directory**
   ```bash
   # Run the install script
   ./install.sh
   
   # Then reload VS Code
   # Cmd+Shift+P (macOS) or Ctrl+Shift+P (Windows/Linux)
   # Type "Reload Window" and press Enter
   ```

2. **Method 2: Open in VS Code Extension Development Host**
   ```bash
   # Open the extension folder in VS Code
   code shellux-syntax/
   
   # Press F5 to launch Extension Development Host
   # This opens a new VS Code window with the extension loaded
   # Open any .sx file to test syntax highlighting
   ```

### Making Changes

1. Edit the grammar file: `syntaxes/shellux.tmLanguage.json`
2. Edit language configuration: `language-configuration.json`
3. Update metadata: `package.json`
4. Reload the Extension Development Host window to see changes

## Packaging the Extension

### Prerequisites

Install `vsce` (Visual Studio Code Extensions tool):

```bash
npm install -g @vscode/vsce
```

### Create a VSIX Package

```bash
cd shellux-syntax
vsce package
```

This creates a `.vsix` file that can be shared and installed by others.

### Install VSIX Package

```bash
code --install-extension shellux-syntax-0.1.0.vsix
```

Or through VS Code UI:
1. Open Extensions view (`Cmd+Shift+X` or `Ctrl+Shift+X`)
2. Click `...` menu → "Install from VSIX..."
3. Select the `.vsix` file

## Publishing to VS Code Marketplace

### Prerequisites

1. Create a [Visual Studio Marketplace publisher account](https://marketplace.visualstudio.com/manage)
2. Get a Personal Access Token (PAT) from Azure DevOps
3. Login with vsce:
   ```bash
   vsce login <publisher-name>
   ```

### Publish

```bash
vsce publish
```

Or publish a specific version:

```bash
vsce publish 0.1.1
vsce publish minor
vsce publish major
```

## Testing Checklist

Test the following features with your `.sx` files:

- [ ] Keywords are highlighted correctly (`fn`, `if`, `else`, `for`, etc.)
- [ ] Built-in functions are highlighted (`print`, `show`, `input`, etc.)
- [ ] Types are highlighted (`int`, `string`, `bool`, etc.)
- [ ] Comments work (both `#` and `/* */`)
- [ ] String interpolation `${}` is highlighted
- [ ] Command substitution `$()` is highlighted
- [ ] Numbers are highlighted (integers, floats, hex, binary)
- [ ] Operators are highlighted correctly
- [ ] Auto-closing works for brackets, braces, parentheses, quotes
- [ ] Comment toggling works (`Cmd+/` or `Ctrl+/`)
- [ ] Bracket matching works
- [ ] Indentation works correctly
- [ ] Code folding works

## File Structure

```
shellux-syntax/
├── package.json                  # Extension manifest
├── language-configuration.json   # Language configuration
├── syntaxes/
│   └── shellux.tmLanguage.json  # TextMate grammar
├── README.md                     # User documentation
├── CHANGELOG.md                  # Version history
├── LICENSE                       # License file
├── ICONS.md                      # Icon guidelines
├── DEVELOPMENT.md               # This file
├── .vscodeignore                # Files to exclude from package
└── install.sh                   # Installation script
```

## TextMate Grammar Resources

- [TextMate Language Grammars](https://macromates.com/manual/en/language_grammars)
- [VS Code Syntax Highlighting Guide](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide)
- [Scope Naming](https://www.sublimetext.com/docs/scope_naming.html)
- [vsce Publishing Tool](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)

## Troubleshooting

### Extension not loading

1. Check the Output panel (View → Output) and select "Extension Host"
2. Look for error messages related to `shellux-syntax`
3. Verify all JSON files are valid (use a JSON validator)

### Syntax highlighting not working

1. Verify the file extension is `.sx` or `.shx`
2. Check if the language is set correctly (bottom-right of VS Code)
3. Reload the window (`Cmd+R` or `Ctrl+R`)
4. Check the TextMate scope inspector (`Cmd+Shift+P` → "Developer: Inspect Editor Tokens and Scopes")

### Changes not appearing

1. Reload the Extension Development Host window
2. Or restart VS Code completely
3. Clear VS Code's cache if needed

## Contributing

When contributing to the syntax highlighting:

1. Test your changes thoroughly
2. Update the CHANGELOG.md
3. Update version number in package.json
4. Provide example code that demonstrates the new/fixed highlighting
5. Submit a pull request with clear description of changes

## Version Management

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

Update version in `package.json` and document changes in `CHANGELOG.md`.
