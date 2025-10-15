; Shellux Syntax Highlighting for Zed Editor
; Using bash tree-sitter grammar

; Comments
(comment) @comment

; Keywords
[
  "if"
  "then"
  "else"
  "elif"
  "fi"
  "case"
  "esac"
  "for"
  "select"
  "while"
  "until"
  "do"
  "done"
  "in"
  "function"
] @keyword

; Control flow
[
  "break"
  "continue"
  "return"
] @keyword.control

; Declaration keywords
[
  "declare"
  "typeset"
  "export"
  "readonly"
  "local"
] @keyword.storage

; Commands
(command_name) @function

; Built-in commands
(command_name) @function.builtin
  (#match? @function.builtin "^(echo|printf|read|cd|pwd|pushd|popd|exit|return|eval|exec|source|test|shift|unset|alias|bg|fg|jobs|kill|wait|trap|let|set|unset)$")

; Function definitions
(function_definition
  name: (word) @function)

; Variables
(variable_name) @variable

; Special variables
(special_variable_name) @variable.special

; Variable expansions
(simple_expansion) @variable
(expansion
  "${" @punctuation.special
  "}" @punctuation.special)

; Command substitution
(command_substitution
  "$(" @punctuation.special
  ")" @punctuation.special)

(command_substitution
  "`" @punctuation.special)

; Process substitution
(process_substitution
  "<(" @punctuation.special
  ")" @punctuation.special)

(process_substitution
  ">(" @punctuation.special
  ")" @punctuation.special)

; Strings
(string) @string
(raw_string) @string
(ansii_c_string) @string

; String content
(string_content) @string

; Escape sequences
(escape_sequence) @constant.character.escape

; Numbers
(number) @constant.numeric

; Operators
[
  ";"
  ";;"
  ";&"
  ";;&"
  "&"
  "&&"
  "|"
  "||"
  "|&"
  "!"
] @operator

; Redirects
[
  "<"
  ">"
  ">>"
  "<<<"
  "<&"
  ">&"
  "&>"
  "&>>"
  "<>"
] @operator

; Pipes
"|" @operator

; Delimiters
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "[["
  "]]"
  "(("
  "))"
] @punctuation.bracket

; Test operators
(test_operator) @operator

; Binary expressions
(binary_expression
  operator: _ @operator)

; Unary expressions
(unary_expression
  operator: _ @operator)

; File descriptors
(file_redirect
  descriptor: (file_descriptor) @constant.numeric)

; Glob patterns
(expansion
  (subscript
    "[" @punctuation.bracket
    "]" @punctuation.bracket))

; Regex
(regex) @string.regex

; Words and literals
(word) @variable

; Concatenation
(concatenation) @string
