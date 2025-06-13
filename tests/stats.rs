use rust_challenge::model::Transfer;
use rust_challenge::stats::calculate_user_stats;

fn make_transfer(from: &str, to: &str, amount: f64, price: f64, ts: u64) -> Transfer {
    Transfer {
        ts,
        address_from: from.to_string(),
        address_to: to.to_string(),
        amount,
        usd_price: price,
    }
}

#[test]
fn test_empty() {
    let stats = calculate_user_stats(&[]);
    assert!(stats.is_empty());
}

#[test]
fn test_single_transfer() {
    let t = make_transfer("A", "B", 10.0, 2.0, 1);
    let stats = calculate_user_stats(&[t.clone()]);
    assert_eq!(stats.len(), 2);
    let a = stats.iter().find(|s| s.address == "A").unwrap();
    let b = stats.iter().find(|s| s.address == "B").unwrap();
    assert_eq!(a.total_volume + b.total_volume, 20.0);
    assert_eq!(a.avg_sell_price, 2.0);
    assert_eq!(b.avg_buy_price, 2.0);
}

#[test]
fn test_multiple_transfers() {
    let t1 = make_transfer("A", "B", 10.0, 2.0, 1);
    let t2 = make_transfer("B", "C", 5.0, 3.0, 2);
    let stats = calculate_user_stats(&[t1, t2]);
    assert_eq!(stats.len(), 3);
    let a = stats.iter().find(|s| s.address == "A").unwrap();
    let b = stats.iter().find(|s| s.address == "B").unwrap();
    let c = stats.iter().find(|s| s.address == "C").unwrap();
    assert!(a.total_volume > 0.0);
    assert!(b.total_volume > 0.0);
    assert!(c.total_volume > 0.0);
}

#[test]
fn test_same_address() {
    let t = make_transfer("A", "A", 100.0, 1.0, 1);
    let stats = calculate_user_stats(&[t]);
    let a = stats.iter().find(|s| s.address == "A").unwrap();
    assert_eq!(a.total_volume, 100.0);
    assert_eq!(a.avg_buy_price, 1.0);
    assert_eq!(a.avg_sell_price, 1.0);
}

#[test]
fn test_negative_and_zero_amounts() {
    let t1 = make_transfer("A", "B", 0.0, 1.0, 1);
    let t2 = make_transfer("B", "A", -5.0, 2.0, 2);
    let stats = calculate_user_stats(&[t1, t2]);
    let a = stats.iter().find(|s| s.address == "A").unwrap();
    let b = stats.iter().find(|s| s.address == "B").unwrap();
    assert!(a.total_volume >= 0.0);
    assert!(b.total_volume >= 0.0);
}

#[test]
fn test_large_values() {
    let t = make_transfer("A", "B", 1e12, 1e6, 1);
    let stats = calculate_user_stats(&[t]);
    let a = stats.iter().find(|s| s.address == "A").unwrap();
    let b = stats.iter().find(|s| s.address == "B").unwrap();
    assert!(a.total_volume > 0.0);
    assert!(b.total_volume > 0.0);
}
