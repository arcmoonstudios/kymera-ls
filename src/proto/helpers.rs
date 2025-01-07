// src/proto/helpers.rs
//! Helper functions for working with protobuf types.
//!
//! Maps each KymeraConstruct variant to a corresponding Rust, Python, TypeScript, JavaScript,
//! Java, Go, Ruby, C++, or C# construct (when applicable). If no equivalent construct exists,
//! returns `None`.

use super::generated::kymera_mappings::*;

/// Convert a Kymera construct to its Rust equivalent
pub fn kymera_to_rust(construct: KymeraConstruct) -> Option<RustConstruct> {
    match construct {
        KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT => Some(RustConstruct::RUST_UNKNOWN_CONSTRUCT),
        KymeraConstruct::des    => Some(RustConstruct::RUST_USE),
        KymeraConstruct::SPACS  => Some(RustConstruct::RUST_SCOPE),
        KymeraConstruct::forma  => Some(RustConstruct::RUST_STRUCT),
        KymeraConstruct::enum_  => Some(RustConstruct::RUST_ENUM),
        KymeraConstruct::imp    => Some(RustConstruct::RUST_IMPL),
        KymeraConstruct::fnc    => Some(RustConstruct::RUST_FN),
        KymeraConstruct::soy    => Some(RustConstruct::RUST_SELF),
        KymeraConstruct::SNC    => None, // No direct "sync" construct in Rust
        KymeraConstruct::XNC    => None, // No direct "async operator" in Rust (use async/await syntaxes)
        KymeraConstruct::SPRO   => Some(RustConstruct::RUST_AWAIT),
        KymeraConstruct::Res    => Some(RustConstruct::RUST_RESULT),
        KymeraConstruct::djq    => Some(RustConstruct::RUST_LET),
        KymeraConstruct::ret    => Some(RustConstruct::RUST_RETURN),
        KymeraConstruct::REV    => Some(RustConstruct::RUST_TRY), // 'rev' partially matches 'try' error propagation
        KymeraConstruct::wyo    => Some(RustConstruct::RUST_WHILE),
        KymeraConstruct::ate    => Some(RustConstruct::RUST_TRY), // 'ate' -> try block
        KymeraConstruct::MTH    => Some(RustConstruct::RUST_MATCH),
        KymeraConstruct::SPA    => Some(RustConstruct::RUST_FOR),
        KymeraConstruct::Optn   => Some(RustConstruct::RUST_OPTION),
        KymeraConstruct::Stilo  => Some(RustConstruct::RUST_STR),
        KymeraConstruct::Strng  => Some(RustConstruct::RUST_STRING),
        KymeraConstruct::MUTA   => Some(RustConstruct::RUST_MUT),
        KymeraConstruct::NMUT   => None, // Rust references are &T or &mut T; no direct "immutable" keyword
        KymeraConstruct::IDIT   => Some(RustConstruct::RUST_IDENT),
        KymeraConstruct::IFZ    => Some(RustConstruct::RUST_TRAIT),
        // Numeric types
        KymeraConstruct::i8     => Some(RustConstruct::RUST_I8),
        KymeraConstruct::i16    => Some(RustConstruct::RUST_I16),
        KymeraConstruct::i32    => Some(RustConstruct::RUST_I32),
        KymeraConstruct::i64    => Some(RustConstruct::RUST_I64),
        KymeraConstruct::i128   => Some(RustConstruct::RUST_I128),
        KymeraConstruct::ISZE   => Some(RustConstruct::RUST_ISIZE),
        KymeraConstruct::u8     => Some(RustConstruct::RUST_U8),
        KymeraConstruct::u16    => Some(RustConstruct::RUST_U16),
        KymeraConstruct::u32    => Some(RustConstruct::RUST_U32),
        KymeraConstruct::u64    => Some(RustConstruct::RUST_U64),
        KymeraConstruct::u128   => Some(RustConstruct::RUST_U128),
        KymeraConstruct::USZE   => Some(RustConstruct::RUST_USIZE),
        KymeraConstruct::f32    => Some(RustConstruct::RUST_F32),
        KymeraConstruct::f64    => Some(RustConstruct::RUST_F64),
        // Print / comment constructs
        KymeraConstruct::PRNT   => None, // No direct Rust macro enumerator for print (println! is separate)
        KymeraConstruct::CMT    => Some(RustConstruct::RUST_COMMENT),
        KymeraConstruct::BMT    => Some(RustConstruct::RUST_BLOCK_COMMENT),
        KymeraConstruct::DMT    => Some(RustConstruct::RUST_DOC_COMMENT),
        KymeraConstruct::AICG   => None, // AI code generation => no direct RustConstruct
        KymeraConstruct::VERX   => None, // Built-in AI debugger => no direct RustConstruct
    }
}

