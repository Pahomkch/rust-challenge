use rust_challenge::model::{Transfer, UserStats};
use serde_json;

#[test]
fn test_transfer_creation() {
    let t = Transfer {
        ts: 123,
        address_from: "A".to_string(),
        address_to: "B".to_string(),
        amount: 42.0,
        usd_price: 1.5,
    };

    assert_eq!(t.ts, 123);
    assert_eq!(t.address_from, "A");
    assert_eq!(t.address_to, "B");
    assert_eq!(t.amount, 42.0);
    assert_eq!(t.usd_price, 1.5);
}

#[test]
fn test_transfer_zero_and_negative() {
    let t = Transfer {
        ts: 0,
        address_from: "".to_string(),
        address_to: "".to_string(),
        amount: 0.0,
        usd_price: -1.0,
    };

    assert_eq!(t.ts, 0);
    assert_eq!(t.amount, 0.0);
    assert_eq!(t.usd_price, -1.0);
}

#[test]
fn test_transfer_large_values() {
    let t = Transfer {
        ts: u64::MAX,
        address_from: "X".repeat(1000),
        address_to: "Y".repeat(1000),
        amount: f64::MAX,
        usd_price: f64::MAX,
    };

    assert_eq!(t.ts, u64::MAX);
    assert_eq!(t.amount, f64::MAX);
    assert_eq!(t.usd_price, f64::MAX);
}

#[test]
fn test_transfer_serde() {
    let t = Transfer {
        ts: 1,
        address_from: "A".to_string(),
        address_to: "B".to_string(),
        amount: 2.0,
        usd_price: 3.0,
    };

    let json = serde_json::to_string(&t).unwrap();
    let t2: Transfer = serde_json::from_str(&json).unwrap();
    assert_eq!(t.ts, t2.ts);
    assert_eq!(t.address_from, t2.address_from);
    assert_eq!(t.address_to, t2.address_to);
    assert_eq!(t.amount, t2.amount);
    assert_eq!(t.usd_price, t2.usd_price);
}

#[test]
fn test_user_stats_creation() {
    let s = UserStats {
        address: "A".to_string(),
        total_volume: 10.0,
        avg_buy_price: 1.0,
        avg_sell_price: 2.0,
        max_balance: 5.0,
    };

    assert_eq!(s.address, "A");
    assert_eq!(s.total_volume, 10.0);
    assert_eq!(s.avg_buy_price, 1.0);
    assert_eq!(s.avg_sell_price, 2.0);
    assert_eq!(s.max_balance, 5.0);
}

#[test]
fn test_user_stats_serde() {
    let s = UserStats {
        address: "A".to_string(),
        total_volume: 10.0,
        avg_buy_price: 1.0,
        avg_sell_price: 2.0,
        max_balance: 5.0,
    };

    let json = serde_json::to_string(&s).unwrap();
    let s2: UserStats = serde_json::from_str(&json).unwrap();
    assert_eq!(s.address, s2.address);
    assert_eq!(s.total_volume, s2.total_volume);
    assert_eq!(s.avg_buy_price, s2.avg_buy_price);
    assert_eq!(s.avg_sell_price, s2.avg_sell_price);
    assert_eq!(s.max_balance, s2.max_balance);
}
