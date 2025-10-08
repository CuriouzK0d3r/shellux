# Show Command Implementation

## Overview
Added a `show` command that provides a simpler alternative to `print()` by allowing function calls without parentheses.

## Syntax

```shellux
# Traditional print with parentheses
print(x)
print("Hello, World!")

# New show command without parentheses
show x
show "Hello, World!"
```

## Implementation Details

### 1. Added Builtin Function
Modified `src/builtins/mod.rs`:
- Added "show" to the `register_builtins()` function
- Implemented `call_builtin()` case for "show" (identical to "print")
- Added "show" to the `is_builtin()` check

### 2. Parser Enhancement
Modified `src/parser/mod.rs`:
- Added `is_command_style_call()` method to detect command-style syntax
- Added `parse_command_style_call()` method to parse calls without parentheses
- Modified `parse_statement()` to check for command-style calls before parsing as expression statements

### 3. Parsing Logic
When the parser encounters a statement like `show x`:
1. Checks if it's an identifier followed by non-parenthesis tokens
2. Parses the identifier as a function name
3. Parses following tokens as arguments (using `parse_primary()`)
4. Creates an `Expr::Call` with the function name and arguments
5. Wraps it in `Stmt::Expression`

## Examples

```shellux
# Variables
x is 42
show x                  # Output: 42

# Literals
show "Hello"           # Output: Hello
show 123               # Output: 123
show true              # Output: true

# Expressions
result is 10 + 20
show result            # Output: 30

# Multiple arguments
show "Value:" 42       # Output: Value: 42

# String concatenation
name is "Alice"
greeting is "Hello, " + name
show greeting          # Output: Hello, Alice
```

## Equivalence
`show x` is functionally equivalent to `print(x)`. Both:
- Print the value to stdout
- Support multiple arguments
- Work with all value types
- Can be used anywhere in the code

## Benefits
1. **Simpler syntax** - No parentheses needed for simple cases
2. **Shell-like feel** - Matches command-line intuition
3. **Backward compatible** - Existing `print()` calls still work
4. **Flexible** - Can use either syntax based on preference

## Testing
Comprehensive tests verify:
- Basic variable display
- Literal values
- Expression results
- String concatenation
- Function return values
- Multiple arguments
- Boolean expressions
- Edge cases (0, empty string, false)
- Usage within functions
- Equivalence with print()

All tests pass successfully.