/// Convert a Kymera construct to its Python equivalent
pub fn kymera_to_python(construct: KymeraConstruct) -> Option<PythonConstruct> {
    match construct {
        KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT => Some(PythonConstruct::PYTHON_UNKNOWN_CONSTRUCT),
        KymeraConstruct::des    => Some(PythonConstruct::PYTHON_IMPORT),
        KymeraConstruct::SPACS  => Some(PythonConstruct::PYTHON_DOT),
        KymeraConstruct::forma  => Some(PythonConstruct::PYTHON_CLASS),
        KymeraConstruct::enum_  => Some(PythonConstruct::PYTHON_ENUM),
        KymeraConstruct::imp    => Some(PythonConstruct::PYTHON_DECORATOR), 
        KymeraConstruct::fnc    => Some(PythonConstruct::PYTHON_DEF),
        KymeraConstruct::soy    => Some(PythonConstruct::PYTHON_SELF),
        KymeraConstruct::SNC    => None, // No direct "sync" operator
        KymeraConstruct::XNC    => None, // Typically "async def", but not a single operator
        KymeraConstruct::SPRO   => Some(PythonConstruct::PYTHON_AWAIT),
        KymeraConstruct::Res    => None, // Python has no direct "Result" type
        KymeraConstruct::djq    => Some(PythonConstruct::PYTHON_ASSIGN),
        KymeraConstruct::ret    => Some(PythonConstruct::PYTHON_RETURN),
        KymeraConstruct::REV    => None, // Python uses "raise" or "except"
        KymeraConstruct::wyo    => Some(PythonConstruct::PYTHON_WHILE),
        KymeraConstruct::ate    => Some(PythonConstruct::PYTHON_TRY),
        KymeraConstruct::MTH    => Some(PythonConstruct::PYTHON_MATCH),
        KymeraConstruct::SPA    => Some(PythonConstruct::PYTHON_FOR),
        KymeraConstruct::Optn   => Some(PythonConstruct::PYTHON_OPTIONAL_TYPE),
        KymeraConstruct::Stilo  => Some(PythonConstruct::PYTHON_STR),
        KymeraConstruct::Strng  => Some(PythonConstruct::PYTHON_STRING),
        KymeraConstruct::MUTA   => None, // No direct "mut" in Python
        KymeraConstruct::NMUT   => None, // No direct "immutable" keyword in Python
        KymeraConstruct::IDIT   => Some(PythonConstruct::PYTHON_IDENTIFIER),
        KymeraConstruct::IFZ    => Some(PythonConstruct::PYTHON_PROTOCOL),
        // Numeric types (Python has int/float only, with arbitrary precision int)
        KymeraConstruct::i8     => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::i16    => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::i32    => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::i64    => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::i128   => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::ISZE   => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::u8     => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::u16    => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::u32    => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::u64    => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::u128   => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::USZE   => Some(PythonConstruct::PYTHON_INT),
        KymeraConstruct::f32    => Some(PythonConstruct::PYTHON_FLOAT),
        KymeraConstruct::f64    => Some(PythonConstruct::PYTHON_FLOAT),
        // Print / comment constructs
        KymeraConstruct::PRNT   => None, // No single "print statement" enum in PythonConstruct
        KymeraConstruct::CMT    => Some(PythonConstruct::PYTHON_COMMENT),
        KymeraConstruct::BMT    => Some(PythonConstruct::PYTHON_BLOCK_COMMENT),
        KymeraConstruct::DMT    => Some(PythonConstruct::PYTHON_DOCSTRING),
        KymeraConstruct::AICG   => None, 
        KymeraConstruct::VERX   => None,
    }
}

