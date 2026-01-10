; ============================================================================
; MACROS (Noir-specific attribute system)
; ============================================================================

(macro) @function.macro
(macro (identifier) @function.macro)

; ============================================================================
; FUNCTIONS & METHODS
; ============================================================================

; Method calls (struct.method())
(struct_function
  (function_call (identifier) @function.method))

; Self method calls (self.method())
(self_method
  (function_call (identifier) @function.method))

; Function definitions
(function_definition
  (identifier) @function)

; Function calls
(function_call
  (identifier) @function)

; ============================================================================
; TYPES
; ============================================================================

; Built-in primitive types
"Field" @type.builtin
"bool" @type.builtin
"Self" @type.builtin

; Numeric types (u8, i8, u16, i16, u32, i32, u64, i64, u128, i128)
(single_type) @type.builtin

; Generic type parameters
(generic (identifier) @type.parameter)

; Generic types
(generic_type (identifier) @type)

; Array types
(array_type) @type

; Struct definitions
(struct_definition
  name: (identifier) @type)

; Type annotations in typed identifiers
(typed_identifier
  type: (identifier) @type)

; Return types
(return_type (identifier) @type)

; ============================================================================
; STRUCT FIELDS & PROPERTIES
; ============================================================================

; Struct field definitions
(struct_definition
  var: (identifier) @property)

; Struct field access (dot notation)
(struct_expression
  (identifier) @property)

; Struct initialization fields
(struct_initialization
  var: (identifier) @property)

; ============================================================================
; VARIABLES & PARAMETERS
; ============================================================================

; Let declarations
(let_declaration
  (identifier) @variable)

; Typed identifier variables
(typed_identifier
  var: (identifier) @variable)

; Function parameters
(parameter
  (typed_identifier
    var: (identifier) @variable.parameter))

; Self reference
(self) @variable.builtin

; ============================================================================
; CONSTANTS (ALL_CAPS naming convention)
; ============================================================================

((identifier) @constant
 (#match? @constant "^[A-Z][A-Z\\d_]+$"))

; ============================================================================
; KEYWORDS
; ============================================================================

; Special Noir keywords for assertions (important for ZK proofs)
(assert) @keyword

; Compile-time evaluation
(comptime) @keyword

; Control flow
"if" @keyword
"else" @keyword
"for" @keyword
"in" @keyword
(return) @keyword

; Definitions
"fn" @keyword
"struct" @keyword
"impl" @keyword
"mod" @keyword
"use" @keyword
"let" @keyword
"as" @keyword
"global" @keyword

; Mutability
(mutable) @keyword
(viewer) @keyword

; ============================================================================
; MODULES & IMPORTS
; ============================================================================

; Special module path keywords
(crate) @namespace
(super) @namespace

; Import paths
(import_identifier (identifier) @namespace)
(import_variable (identifier) @namespace)

; ============================================================================
; OPERATORS
; ============================================================================

; Arithmetic operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator

; Comparison operators
"==" @operator
"!=" @operator
"<" @operator
"<=" @operator
">" @operator
">=" @operator

; Logical operators
"&&" @operator
"||" @operator
"!" @operator

; Bitwise operators
"&" @operator
"|" @operator
"^" @operator
"<<" @operator
">>" @operator

; Assignment operators
"=" @operator
"+=" @operator
"-=" @operator
"&=" @operator
"|=" @operator

; Range operator
".." @operator

; ============================================================================
; LITERALS
; ============================================================================

; Strings and characters
(string_literal) @string
(character) @string

; Numbers
(integer) @number
(float) @number

; Booleans
(boolean) @constant.builtin

; ============================================================================
; COMMENTS
; ============================================================================

(comment) @comment

; ============================================================================
; PUNCTUATION
; ============================================================================

; Brackets
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

; Generic angle brackets
(generic
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

; Delimiters
"::" @punctuation.delimiter
":" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter
"->" @punctuation.delimiter
