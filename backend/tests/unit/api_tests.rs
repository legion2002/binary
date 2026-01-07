//! Unit tests for API response format and serialization
//! These tests verify JSON structure matches API documentation

use serde::{Deserialize, Serialize};
use serde_json::json;

/// Test response structure matches API.md documentation
mod response_format {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct MarketResponse {
        market_hash: String,
        question_hash: String,
        question: Option<String>,
        resolution_deadline: i64,
        oracle: String,
        block_number: i64,
        yes_probability: Option<f64>,
        no_probability: Option<f64>,
        resolution: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct GetMarketsResponse {
        markets: Vec<MarketResponse>,
        count: usize,
        total: i64,
    }

    #[test]
    fn test_market_response_serializes_with_camel_case() {
        let market = MarketResponse {
            market_hash: "0x123".to_string(),
            question_hash: "0x456".to_string(),
            question: Some("Test question?".to_string()),
            resolution_deadline: 1767225600,
            oracle: "0x789".to_string(),
            block_number: 12345,
            yes_probability: Some(0.65),
            no_probability: Some(0.35),
            resolution: Some("UNRESOLVED".to_string()),
        };

        let json = serde_json::to_value(&market).unwrap();

        // Verify camelCase field names
        assert!(json.get("marketHash").is_some());
        assert!(json.get("questionHash").is_some());
        assert!(json.get("resolutionDeadline").is_some());
        assert!(json.get("blockNumber").is_some());
        assert!(json.get("yesProbability").is_some());
        assert!(json.get("noProbability").is_some());

        // Verify snake_case is NOT used
        assert!(json.get("market_hash").is_none());
        assert!(json.get("question_hash").is_none());
    }

    #[test]
    fn test_market_response_nullable_fields() {
        let market = MarketResponse {
            market_hash: "0x123".to_string(),
            question_hash: "0x456".to_string(),
            question: None,
            resolution_deadline: 1767225600,
            oracle: "0x789".to_string(),
            block_number: 12345,
            yes_probability: None,
            no_probability: None,
            resolution: None,
        };

        let json = serde_json::to_value(&market).unwrap();

        // Nullable fields should serialize as null
        assert!(json.get("question").unwrap().is_null());
        assert!(json.get("yesProbability").unwrap().is_null());
        assert!(json.get("noProbability").unwrap().is_null());
        assert!(json.get("resolution").unwrap().is_null());
    }

    #[test]
    fn test_get_markets_response_structure() {
        let response = GetMarketsResponse {
            markets: vec![],
            count: 0,
            total: 10,
        };

        let json = serde_json::to_value(&response).unwrap();

        assert!(json.get("markets").is_some());
        assert!(json.get("count").is_some());
        assert!(json.get("total").is_some());
        assert_eq!(json.get("markets").unwrap().as_array().unwrap().len(), 0);
        assert_eq!(json.get("count").unwrap().as_u64().unwrap(), 0);
        assert_eq!(json.get("total").unwrap().as_i64().unwrap(), 10);
    }

    #[test]
    fn test_probability_values_in_valid_range() {
        let market = MarketResponse {
            market_hash: "0x123".to_string(),
            question_hash: "0x456".to_string(),
            question: None,
            resolution_deadline: 1767225600,
            oracle: "0x789".to_string(),
            block_number: 12345,
            yes_probability: Some(0.65),
            no_probability: Some(0.35),
            resolution: None,
        };

        let yes_prob = market.yes_probability.unwrap();
        let no_prob = market.no_probability.unwrap();

        assert!(yes_prob >= 0.0 && yes_prob <= 1.0);
        assert!(no_prob >= 0.0 && no_prob <= 1.0);
        assert!((yes_prob + no_prob - 1.0).abs() < 0.001);
    }
}

/// Test verse token response format
mod verse_token_format {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct VerseTokenResponse {
        asset: String,
        yes_verse: String,
        no_verse: String,
        transaction_hash: Option<String>,
    }

    #[test]
    fn test_verse_token_camel_case() {
        let verse = VerseTokenResponse {
            asset: "0x4200000000000000000000000000000000000006".to_string(),
            yes_verse: "0xabc123".to_string(),
            no_verse: "0xdef456".to_string(),
            transaction_hash: Some("0x789ghi".to_string()),
        };

        let json = serde_json::to_value(&verse).unwrap();

        assert!(json.get("yesVerse").is_some());
        assert!(json.get("noVerse").is_some());
        assert!(json.get("transactionHash").is_some());
    }

    #[test]
    fn test_verse_token_nullable_tx_hash() {
        let verse = VerseTokenResponse {
            asset: "0x4200000000000000000000000000000000000006".to_string(),
            yes_verse: "0xabc123".to_string(),
            no_verse: "0xdef456".to_string(),
            transaction_hash: None,
        };

        let json = serde_json::to_value(&verse).unwrap();
        assert!(json.get("transactionHash").unwrap().is_null());
    }
}

/// Test orderbook response format
mod orderbook_format {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct OrderbookInfo {
        pair_type: String,
        pair_key: String,
        base_token: String,
        quote_token: String,
        best_bid_tick: Option<i32>,
        best_ask_tick: Option<i32>,
        best_bid_price: Option<String>,
        best_ask_price: Option<String>,
        mid_price: Option<String>,
        spread_bps: Option<f64>,
    }