/// Convert a Kymera construct to its TypeScript equivalent
pub fn kymera_to_typescript(construct: KymeraConstruct) -> Option<TSConstruct> {
    match construct {
        KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT => Some(TSConstruct::TS_UNKNOWN_CONSTRUCT),
        KymeraConstruct::des    => Some(TSConstruct::TS_IMPORT),
        KymeraConstruct::SPACS  => Some(TSConstruct::TS_NAMESPACE),
        KymeraConstruct::forma  => Some(TSConstruct::TS_CLASS),
        KymeraConstruct::enum_  => Some(TSConstruct::TS_ENUM),
        KymeraConstruct::imp    => Some(TSConstruct::TS_IMPL),
        KymeraConstruct::fnc    => Some(TSConstruct::TS_FUNCTION),
        KymeraConstruct::soy    => Some(TSConstruct::TS_THIS),
        KymeraConstruct::SNC    => None, // Synchronous operator => no direct TS
        KymeraConstruct::XNC    => None, // "xn>" => no direct TS operator
        KymeraConstruct::SPRO   => Some(TSConstruct::TS_AWAIT),
        KymeraConstruct::Res    => Some(TSConstruct::TS_PROMISE), 
        KymeraConstruct::djq    => Some(TSConstruct::TS_LET),
        KymeraConstruct::ret    => Some(TSConstruct::TS_RETURN),
        KymeraConstruct::REV    => None, // No direct "error propagation"
        KymeraConstruct::wyo    => Some(TSConstruct::TS_WHILE),
        KymeraConstruct::ate    => Some(TSConstruct::TS_TRY),
        KymeraConstruct::MTH    => Some(TSConstruct::TS_SWITCH),
        KymeraConstruct::SPA    => Some(TSConstruct::TS_FOR),
        KymeraConstruct::Optn   => Some(TSConstruct::TS_OPTIONAL),
        KymeraConstruct::Stilo  => Some(TSConstruct::TS_STRING_LITERAL),
        KymeraConstruct::Strng  => Some(TSConstruct::TS_STRING),
        KymeraConstruct::MUTA   => None, 
        KymeraConstruct::NMUT   => None, 
        KymeraConstruct::IDIT   => Some(TSConstruct::TS_IDENTIFIER),
        KymeraConstruct::IFZ    => Some(TSConstruct::TS_INTERFACE),
        // Numeric types (TypeScript => number, bigint)
        KymeraConstruct::i8     => Some(TSConstruct::TS_NUMBER),
        KymeraConstruct::i16    => Some(TSConstruct::TS_NUMBER),
        KymeraConstruct::i32    => Some(TSConstruct::TS_NUMBER),
        KymeraConstruct::i64    => Some(TSConstruct::TS_BIGINT),
        KymeraConstruct::i128   => Some(TSConstruct::TS_BIGINT),
        KymeraConstruct::ISZE   => Some(TSConstruct::TS_NUMBER),
        KymeraConstruct::u8     => Some(TSConstruct::TS_NUMBER),
        KymeraConstruct::u16    => Some(TSConstruct::TS_NUMBER),
        KymeraConstruct::u32    => Some(TSConstruct::TS_NUMBER),
        KymeraConstruct::u64    => Some(TSConstruct::TS_BIGINT),
        KymeraConstruct::u128   => Some(TSConstruct::TS_BIGINT),
        KymeraConstruct::USZE   => Some(TSConstruct::TS_NUMBER),
        KymeraConstruct::f32    => Some(TSConstruct::TS_NUMBER),
        KymeraConstruct::f64    => Some(TSConstruct::TS_NUMBER),
        // Print / comment constructs
        KymeraConstruct::PRNT   => None,
        KymeraConstruct::CMT    => Some(TSConstruct::TS_COMMENT),
        KymeraConstruct::BMT    => Some(TSConstruct::TS_BLOCK_COMMENT),
        KymeraConstruct::DMT    => Some(TSConstruct::TS_DOC_COMMENT),
        KymeraConstruct::AICG   => None,
        KymeraConstruct::VERX   => None,
    }
}

