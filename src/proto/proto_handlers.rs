//! src/proto_handlers.rs
//! Protobuf handler for Kymera.

use crate::proto::KymeraConstruct;
use std::collections::HashMap;

/// Represents a handler for Protobuf-related operations.
pub struct ProtoHandler {
    construct_map: HashMap<String, KymeraConstruct>,
    symbol_map: HashMap<String, KymeraConstruct>,
}

impl ProtoHandler {
    /// Creates a new `ProtoHandler`.
    pub fn new() -> Self {
        let construct_map = Self::build_construct_map();
        let symbol_map = Self::build_symbol_map();
        Self {
            construct_map,
            symbol_map,
        }
    }

    /// Builds a mapping from string representations to `KymeraConstruct` values.
    fn build_construct_map() -> HashMap<String, KymeraConstruct> {
        let mut map = HashMap::new();
        map.insert("KYMERA_UNKNOWN_CONSTRUCT".to_string(), KymeraConstruct::KYMERA_UNKNOWN_CONSTRUCT);
        map.insert("des".to_string(), KymeraConstruct::des);
        map.insert("SPACS".to_string(), KymeraConstruct::SPACS);
        map.insert("forma".to_string(), KymeraConstruct::forma);
        map.insert("enum".to_string(), KymeraConstruct::enum_);
        map.insert("imp".to_string(), KymeraConstruct::imp);
        map.insert("fnc".to_string(), KymeraConstruct::fnc);
        map.insert("soy".to_string(), KymeraConstruct::soy);
        map.insert("SNC".to_string(), KymeraConstruct::SNC);
        map.insert("XNC".to_string(), KymeraConstruct::XNC);
        map.insert("SPRO".to_string(), KymeraConstruct::SPRO);
        map.insert("Res".to_string(), KymeraConstruct::Res);
        map.insert("djq".to_string(), KymeraConstruct::djq);
        map.insert("ret".to_string(), KymeraConstruct::ret);
        map.insert("REV".to_string(), KymeraConstruct::REV);
        map.insert("wyo".to_string(), KymeraConstruct::wyo);
        map.insert("ate".to_string(), KymeraConstruct::ate);
        map.insert("MTH".to_string(), KymeraConstruct::MTH);
        map.insert("SPA".to_string(), KymeraConstruct::SPA);
        map.insert("Optn".to_string(), KymeraConstruct::Optn);
        map.insert("Stilo".to_string(), KymeraConstruct::Stilo);
        map.insert("Strng".to_string(), KymeraConstruct::Strng);
        map.insert("MUTA".to_string(), KymeraConstruct::MUTA);
        map.insert("NMUT".to_string(), KymeraConstruct::NMUT);
        map.insert("IDIT".to_string(), KymeraConstruct::IDIT);
        map.insert("IFZ".to_string(), KymeraConstruct::IFZ);
        map.insert("i8".to_string(), KymeraConstruct::i8);
        map.insert("i16".to_string(), KymeraConstruct::i16);
        map.insert("i32".to_string(), KymeraConstruct::i32);
        map.insert("i64".to_string(), KymeraConstruct::i64);
        map.insert("i128".to_string(), KymeraConstruct::i128);
        map.insert("ISZE".to_string(), KymeraConstruct::ISZE);
        map.insert("u8".to_string(), KymeraConstruct::u8);
        map.insert("u16".to_string(), KymeraConstruct::u16);
        map.insert("u32".to_string(), KymeraConstruct::u32);
        map.insert("u64".to_string(), KymeraConstruct::u64);
        map.insert("u128".to_string(), KymeraConstruct::u128);
        map.insert("USZE".to_string(), KymeraConstruct::USZE);
        map.insert("f32".to_string(), KymeraConstruct::f32);
        map.insert("f64".to_string(), KymeraConstruct::f64);
        map.insert("PRNT".to_string(), KymeraConstruct::PRNT);
        map.insert("CMT".to_string(), KymeraConstruct::CMT);
        map.insert("BMT".to_string(), KymeraConstruct::BMT);
        map.insert("DMT".to_string(), KymeraConstruct::DMT);
        map.insert("AICG".to_string(), KymeraConstruct::AICG);
        map.insert("VERX".to_string(), KymeraConstruct::VERX);
        map
    }

