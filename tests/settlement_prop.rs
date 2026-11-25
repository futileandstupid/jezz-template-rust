use jezz_template_rust::domain::{
    AccountId, SettlementRequest, build_settlement_quote, net_after_fee,
};
use jezz_template_rust::gateway::parse_and_quote_json;

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

#[test]
fn property_settlement_conservation_across_seeded_domain() {
    let mut seed: u64 = 0x5eed_1234_abcd_ef01;
    let account_id = AccountId::parse("acct_rs_01").expect("valid account id");
    for _ in 0..256 {
        let gross_cents = ((splitmix64(&mut seed) % 50_000_000) + 1) as i64;
        let fee_bps = (splitmix64(&mut seed) % 10_001) as i64;
        let req = SettlementRequest {
            account_id: account_id.clone(),
            gross_cents,
            fee_bps,
        };
        let quote = build_settlement_quote(&req).expect("seeded domain input must succeed");
        assert!(quote.net_cents >= 0 && quote.net_cents <= gross_cents);
        assert_eq!(quote.fee_cents + quote.net_cents, gross_cents);
        assert_eq!(quote.account_id.as_str(), "acct_rs_01");
        assert_eq!(
            net_after_fee(gross_cents, fee_bps).expect("valid domain input"),
            quote.net_cents
        );
    }
}

#[test]
fn gateway_accepts_canonical_json_and_rejects_unknown_fields() {
    let valid = r#"{"account_id":"acct_primary","gross_cents":10000,"fee_bps":250}"#;
    let quote = parse_and_quote_json(valid).expect("canonical wire payload must decode");
    assert_eq!(quote.fee_cents, 250);
    assert_eq!(quote.net_cents, 9_750);

    let spoofed =
        r#"{"account_id":"acct_primary","gross_cents":10000,"fee_bps":250,"extra":true}"#;
    assert!(parse_and_quote_json(spoofed).is_err());
}