/// Convert a Kymera construct to its JavaScript equivalent
pub fn kymera_to_javascript(construct: KymeraConstruct) -> Option<JSConstruct> {
    match construct {
        KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT => Some(JSConstruct::JS_UNKNOWN_CONSTRUCT),
        KymeraConstruct::des    => Some(JSConstruct::JS_IMPORT),
        KymeraConstruct::SPACS  => Some(JSConstruct::JS_DOT),
        KymeraConstruct::forma  => Some(JSConstruct::JS_CLASS),
        KymeraConstruct::enum_  => None, // JavaScript does not have native "enum" (ES6+ can mimic with const)
        KymeraConstruct::imp    => None, // No direct "implementation" block
        KymeraConstruct::fnc    => Some(JSConstruct::JS_FUNCTION),
        KymeraConstruct::soy    => Some(JSConstruct::JS_THIS),
        KymeraConstruct::SNC    => None,
        KymeraConstruct::XNC    => None,
        KymeraConstruct::SPRO   => Some(JSConstruct::JS_AWAIT),
        KymeraConstruct::Res    => Some(JSConstruct::JS_PROMISE),
        KymeraConstruct::djq    => Some(JSConstruct::JS_LET),
        KymeraConstruct::ret    => Some(JSConstruct::JS_RETURN),
        KymeraConstruct::REV    => None, 
        KymeraConstruct::wyo    => Some(JSConstruct::JS_WHILE),
        KymeraConstruct::ate    => Some(JSConstruct::JS_TRY),
        KymeraConstruct::MTH    => Some(JSConstruct::JS_SWITCH),
        KymeraConstruct::SPA    => Some(JSConstruct::JS_FOR),
        KymeraConstruct::Optn   => None, // JavaScript uses undefined or null
        KymeraConstruct::Stilo  => Some(JSConstruct::JS_STRING),
        KymeraConstruct::Strng  => Some(JSConstruct::JS_STRING),
        KymeraConstruct::MUTA   => None,
        KymeraConstruct::NMUT   => None,
        KymeraConstruct::IDIT   => Some(JSConstruct::JS_IDENTIFIER),
        KymeraConstruct::IFZ    => None, // No direct "interface" in JS (use TS or JSDoc)
        // Numeric types
        KymeraConstruct::i8
        | KymeraConstruct::i16
        | KymeraConstruct::i32
        | KymeraConstruct::ISZE
        | KymeraConstruct::u8
        | KymeraConstruct::u16
        | KymeraConstruct::u32
        | KymeraConstruct::USZE
        | KymeraConstruct::f32
        | KymeraConstruct::f64 => Some(JSConstruct::JS_NUMBER),
        KymeraConstruct::i64
        | KymeraConstruct::i128
        | KymeraConstruct::u64
        | KymeraConstruct::u128 => Some(JSConstruct::JS_BIGINT),
        // Print / comment constructs
        KymeraConstruct::PRNT   => None,
        KymeraConstruct::CMT    => Some(JSConstruct::JS_COMMENT),
        KymeraConstruct::BMT    => Some(JSConstruct::JS_BLOCK_COMMENT),
        KymeraConstruct::DMT    => Some(JSConstruct::JS_DOC_COMMENT),
        KymeraConstruct::AICG   => None,
        KymeraConstruct::VERX   => None,
    }
}

