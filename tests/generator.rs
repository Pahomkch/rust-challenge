use rust_challenge::generator::{DefaultTransferGenerator, TransferGenConfig, TransferGenerator};

#[test]
fn test_generate_zero() {
    let gen = DefaultTransferGenerator::default();
    let transfers = gen.generate(0).unwrap();
    assert!(transfers.is_empty());
}

#[test]
fn test_generate_one() {
    let gen = DefaultTransferGenerator::default();
    let transfers = gen.generate(1).unwrap();
    assert_eq!(transfers.len(), 1);
}

#[test]
fn test_generate_many() {
    let gen = DefaultTransferGenerator::default();
    let transfers = gen.generate(1000).unwrap();
    assert_eq!(transfers.len(), 1000);
}

#[test]
fn test_amount_and_price_ranges() {
    let config = TransferGenConfig {
        min_amount: 10.0,
        max_amount: 20.0,
        min_price: 1.0,
        max_price: 2.0,
        max_age_secs: 100,
    };

    let gen = DefaultTransferGenerator { config };
    let transfers = gen.generate(100).unwrap();

    for t in &transfers {
        assert!(
            t.amount >= 10.0 && t.amount < 20.0,
            "amount out of range: {}",
            t.amount
        );

        assert!(
            t.usd_price >= 1.0 && t.usd_price < 2.0,
            "usd_price out of range: {}",
            t.usd_price
        );
    }
}

#[test]
fn test_min_equals_max_amount() {
    let config = TransferGenConfig {
        min_amount: 42.0,
        max_amount: 42.0,
        min_price: 1.0,
        max_price: 2.0,
        max_age_secs: 100,
    };

    let gen = DefaultTransferGenerator { config };
    let transfers = gen.generate(10).unwrap();

    for t in &transfers {
        assert_eq!(t.amount, 42.0);
    }
}

#[test]
fn test_min_equals_max_price() {
    let config = TransferGenConfig {
        min_amount: 1.0,
        max_amount: 2.0,
        min_price: 3.14,
        max_price: 3.14,
        max_age_secs: 100,
    };

    let gen = DefaultTransferGenerator { config };
    let transfers = gen.generate(10).unwrap();

    for t in &transfers {
        assert_eq!(t.usd_price, 3.14);
    }
}

#[test]
#[should_panic]
fn test_invalid_range_amount() {
    let config = TransferGenConfig {
        min_amount: 1000.0,
        max_amount: 5.0,
        min_price: 1.0,
        max_price: 2.0,
        max_age_secs: 100,
    };

    let gen = DefaultTransferGenerator { config };
    let _ = gen.generate(1).unwrap();
}

#[test]
#[should_panic]
fn test_invalid_range_price() {
    let config = TransferGenConfig {
        min_amount: 1.0,
        max_amount: 2.0,
        min_price: 10000.0,
        max_price: 5.0,
        max_age_secs: 100,
    };
    let gen = DefaultTransferGenerator { config };
    let _ = gen.generate(1);
}