    /// Builds a mapping from symbols to `KymeraConstruct` values.
    fn build_symbol_map() -> HashMap<String, KymeraConstruct> {
        let mut map = HashMap::new();
        map.insert(":>".to_string(), KymeraConstruct::SPACS); // Scope resolution operator
        map.insert("m>".to_string(), KymeraConstruct::MTH);   // Match statement
        map.insert("4>".to_string(), KymeraConstruct::SPA);   // For loop
        map.insert("~".to_string(), KymeraConstruct::MUTA);   // Mutable designator
        map.insert("&".to_string(), KymeraConstruct::NMUT);   // Immutable designator
        map.insert("<id?>".to_string(), KymeraConstruct::IDIT); // Identifier
        map.insert("[=-]".to_string(), KymeraConstruct::IFZ);  // Interface definition
        map.insert("|>".to_string(), KymeraConstruct::CMT);    // Line comment
        map.insert("|D>".to_string(), KymeraConstruct::DMT);   // Documentation comment
        map.insert("|A>".to_string(), KymeraConstruct::AICG);  // AI-assisted code generation
        map.insert("<v?x>".to_string(), KymeraConstruct::VERX); // VERX debugger
        map
    }

    /// Parses a string into a `KymeraConstruct`.
    ///
    /// # Arguments
    /// * `text` - The string representation of the construct (either name or symbol).
    ///
    /// # Returns
    /// * `Some(KymeraConstruct)` if the text matches a valid construct.
    /// * `None` if the text does not match any construct.
    pub fn parse_construct(&self, text: &str) -> Option<KymeraConstruct> {
        // Try to match the text as a symbol first
        if let Some(construct) = self.symbol_map.get(text) {
            return Some(*construct);
        }
        // If no symbol match, try to match the text as a construct name
        self.construct_map.get(text).cloned()
    }

    /// Returns a list of all supported `KymeraConstruct` values.
    pub fn list_constructs(&self) -> Vec<KymeraConstruct> {
        self.construct_map.values().cloned().collect()
    }

    /// Returns a list of all supported symbols and their corresponding constructs.
    pub fn list_symbols(&self) -> Vec<(String, KymeraConstruct)> {
        self.symbol_map
            .iter()
            .map(|(symbol, construct)| (symbol.clone(), *construct))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_construct() {
        let handler = ProtoHandler::new();

        // Test valid constructs by name
        assert_eq!(handler.parse_construct("des"), Some(KymeraConstruct::des));
        assert_eq!(handler.parse_construct("fnc"), Some(KymeraConstruct::fnc));
        assert_eq!(handler.parse_construct("i32"), Some(KymeraConstruct::i32));
        assert_eq!(handler.parse_construct("VERX"), Some(KymeraConstruct::VERX));

        // Test valid constructs by symbol
        assert_eq!(handler.parse_construct("m>"), Some(KymeraConstruct::MTH));
        assert_eq!(handler.parse_construct("~"), Some(KymeraConstruct::MUTA));
        assert_eq!(handler.parse_construct("|>"), Some(KymeraConstruct::CMT));
        assert_eq!(handler.parse_construct("<v?x>"), Some(KymeraConstruct::VERX));

        // Test invalid constructs
        assert_eq!(handler.parse_construct("unknown"), None);
        assert_eq!(handler.parse_construct(""), None);
    }

    #[test]
    fn test_list_constructs() {
        let handler = ProtoHandler::new();
        let constructs = handler.list_constructs();

        // Ensure all constructs are included
        assert!(constructs.contains(&KymeraConstruct::des));
        assert!(constructs.contains(&KymeraConstruct::fnc));
        assert!(constructs.contains(&KymeraConstruct::i32));
        assert!(constructs.contains(&KymeraConstruct::VERX));
    }

    #[test]
    fn test_list_symbols() {
        let handler = ProtoHandler::new();
        let symbols = handler.list_symbols();

        // Ensure all symbols are included
        assert!(symbols.contains(&("m>".to_string(), KymeraConstruct::MTH)));
        assert!(symbols.contains(&("~".to_string(), KymeraConstruct::MUTA)));
        assert!(symbols.contains(&("|>".to_string(), KymeraConstruct::CMT)));
        assert!(symbols.contains(&("<v?x>".to_string(), KymeraConstruct::VERX)));
    }
}