/// Convert a Kymera construct to its Java equivalent
pub fn kymera_to_java(construct: KymeraConstruct) -> Option<JavaConstruct> {
    match construct {
        KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT => Some(JavaConstruct::JAVA_UNKNOWN_CONSTRUCT),
        KymeraConstruct::des    => Some(JavaConstruct::JAVA_IMPORT),
        KymeraConstruct::SPACS  => Some(JavaConstruct::JAVA_DOT),
        KymeraConstruct::forma  => Some(JavaConstruct::JAVA_CLASS),
        KymeraConstruct::enum_  => Some(JavaConstruct::JAVA_ENUM),
        KymeraConstruct::imp    => Some(JavaConstruct::JAVA_IMPLEMENTS),
        KymeraConstruct::fnc    => Some(JavaConstruct::JAVA_FUNCTION),
        KymeraConstruct::soy    => Some(JavaConstruct::JAVA_THIS),
        KymeraConstruct::SNC    => None,
        KymeraConstruct::XNC    => None,
        KymeraConstruct::SPRO   => None, // No built-in "await" in standard Java
        KymeraConstruct::Res    => Some(JavaConstruct::JAVA_OPTIONAL),
        KymeraConstruct::djq    => Some(JavaConstruct::JAVA_VAR),
        KymeraConstruct::ret    => Some(JavaConstruct::JAVA_RETURN),
        KymeraConstruct::REV    => None,
        KymeraConstruct::wyo    => Some(JavaConstruct::JAVA_WHILE),
        KymeraConstruct::ate    => Some(JavaConstruct::JAVA_TRY),
        KymeraConstruct::MTH    => Some(JavaConstruct::JAVA_SWITCH),
        KymeraConstruct::SPA    => Some(JavaConstruct::JAVA_FOR),
        KymeraConstruct::Optn   => Some(JavaConstruct::JAVA_OPTIONAL_TYPE),
        KymeraConstruct::Stilo  => Some(JavaConstruct::JAVA_CHAR_SEQUENCE),
        KymeraConstruct::Strng  => Some(JavaConstruct::JAVA_STRING),
        KymeraConstruct::MUTA   => None,
        KymeraConstruct::NMUT   => None,
        KymeraConstruct::IDIT   => Some(JavaConstruct::JAVA_IDENTIFIER),
        KymeraConstruct::IFZ    => Some(JavaConstruct::JAVA_INTERFACE),
        // Numeric types
        KymeraConstruct::i8     => Some(JavaConstruct::JAVA_BYTE),
        KymeraConstruct::i16    => Some(JavaConstruct::JAVA_SHORT),
        KymeraConstruct::i32    => Some(JavaConstruct::JAVA_INT),
        KymeraConstruct::i64    => Some(JavaConstruct::JAVA_LONG),
        KymeraConstruct::i128   => Some(JavaConstruct::JAVA_BIGINTEGER),
        KymeraConstruct::ISZE   => Some(JavaConstruct::JAVA_INT),
        KymeraConstruct::u8
        | KymeraConstruct::u16
        | KymeraConstruct::u32
        | KymeraConstruct::u64
        | KymeraConstruct::u128
        | KymeraConstruct::USZE => Some(JavaConstruct::JAVA_BIGINTEGER),
        KymeraConstruct::f32
        | KymeraConstruct::f64  => None, // Could map to Java float or double, but not enumerated
        // Print / comment constructs
        KymeraConstruct::PRNT   => None,
        KymeraConstruct::CMT    => Some(JavaConstruct::JAVA_COMMENT),
        KymeraConstruct::BMT    => Some(JavaConstruct::JAVA_BLOCK_COMMENT),
        KymeraConstruct::DMT    => Some(JavaConstruct::JAVA_DOC_COMMENT),
        KymeraConstruct::AICG   => None,
        KymeraConstruct::VERX   => None,
    }
}

/// Convert a Kymera construct to its Go equivalent
pub fn kymera_to_go(construct: KymeraConstruct) -> Option<GoConstruct> {
    match construct {
        KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT => Some(GoConstruct::GO_UNKNOWN_CONSTRUCT),
        KymeraConstruct::des    => Some(GoConstruct::GO_IMPORT),
        KymeraConstruct::SPACS  => Some(GoConstruct::GO_DOT),
        KymeraConstruct::forma  => Some(GoConstruct::GO_STRUCT),
        KymeraConstruct::enum_  => Some(GoConstruct::GO_IOTA), // Go's typical enum-like pattern
        KymeraConstruct::imp    => Some(GoConstruct::GO_IMPLEMENTS),
        KymeraConstruct::fnc    => Some(GoConstruct::GO_FUNC),
        KymeraConstruct::soy    => Some(GoConstruct::GO_RECEIVER),
        KymeraConstruct::SNC    => None,
        KymeraConstruct::XNC    => None,
        KymeraConstruct::SPRO   => None, 
        KymeraConstruct::Res    => Some(GoConstruct::GO_ERROR), // Go uses error type commonly
        KymeraConstruct::djq    => Some(GoConstruct::GO_VAR),
        KymeraConstruct::ret    => Some(GoConstruct::GO_RETURN),
        KymeraConstruct::REV    => Some(GoConstruct::GO_RECOVER),
        KymeraConstruct::wyo    => Some(GoConstruct::GO_FOR), // while in Go is a for condition
        KymeraConstruct::ate    => Some(GoConstruct::GO_DEFER),
        KymeraConstruct::MTH    => Some(GoConstruct::GO_SWITCH),
        KymeraConstruct::SPA    => Some(GoConstruct::GO_RANGE),
        KymeraConstruct::Optn   => None, // No direct Option type in Go
        KymeraConstruct::Stilo  => Some(GoConstruct::GO_STRING),
        KymeraConstruct::Strng  => Some(GoConstruct::GO_STRING),
        KymeraConstruct::MUTA   => None,
        KymeraConstruct::NMUT   => None,
        KymeraConstruct::IDIT   => Some(GoConstruct::GO_IDENTIFIER),
        KymeraConstruct::IFZ    => Some(GoConstruct::GO_INTERFACE),
        // Numeric types
        KymeraConstruct::i8     => Some(GoConstruct::GO_INT8),
        KymeraConstruct::i16    => Some(GoConstruct::GO_INT16),
        KymeraConstruct::i32    => Some(GoConstruct::GO_INT32),
        KymeraConstruct::i64    => Some(GoConstruct::GO_INT64),
        KymeraConstruct::i128   => None, // Not standard in Go
        KymeraConstruct::ISZE   => Some(GoConstruct::GO_INT),
        KymeraConstruct::u8     => None, // Possibly byte or uint8, but not enumerated in GoConstruct
        KymeraConstruct::u16
        | KymeraConstruct::u32
        | KymeraConstruct::u64
        | KymeraConstruct::u128
        | KymeraConstruct::USZE => None, // Could be uint, but not enumerated
        KymeraConstruct::f32
        | KymeraConstruct::f64  => None, // Not enumerated (float32, float64 not in GoConstruct)
        // Print / comment constructs
        KymeraConstruct::PRNT   => None,
        KymeraConstruct::CMT    => Some(GoConstruct::GO_COMMENT),
        KymeraConstruct::BMT    => Some(GoConstruct::GO_BLOCK_COMMENT),
        KymeraConstruct::DMT    => Some(GoConstruct::GO_DOC_COMMENT),
        KymeraConstruct::AICG   => None,
        KymeraConstruct::VERX   => None,
    }
}

