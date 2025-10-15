# Creating a Tree-sitter Grammar for Shellux

This guide walks you through creating a tree-sitter grammar for Shellux, which is required for full syntax highlighting support in Zed editor.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Understanding Tree-sitter](#understanding-tree-sitter)
- [Shellux Grammar Structure](#shellux-grammar-structure)
- [Step-by-Step Development](#step-by-step-development)
- [Testing Your Grammar](#testing-your-grammar)
- [Integration with Zed](#integration-with-zed)
- [Resources](#resources)

## Prerequisites

Before you start, make sure you have:

1. **Node.js** (v12 or higher) - [Install Node.js](https://nodejs.org/)
2. **Git** - For version control
3. **A C compiler** - GCC or Clang (usually pre-installed on macOS/Linux)
4. **Rust** (optional) - For testing with Zed locally

Check your installations:

```bash
node --version
npm --version
git --version
gcc --version  # or clang --version
```

## Quick Start

### 1. Create the Grammar Repository

```bash
# Create a new directory for the grammar
mkdir tree-sitter-shellux
cd tree-sitter-shellux

# Initialize npm package
npm init -y

# Install tree-sitter-cli as a dev dependency
npm install --save-dev tree-sitter-cli

# Create basic structure
mkdir -p src test/corpus
```

### 2. Create Basic `grammar.js`

Create a file called `grammar.js` in the root directory:

```javascript
module.exports = grammar({
  name: 'shellux',

  extras: $ => [
    /\s/,           // Whitespace
    $.comment,      // Comments
  ],

  rules: {
    source_file: $ => repeat($._statement),

    _statement: $ => choice(
      $.variable_declaration,
      $.function_declaration,
      $.if_statement,
      $.for_statement,
      $.expression_statement,
      $.return_statement,
      $.comment,
    ),

    comment: $ => token(choice(
      seq('#', /.*/),
      seq('/*', /[^*]*\*+([^/*][^*]*\*+)*/, '/')
    )),

    variable_declaration: $ => seq(
      choice('let', 'const'),
      field('name', $.identifier),
      choice('=', 'is', ':='),
      field('value', $.expression),
    ),

    function_declaration: $ => seq(
      'fn',
      field('name', $.identifier),
      field('parameters', $.parameter_list),
      optional(seq('->', field('return_type', $.type))),
      field('body', $.block),
    ),

    parameter_list: $ => seq(
      '(',
      optional(seq(
        $.parameter,
        repeat(seq(',', $.parameter)),
        optional(','),
      )),
      ')',
    ),

    parameter: $ => seq(
      field('name', $.identifier),
      optional(seq(':', field('type', $.type))),
    ),

    type: $ => choice(
      'int',
      'float',
      'string',
      'bool',
      'any',
      'error',
      'map',
      'array',
    ),

    block: $ => seq(
      '{',
      repeat($._statement),
      '}',
    ),

    if_statement: $ => seq(
      'if',
      field('condition', $.expression),
      field('consequence', $.block),
      optional(seq(
        'else',
        choice(
          field('alternative', $.block),
          field('alternative', $.if_statement),
        ),
      )),
    ),

    for_statement: $ => seq(
      'for',
      field('variable', $.identifier),
      'in',
      field('iterable', $.expression),
      field('body', $.block),
    ),

    return_statement: $ => seq(
      'return',
      optional($.expression),
    ),

    expression_statement: $ => seq(
      $.expression,
      optional(';'),
    ),

    expression: $ => choice(
      $.identifier,
      $.string,
      $.number,
      $.boolean,
      $.array,
      $.map,
      $.binary_expression,
      $.unary_expression,
      $.call_expression,
      $.member_expression,
      $.command_substitution,
      $.parenthesized_expression,
    ),

    binary_expression: $ => choice(
      ...[
        ['||', 1],
        ['&&', 2],
        ['==', 3],
        ['!=', 3],
        ['<', 4],
        ['<=', 4],
        ['>', 4],
        ['>=', 4],
        ['+', 5],
        ['-', 5],
        ['*', 6],
        ['/', 6],
        ['%', 6],
        ['**', 7],
        ['|>', 8],
      ].map(([operator, precedence]) =>
        prec.left(precedence, seq(
          field('left', $.expression),
          field('operator', operator),
          field('right', $.expression),
        ))
      )
    ),

    unary_expression: $ => choice(
      prec(10, seq('!', $.expression)),
      prec(10, seq('-', $.expression)),
      prec(10, seq('not', $.expression)),
    ),

    call_expression: $ => seq(
      field('function', $.identifier),
      field('arguments', $.argument_list),
    ),

    argument_list: $ => seq(
      '(',
      optional(seq(
        $.expression,
        repeat(seq(',', $.expression)),
        optional(','),
      )),
      ')',
    ),

    member_expression: $ => seq(
      field('object', $.expression),
      '.',
      field('property', $.identifier),
    ),

    command_substitution: $ => seq(
      '$(',
      /[^)]*/,
      ')',
    ),

    parenthesized_expression: $ => seq(
      '(',
      $.expression,
      ')',
    ),

    array: $ => seq(
      '[',
      optional(seq(
        $.expression,
        repeat(seq(',', $.expression)),
        optional(','),
      )),
      ']',
    ),

    map: $ => seq(
      '{',
      optional(seq(
        $.map_entry,
        repeat(seq(',', $.map_entry)),
        optional(','),
      )),
      '}',
    ),

    map_entry: $ => seq(
      field('key', choice($.string, $.identifier)),
      ':',
      field('value', $.expression),
    ),

    string: $ => choice(
      seq('"', repeat(choice($.string_content, $.interpolation)), '"'),
      seq("'", /[^']*/, "'"),
      seq('"""', /.*?"""/, '"""'),
    ),

    string_content: $ => token.immediate(prec(1, /[^"$\\]+/)),

    interpolation: $ => seq(
      '${',
      $.expression,
      '}',
    ),

    number: $ => token(choice(
      /\d+\.\d+([eE][+-]?\d+)?/,  // Float
      /\d+/,                       // Integer
      /0[xX][0-9a-fA-F]+/,        // Hex
      /0[oO][0-7]+/,              // Octal
      /0[bB][01]+/,               // Binary
    )),

    boolean: $ => choice('true', 'false'),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
  }
});
```

### 3. Generate the Parser

```bash
# Generate the parser
npx tree-sitter generate

# This creates:
# - src/parser.c (the generated parser)
# - src/tree_sitter/parser.h (header file)
# - src/node-types.json (node type definitions)
```

### 4. Create Test Cases

Create `test/corpus/basic.txt`:

```
==================
Variable declaration with let
==================

let x = 5

---

(source_file
  (variable_declaration
    name: (identifier)
    value: (number)))

==================
Function declaration
==================

fn greet(name: string) -> string {
    return "Hello"
}

---

(source_file
  (function_declaration
    name: (identifier)
    parameters: (parameter_list
      (parameter
        name: (identifier)
        type: (type)))
    return_type: (type)
    body: (block
      (return_statement
        (string)))))

==================
If statement
==================

if x > 5 {
    print("big")
}

---

(source_file
  (if_statement
    condition: (binary_expression
      left: (identifier)
      operator: ">"
      right: (number))
    consequence: (block
      (expression_statement
        (call_expression
          function: (identifier)
          arguments: (argument_list
            (string)))))))
```

### 5. Test the Grammar

```bash
# Run tests
npx tree-sitter test

# Parse a specific file
npx tree-sitter parse examples/test.shx

# Open in playground (web UI)
npx tree-sitter playground
```

## Understanding Tree-sitter

### How Tree-sitter Works

Tree-sitter builds a **concrete syntax tree** (CST) that includes:
- Every token in the source code
- All whitespace and comments
- Full structural information

This is different from an **abstract syntax tree** (AST) which omits details.

### Key Concepts

1. **Rules**: Define syntax patterns
   ```javascript
   rule_name: $ => seq('keyword', $.other_rule, 'punctuation')
   ```

2. **Precedence**: Control operator precedence
   ```javascript
   prec.left(5, seq($.left, '+', $.right))
   ```

3. **Associativity**: Left vs right associative
   - `prec.left()` - Left associative (most operators)
   - `prec.right()` - Right associative (e.g., `**`, assignment)

4. **Fields**: Name important parts of rules
   ```javascript
   field('name', $.identifier)
   ```

5. **Tokens**: Match text directly
   ```javascript
   token(/\d+/)  // Match one or more digits
   ```

## Shellux Grammar Structure

### Core Language Features to Implement

#### 1. Variables
```shellux
let x = 5
const MAX = 100
x := 10
name is "Alice"
```

#### 2. Functions
```shellux
fn add(a: int, b: int) -> int {
    return a + b
}
```

#### 3. Control Flow
```shellux
if condition {
    // ...
} else {
    // ...
}

for item in array {
    // ...
}

while condition {
    // ...
}

match value {
    0 => "zero",
    1 => "one",
    _ => "other"
}
```

#### 4. Expressions
```shellux
# Binary operators
a + b
a && b
a == b

# Pipeline operator
"hello" |> upper() |> trim()

# Member access
obj.property

# Array/Map indexing
arr[0]
map["key"]
```

#### 5. String Interpolation
```shellux
"Hello, ${name}!"
```

#### 6. Command Substitution
```shellux
files is $(ls -la)
```

#### 7. Comments
```shellux
# Single line comment
/* Multi-line
   comment */
```

## Step-by-Step Development

### Phase 1: Basic Structure (Day 1)

1. Variables and assignments
2. Simple expressions (literals, identifiers)
3. Comments
4. Basic operators (+, -, *, /)

### Phase 2: Functions (Day 2)

1. Function declarations
2. Parameters with types
3. Return types
4. Function calls
5. Return statements

### Phase 3: Control Flow (Day 3)

1. If/else statements
2. For loops
3. While loops
4. Break/continue
5. Match expressions

### Phase 4: Advanced Features (Day 4-5)

1. String interpolation
2. Command substitution
3. Pipeline operator
4. Member expressions
5. Array/Map literals
6. Try/catch blocks

### Phase 5: Polish (Day 6-7)

1. Error handling
2. Edge cases
3. Performance optimization
4. Comprehensive tests
5. Documentation

## Testing Your Grammar

### Running Tests

```bash
# Run all tests
npx tree-sitter test

# Run specific test file
npx tree-sitter test test/corpus/basic.txt

# Parse and print tree
npx tree-sitter parse examples/test.shx
```

### Writing Good Tests

Each test should:
1. Have a descriptive name
2. Show the input code
3. Show the expected tree structure

Format:
```
==================
Test Name
==================

input code here

---

(expected
  (tree structure))
```

### Debugging Tips

1. **Use the playground**: `npx tree-sitter playground`
2. **Check for conflicts**: Look for "conflict" in generator output
3. **Use `tree-sitter parse`**: See what tree your grammar produces
4. **Add debug logging**: Use `console.log()` in grammar.js (during development)

## Integration with Zed

### 1. Publish Your Grammar to GitHub

```bash
git init
git add .
git commit -m "Initial Shellux tree-sitter grammar"
git remote add origin https://github.com/yourusername/tree-sitter-shellux.git
git push -u origin main
```

### 2. Update the Zed Extension

Edit `shellux-syntax-zed/extension.toml`:

```toml
id = "shellux"
name = "Shellux"
version = "0.2.0"
schema_version = 1
authors = ["Your Name <your.email@example.com>"]
description = "Full syntax highlighting for Shellux scripting language"
repository = "https://github.com/yourusername/shellux"

[grammars.shellux]
repository = "https://github.com/yourusername/tree-sitter-shellux"
rev = "main"  # Or specific commit SHA
```

### 3. Create Tree-sitter Queries

Create `shellux-syntax-zed/languages/shellux/highlights.scm`:

```scheme
; Keywords
[
  "fn"
  "let"
  "const"
  "if"
  "else"
  "for"
  "while"
  "in"
  "match"
  "return"
  "break"
  "continue"
  "try"
  "catch"
] @keyword

; Functions
(function_declaration
  name: (identifier) @function)

(call_expression
  function: (identifier) @function)

; Types
(type) @type

; Strings
(string) @string

; Numbers
(number) @number

; Booleans
(boolean) @constant.builtin

; Comments
(comment) @comment

; Operators
[
  "+"
  "-"
  "*"
  "/"
  "%"
  "**"
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "&&"
  "||"
  "!"
  "|>"
] @operator

; Variables
(identifier) @variable

; Parameters
(parameter name: (identifier) @variable.parameter)
```

### 4. Test in Zed

```bash
# Install as dev extension
cd shellux-syntax-zed
# In Zed: Cmd+Shift+P → "zed: install dev extension" → select this directory

# Or install normally
./install.sh
```

## Resources

### Official Documentation

- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Creating Parsers](https://tree-sitter.github.io/tree-sitter/creating-parsers)
- [Zed Language Extensions](https://zed.dev/docs/extensions/languages)

### Example Grammars

Study these well-maintained grammars:

- [tree-sitter-javascript](https://github.com/tree-sitter/tree-sitter-javascript)
- [tree-sitter-python](https://github.com/tree-sitter/tree-sitter-python)
- [tree-sitter-rust](https://github.com/tree-sitter/tree-sitter-rust)
- [tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash)
- [tree-sitter-go](https://github.com/tree-sitter/tree-sitter-go)

### Tools

- [Tree-sitter Playground](http://tree-sitter.github.io/tree-sitter/playground) - Web-based testing
- [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) - Test in Neovim
- [Zed](https://zed.dev) - Test in Zed editor

### Community

- [Tree-sitter Discussions](https://github.com/tree-sitter/tree-sitter/discussions)
- [Zed Discord](https://discord.gg/zed)
- [Zed GitHub Discussions](https://github.com/zed-industries/zed/discussions)

## Common Issues and Solutions

### Issue: Conflicts in Grammar

**Problem**: Generator reports conflicts between rules.

**Solution**: 
- Use `prec()` to set precedence
- Restructure ambiguous rules
- Use negative lookahead if needed

### Issue: Slow Parsing

**Problem**: Grammar is slow on large files.

**Solution**:
- Avoid excessive backtracking
- Use `token()` for simple patterns
- Optimize repetitions with `repeat1()` vs `repeat()`

### Issue: Missing Nodes in Tree

**Problem**: Some code doesn't appear in parse tree.

**Solution**:
- Check `extras` - might be skipped as whitespace
- Ensure rule is included in parent rules
- Verify rule is actually matched (not skipped by choice)

### Issue: Integration with Zed Fails

**Problem**: Extension doesn't load in Zed.

**Solution**:
- Verify `extension.toml` format
- Check grammar repository is accessible
- Ensure `config.toml` references correct grammar name
- Look at Zed logs: Command Palette → "zed: open log"

## Next Steps

1. **Start Simple**: Begin with variables and basic expressions
2. **Test Frequently**: Run tests after each feature addition
3. **Iterate**: Don't try to be perfect on first pass
4. **Get Feedback**: Share with the community early
5. **Document**: Keep notes on design decisions

## Contributing

Once you have a working grammar:

1. Add it to the [tree-sitter organization](https://github.com/tree-sitter) (optional)
2. Add queries for other tools (nvim-treesitter, etc.)
3. Share with the Shellux community
4. Keep it maintained as Shellux evolves

---

**Good luck building the Shellux tree-sitter grammar!** 🌳✨

If you have questions, reach out to:
- Zed Discord: https://discord.gg/zed
- Tree-sitter discussions: https://github.com/tree-sitter/tree-sitter/discussions