use crate::domain::{
    SettlementError, SettlementQuote, SettlementRequest, build_settlement_quote,
};

pub fn parse_and_quote_json(raw_json: &str) -> Result<SettlementQuote, SettlementError> {
    if raw_json.trim().len() < 2 || raw_json.len() > 8_192 {
        return Err(SettlementError::WireDecodeFailure(
            "payload length outside permitted bounds".to_owned(),
        ));
    }
    let request = serde_json::from_str::<SettlementRequest>(raw_json)
        .map_err(|err| SettlementError::WireDecodeFailure(err.to_string()))?;
    build_settlement_quote(&request)
}