/// Convert a Kymera construct to its Ruby equivalent
pub fn kymera_to_ruby(construct: KymeraConstruct) -> Option<RubyConstruct> {
    match construct {
        KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT => Some(RubyConstruct::RUBY_UNKNOWN_CONSTRUCT),
        KymeraConstruct::des    => Some(RubyConstruct::RUBY_REQUIRE),
        KymeraConstruct::SPACS  => Some(RubyConstruct::RUBY_SCOPE),
        KymeraConstruct::forma  => Some(RubyConstruct::RUBY_CLASS),
        KymeraConstruct::enum_  => None, // Ruby doesn't have built-in enum
        KymeraConstruct::imp    => Some(RubyConstruct::RUBY_INCLUDE), // "include" modules
        KymeraConstruct::fnc    => Some(RubyConstruct::RUBY_DEF),
        KymeraConstruct::soy    => Some(RubyConstruct::RUBY_SELF),
        KymeraConstruct::SNC    => None,
        KymeraConstruct::XNC    => Some(RubyConstruct::RUBY_ASYNC), // Hypothetical for async libs
        KymeraConstruct::SPRO   => None,
        KymeraConstruct::Res    => Some(RubyConstruct::RUBY_MAYBE), // Rough match for optional
        KymeraConstruct::djq    => Some(RubyConstruct::RUBY_VAR),
        KymeraConstruct::ret    => Some(RubyConstruct::RUBY_RETURN),
        KymeraConstruct::REV    => None, // Ruby uses "rescue" or "raise"
        KymeraConstruct::wyo    => Some(RubyConstruct::RUBY_WHILE),
        KymeraConstruct::ate    => Some(RubyConstruct::RUBY_BEGIN), // begin/rescue block
        KymeraConstruct::MTH    => Some(RubyConstruct::RUBY_CASE),
        KymeraConstruct::SPA    => Some(RubyConstruct::RUBY_EACH),
        KymeraConstruct::Optn   => Some(RubyConstruct::RUBY_NILABLE),
        KymeraConstruct::Stilo  => Some(RubyConstruct::RUBY_STRING),
        KymeraConstruct::Strng  => Some(RubyConstruct::RUBY_STRING),
        KymeraConstruct::MUTA   => None,
        KymeraConstruct::NMUT   => None,
        KymeraConstruct::IDIT   => Some(RubyConstruct::RUBY_IDENTIFIER),
        KymeraConstruct::IFZ    => Some(RubyConstruct::RUBY_MODULE),
        // Numeric types
        KymeraConstruct::i8
        | KymeraConstruct::i16
        | KymeraConstruct::i32
        | KymeraConstruct::i64
        | KymeraConstruct::i128
        | KymeraConstruct::ISZE
        | KymeraConstruct::u8
        | KymeraConstruct::u16
        | KymeraConstruct::u32
        | KymeraConstruct::u64
        | KymeraConstruct::u128
        | KymeraConstruct::USZE => Some(RubyConstruct::RUBY_INTEGER),
        KymeraConstruct::f32
        | KymeraConstruct::f64  => Some(RubyConstruct::RUBY_FLOAT),
        // Print / comment constructs
        KymeraConstruct::PRNT   => None,
        KymeraConstruct::CMT    => Some(RubyConstruct::RUBY_COMMENT),
        KymeraConstruct::BMT    => Some(RubyConstruct::RUBY_BEGIN_END),
        KymeraConstruct::DMT    => Some(RubyConstruct::RUBY_RDOC),
        KymeraConstruct::AICG   => None,
        KymeraConstruct::VERX   => None,
    }
}

