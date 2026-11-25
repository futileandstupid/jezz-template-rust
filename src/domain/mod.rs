use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountId(String);

impl AccountId {
    pub fn parse(raw: &str) -> Result<Self, SettlementError> {
        let trimmed = raw.trim();
        if trimmed.len() < 3 || trimmed.len() > 64 {
            return Err(SettlementError::InvalidAccountId);
        }
        Ok(Self(trimmed.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for AccountId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for AccountId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::parse(&raw).map_err(de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementRequest {
    pub account_id: AccountId,
    pub gross_cents: i64,
    pub fee_bps: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementQuote {
    pub account_id: AccountId,
    pub gross_cents: i64,
    pub fee_cents: i64,
    pub net_cents: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettlementError {
    InvalidAccountId,
    OutOfRangeAmount,
    WireDecodeFailure(String),
}

impl fmt::Display for SettlementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAccountId => f.write_str("account_id length must be within 3..=64"),
            Self::OutOfRangeAmount => f.write_str("settlement cents or basis points out of range"),
            Self::WireDecodeFailure(detail) => write!(f, "wire JSON decode failed: {detail}"),
        }
    }
}

impl std::error::Error for SettlementError {}

/// @ensures result >= 0 and result <= gross_cents
pub fn net_after_fee(gross_cents: i64, fee_bps: i64) -> Result<i64, SettlementError> {
    if gross_cents < 1 || gross_cents > 1_000_000_000 || fee_bps < 0 || fee_bps > 10_000 {
        return Err(SettlementError::OutOfRangeAmount);
    }
    let fee_cents = (gross_cents * fee_bps) / 10_000;
    let net_cents = gross_cents - fee_cents;
    debug_assert!(net_cents >= 0 && net_cents <= gross_cents);
    Ok(net_cents)
}

pub fn build_settlement_quote(
    request: &SettlementRequest,
) -> Result<SettlementQuote, SettlementError> {
    let net_cents = net_after_fee(request.gross_cents, request.fee_bps)?;
    let fee_cents = request.gross_cents - net_cents;
    Ok(SettlementQuote {
        account_id: request.account_id.clone(),
        gross_cents: request.gross_cents,
        fee_cents,
        net_cents,
    })
}
