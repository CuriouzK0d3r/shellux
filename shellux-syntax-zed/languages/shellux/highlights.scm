; Shellux Syntax Highlighting for Zed Editor
; This file defines highlighting rules using Tree-sitter query syntax

; Comments
(comment) @comment

; Keywords
[
  "fn"
  "return"
  "if"
  "else"
  "for"
  "while"
  "in"
  "match"
  "try"
  "catch"
  "break"
  "continue"
] @keyword

; Declaration keywords
[
  "let"
  "const"
] @keyword.storage

; Operators as keywords
[
  "is"
  "as"
  "and"
  "or"
  "not"
] @keyword.operator

; Boolean constants
[
  "true"
  "false"
] @constant.builtin.boolean

; Null/nil
[
  "nil"
  "null"
] @constant.builtin

; Types
[
  "int"
  "float"
  "string"
  "bool"
  "any"
  "error"
  "map"
  "array"
] @type.builtin

; Function definitions
(function_definition
  name: (identifier) @function)

; Function calls
(call_expression
  function: (identifier) @function)

; Built-in functions
(identifier) @function.builtin
  (#match? @function.builtin "^(print|println|show|input|exit|env|set_env|len|contains|starts_with|ends_with|split|join|read_file|write_file|exists|is_file|is_dir|mkdir|rm|mv|cp|parse_json|default_config|join_path|duration|filter|map|sort|trim|lower|upper|range|walk_dir|spawn|kill|wait)$")

; Parameters
(parameter
  name: (identifier) @variable.parameter)

; Variables
(identifier) @variable

; Constants (uppercase identifiers)
((identifier) @constant
  (#match? @constant "^[A-Z][A-Z0-9_]*$"))

; Numbers
(integer) @constant.numeric.integer
(float) @constant.numeric.float
(hex_literal) @constant.numeric.hex
(octal_literal) @constant.numeric.octal
(binary_literal) @constant.numeric.binary

; Strings
(string) @string
(raw_string) @string
(multiline_string) @string

; String interpolation
(interpolation
  "${" @punctuation.special
  "}" @punctuation.special) @embedded

; Command substitution
(command_substitution
  "$(" @punctuation.special
  ")" @punctuation.special) @embedded

; Escape sequences
(escape_sequence) @constant.character.escape

; Operators
[
  "+"
  "-"
  "*"
  "/"
  "%"
  "**"
  "="
  ":="
  "+="
  "-="
  "*="
  "/="
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
  "&&"
  "||"
  "!"
  "&"
  "|"
  "^"
  "~"
  "<<"
  ">>"
  "|>"
] @operator

; Arrows
[
  "->"
  "=>"
] @punctuation.special

; Delimiters
[
  ","
  ";"
  ":"
  "."
] @punctuation.delimiter

; Brackets
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; Shebang
(shebang) @comment.special

; Type annotations
(type_annotation
  ":" @punctuation.special
  type: (type_identifier) @type)

; Return type
(return_type
  "->" @punctuation.special
  type: (type_identifier) @type)

; Errors (optional, for invalid syntax)
(ERROR) @error