/// Convert a Kymera construct to its C++ equivalent
pub fn kymera_to_cpp(construct: KymeraConstruct) -> Option<CPPConstruct> {
    match construct {
        KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT => Some(CPPConstruct::CPP_UNKNOWN_CONSTRUCT),
        KymeraConstruct::des    => Some(CPPConstruct::CPP_INCLUDE),
        KymeraConstruct::SPACS  => Some(CPPConstruct::CPP_SCOPE),
        KymeraConstruct::forma  => Some(CPPConstruct::CPP_STRUCT),
        KymeraConstruct::enum_  => Some(CPPConstruct::CPP_ENUM),
        KymeraConstruct::imp    => Some(CPPConstruct::CPP_INHERITANCE), // Rough equivalence
        KymeraConstruct::fnc    => Some(CPPConstruct::CPP_FUNCTION),
        KymeraConstruct::soy    => Some(CPPConstruct::CPP_THIS),
        KymeraConstruct::SNC    => None,
        KymeraConstruct::XNC    => None,
        KymeraConstruct::SPRO   => Some(CPPConstruct::CPP_CO_AWAIT), // C++20 coroutines
        KymeraConstruct::Res    => Some(CPPConstruct::CPP_EXPECTED), // C++20 std::expected
        KymeraConstruct::djq    => Some(CPPConstruct::CPP_AUTO),
        KymeraConstruct::ret    => Some(CPPConstruct::CPP_RETURN),
        KymeraConstruct::REV    => None, 
        KymeraConstruct::wyo    => Some(CPPConstruct::CPP_WHILE),
        KymeraConstruct::ate    => Some(CPPConstruct::CPP_TRY),
        KymeraConstruct::MTH    => Some(CPPConstruct::CPP_SWITCH),
        KymeraConstruct::SPA    => Some(CPPConstruct::CPP_FOR),
        KymeraConstruct::Optn   => Some(CPPConstruct::CPP_OPTIONAL),
        KymeraConstruct::Stilo  => Some(CPPConstruct::CPP_STRING_VIEW),
        KymeraConstruct::Strng  => Some(CPPConstruct::CPP_STRING),
        KymeraConstruct::MUTA   => None,
        KymeraConstruct::NMUT   => None,
        KymeraConstruct::IDIT   => Some(CPPConstruct::CPP_IDENTIFIER),
        KymeraConstruct::IFZ    => Some(CPPConstruct::CPP_ABSTRACT), // Roughly for pure virtual
        // Numeric types
        KymeraConstruct::i8     => Some(CPPConstruct::CPP_INT8),
        KymeraConstruct::i16    => Some(CPPConstruct::CPP_INT16),
        KymeraConstruct::i32    => Some(CPPConstruct::CPP_INT32),
        KymeraConstruct::i64    => Some(CPPConstruct::CPP_INT64),
        KymeraConstruct::i128   => Some(CPPConstruct::CPP_INT128),
        KymeraConstruct::ISZE   => None, // Arch-dependent, could map to long or int64_t
        KymeraConstruct::u8
        | KymeraConstruct::u16
        | KymeraConstruct::u32
        | KymeraConstruct::u64
        | KymeraConstruct::u128
        | KymeraConstruct::USZE => None, // Not enumerated in CPPConstruct
        KymeraConstruct::f32
        | KymeraConstruct::f64  => None, // Not enumerated in CPPConstruct
        // Print / comment constructs
        KymeraConstruct::PRNT   => None,
        KymeraConstruct::CMT    => Some(CPPConstruct::CPP_COMMENT),
        KymeraConstruct::BMT    => Some(CPPConstruct::CPP_BLOCK_COMMENT),
        KymeraConstruct::DMT    => Some(CPPConstruct::CPP_DOC_COMMENT),
        KymeraConstruct::AICG   => None,
        KymeraConstruct::VERX   => None,
    }
}