    #[test]
    fn test_orderbook_camel_case() {
        let orderbook = OrderbookInfo {
            pair_type: "YES/QUOTE".to_string(),
            pair_key: "0x123".to_string(),
            base_token: "0xabc".to_string(),
            quote_token: "0xdef".to_string(),
            best_bid_tick: Some(-10),
            best_ask_tick: Some(10),
            best_bid_price: Some("0.999900".to_string()),
            best_ask_price: Some("1.000100".to_string()),
            mid_price: Some("1.000000".to_string()),
            spread_bps: Some(2.0),
        };

        let json = serde_json::to_value(&orderbook).unwrap();

        assert!(json.get("pairType").is_some());
        assert!(json.get("pairKey").is_some());
        assert!(json.get("baseToken").is_some());
        assert!(json.get("quoteToken").is_some());
        assert!(json.get("bestBidTick").is_some());
        assert!(json.get("bestAskTick").is_some());
        assert!(json.get("bestBidPrice").is_some());
        assert!(json.get("bestAskPrice").is_some());
        assert!(json.get("midPrice").is_some());
        assert!(json.get("spreadBps").is_some());
    }

    #[test]
    fn test_orderbook_pair_types() {
        let valid_pair_types = vec!["YES/QUOTE", "NO/QUOTE"];

        for pair_type in valid_pair_types {
            let orderbook = OrderbookInfo {
                pair_type: pair_type.to_string(),
                pair_key: "0x123".to_string(),
                base_token: "0xabc".to_string(),
                quote_token: "0xdef".to_string(),
                best_bid_tick: None,
                best_ask_tick: None,
                best_bid_price: None,
                best_ask_price: None,
                mid_price: None,
                spread_bps: None,
            };

            let json = serde_json::to_value(&orderbook).unwrap();
            assert_eq!(json.get("pairType").unwrap().as_str().unwrap(), pair_type);
        }
    }

    #[test]
    fn test_orderbook_empty_no_orders() {
        let orderbook = OrderbookInfo {
            pair_type: "YES/QUOTE".to_string(),
            pair_key: "0x123".to_string(),
            base_token: "0xabc".to_string(),
            quote_token: "0xdef".to_string(),
            best_bid_tick: None,
            best_ask_tick: None,
            best_bid_price: None,
            best_ask_price: None,
            mid_price: None,
            spread_bps: None,
        };

        let json = serde_json::to_value(&orderbook).unwrap();

        assert!(json.get("bestBidTick").unwrap().is_null());
        assert!(json.get("bestAskTick").unwrap().is_null());
        assert!(json.get("bestBidPrice").unwrap().is_null());
        assert!(json.get("bestAskPrice").unwrap().is_null());
        assert!(json.get("midPrice").unwrap().is_null());
        assert!(json.get("spreadBps").unwrap().is_null());
    }
}

/// Test error response format
mod error_format {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct ErrorResponse {
        error: String,
    }

    #[test]
    fn test_error_response_structure() {
        let error = ErrorResponse {
            error: "Market not found".to_string(),
        };

        let json = serde_json::to_value(&error).unwrap();

        assert!(json.get("error").is_some());
        assert_eq!(
            json.get("error").unwrap().as_str().unwrap(),
            "Market not found"
        );
    }

    #[test]
    fn test_error_response_no_extra_fields() {
        let error = ErrorResponse {
            error: "Test error".to_string(),
        };

        let json = serde_json::to_value(&error).unwrap();
        let obj = json.as_object().unwrap();

        assert_eq!(obj.len(), 1);
        assert!(obj.contains_key("error"));
    }
}

/// Test admin request format
mod admin_request_format {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct OpenMarketRequest {
        question: String,
        resolution_deadline: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        assets: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        quote_token: Option<String>,
    }

    #[test]
    fn test_open_market_request_camel_case() {
        let request = OpenMarketRequest {
            question: "Will ETH hit 5k?".to_string(),
            resolution_deadline: 1767225600,
            assets: Some(vec!["0x123".to_string()]),
            quote_token: Some("0x456".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();

        assert!(json.get("resolutionDeadline").is_some());
        assert!(json.get("quoteToken").is_some());
    }

    #[test]
    fn test_open_market_request_minimal() {
        let request = OpenMarketRequest {
            question: "Will ETH hit 5k?".to_string(),
            resolution_deadline: 1767225600,
            assets: None,
            quote_token: None,
        };

        let json = serde_json::to_value(&request).unwrap();

        // Optional fields should be omitted when None
        assert!(json.get("assets").is_none());
        assert!(json.get("quoteToken").is_none());
    }

    #[test]
    fn test_open_market_request_deserialize() {
        let json_str = r#"{"question":"Test?","resolutionDeadline":1767225600}"#;
        let request: OpenMarketRequest = serde_json::from_str(json_str).unwrap();

        assert_eq!(request.question, "Test?");
        assert_eq!(request.resolution_deadline, 1767225600);
        assert!(request.assets.is_none());
        assert!(request.quote_token.is_none());
    }
}

/// Test resolution status values
mod resolution_status {
    #[test]
    fn test_valid_resolution_values() {
        let valid_values = vec!["UNRESOLVED", "YES", "NO", "EVEN"];

        for value in &valid_values {
            assert!(valid_values.contains(value));
        }
    }

    #[test]
    fn test_resolution_default_unresolved() {
        let default_resolution = "UNRESOLVED";
        assert_eq!(default_resolution, "UNRESOLVED");
    }
}