/// Convert a Kymera construct to its C# equivalent
pub fn kymera_to_csharp(construct: KymeraConstruct) -> Option<CSharpConstruct> {
    match construct {
        KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT => Some(CSharpConstruct::CSHARP_UNKNOWN_CONSTRUCT),
        KymeraConstruct::des    => Some(CSharpConstruct::CSHARP_USING),
        KymeraConstruct::SPACS  => Some(CSharpConstruct::CSHARP_DOT),
        KymeraConstruct::forma  => Some(CSharpConstruct::CSHARP_CLASS),
        KymeraConstruct::enum_  => Some(CSharpConstruct::CSHARP_ENUM),
        KymeraConstruct::imp    => Some(CSharpConstruct::CSHARP_IMPLEMENTS),
        KymeraConstruct::fnc    => Some(CSharpConstruct::CSHARP_FUNCTION),
        KymeraConstruct::soy    => Some(CSharpConstruct::CSHARP_THIS),
        KymeraConstruct::SNC    => None,
        KymeraConstruct::XNC    => None, 
        KymeraConstruct::SPRO   => Some(CSharpConstruct::CSHARP_AWAIT),
        KymeraConstruct::Res    => Some(CSharpConstruct::CSHARP_TASK),
        KymeraConstruct::djq    => Some(CSharpConstruct::CSHARP_VAR),
        KymeraConstruct::ret    => Some(CSharpConstruct::CSHARP_RETURN),
        KymeraConstruct::REV    => None,
        KymeraConstruct::wyo    => Some(CSharpConstruct::CSHARP_WHILE),
        KymeraConstruct::ate    => Some(CSharpConstruct::CSHARP_TRY),
        KymeraConstruct::MTH    => Some(CSharpConstruct::CSHARP_SWITCH),
        KymeraConstruct::SPA    => Some(CSharpConstruct::CSHARP_FOREACH),
        KymeraConstruct::Optn   => Some(CSharpConstruct::CSHARP_NULLABLE),
        KymeraConstruct::Stilo  => Some(CSharpConstruct::CSHARP_SPAN), // If we treat Stilo as a slice-like
        KymeraConstruct::Strng  => Some(CSharpConstruct::CSHARP_STRING),
        KymeraConstruct::MUTA   => None,
        KymeraConstruct::NMUT   => None,
        KymeraConstruct::IDIT   => Some(CSharpConstruct::CSHARP_IDENTIFIER),
        KymeraConstruct::IFZ    => Some(CSharpConstruct::CSHARP_INTERFACE),
        // Numeric types
        KymeraConstruct::i8     => Some(CSharpConstruct::CSHARP_SBYTE),
        KymeraConstruct::i16    => Some(CSharpConstruct::CSHARP_SHORT),
        KymeraConstruct::i32    => Some(CSharpConstruct::CSHARP_INT),
        KymeraConstruct::i64    => Some(CSharpConstruct::CSHARP_LONG),
        KymeraConstruct::i128   => Some(CSharpConstruct::CSHARP_BIGINTEGER),
        KymeraConstruct::ISZE   => None, 
        KymeraConstruct::u8
        | KymeraConstruct::u16
        | KymeraConstruct::u32
        | KymeraConstruct::u64
        | KymeraConstruct::u128
        | KymeraConstruct::USZE => Some(CSharpConstruct::CSHARP_BIGINTEGER),
        KymeraConstruct::f32
        | KymeraConstruct::f64  => None, // Not explicitly in CSharpConstruct
        // Print / comment constructs
        KymeraConstruct::PRNT   => None,
        KymeraConstruct::CMT    => Some(CSharpConstruct::CSHARP_COMMENT),
        KymeraConstruct::BMT    => Some(CSharpConstruct::CSHARP_BLOCK_COMMENT),
        KymeraConstruct::DMT    => Some(CSharpConstruct::CSHARP_XML_DOC),
        KymeraConstruct::AICG   => None,
        KymeraConstruct::VERX   => None,
    }
}
